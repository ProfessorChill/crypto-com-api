//! Data for private REST routes.

use anyhow::Result;
use serde::Serialize;

use crate::rest::data::CreateWithdrawalRes;
use crate::{api_request::ApiRequestBuilder, api_response::ApiResponse, utils::config::Config};

/// Create withdrawal params.
#[derive(Serialize, Debug)]
pub struct CreateWithdrawal {
    /// Client withdrawal ID.
    pub client_wid: Option<String>,
    /// e.g. BTC, CRO.
    pub currency: String,
    /// Amount.
    pub amount: f64,
    /// Address.
    pub address: String,
    /// Secondary address identifier for coins like XRP, XLM, etc. Also known as memo or tags.
    pub address_tag: Option<String>,
}

/// Creates a withdrawal request. Withdrawal setting must be enabled for your API Key. If you do
/// not see the option when viewing your API Key, this feature is not yet available to you.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn create_withdrawal(
    config: &Config,
    params: CreateWithdrawal,
) -> Result<ApiResponse<CreateWithdrawalRes>> {
    let client = reqwest::Client::new();

    let Some(rest_url) = &config.rest_url else {
        panic!("Rest URL is not set in config.");
    };

    let Some(api_key) = &config.api_key else {
        panic!("API Key is not set in config.");
    };

    let Some(secret) = &config.secret_key else {
        panic!("Secret is not set in config.");
    };

    let req = ApiRequestBuilder::default()
        .with_id(0)
        .with_method("private/create-withdrawal")
        .with_params(params)
        .with_api_key(api_key)
        .with_digital_signature(secret)
        .build()?;

    let res = client
        .post(rest_url.to_string())
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .json::<ApiResponse<CreateWithdrawalRes>>()
        .await?;

    Ok(ApiResponse {
        id: res.id,
        method: res.method,
        result: res.result,
        code: res.code,
        message: res.message,
        original: res.original,
        detail_code: res.detail_code,
        detail_message: res.detail_message,
    })
}
