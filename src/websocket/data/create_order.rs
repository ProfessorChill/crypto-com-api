//! Data from [private/create-order](https://exchange-docs.crypto.com/spot/index.html#private-create-order)

use serde::Deserialize;

/// Create order response.
#[derive(Deserialize, Debug)]
pub struct CreateOrder {
    /// Newly created order ID.
    pub order_id: u64,
    /// If a Client Order ID was provided in the request, otherwise, will be the nonce in the
    /// request. As nonce can be the same among orders, it is recommended to specify client_oid.
    pub client_oid: Option<String>,
}
