//! Data from [private/get-account-summary](https://exchange-docs.crypto.com/spot/index.html#private-get-account-summary)

use serde::Deserialize;

/// Account data.
#[derive(Deserialize, Debug)]
pub struct Account {
    /// Total balance.
    pub balance: f64,
    /// Available balance (e.g. not in orders, or locked, etc.)
    pub available: f64,
    /// Balance locked in orders.
    pub order: f64,
    /// Balanced locked for staking (typically only used for CRO)
    pub stake: f64,
    /// e.g. CRO.
    pub currency: String,
}

/// Account summary.
#[derive(Deserialize, Debug, Clone)]
pub struct AccountSummary {
    /// An array of accounts.
    pub accounts: Vec<Account>,
}
