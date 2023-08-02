//! Data from [private/get-deposit-address](https://exchange-docs.crypto.com/spot/index.html#private-get-deposit-address)

use serde::{Deserialize, Serialize};

/// Deposit address params.
#[derive(Serialize, Debug)]
pub struct DepositAddressParams {
    /// e.g. BTC, CRO.
    pub currency: String,
}

/// Deposit address item.
#[derive(Deserialize, Debug)]
pub struct DepositAddressItem {
    /// Newly created deposit ID.
    pub id: usize,
    /// e.g. BTC, CRO
    pub currency: String,
    /// e.g. ETH, CRO
    ///
    /// When currency = CRO, network = CRO, it is a main net address.
    /// When currency = CRO, network = ETH, it is an ERC20 address.
    pub network: String,
    /// Address with Address Tag (if any).
    pub address: Option<String>,
    /// Create time.
    pub create_time: u64,
    /// "0"
    ///
    /// 0 - Inactive.
    /// 1 - Active.
    pub status: String,
}

/// Deposit address.
#[derive(Deserialize, Debug)]
pub struct DepositAddress {
    /// Array of deposit address items.
    pub deposit_address_list: Vec<DepositAddressItem>,
}
