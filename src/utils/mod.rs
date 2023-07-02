//! Utility functions that are typically used for one situation in multiple places.

use std::{
    str,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use futures_channel::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use crate::api_response::ApiResponse;

pub mod action;
pub mod config;

/// Process parameters to a format of key + value with no spaces and no delimiters.
///
/// This should only be used for [`crate::api_request::ApiRequestBuilder::with_digital_signature`] realistically.
///
/// # Panics
///
/// Will panic if the numerical value is not f64, u64, or i64.
#[must_use]
pub fn params_to_str(obj: &serde_json::Value) -> String {
    use serde_json::value::Value;

    let mut return_str = String::new();

    match obj {
        Value::Null => return_str += "null",
        Value::Bool(val) => {
            if *val {
                return_str += "true";
            } else {
                return_str += "false";
            }
        }
        Value::Number(val) => {
            if let Some(float_val) = val.as_f64() {
                return_str += &float_val.to_string();
            } else if let Some(u64_val) = val.as_u64() {
                return_str += &u64_val.to_string();
            } else if let Some(i64_val) = val.as_i64() {
                return_str += &i64_val.to_string();
            } else {
                panic!("Number in params JSON is not a readable value");
            }
        }
        Value::String(val) => return_str += val,
        Value::Array(vals) => {
            for val in vals.iter() {
                return_str += &params_to_str(val);
            }
        }
        Value::Object(val) => {
            for (k, v) in val.iter() {
                return_str += k;
                return_str += &params_to_str(v);
            }
        }
    }

    log::info!("{return_str}");

    return_str
}

/// The the epoch since the UNIX epoch in ms AKA the nonce value.
#[must_use]
pub fn get_epoch_ms() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_nanos()) / 1_000_000
}

/// An odd function I needed for handling turning a message string into a typed value when
/// processing data from crypto.com.
///
/// This is a very inefficient method however appears to work fine.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] fails to process msg string data.
pub fn reprocess_data<'a, T, A>(msg: &'a str) -> Result<A>
where
    T: serde::Deserialize<'a>,
    A: From<T> + std::fmt::Debug,
{
    let raw_msg = serde_json::from_str(msg)?;

    Ok(A::from(raw_msg))
}

/// A function to convert a `tungstenite::Message` into a `ApiResponse<serde_json::Value>`.
///
/// NOTE: This function does also handle sending back ping messages as they arrive.
///
/// # Errors
///
/// Will return [`serde_json::Error`] if [`serde_json::from_str`] cannot process the text or binary
/// data.
///
/// Will return [`std::str::Utf8Error`] if [`str::from_utf8`] cannot process a binary message that
/// was recieved.
///
/// Will return [`futures_channel::mpsc::TrySendError`] if `unbounded_send` fails anywhere.
pub async fn message_to_api_response(
    tx_arc: &Arc<Mutex<UnboundedSender<Message>>>,
    msg: &Message,
) -> Result<ApiResponse<serde_json::Value>> {
    Ok(match msg {
        Message::Text(msg) => serde_json::from_str(msg)?,
        Message::Binary(msg) => serde_json::from_str(str::from_utf8(msg)?)?,
        Message::Ping(msg) => {
            let tx = tx_arc.lock().await;

            tx.unbounded_send(Message::Pong(msg.clone()))?;
            drop(tx);

            ApiResponse::default()
        }
        msg => {
            log::warn!("Unsupported message recieved.");
            log::info!("{:?}", msg);

            ApiResponse::default()
        }
    })
}
