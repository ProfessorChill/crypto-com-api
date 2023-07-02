//! Data from [private/cancel-order-list](https://exchange-docs.crypto.com/spot/index.html#private-cancel-order-list)

use serde::Deserialize;

/// Cancel order list item.
#[derive(Deserialize, Debug)]
pub struct CancelOrderListItem {
    /// The index of corresponding order request (Start from 0).
    pub index: u64,
    /// 0 if success.
    pub code: u64,
    /// For server or error messages.
    pub message: Option<String>,
}

/// Cancel order list.
#[derive(Deserialize, Debug)]
pub struct CancelOrderList {
    /// List of order cancellation result.
    pub result_list: Vec<CancelOrderListItem>,
}
