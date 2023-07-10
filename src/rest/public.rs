//! Public REST API functions.

use anyhow::Result;

use crate::api_response::ApiResponse;
use crate::prelude::ApiError;
use crate::rest::data::{
    book::{BookRes, RawBookRes},
    candlestick::{CandlestickRes, RawCandlestickRes},
    instruments::{InstrumentsRes, RawInstrumentsRes},
    ticker::{RawTickerRes, TickerRes},
    trades::{RawTradesRes, TradesRes},
};
use crate::utils::config::Config;

/// Try to get the instrument data.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_instruments(config: &Config) -> Result<ApiResponse<InstrumentsRes>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let res = client
        .get(format!("{rest_url}public/get-instruments"))
        .send()
        .await?
        .json::<ApiResponse<RawInstrumentsRes>>()
        .await?;

    Ok(ApiResponse {
        id: res.id,
        method: res.method,
        result: if let Some(raw_instrument_res) = res.result {
            Some(InstrumentsRes::try_from(raw_instrument_res)?)
        } else {
            None
        },
        code: res.code,
        message: res.message,
        original: res.original,
        detail_code: res.detail_code,
        detail_message: res.detail_message,
    })
}

/// Try to get the book data.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_book(
    config: &Config,
    instrument_name: String,
    depth: u8,
) -> Result<ApiResponse<BookRes>> {
    let client = reqwest::Client::new();

    let params = [
        ("instrument_name", &instrument_name),
        ("depth", &depth.to_string()),
    ];

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let res = client
        .get(format!("{rest_url}public/get-book"))
        .form(&params)
        .send()
        .await?
        .json::<ApiResponse<RawBookRes>>()
        .await?;

    Ok(ApiResponse {
        id: res.id,
        method: res.method,
        result: if let Some(ref res) = res.result {
            Some(BookRes::try_from(res)?)
        } else {
            None
        },
        code: res.code,
        message: res.message,
        original: res.original,
        detail_code: res.detail_code,
        detail_message: res.detail_message,
    })
}

/// Try to get the candlestick data.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_candlestick(
    config: &Config,
    instrument_name: String,
    timeframe: String,
) -> Result<ApiResponse<CandlestickRes>> {
    let client = reqwest::Client::new();

    let params = [
        ("instrument_name", &instrument_name),
        ("timeframe", &timeframe),
    ];

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let res = client
        .get(format!("{rest_url}public/get-candlestick"))
        .form(&params)
        .send()
        .await?
        .json::<ApiResponse<RawCandlestickRes>>()
        .await?;

    Ok(ApiResponse {
        id: res.id,
        method: res.method,
        result: if let Some(ref res) = res.result {
            Some(CandlestickRes::try_from(res)?)
        } else {
            None
        },
        code: res.code,
        message: res.message,
        original: res.original,
        detail_code: res.detail_code,
        detail_message: res.detail_message,
    })
}

/// Try to get the ticker data.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_ticker(
    config: &Config,
    instrument_name: Option<String>,
) -> Result<ApiResponse<TickerRes>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let mut res = client.get(format!("{rest_url}public/get-ticker"));

    if let Some(instrument_name) = instrument_name {
        let params = [("instrument_name", &instrument_name)];

        res = res.form(&params);
    }

    let res = res
        .send()
        .await?
        .json::<ApiResponse<RawTickerRes>>()
        .await?;

    Ok(ApiResponse {
        id: res.id,
        method: res.method,
        result: if let Some(ref res) = res.result {
            Some(TickerRes::try_from(res)?)
        } else {
            None
        },
        code: res.code,
        message: res.message,
        original: res.original,
        detail_code: res.detail_code,
        detail_message: res.detail_message,
    })
}

/// Try to get the trades data.
///
/// # Errors
///
/// Will return [`reqwest::Error`] if send fails or if serialization fails.
pub async fn get_trades(
    config: &Config,
    instrument_name: Option<String>,
) -> Result<ApiResponse<TradesRes>> {
    let client = reqwest::Client::new();

    let Some(ref rest_url) = config.rest_url else {
        anyhow::bail!(ApiError::ConfigMissing("rest_url".to_owned()));
    };

    let mut res = client.get(format!("{rest_url}public/get-trades"));

    if let Some(instrument_name) = instrument_name {
        let params = [("instrument_name", &instrument_name)];

        res = res.form(&params);
    }

    let res = res
        .send()
        .await?
        .json::<ApiResponse<RawTradesRes>>()
        .await?;

    Ok(ApiResponse {
        id: res.id,
        method: res.method,
        result: if let Some(ref res) = res.result {
            Some(TradesRes::try_from(res)?)
        } else {
            None
        },
        code: res.code,
        message: res.message,
        original: res.original,
        detail_code: res.detail_code,
        detail_message: res.detail_message,
    })
}
