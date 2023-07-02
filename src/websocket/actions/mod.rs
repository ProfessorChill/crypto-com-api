//! A crate for holding websocket actions.

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::utils::action::Action;
use crate::websocket::auth;
use crate::websocket::send_params_msg;

pub mod spot_trading_api;
pub mod wallet_management_api;

/// Subscription action.
#[derive(Serialize, Clone, Debug)]
pub struct Subscribe {
    /// A list of channels to subscribe to.
    pub channels: Vec<String>,
}

impl Action for Subscribe {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "subscribe", self)
    }
}

/// Auth action.
#[derive(Debug)]
pub struct Auth {
    /// Your API key.
    pub api_key: String,
    /// Your secret key.
    pub secret_key: String,
}

impl Action for Auth {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        auth(tx, id, &self.api_key, &self.secret_key)
    }
}
