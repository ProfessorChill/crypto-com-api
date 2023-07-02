//! Crate config module.

/// The config of the API, this is passed often through the system.
#[derive(Default, Debug)]
pub struct Config {
    /// User API key.
    pub api_key: Option<String>,
    /// User secret.
    pub secret_key: Option<String>,
    /// Websocket user api URL.
    pub websocket_user_api: Option<url::Url>,
    /// Websocket market api URL.
    pub websocket_market_api: Option<url::Url>,
    /// REST URL.
    pub rest_url: Option<url::Url>,
}
