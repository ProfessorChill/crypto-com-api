//! Data from [user.order.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#user-order-instrument_name)

use serde::Deserialize;

use crate::websocket::data::OrderItem;

/// Processed user order data, this JSON already comes in correctly from crypto.com.
#[derive(Deserialize, Debug)]
pub struct UserOrderRes {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// user.margin.order.{instrument_name} -- even in the all case.
    pub subscription: String,
    /// user.order
    pub channel: String,
    /// [`UserOrder`]
    pub data: Vec<OrderItem>,
}
