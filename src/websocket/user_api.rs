//! The interaction systems for the websocket User API.

use std::sync::Arc;

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::api_response::ApiResponse;
use crate::error::{convert_tungstenite_error, processing_error};
use crate::prelude::{DataSender, MessageSender};
use crate::utils::action::ActionStore;
use crate::utils::config::Config;
use crate::utils::{message_to_api_response, reprocess_data};
use crate::websocket::data::{
    AccountSummary, CancelOrderList, CreateOrder, CreateOrderList, CreateWithdrawal, OpenOrders,
    OrderDetail, OrderHistory, RawRes, RawUserTradeRes, Trades, UserBalance, UserOrderRes,
    UserTradeRes, WithdrawalHistory,
};
use crate::websocket::{respond_heartbeat, WebsocketData};

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
pub async fn initialize_user_actions(user_tx_arc: MessageSender) -> UnboundedSender<ActionStore> {
    let (actions_tx, mut actions_rx) = futures_channel::mpsc::unbounded::<ActionStore>();

    tokio::spawn(async move {
        let user_tx_arc = user_tx_arc.clone();

        while let Some(item) = actions_rx.next().await {
            process_user_actions(item, user_tx_arc.clone())
                .await
                .expect("Failed to process market action");
        }
    });

    actions_tx
}

/// Initialize the user websocket stream.
///
/// # Errors
///
/// Will return [`tungstenite::error::Error`] if [`connect_async`] fails.
pub async fn initialize_user_stream(
    config: &Config,
    data_tx_arc: DataSender,
) -> Result<MessageSender> {
    let (user_tx, user_rx) = futures_channel::mpsc::unbounded();
    let user_tx_arc = Arc::new(Mutex::new(user_tx));
    let Some(websocket_user_api) = &config.websocket_user_api else {
        panic!("Websocket User API not in config.");
    };

    let (user_stream, _) = connect_async(websocket_user_api).await?;
    log::info!("WebSocket User API handshake has been successfully completed.");

    {
        let data_tx_arc = data_tx_arc.clone();
        let data_tx = data_tx_arc.lock().await;

        data_tx.unbounded_send(WebsocketData::UserHandshake)?;
    }

    let (user_write, user_read) = user_stream.split();
    let rx_to_user = user_rx.map(Ok).forward(user_write);

    {
        let user_tx_arc = user_tx_arc.clone();

        tokio::spawn(async move {
            let user_to_process = {
                user_read
                    .map_err(|err| convert_tungstenite_error(err))
                    .try_for_each(|message| async {
                        match process_user(message, user_tx_arc.clone(), data_tx_arc.clone()).await
                        {
                            Ok(res) => Ok(res),
                            Err(err) => Err(processing_error(err)),
                        }
                    })
            };

            pin_mut!(rx_to_user, user_to_process);
            future::select(rx_to_user, user_to_process).await;
            log::info!("User process completed");
        });
    }

    Ok(user_tx_arc)
}

/// Process the subscribe return data from the market api.
///
/// # Errors
///
/// Will return `Err` if subscription fails to process.
async fn process_subscribe_result(
    data_tx: DataSender,
    res: &serde_json::Value,
    sub: &RawRes,
) -> Result<()> {
    match sub.channel.as_str() {
        "user.order" => {
            let data_tx = data_tx.lock().await;

            let user_order_data: UserOrderRes = serde_json::from_str(&res.to_string())?;
            data_tx.unbounded_send(WebsocketData::UserOrder(user_order_data))?;
        }
        "user.trade" => {
            let data_tx = data_tx.lock().await;

            let user_trade_data =
                reprocess_data::<RawUserTradeRes, UserTradeRes>(&res.to_string())?;
            data_tx.unbounded_send(WebsocketData::UserTrade(user_trade_data))?;
        }
        "user.balance" => {
            let data_tx = data_tx.lock().await;

            let user_balance_data: Vec<UserBalance> = serde_json::from_str(&res.to_string())?;
            data_tx.unbounded_send(WebsocketData::UserBalance(user_balance_data))?;
        }
        _ => {}
    }

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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let create_withdrawal_data: CreateWithdrawal = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::CreateWithdrawal(create_withdrawal_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let withdrawal_history_data: WithdrawalHistory = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::GetWithdrawalHistory(withdrawal_history_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let account_summary_data: AccountSummary = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::GetAccountSummary(account_summary_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let create_order_data: CreateOrder = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::CreateOrder(create_order_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let create_order_list_data: CreateOrderList = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::CreateOrderList(create_order_list_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let cancel_order_list_data: CancelOrderList = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::CancelOrderList(cancel_order_list_data))?;
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

    tx.unbounded_send(WebsocketData::CancelAllOrders(msg.id as u64))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let order_history_data: OrderHistory = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::GetOrderHistory(order_history_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let open_orders_data: OpenOrders = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::GetOpenOrders(open_orders_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let order_detail_data: OrderDetail = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::GetOrderDetail(order_detail_data))?;
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
    let Some(res) = &msg.result else {
        log::warn!("Message had no result. {msg:#?}");

        return Ok(());
    };

    let tx = arc_tx.lock().await;

    let trades_data: Trades = serde_json::from_str(&res.to_string())?;
    tx.unbounded_send(WebsocketData::GetTrades(trades_data))?;
    drop(tx);

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

    match msg.method.as_str() {
        "public/heartbeat" => {
            let user_tx = user_tx.lock().await;
            let data_tx = data_tx.lock().await;

            respond_heartbeat(&user_tx, msg.id as u64)?;
            data_tx.unbounded_send(WebsocketData::UserHeartbeat)?;
        }
        "public/auth" => {
            let data_tx = data_tx.lock().await;

            data_tx.unbounded_send(WebsocketData::Auth(msg.code.unwrap_or(0) as u64))?;
        }
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
        "subscribe" => {
            let Some(res) = msg.result else {
                log::warn!("Subscribe message had no result. {msg:#?}");

                return Ok(());
            };

            let sub_result: RawRes = serde_json::from_str(&res.to_string())?;

            process_subscribe_result(data_tx, &res, &sub_result).await?;
        }
        _ => {}
    }

    Ok(())
}
