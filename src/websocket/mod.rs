//! Data and helper functions for interacting with the websocket system.

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::api_request::ApiRequestBuilder;
use crate::rest::data::InstrumentsRes;
use crate::websocket::data::{
    AccountSummary, BookRes, CancelOrderList, CandlestickRes, CreateOrder, CreateOrderList,
    CreateWithdrawal, OpenOrders, OrderDetail, OrderHistory, OtcBookRes, TickerRes, TradeRes,
    Trades, UserBalance, UserOrderRes, UserTradeRes, WithdrawalHistory,
};

use self::data::Scope;

pub mod actions;
pub mod data;
pub mod market_api;
pub mod user_api;

/// Data that could be recieved from the websocket.
///
/// `private/cancel-order` doesn't return data.
#[derive(Debug)]
pub enum WebsocketData {
    /// Auth with auth code.
    Auth,
    /// Data from `ticker.{instrument_name}` subscription.
    Ticker(TickerRes),
    /// Data from `book.{instrument_name}` subscription.
    Book(BookRes),
    /// Data from `trade.{instrument_name}` subscription.
    Trade(TradeRes),
    /// Data from `candlestick.{time_frame}.{instrument_name}` subscription.
    Candlestick(CandlestickRes),
    /// Data from `otc_book.{instrument_name}` subscription.
    OtcBook(OtcBookRes),
    /// Data from `user.order.{instrument_name}` subscription.
    UserOrder(UserOrderRes),
    /// Data from `user.trade.{instrument_name}` subscription.
    UserTrade(UserTradeRes),
    /// Data from `user.balance` subscription.
    UserBalance(Vec<UserBalance>),
    /// Data from `public/get-instruments`
    GetInstruments(InstrumentsRes),
    /// Data from `private/create-withdrawal`.
    CreateWithdrawal(CreateWithdrawal),
    /// Data from `private/get-withdrawal-history`.
    GetWithdrawalHistory(WithdrawalHistory),
    /// Data from `private/get-account-summary`.
    GetAccountSummary(AccountSummary),
    /// Data from `private/create-order`.
    CreateOrder(CreateOrder),
    /// Data from `private/cancel-order`.
    ///
    /// This only returns the ID of the request as a confirmation of the request.
    CancelOrder(u64),
    /// Data from `private/create-order-list`.
    CreateOrderList(CreateOrderList),
    /// Data from `private/cancel-order-list`.
    CancelOrderList(CancelOrderList),
    /// Data from `private/cancel-all-orders`.
    CancelAllOrders,
    /// Data from `private/get-order-history`.
    GetOrderHistory(OrderHistory),
    /// Data from `private/get-open-orders`.
    GetOpenOrders(OpenOrders),
    /// Data from `private/get-order-detail`.
    GetOrderDetail(Box<OrderDetail>),
    /// Data from `private/get-trades`.
    GetTrades(Trades),
    /// User Heartbeat.
    UserHeartbeat,
    /// User Handshake.
    UserHandshake,
    /// Market Heartbeat.
    MarketHeartbeat,
    /// Market Handshake.
    MarketHandshake,
    /// Data from `private/set-cancel-on-disconnect`.
    SetCancelOnDisconnect(Scope),
    /// Data from `private/get-cancel-on-disconnect`.
    GetCancelOnDisconnect(Scope),
}

/// Sends an API message with params to the websocket server. This is helpful for non-REST requests
/// since we are processing the data asychronously later in another thread.
///
/// # Errors
///
/// Will return `Err` if `ApiRequestBuilder` does not contain method or if
/// `get_order_history_message` fails to serialize into a string.
pub fn send_params_msg<T: Serialize, S: Into<String>>(
    tx: &UnboundedSender<Message>,
    id: u64,
    method: S,
    params: T,
) -> Result<()> {
    let method = method.into();

    log::info!("Sending message to {}", method);

    let msg = ApiRequestBuilder::default()
        .with_id(id)
        .with_method(method)
        .with_params(params)
        .with_nonce()
        .build()?;

    let msg = Message::Text(serde_json::to_string(&msg)?);

    tx.unbounded_send(msg)?;

    Ok(())
}

/// Sends an API message to the websocket server. This is helpful for non-REST requests
/// since we are processing the data asychronously later in another thread.
///
/// # Errors
///
/// Will return `Err` if `ApiRequestBuilder` does not contain method or if
/// `get_order_history_message` fails to serialize into a string.
pub fn send_msg<S: Into<String>>(tx: &UnboundedSender<Message>, id: u64, method: S) -> Result<()> {
    let method = method.into();

    log::info!("Sending message to {}", method);

    let msg = ApiRequestBuilder::default()
        .with_id(id)
        .with_method(method)
        .with_nonce()
        .build()?;

    let msg = Message::Text(serde_json::to_string(&msg)?);

    tx.unbounded_send(msg)?;

    Ok(())
}

/// Send a heartbeat response to the websocket.
///
/// # Errors
///
/// Will return `Err` if `ApiRequestBuilder` does not contain method or if `auth_msg` fails to
/// serialize into a string.
pub fn respond_heartbeat(tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
    log::info!("Responding to heartbeat!");

    let heartbeat_msg = ApiRequestBuilder::default()
        .with_id(id)
        .with_method("public/respond-heartbeat")
        .build()?;

    let msg = Message::Text(serde_json::to_string(&heartbeat_msg)?);

    tx.unbounded_send(msg)?;

    Ok(())
}

/// The an authorization request to the server.
///
/// # Errors
///
/// Will return `Err` if `ApiRequestBuilder` does not contain method or if `auth_msg` fails to
/// serialize into a string.
pub fn auth<S: Into<String>>(
    tx: &UnboundedSender<Message>,
    id: u64,
    api_key: S,
    secret_key: S,
) -> Result<()> {
    log::info!("Authorizing user");

    let auth_msg = ApiRequestBuilder::default()
        .with_id(id)
        .with_method("public/auth")
        .with_api_key(api_key)
        .with_nonce()
        .with_digital_signature(secret_key)
        .build()?;

    let msg = Message::Text(serde_json::to_string(&auth_msg)?);

    tx.unbounded_send(msg)?;

    Ok(())
}
