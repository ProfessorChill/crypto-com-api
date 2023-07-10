//! All possible websocket data that can be returned from the server.
//!
//! `private/cancel-order` does not return data.
//!
//! `private/cancel-all-orders` does not return data.

pub mod account_summary;
pub mod book;
pub mod cancel_order_list;
pub mod candlestick;
pub mod create_order;
pub mod create_order_list;
pub mod create_withdrawal;
pub mod get_trades;
pub mod open_orders;
pub mod order_detail;
pub mod order_history;
pub mod otc_book;
pub mod ticker;
pub mod trade;
pub mod user_balance;
pub mod user_order;
pub mod user_trade;
pub mod withdrawal_history;

use serde::Deserialize;

pub use account_summary::*;
pub use book::*;
pub use cancel_order_list::*;
pub use candlestick::*;
pub use create_order::*;
pub use create_order_list::*;
pub use create_withdrawal::*;
pub use get_trades::*;
pub use open_orders::*;
pub use order_detail::*;
pub use order_history::*;
pub use otc_book::*;
pub use ticker::*;
pub use trade::*;
pub use user_balance::*;
pub use user_order::*;
pub use user_trade::*;
pub use withdrawal_history::*;

/// Order item (used in many order sections).
#[derive(Deserialize, Debug)]
pub struct OrderItem {
    /// ACTIVE, CANCELED, FILLED, REJECTED or EXPIRED.
    pub status: String,
    /// Reason code.
    pub reason: Option<String>,
    /// BUY, SELL.
    pub side: String,
    /// Price specified in the order.
    pub price: f64,
    /// Quantity specified in the order.
    pub quantity: f64,
    /// Order ID,
    pub order_id: String,
    /// Client order ID if included in request. (Maximum 36 characters).
    pub client_oid: String,
    /// Order creation time (Unix timestamp).
    pub create_time: u64,
    /// Order update time (Unix timestamp).
    pub update_time: u64,
    /// LIMIT, MARKET, STOP_LOSS, STOP_LIMIT, TAKE_PROFIT, TAKE_PROFIT_LIMIT.
    #[serde(rename(deserialize = "type"))]
    pub order_type: String,
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// Cumulative executed quantity (for partially filled orders).
    pub cumulative_quantity: f64,
    /// Cumulative executed value (for partially filled orders).
    pub cumulative_value: f64,
    /// Average filled price. If none is filled, returns 0.
    pub avg_price: f64,
    /// Currency used for the fees (e.g. CRO).
    pub fee_currency: String,
    /// GOOD_TILL_CANCEL, FILL_OR_KILL, IMMEDIATE_OR_CANCEL.
    pub time_in_force: String,
    /// Empty or POST_ONLY (Limit Orders Only).
    pub exec_inst: Option<String>,
    /// Used for trigger-related orders.
    pub trigger_price: Option<f64>,
}

/// Scope, used in `private/set-cancel-on-disconnect` and
/// `private/get-cancel-on-disconnect`.
#[derive(Deserialize, Debug)]
pub struct Scope {
    /// The scope parameter, `ACCOUNT` or `CONNECTION`.
    pub scope: String,
}

/// Raw response values from the websocket connections.
#[derive(Deserialize, Debug, Clone)]
pub struct RawRes {
    /// Channel the response is coming from.
    pub channel: String,
    /// Subscription the response is from.
    pub subscription: String,
    /// Interval the response is replying at.
    pub interval: Option<String>,
    /// Data from the response.
    pub data: Option<serde_json::Value>,
    /// Instrument requested.
    pub instrument_name: Option<String>,
    /// Echo millis of message publish.
    pub t: Option<u64>,
}
