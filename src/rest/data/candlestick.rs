//! Data from [public/get-candlestick](https://exchange-docs.crypto.com/spot/index.html#public-get-candlestick)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw candlestick data response.
#[derive(Deserialize, Debug)]
pub struct RawCandlestick {
    /// End time of candlestick (Unix timestamp).
    pub t: u64,
    /// Open.
    pub o: String,
    /// High.
    pub h: String,
    /// Low.
    pub l: String,
    /// Close.
    pub c: String,
    /// Volume.
    pub v: String,
}

/// The raw candlestick response.
#[derive(Deserialize, Debug)]
pub struct RawCandlestickRes {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// The period (e.g. 5m).
    pub interval: String,
    /// [`RawCandlestick`]
    pub data: Vec<RawCandlestick>,
}

/// The processed candlestick data response.
#[derive(Debug, Default)]
pub struct Candlestick {
    /// End time of candlestick (Unix timestamp).
    pub t: u64,
    /// Open.
    pub o: f64,
    /// High.
    pub h: f64,
    /// Low.
    pub l: f64,
    /// Close.
    pub c: f64,
    /// Volume.
    pub v: f64,
}

impl TryFrom<&RawCandlestick> for Candlestick {
    type Error = ApiError;

    fn try_from(value: &RawCandlestick) -> Result<Self, Self::Error> {
        Ok(Self {
            t: value.t,
            o: value.o.parse::<f64>()?,
            h: value.h.parse::<f64>()?,
            l: value.l.parse::<f64>()?,
            c: value.c.parse::<f64>()?,
            v: value.v.parse::<f64>()?,
        })
    }
}

/// The processed candlestick response.
#[derive(Debug, Default)]
pub struct CandlestickRes {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// The period (e.g. 5m).
    pub interval: String,
    /// [`Candlestick`]
    pub data: Vec<Candlestick>,
}

impl TryFrom<&RawCandlestickRes> for CandlestickRes {
    type Error = ApiError;

    fn try_from(value: &RawCandlestickRes) -> Result<Self, Self::Error> {
        let mut data = vec![];

        for candlestick in &value.data {
            data.push(Candlestick::try_from(candlestick)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name.clone(),
            interval: value.interval.clone(),
            data,
        })
    }
}

impl TryFrom<RawCandlestickRes> for CandlestickRes {
    type Error = ApiError;

    fn try_from(value: RawCandlestickRes) -> Result<Self, Self::Error> {
        let mut data = vec![];

        for candlestick in &value.data {
            data.push(Candlestick::try_from(candlestick)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name,
            interval: value.interval,
            data,
        })
    }
}
