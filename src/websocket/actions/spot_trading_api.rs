//! The interaction systems for the websocket Spot Trading API.

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::utils::action::Action;
use crate::websocket::send_params_msg;

/// Paginated params.
#[derive(Serialize, Clone, Debug)]
pub struct Paginated {
    /// e.g. ETH_CRO, BTC_USDT. Omnit for 'all'.
    pub instrument_name: Option<String>,
    /// Start timestamp (milliseconds since the Unix epoch) - defaults to 24 hours ago.
    pub start_ts: Option<u64>,
    /// End timestamp (milliseconds since the Unix epoch) - defaults to 'now'.
    pub end_ts: Option<u64>,
    /// Page size (Default: 20, max: 200).
    pub page_size: Option<u64>,
    /// Page number (0-based).
    pub page: Option<u64>,
}

/// Returns the account balance of a user for a particular token.
#[derive(Serialize, Clone, Debug)]
pub struct GetAccountSummary {
    /// Specific currency, e.g. BTC, CRO. Omit for 'all'.
    pub currency: Option<String>,
}

impl Action for GetAccountSummary {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-account-summary", self)
    }
}

/// Creates a new BUY or SELL on the Exchange.
///
/// The `user.order` subscription can be used to check when the order is successfully created.
///
/// | Type                | Side | Additional Mandatory Parameters            |
/// |---------------------|------|--------------------------------------------|
/// | `LIMIT`             | Both | quantity, price                            |
/// | `MARKET`            | BUY  | notional or quantity, mutually exclusive   |
/// | `MARKET`            | SELL | quantity                                   |
/// | `STOP_LIMIT`        | Both | price, quantity, `trigger_price`           |
/// | `TAKE_PROFIT_LIMIT` | Both | price, quantity, `trigger_price`           |
/// | `STOP_LOSS`         | BUY  | notional, `trigger_price`                  |
/// | `STOP_LOSS`         | SELL | quantity, `trigger_price`                  |
/// | `TAKE_PROFIT`       | BUY  | notional, `trigger_price`                  |
/// | `TAKE_PROFIT`       | SELL | quantity, `trigger_price`                  |
///
/// *Helpful information:*
///
/// - `STOP_LIMIT` and `TAKE_PROFIT_LIMIT` will execute a `LIMIT` order when the
/// `trigger_price` is reached.
/// - `STOP_LOSS` and `TAKE_PROFIT` will execute a `MARKET` order when the
/// `trigger_price` is reached.
///
/// *To create trigger orders against market price:*
///
/// - `trigger_price` below market price: `BUY` `STOP_LOSS` and `STOP_LIMIT`,
/// `SELL` `TAKE_PROFIT` and `TAKE_PROFIT_LIMIT`.
/// - `trigger_price` above market price: `SELL` `STOP_LOSS` and `STOP_LIMIT`,
/// `BUY` `TAKE_PROFIT` and `TAKE_PROFIT_LIMIT`.
#[derive(Serialize, Clone, Debug)]
pub struct CreateOrder {
    /// e.g. ETH_CRO, BTC_USTD.
    pub instrument_name: String,
    /// BUY, SELL
    pub side: String,
    /// LIMIT, MARKET, STOP_LOSS, STOP_LIMIT, TAKE_PROFIT, TAKE_PROFIT_LIMIT.
    #[serde(rename(serialize = "type"))]
    pub order_type: String,
    /// For LIMIT and STOP_LIMIT orders only: Unit price.
    pub price: Option<f64>,
    /// For LIMIT Orders, MARKET, STOP_LOSS, TAKE_PROFIT orders only: Order Quantity to be sold.
    pub quantity: Option<f64>,
    /// For MARKET (BUY), STOP_LOSS (BUY), TAKE_PROFIT (BUY) orders only: Amount to spend.
    pub notional: Option<f64>,
    /// Option Client order ID (Maximum 36 characters). If a Client Order ID is not provided, it
    /// will be the nonce in the request. As nonce can be the same among orders, it is recommended
    /// to specify client_oid to prevent order being rejected.
    pub client_oid: Option<String>,
    /// (Limit Orders Only)
    /// Options are:
    /// - GOOD_TILL_CANCEL (Default if unspecified)
    /// - FILL_OR_KILL
    /// - IMMEDIATE_OR_CANCEL
    pub time_in_force: Option<String>,
    /// (Limit Orders Only)
    /// Options are:
    /// - POST_ONLY
    /// - Or leave empty
    pub exec_inst: Option<String>,
    /// Used with STOP_LOSS, STOP_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders. Dictates when
    /// order will be triggered.
    pub trigger_price: Option<f64>,
}

impl Action for CreateOrder {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/create-order", self)
    }
}

/// Cancels an existing order on the Exchange.
///
/// The `user.order` subscription can be used to check when the order is successfully cancelled.
#[derive(Serialize, Clone, Debug)]
pub struct CancelOrder {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// Order ID.
    pub order_id: String,
}

impl Action for CancelOrder {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/cancel-order", self)
    }
}

/// Create a list of orders on the Exchange.
///
/// `contingency_type` must be LIST, for list of orders creation.
///
/// The `user.order` subscription can be used to check when the orders are successfully created.
///
/// Refer to [`CreateOrder`] for more information and how to create trigger orders against market
/// price.
#[derive(Serialize, Clone, Debug)]
pub struct CreateOrderList {
    /// LIST
    pub contingency_type: String,
    /// LIST: 1-10 orders.
    pub order_list: Vec<CreateOrder>,
}

impl Action for CreateOrderList {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/cancel-order", self)
    }
}

/// Cancel a list of orders on the Exchange.
///
/// The `user.order` subscription can be used to check when each of the orders is successfully
/// cancelled.
///
/// The `user.order` subscription can be used to check when each of the orders is successfully
/// cancelled.
#[derive(Serialize, Clone, Debug)]
pub struct CancelOrderList {
    /// For non contingency orders, A list of orders to be cancelled.
    pub order_list: Option<Vec<CancelOrder>>,
    /// Instrument name of the contingency order, e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: Option<String>,
    /// ID of the contingency order.
    pub contingency_id: Option<String>,
}

impl Action for CancelOrderList {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/cancel-order-list", self)
    }
}

/// Cancel all orders on the Exchange.
///
/// The `user.order` subscription can be used to check the order is successfully
/// cancelled.
#[derive(Serialize, Clone, Debug)]
pub struct CancelAllOrders {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
}

impl Action for CancelAllOrders {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/cancel-all-orders", self)
    }
}

/// Gets the order history for a particular instrument.
///
/// If paging is used, enumerate each page (starting with 0) until an empty `order_list` array
/// appears in the response.
///
/// Users should use `user.order` to keep track of real-time order updates, and
/// `private/get-order-history` should primarily be used for recovery; typically when the websocket
/// is disconnected.
#[derive(Serialize, Clone, Debug)]
pub struct GetOrderHistory(pub Paginated);

impl Action for GetOrderHistory {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-order-history", self.0.clone())
    }
}

/// Gets all *open* orders for a particular instrument.
#[derive(Serialize, Clone, Debug)]
pub struct GetOpenOrders {
    /// e.g. ETH_CRO, BTC_USDT. Omit for 'all'.
    pub instrument_name: Option<String>,
    /// Page size (Default: 20, max: 200).
    pub page_size: Option<u64>,
    /// Page number (0-based).
    pub page: Option<u64>,
}

impl Action for GetOpenOrders {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-open-orders", self)
    }
}

/// Get details on a particular order ID.
#[derive(Serialize, Clone, Debug)]
pub struct GetOrderDetail {
    /// Order ID.
    pub order_id: String,
}

impl Action for GetOrderDetail {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-order-detail", self)
    }
}

/// Gets all executed trades for a particular instrument.
///
/// If paging is used, enumerate each page (starting with 0) until an empty `trade_list` array
/// appears in the response.
///
/// Users should use `user.trade` to keep track of real-time trades, and `private/get-trades`
/// should primarily be used for recovery; typically when the websocket is disconnected.
#[derive(Serialize, Clone, Debug)]
pub struct GetTrades(pub Paginated);

impl Action for GetTrades {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-trades", self)
    }
}
