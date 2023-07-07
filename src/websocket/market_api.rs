//! The interaction systems for the websocket User API.

use std::sync::Arc;

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use futures_util::future::Either;
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::api_request::ApiRequestBuilder;
use crate::api_response::ApiResponse;
use crate::error::{convert_tungstenite_error, processing_error};
use crate::prelude::{ApiError, DataSender, MessageSender};
use crate::utils::action::ActionStore;
use crate::utils::config::Config;
use crate::utils::{message_to_api_response, reprocess_data};
use crate::websocket::data::{
    book::{BookRes, RawBookRes},
    candlestick::{CandlestickRes, RawCandlestickRes},
    otc_book::{OtcBookRes, RawOtcBookRes},
    ticker::{RawTickerRes, TickerRes},
    trade::{RawTradeRes, TradeRes},
    RawRes,
};
use crate::websocket::{respond_heartbeat, WebsocketData};

/// Parameters of the subscription request.
#[derive(Serialize, Debug)]
pub struct SubscribeParams {
    channels: Vec<String>,
}

/// Handle the actions that are to be pushed to the server from [`crate::controller::Controller::push_market_action`]
///
/// # Errors
///
/// Will return `Err` if `Action` in `ActionStore` fails to process or run.
pub async fn process_market_actions(action: ActionStore, market_tx: MessageSender) -> Result<()> {
    let market_tx = market_tx.lock().await;

    action.action.process(&market_tx, action.id)?;

    Ok(())
}

/// Initialize the market action processing system.
pub async fn initialize_market_actions(
    market_tx_arc: MessageSender,
) -> (JoinHandle<Result<()>>, UnboundedSender<ActionStore>) {
    let (actions_tx, mut actions_rx) = futures_channel::mpsc::unbounded::<ActionStore>();

    let join_handle = tokio::spawn(async move {
        let market_tx_arc = market_tx_arc.clone();

        while let Some(item) = actions_rx.next().await {
            process_market_actions(item, market_tx_arc.clone()).await?;
        }

        Ok(())
    });

    (join_handle, actions_tx)
}

/// Initialize the websocket market stream.
///
/// # Errors
///
/// Will return [`tungstenite::error::Error`] if [`connect_async`] fails.
pub async fn initialize_market_stream(
    config: &Config,
    data_tx_arc: DataSender,
) -> Result<(JoinHandle<Result<()>>, MessageSender)> {
    let (market_tx, market_rx) = futures_channel::mpsc::unbounded();
    let market_tx_arc = Arc::new(Mutex::new(market_tx));
    let Some(websocket_market_api) = &config.websocket_market_api else {
        panic!("Websocket Market API not set.");
    };

    let (market_stream, _) = connect_async(websocket_market_api).await?;
    log::info!("WebSocket Market API handshake has been successfully completed.");

    {
        let data_tx_arc = data_tx_arc.clone();
        let data_tx = data_tx_arc.lock().await;

        data_tx.unbounded_send(
            ApiResponse::<WebsocketData>::default().websocket_data(WebsocketData::MarketHandshake),
        )?;
    }

    let (market_write, market_read) = market_stream.split();
    let rx_to_market = market_rx.map(Ok).forward(market_write);

    let join_handle: JoinHandle<Result<()>> = {
        let market_tx_arc = market_tx_arc.clone();

        tokio::spawn(async move {
            let market_to_process = {
                market_read
                    .map_err(convert_tungstenite_error)
                    .try_for_each(|message| async {
                        match process_market(message, market_tx_arc.clone(), data_tx_arc.clone())
                            .await
                        {
                            Ok(res) => Ok(res),
                            Err(err) => Err(processing_error(err)),
                        }
                    })
            };

            pin_mut!(rx_to_market, market_to_process);
            match future::select(rx_to_market, market_to_process).await {
                Either::Left((_rx_to_market_res, _)) => {
                    log::info!("Market process completed");

                    Ok(())
                }
                Either::Right((market_to_process_res, _)) => match market_to_process_res {
                    Ok(_) => {
                        log::info!("Market process completed");

                        Ok(())
                    }
                    Err(err) => anyhow::bail!(err),
                },
            }
        })
    };

    Ok((join_handle, market_tx_arc))
}

/// Send a subscription request to the market api.
///
/// # Errors
///
/// Will return `Err` if `ApiRequestBuilder` does not have method.
///
/// Will return [`serde_json::Error`] if [`serde_json::to_string`] failed to
/// convert `subscribe_msg` to a string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails.
pub fn subscribe(tx: &UnboundedSender<Message>, id: u64, channels: Vec<String>) -> Result<()> {
    log::info!("Sending subscribe.");

    let subscribe_msg = ApiRequestBuilder::default()
        .with_id(id)
        .with_method("subscribe")
        .with_params(SubscribeParams { channels })
        .with_nonce()
        .build()?;

    let msg = Message::Text(serde_json::to_string(&subscribe_msg)?);

    tx.unbounded_send(msg)?;

    Ok(())
}

/// Process the subscribe return data from the market api.
///
/// # Errors
///
/// Will return `Err` if subscription fails to process.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn process_subscribe_result(
    data_tx: DataSender,
    res: &serde_json::Value,
    msg: &ApiResponse<serde_json::Value>,
    sub: &RawRes,
) -> Result<()> {
    let data_tx = data_tx.lock().await;

    match sub.channel.as_str() {
        "book" => {
            let book_data = reprocess_data::<RawBookRes, BookRes>(&res.to_string())?;
            data_tx.unbounded_send(msg.websocket_data(WebsocketData::Book(book_data)))?;
        }
        "ticker" => {
            let ticker_data = reprocess_data::<RawTickerRes, TickerRes>(&res.to_string())?;
            data_tx.unbounded_send(msg.websocket_data(WebsocketData::Ticker(ticker_data)))?;
        }
        "trade" => {
            let trade_data = reprocess_data::<RawTradeRes, TradeRes>(&res.to_string())?;
            data_tx.unbounded_send(msg.websocket_data(WebsocketData::Trade(trade_data)))?;
        }
        "candlestick" => {
            let candlestick_data =
                reprocess_data::<RawCandlestickRes, CandlestickRes>(&res.to_string())?;
            data_tx
                .unbounded_send(msg.websocket_data(WebsocketData::Candlestick(candlestick_data)))?;
        }
        "otc_book" => {
            let otc_book_data = reprocess_data::<RawOtcBookRes, OtcBookRes>(&res.to_string())?;
            data_tx.unbounded_send(msg.websocket_data(WebsocketData::OtcBook(otc_book_data)))?;
        }
        _ => anyhow::bail!(ApiError::UnsupportedSubscription(msg.clone())),
    }

    Ok(())
}

/// Process data recieved from the market api.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the text or
/// binary data as a string. This error can also be passed down from
/// [`crate::utils::reprocess_data`].
///
/// Will return [`std::str::Utf8Error`] if [`std::str::from_utf8`] cannot process a binary message that was recieved.
///
/// Will return `Err` if [`crate::websocket::respond_heartbeat`] fails to respond to heartbeat message.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
pub async fn process_market(
    message: Message,
    market_tx: MessageSender,
    data_tx: DataSender,
) -> Result<()> {
    let msg = message_to_api_response(&market_tx, &message).await?;
    let method = msg.method.as_str();
    let res = msg.result.clone();

    match method {
        "public/heartbeat" => {
            let market_tx = market_tx.lock().await;
            let data_tx = data_tx.lock().await;

            respond_heartbeat(&market_tx, msg.id as u64)?;
            data_tx.unbounded_send(msg.websocket_data(WebsocketData::MarketHeartbeat))?;
        }
        "subscribe" => {
            let Some(res) = &res else {
                log::warn!("Subscribe message had no result. {msg:#?}");

                return Ok(());
            };

            let sub_result: RawRes = serde_json::from_str(&res.to_string())?;

            process_subscribe_result(data_tx, res, &msg, &sub_result).await?;
        }
        "ping" => {}
        _ => anyhow::bail!(ApiError::UnsupportedMethod(msg.clone())),
    }

    Ok(())
}
