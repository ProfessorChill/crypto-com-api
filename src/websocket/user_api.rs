//! The interaction systems for the websocket User API.

use std::sync::Arc;

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use futures_util::future::Either;
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::api_response::ApiResponse;
use crate::error::{convert_tungstenite_error, processing_error};
use crate::prelude::{ApiError, DataSender, MessageSender};
use crate::rest::data::{InstrumentsRes, RawInstrumentsRes};
use crate::utils::action::ActionStore;
use crate::utils::config::Config;
use crate::utils::{message_to_api_response, reprocess_data};
use crate::websocket::data::{
    AccountSummary, CancelOrderList, CreateOrder, CreateOrderList, CreateWithdrawal, OpenOrders,
    OrderDetail, OrderHistory, RawRes, RawUserTradeRes, Trades, UserBalance, UserOrderRes,
    UserTradeRes, WithdrawalHistory,
};
use crate::websocket::{respond_heartbeat, WebsocketData};

use super::data::Scope;

/// Handle the actions that are to be pushed to the server from [`crate::controller::Controller::push_user_action`]
///
/// # Errors
///
/// Will return `Err` if `Action` in `ActionStore` fails to process or run.
pub async fn process_user_actions(action: ActionStore, user_tx: MessageSender) -> Result<()> {
    let user_tx = user_tx.lock().await;

    action.action.process(&user_tx, action.id)?;

    Ok(())
}

/// Initialize the market action processing system.
pub async fn initialize_user_actions(
    user_tx_arc: MessageSender,
) -> (JoinHandle<Result<()>>, UnboundedSender<ActionStore>) {
    let (actions_tx, mut actions_rx) = futures_channel::mpsc::unbounded::<ActionStore>();

    let join_handle = tokio::spawn(async move {
        let user_tx_arc = Arc::clone(&user_tx_arc);

        while let Some(item) = actions_rx.next().await {
            process_user_actions(item, Arc::clone(&user_tx_arc)).await?;
        }

        Ok(())
    });

    (join_handle, actions_tx)
}

/// Initialize the user websocket stream.
///
/// # Errors
///
/// Will return [`tungstenite::error::Error`] if [`connect_async`] fails.
pub async fn initialize_user_stream(
    config: &Config,
    data_tx_arc: DataSender,
) -> Result<(JoinHandle<Result<()>>, MessageSender)> {
    let (user_tx, user_rx) = futures_channel::mpsc::unbounded();
    let user_tx_arc = Arc::new(Mutex::new(user_tx));
    let Some(ref websocket_user_api) = config.websocket_user_api else {
        anyhow::bail!(ApiError::ConfigMissing("websocket_user_api".to_owned()));
    };

    let (user_stream, _) = connect_async(websocket_user_api.to_string()).await?;
    log::info!("WebSocket User API handshake has been successfully completed.");

    {
        let data_tx_arc = Arc::clone(&data_tx_arc);
        let data_tx = data_tx_arc.lock().await;

        data_tx.unbounded_send(
            ApiResponse::<WebsocketData>::default().websocket_data(WebsocketData::UserHandshake),
        )?;
    }

    let (user_write, user_read) = user_stream.split();
    let rx_to_user = user_rx.map(Ok).forward(user_write);

    let join_handle: JoinHandle<Result<()>> = {
        let user_tx_arc = Arc::clone(&user_tx_arc);

        tokio::spawn(async move {
            let user_to_process = {
                user_read
                    .map_err(convert_tungstenite_error)
                    .try_for_each(|message| async {
                        match process_user(
                            message,
                            Arc::clone(&user_tx_arc),
                            Arc::clone(&data_tx_arc),
                        )
                        .await
                        {
                            Ok(res) => Ok(res),
                            Err(err) => Err(processing_error(err)),
                        }
                    })
            };

            pin_mut!(rx_to_user, user_to_process);
            match future::select(rx_to_user, user_to_process).await {
                Either::Left((_rx_to_user_res, _)) => {
                    log::info!("User process completed");

                    Ok(())
                }
                Either::Right((user_to_process_res, _)) => match user_to_process_res {
                    Ok(_) => {
                        log::info!("User process completed");

                        Ok(())
                    }
                    Err(err) => anyhow::bail!(err),
                },
            }
        })
    };

    Ok((join_handle, user_tx_arc))
}

/// Handle the `public/get-instruments` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn public_get_instruments(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let instrument_data = reprocess_data::<RawInstrumentsRes, InstrumentsRes>(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::GetInstruments(instrument_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/create-withdrawal` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_create_withdrawal(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let create_withdrawal_data: CreateWithdrawal = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::CreateWithdrawal(create_withdrawal_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-withdrawal-history` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_withdrawal_history(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let withdrawal_history_data: WithdrawalHistory = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(
        msg.websocket_data(WebsocketData::GetWithdrawalHistory(withdrawal_history_data)),
    )?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-account-summary` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_account_summary(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let account_summary_data: AccountSummary = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::GetAccountSummary(account_summary_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/create-order` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_create_order(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let create_order_data: CreateOrder = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::CreateOrder(create_order_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/create-order-list` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_create_order_list(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let create_order_list_data: CreateOrderList = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::CreateOrderList(create_order_list_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/cancel-order-list` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_cancel_order_list(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let cancel_order_list_data: CancelOrderList = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::CancelOrderList(cancel_order_list_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/cancel-all-orders` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_cancel_all_orders(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let tx = arc_tx.lock().await;

    tx.unbounded_send(msg.websocket_data(WebsocketData::CancelAllOrders))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-order-history` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_order_history(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let order_history_data: OrderHistory = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::GetOrderHistory(order_history_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-open-orders` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_open_orders(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let open_orders_data: OpenOrders = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::GetOpenOrders(open_orders_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-order-detail` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_order_detail(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let order_detail_data: OrderDetail = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(
        msg.websocket_data(WebsocketData::GetOrderDetail(Box::new(order_detail_data))),
    )?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-trades` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_trades(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let trades_data: Trades = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::GetTrades(trades_data)))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/set-cancel-on-disconnect` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_set_cancel_on_disconnect(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let cancel_on_disconnect_data: Scope = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::SetCancelOnDisconnect(
        cancel_on_disconnect_data,
    )))?;
    drop(tx);

    Ok(())
}

/// Handle the `private/get-cancel-on-disconnect` result.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the result string.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
async fn private_get_cancel_on_disconnect(
    arc_tx: &DataSender,
    msg: &ApiResponse<serde_json::Value>,
) -> Result<()> {
    let Some(ref res) = msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let cancel_on_disconnect_data: Scope = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(msg.websocket_data(WebsocketData::GetCancelOnDisconnect(
        cancel_on_disconnect_data,
    )))?;
    drop(tx);

    Ok(())
}

/// Process the subscribe return data from the market api.
///
/// # Errors
///
/// Will return `Err` if subscription fails to process.
async fn process_subscribe_result(
    data_tx: DataSender,
    res: &serde_json::Value,
    msg: &ApiResponse<serde_json::Value>,
    sub: &RawRes,
) -> Result<()> {
    match sub.channel.as_str() {
        "user.order" => {
            let data_tx = data_tx.lock().await;

            let user_order_data: UserOrderRes = serde_json::from_str(&res.to_string())?;
            data_tx
                .unbounded_send(msg.websocket_data(WebsocketData::UserOrder(user_order_data)))?;
        }
        "user.trade" => {
            let data_tx = data_tx.lock().await;

            let user_trade_data =
                reprocess_data::<RawUserTradeRes, UserTradeRes>(&res.to_string())?;
            data_tx
                .unbounded_send(msg.websocket_data(WebsocketData::UserTrade(user_trade_data)))?;
        }
        "user.balance" => {
            let data_tx = data_tx.lock().await;

            let user_balance_data: Vec<UserBalance> = serde_json::from_str(&res.to_string())?;
            data_tx.unbounded_send(
                msg.websocket_data(WebsocketData::UserBalance(user_balance_data)),
            )?;
        }
        _ => anyhow::bail!(ApiError::UnsupportedSubscription(msg.clone())),
    }

    Ok(())
}

/// Process the user data.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the text or
/// binary data as string. This error can also be passed down from
/// [`crate::utils::reprocess_data`].
///
/// Will return [`std::str::Utf8Error`] if [`str::from_utf8`] cannot process a binary message that was recieved.
///
/// Will return `Err` if [`crate::websocket::respond_heartbeat`] fails to respond to heartbeat message.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
pub async fn process_user(
    message: Message,
    user_tx: MessageSender,
    data_tx: DataSender,
) -> Result<()> {
    let msg = message_to_api_response(&user_tx, &message).await?;
    let method = if let Some(ref method) = msg.method {
        method.as_str()
    } else {
        ""
    };
    let res = msg.result.clone();

    match method {
        "public/heartbeat" => {
            let user_tx = user_tx.lock().await;
            let data_tx = data_tx.lock().await;

            respond_heartbeat(&user_tx, msg.id.try_into()?)?;
            data_tx.unbounded_send(msg.websocket_data(WebsocketData::UserHeartbeat))?;
        }
        "public/auth" => {
            let data_tx = data_tx.lock().await;

            data_tx.unbounded_send(msg.websocket_data(WebsocketData::Auth))?;
        }
        "public/get-instruments" => public_get_instruments(&data_tx, &msg).await?,
        "private/create-withdrawal" => private_create_withdrawal(&data_tx, &msg).await?,
        "private/get-withdrawal-history" => private_get_withdrawal_history(&data_tx, &msg).await?,
        "private/get-account-summary" => private_get_account_summary(&data_tx, &msg).await?,
        "private/create-order" => private_create_order(&data_tx, &msg).await?,
        "private/create-order-list" => private_create_order_list(&data_tx, &msg).await?,
        "private/cancel-order-list" => private_cancel_order_list(&data_tx, &msg).await?,
        "private/cancel-all-orders" => private_cancel_all_orders(&data_tx, &msg).await?,
        "private/get-order-history" => private_get_order_history(&data_tx, &msg).await?,
        "private/get-open-orders" => private_get_open_orders(&data_tx, &msg).await?,
        "private/get-order-detail" => private_get_order_detail(&data_tx, &msg).await?,
        "private/get-trades" => private_get_trades(&data_tx, &msg).await?,
        "private/set-cancel-on-disconnect" => {
            private_set_cancel_on_disconnect(&data_tx, &msg).await?;
        }
        "private/get-cancel-on-disconnect" => {
            private_get_cancel_on_disconnect(&data_tx, &msg).await?;
        }
        "subscribe" => {
            let Some(ref res) = res else {
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
