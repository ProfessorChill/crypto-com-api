//! Data from [user.balance](https://exchange-docs.crypto.com/spot/index.html#user-balance)

use serde::Deserialize;

/// Balance of the users currencies.
#[derive(Deserialize, Debug, Clone)]
pub struct UserBalance {
    /// e.g. CRO.
    pub currency: String,
    /// Total balance.
    pub balance: f64,
    /// Available balance (e.g. not in orders, or locked, etc.)
    pub available: f64,
    /// Balance locked in orders.
    pub order: f64,
    /// Balance locked for staking (typically only used for CRO).
    pub stake: f64,
}
