//! Data from [private/get-deposit-history](https://exchange-docs.crypto.com/spot/index.html#private-get-deposit-history)

use serde::{Deserialize, Serialize};

/// Deposit history params.
#[derive(Serialize, Debug)]
pub struct DepositHistoryParams {
    /// e.g. BTC, CRO.
    pub currency: Option<String>,
    /// Default is 90 days from current timestamp.
    pub start_ts: Option<u64>,
    /// Default is current timestamp.
    pub end_ts: Option<u64>,
    /// Page size (Default: 20, Max: 200).
    pub page_size: Option<u64>,
    /// 0 - Not Arrived.
    /// 1 - Arrived.
    /// 2 - Failed.
    /// 3 - Pending.
    pub status: Option<String>,
}

/// Deposit history item.
#[derive(Deserialize, Debug)]
pub struct DepositHistoryItem {
    /// Newly created deposit ID.
    pub id: u64,
    /// e.g. BTC, CRO.
    pub currency: String,
    /// Amount.
    pub amount: f64,
    /// Fee.
    pub fee: f64,
    /// Address with Address Tag (if any).
    pub address: String,
    /// Create time.
    pub create_time: u64,
    /// 0 - Not Arrived.
    /// 1 - Arrived.
    /// 2 - Failed.
    /// 3 - Pending.
    pub status: String,
}

/// Deposit history.
#[derive(Deserialize, Debug)]
pub struct DepositHistory {
    /// Deposit list.
    pub deposit_list: Vec<DepositHistoryItem>,
}
