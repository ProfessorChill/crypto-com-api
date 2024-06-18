//! Data from [private/create-order-list](https://exchange-docs.crypto.com/spot/index.html#private-create-order-list)

use serde::Deserialize;

/// Create order list item.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateOrderListItem {
    /// The index of corresponding order request (Start from 0).
    pub index: u64,
    /// 0 if success.
    pub code: u64,
    /// For server or error messages.
    pub message: Option<String>,
    /// Newly created order ID.
    pub order_id: u64,
    /// If a client order ID was provided in the request. (Maximum 36 characters).
    pub client_oid: Option<String>,
}

/// Create order list.
#[derive(Deserialize, Debug)]
pub struct CreateOrderList {
    /// List of order creation result.
    pub result_list: Vec<CreateOrderListItem>,
}
