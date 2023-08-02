//! Data for private REST routes.

use anyhow::Result;
use serde::Serialize;

use crate::prelude::ApiError;
use crate::rest::data::{
    account_summary::{AccountSummary, AccountSummaryParams},
    currency_networks::CurrencyNetworks,
    deposit_address::{DepositAddress, DepositAddressParams},
    deposit_history::{DepositHistory, DepositHistoryParams},
    withdrawal_history::WithdrawalHistory,
    CreateWithdrawalRes,
};
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

/// Get withdrawal history params.
#[derive(Serialize, Debug)]
pub struct GetWithdrawalHistoryParams {
    /// e.g. BTC, CRO
    pub currency: Option<String>,
    /// Default is 90 days from current timestamp.
    pub start_ts: Option<u64>,
    /// Default is current timestamp.
    pub end_ts: Option<u64>,
    /// Page size (Default: 20, Max: 200).
    pub page_size: Option<u64>,
    /// Page number (0-based).
    pub page: Option<u64>,
    /// 0 - Pending
    /// 1 - Processing
    /// 2 - Rejected
    /// 3 - Payment In-progress
    /// 4 - Payment Failed
    /// 5 - Completed
    /// 6 - Cancelled
    pub status: String,
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

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let Some(ref api_key) = config.api_key else {
        anyhow::bail!(ApiError::ConfigMissing("api_key".to_owned()));
    };

    let Some(ref secret) = config.secret_key else {
        anyhow::bail!(ApiError::ConfigMissing("secret_key".to_owned()));
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

    Ok(res)
}

/// Try to get the currency network data.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_currency_networks(config: &Config) -> Result<ApiResponse<CurrencyNetworks>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let Some(ref api_key) = config.api_key else {
        anyhow::bail!(ApiError::ConfigMissing("api_key".to_owned()));
    };

    let Some(ref secret) = config.secret_key else {
        anyhow::bail!(ApiError::ConfigMissing("secret_key".to_owned()));
    };

    let req = ApiRequestBuilder::default()
        .with_id(0)
        .with_method("private/get-currency-networks")
        .with_api_key(api_key)
        .with_digital_signature(secret)
        .build()?;

    let res = client
        .post(rest_url.to_string())
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .json::<ApiResponse<CurrencyNetworks>>()
        .await?;

    Ok(res)
}

/// Fetches withdrawal history. Withdrawal setting must be enabled for your
/// API Key. If you do not see the option when viewing your API Keys, this
/// feature is not yet available for you.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_withdrawal_history(
    config: &Config,
    params: GetWithdrawalHistoryParams,
) -> Result<ApiResponse<WithdrawalHistory>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let Some(ref api_key) = config.api_key else {
        anyhow::bail!(ApiError::ConfigMissing("api_key".to_owned()));
    };

    let Some(ref secret) = config.secret_key else {
        anyhow::bail!(ApiError::ConfigMissing("secret_key".to_owned()));
    };

    let req = ApiRequestBuilder::default()
        .with_id(0)
        .with_method("private/get-withdrawal-history")
        .with_params(params)
        .with_api_key(api_key)
        .with_digital_signature(secret)
        .build()?;

    let res = client
        .post(rest_url.to_string())
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .json::<ApiResponse<WithdrawalHistory>>()
        .await?;

    Ok(res)
}

/// Fetches deposit history. Withdrawal setting must be enabled for your
/// API Key. If you do not see the option when viewing your API Keys, this
/// feature is not yet available for you.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_deposit_history(
    config: &Config,
    params: DepositHistoryParams,
) -> Result<ApiResponse<DepositHistory>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let Some(ref api_key) = config.api_key else {
        anyhow::bail!(ApiError::ConfigMissing("api_key".to_owned()));
    };

    let Some(ref secret) = config.secret_key else {
        anyhow::bail!(ApiError::ConfigMissing("secret_key".to_owned()));
    };

    let req = ApiRequestBuilder::default()
        .with_id(0)
        .with_method("private/get-deposit-history")
        .with_params(params)
        .with_api_key(api_key)
        .with_digital_signature(secret)
        .build()?;

    let res = client
        .post(rest_url.to_string())
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .json::<ApiResponse<DepositHistory>>()
        .await?;

    Ok(res)
}

/// Fetches deposit address. Withdrawal setting must be enabled for your
/// API Key. If you do not see the option when viewing your API Keys, this
/// feature is not yet available for you.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_deposit_address(
    config: &Config,
    params: DepositAddressParams,
) -> Result<ApiResponse<DepositAddress>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let Some(ref api_key) = config.api_key else {
        anyhow::bail!(ApiError::ConfigMissing("api_key".to_owned()));
    };

    let Some(ref secret) = config.secret_key else {
        anyhow::bail!(ApiError::ConfigMissing("secret_key".to_owned()));
    };

    let req = ApiRequestBuilder::default()
        .with_id(0)
        .with_method("private/get-deposit-address")
        .with_params(params)
        .with_api_key(api_key)
        .with_digital_signature(secret)
        .build()?;

    let res = client
        .post(rest_url.to_string())
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .json::<ApiResponse<DepositAddress>>()
        .await?;

    Ok(res)
}

/// Returns the account balance of a user for a particular token.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_account_summary(
    config: &Config,
    params: AccountSummaryParams,
) -> Result<ApiResponse<AccountSummary>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let Some(ref api_key) = config.api_key else {
        anyhow::bail!(ApiError::ConfigMissing("api_key".to_owned()));
    };

    let Some(ref secret) = config.secret_key else {
        anyhow::bail!(ApiError::ConfigMissing("secret_key".to_owned()));
    };

    let req = ApiRequestBuilder::default()
        .with_id(0)
        .with_method("private/get-account-summary")
        .with_params(params)
        .with_api_key(api_key)
        .with_digital_signature(secret)
        .build()?;

    let res = client
        .post(rest_url.to_string())
        .body(serde_json::to_string(&req)?)
        .send()
        .await?
        .json::<ApiResponse<AccountSummary>>()
        .await?;

    Ok(res)
}
