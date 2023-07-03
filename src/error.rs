//! Main crate error.

use std::num::{ParseFloatError, ParseIntError};

use anyhow::Error as AnyError;

use crate::websocket::WebsocketData;

/// Handles conversion of an anyhow error into a ApiError when `process_user` or `process_market`
/// encounters an error.
pub fn processing_error(err: AnyError) -> ApiError {
    if let Some(_err) = err.downcast_ref::<serde_json::Error>() {
        return ApiError::SerdeJSON;
    }

    if let Some(_err) = err.downcast_ref::<std::str::Utf8Error>() {
        return ApiError::Utf8Error;
    }

    if let Some(_err) = err.downcast_ref::<futures_channel::mpsc::TrySendError<WebsocketData>>() {
        return ApiError::WebsocketSend;
    }

    if let Ok(err) = err.downcast::<ApiError>() {
        return err;
    }

    ApiError::Downcast
}

/// Handles converting from [`tokio_tungstenite::tungstenite::Error`] to [`ApiError`].
pub fn convert_tungstenite_error(_error: tokio_tungstenite::tungstenite::Error) -> ApiError {
    ApiError::Unhandled
}

/// Main error enum.
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    /// API Request is missing a value.
    #[error("missing `{0}` from ApiRequest")]
    InvalidApiRequest(String),
    /// API Auth failed.
    #[error("authorization failed code: `{0}`")]
    AuthFail(u64),
    /// Serde JSON from_str failed.
    #[error("serde_json error occurred")]
    SerdeJSON,
    /// UTF-8 conversion error.
    #[error("failed to conversion from data to utf8")]
    Utf8Error,
    /// Futures failed to send a message.
    #[error("failed to send websocket message")]
    WebsocketSend,
    /// Unhandled error downcasts.
    #[error("unable to downcast error")]
    Downcast,
    /// Parse number error.
    #[error("failed to parse number")]
    ParseNumber,
    /// An error we don't handle at the time.
    #[error("we aren't handling this right now")]
    Unhandled,
}

impl From<ParseFloatError> for ApiError {
    fn from(_value: ParseFloatError) -> Self {
        ApiError::ParseNumber
    }
}

impl From<ParseIntError> for ApiError {
    fn from(_value: ParseIntError) -> Self {
        ApiError::ParseNumber
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_value: serde_json::Error) -> Self {
        ApiError::SerdeJSON
    }
}
