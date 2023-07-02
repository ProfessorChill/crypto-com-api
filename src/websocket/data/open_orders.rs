//! Data from [private/get-open-orders](https://exchange-docs.crypto.com/spot/index.html#private-get-open-orders)

use serde::Deserialize;

use crate::websocket::data::OrderItem;

/// Order history.
#[derive(Deserialize, Debug)]
pub struct OpenOrders {
    /// Total count of orders.
    pub count: u64,
    /// List of order history items.
    pub order_list: Vec<OrderItem>,
}
