//! Data from [private/create-withdrawal](https://exchange-docs.crypto.com/spot/index.html#private-create-withdrawal)

use serde::Deserialize;

/// Create withdrawal data response.
#[derive(Deserialize, Debug)]
pub struct CreateWithdrawal {
    /// Newly created withdrawal ID.
    pub id: u64,
    /// If a client withdrawal ID was provided in the request.
    pub client_wid: Option<String>,
    /// e.g. BTC, CRO.
    pub currency: String,
    /// Withdrawal amount.
    pub amount: f64,
    /// Withdrawal fee.
    pub fee: f64,
    /// Address with Address Tag (if any).
    pub address: Option<String>,
    /// Create time.
    pub create_time: u64,
}
