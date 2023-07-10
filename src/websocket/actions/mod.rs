//! A crate for holding websocket actions.

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::utils::action::Action;
use crate::websocket::auth;
use crate::websocket::{send_msg, send_params_msg};

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

/// Get instruments action.
#[derive(Debug)]
pub struct GetInstruments;

impl Action for GetInstruments {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_msg(tx, id, "public/get-instruments")
    }
}

/// Cancel on Disconnect is an optional feature that will cancel all open orders created by the
/// connection upon loss of connectivity between client or server.
#[derive(Serialize, Debug)]
pub struct SetCancelOnDisconnect {
    /// `ACCOUNT` or `CONNECTION`.
    pub scope: String,
}

impl Action for SetCancelOnDisconnect {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_params_msg(tx, id, "private/set-cancel-on-disconnect", self)
    }
}

/// Returns the scope of cancellation for the connection.
#[derive(Debug)]
pub struct GetCancelOnDisconnect;

impl Action for GetCancelOnDisconnect {
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()> {
        send_msg(tx, id, "private/get-cancel-on-disconnect")
    }
}
