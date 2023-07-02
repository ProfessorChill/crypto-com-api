//! Action crate.

use core::fmt::Debug;

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

/// A trait to generically handle process "Actions".
pub trait Action: Send + Debug + Sync {
    /// Process the individual action.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `ApiRequestBuilder` does not contain method or if
    /// `get_order_history_message` fails to serialize into a string.
    fn process(&self, tx: &UnboundedSender<Message>, id: u64) -> Result<()>;
}

/// Used to store an action with an id.
#[derive(Debug)]
pub struct ActionStore {
    /// The ID to send to the server.
    pub id: u64,
    /// The action to be sent.
    pub action: Box<dyn Action>,
}
