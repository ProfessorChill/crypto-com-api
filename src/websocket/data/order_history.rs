//! Data from [private/get-order-history](https://exchange-docs.crypto.com/spot/index.html#private-get-order-history)

use serde::Deserialize;

use crate::websocket::data::OrderItem;

/// Order history.
#[derive(Deserialize, Debug)]
pub struct OrderHistory {
    /// List of order history items.
    pub order_list: Vec<OrderItem>,
}
