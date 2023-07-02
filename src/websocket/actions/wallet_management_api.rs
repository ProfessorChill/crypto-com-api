//! The interaction systems for the websocket Wallet Management API.

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::utils::action::Action;
use crate::websocket::send_params_msg;

/// Creates a withdrawal request. Withdrawal setting must be enabled for your API key.
#[derive(Serialize, Debug)]
pub struct CreateWithdrawal {
    /// Optional Client withdrawal ID.
    pub client_wid: Option<String>,
    /// e.g. BTC, CRO.
    pub currency: String,
    /// Amount to withdrawal.
    pub amount: f64,
    /// Address to withdrawal to.
    pub address: String,
    /// Secondary address identifier for coins like XRP, XLM, etc. Also known as memo or tags.
    pub address_tag: Option<String>,
    /// Select the desired network, requires the address to be whitelisted first.
    pub network_id: Option<String>,
}

impl Action for CreateWithdrawal {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/create-withdrawal", self)
    }
}

/// Get withdrawal and deposit history params.
#[derive(Serialize, Clone, Debug)]
pub struct History {
    /// e.g. BTC, CRO.
    pub currency: Option<String>,
    /// Default is 90 days from current timestamp.
    pub start_ts: Option<u64>,
    /// Default is current timestamp.
    pub end_ts: Option<u64>,
    /// Page size (Default: 20, Max: 200).
    pub page_size: Option<u64>,
    /// Page number(0-based)
    pub page: Option<u64>,
    /// 0 - Pending
    /// 1 - Processing
    /// 2 - Rejected
    /// 3 - Payment In-progress
    /// 4 - Payment Failed
    /// 5 - Completed
    /// 6 - Canceled
    pub status: Option<String>,
}

/// Fetches withdrawal history. Withdrawal setting must be enabled for your API Key. If you do not
/// see the option when viewing your API Keys, this feature is not yet available for you.
#[derive(Serialize, Debug)]
pub struct GetWithdrawalHistory(History);

impl Action for GetWithdrawalHistory {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-withdrawal-history", self.0.clone())
    }
}

/// Get deposit address params.
#[derive(Serialize, Debug)]
pub struct GetDepositAddress {
    /// e.g. BTC, CRO.
    pub currency: String,
}

impl Action for GetDepositAddress {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/get-deposit-address", self)
    }
}
