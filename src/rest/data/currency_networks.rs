//! Data from [private/get-currency-networks](https://exchange-docs.crypto.com/spot/index.html#private-get-currency-networks)

use std::collections::HashMap;

use serde::Deserialize;

/// Currency network.
#[derive(Deserialize, Debug)]
pub struct CurrencyNetwork {
    /// The network id, can be used in `create-withdrawal`.
    pub network_id: String,
    /// Withdrawal enabled.
    pub withdrawal_enabled: bool,
    /// Deposit enabled.
    pub deposit_enabled: bool,
    /// Withdrawal fee (if any).
    pub withdrawal_fee: Option<f64>,
    /// Min withdrawal amount.
    pub min_withdrawal_amount: f64,
    /// Confirmations required.
    pub confirmation_required: u64,
}

/// Currency map.
#[derive(Deserialize, Debug)]
pub struct CurrencyMap {
    /// e.g. SHIBA INU
    pub full_name: String,
    /// If network is not provided in `create-withdrawal`, it will search for `default_network`, if
    /// there is more than 1 network available.
    pub default_network: String,
    /// A list of networks.
    pub network_list: Vec<CurrencyNetwork>,
}

/// Currency Networks.
#[derive(Deserialize, Debug)]
pub struct CurrencyNetworks {
    /// Update time.
    pub update_time: u64,
    /// Currency map.
    pub currency_map: HashMap<String, CurrencyMap>,
}
