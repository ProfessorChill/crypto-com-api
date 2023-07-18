//! Data from [private/get-withdrawal-history](https://exchange-docs.crypto.com/spot/index.html#private-get-withdrawal-history)

use serde::Deserialize;

/// Withdrawal history item.
#[derive(Deserialize, Debug)]
pub struct WithdrawalHistoryItem {
    /// Newly created withdrawal ID.
    pub id: u64,
    /// (Optional) if a Client withdrawal ID was provided in the request.
    pub client_wid: Option<String>,
    /// e.g. BTC, CRO.
    pub currency: String,
    /// Amount.
    pub amount: f64,
    /// Fee.
    pub fee: f64,
    /// Address with Address Tag (if any).
    pub address: Option<String>,
    /// Create time.
    pub create_time: u64,
    /// 0 - Pending
    /// 1 - Processing
    /// 2 - Rejected
    /// 3 - Payment In-progress
    /// 4 - Payment failed
    /// 5 - Completed
    /// 6 - Cancelled
    pub status: String,
    /// Transaction hash.
    pub txid: String,
    /// Network for the transaction - please see `get-currency-networks`. Only available when
    /// Exchange supports multiple networks on the currency.
    pub network_id: String,
}

/// Withdrawal result.
#[derive(Deserialize, Debug)]
pub struct WithdrawalHistory {
    pub withdrawal_list: Vec<WithdrawalHistoryItem>,
}
