//! Data from [private/get-order-detail](https://exchange-docs.crypto.com/spot/index.html#private-get-order-detail)

use serde::Deserialize;

use crate::websocket::data::OrderItem;

/// Trade list item.
#[derive(Deserialize, Debug, Clone)]
pub struct OrderDetailTradeListItem {
    /// BUY, SELL.
    pub side: String,
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// Trade fee.
    pub fee: f64,
    /// Trade ID,
    pub trade_id: String,
    /// Trade creation time.
    pub create_time: u64,
    /// Executed trade price.
    pub traded_price: f64,
    /// Executed trade quantity.
    pub traded_quantity: f64,
    /// Currency used for the fees (e.g. CRO).
    pub fee_currency: String,
    /// Order ID.
    pub order_id: String,
}

/// Order detail.
#[derive(Deserialize, Debug)]
pub struct OrderDetail {
    /// List of trade list items.
    pub trade_list: Vec<OrderDetailTradeListItem>,
    /// Order info.
    pub order_info: OrderItem,
}
