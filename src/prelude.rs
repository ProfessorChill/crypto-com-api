//! Crate prelude.

use std::sync::Arc;

use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use hmac::Hmac;
pub use hmac::Mac;
use sha2::Sha256;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use crate::api_response::ApiResponse;
pub use crate::error::ApiError;
use crate::utils::action::ActionStore;
use crate::websocket::WebsocketData;

/// Hashing type for digital signatures.
pub type HmacSha256 = Hmac<Sha256>;

/// Message sender type.
pub type MessageSender = Arc<Mutex<UnboundedSender<Message>>>;

/// Action store type.
pub type ActionStoreSender = Arc<Mutex<UnboundedSender<ActionStore>>>;

/// Data sender type.
pub type DataSender = Arc<Mutex<UnboundedSender<ApiResponse<WebsocketData>>>>;

/// Data reciever type.
pub type DataReciever = Arc<Mutex<UnboundedReceiver<ApiResponse<WebsocketData>>>>;
