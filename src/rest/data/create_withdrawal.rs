//! Data from [private/create-withdrawal](https://exchange-docs.crypto.com/spot/index.html#private-create-withdrawal)

use serde::Deserialize;

/// Create withdrawal return values.
#[derive(Deserialize, Debug)]
pub struct CreateWithdrawalRes {
    /// Newly created withdrawal ID.
    pub id: u64,
    /// If a Client withdrawal ID was provided in the request.
    pub client_wid: Option<String>,
    /// e.g. BTC, CRO
    pub currency: String,
    /// Amount.
    pub amount: f64,
    /// Fee.
    pub fee: f64,
    /// Address with Address Tag (if any).
    pub address: Option<String>,
    /// Create time.
    pub create_time: u64,
}
