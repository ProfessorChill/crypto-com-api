//! Crate for `ApiResponse`.

use serde::{Deserialize, Serialize};

use crate::websocket::WebsocketData;

/// The format of an API response from the crypto.com server.
#[derive(Deserialize, Debug, Serialize)]
pub struct ApiResponse<T> {
    /// Original request identifier.
    pub id: i64,
    /// Method invoked.
    pub method: String,
    /// Result object.
    pub result: Option<T>,
    /// 0 for success, see docs for a
    /// [full list](https://exchange-docs.crypto.com/spot/index.html#response-and-reason-codes).
    pub code: Option<u64>,
    /// For server or error messages.
    pub message: Option<String>,
    /// Original request as a string, for error cases.
    pub original: Option<String>,
    /// Detail Response Code. Please refer to
    /// [Exchange v1 API](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#response-and-reason-codes)
    /// for the list of values.
    pub detail_code: Option<String>,
    /// Detail Message (if any).
    pub detail_message: Option<String>,
}

impl<T> Default for ApiResponse<T> {
    fn default() -> Self {
        Self {
            id: -1,
            method: String::new(),
            result: None,
            code: None,
            message: None,
            original: None,
            detail_code: None,
            detail_message: None,
        }
    }
}

impl<T> ApiResponse<T> {
    /// Make the [`ApiResponse`] convert to send [`WebsocketData`]
    pub fn websocket_data(&self, data: WebsocketData) -> ApiResponse<WebsocketData> {
        ApiResponse {
            id: self.id,
            method: self.method.clone(),
            result: Some(data),
            code: self.code,
            message: self.message.clone(),
            original: self.original.clone(),
            detail_code: self.detail_code.clone(),
            detail_message: self.detail_message.clone(),
        }
    }
}
