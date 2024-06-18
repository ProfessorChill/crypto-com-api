//! Data from [candlestick.{time_frame}.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#candlestick-time_frame-instrument_name)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw Candlestick data response.
#[derive(Deserialize, Debug)]
pub struct RawCandlestick {
    /// Start time of candlestick (Unix timestamp).
    pub t: u64,
    /// Update time of candlestick (Unix timestamp).
    pub ut: u64,
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

/// The processed Candlestick data response.
#[derive(Debug, Clone)]
pub struct Candlestick {
    /// Start time of candlestick (Unix timestamp).
    pub t: u64,
    /// Update time of candlestick (Unix timestamp).
    pub ut: u64,
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
            ut: value.ut,
            o: value.o.parse::<f64>()?,
            h: value.h.parse::<f64>()?,
            l: value.l.parse::<f64>()?,
            c: value.c.parse::<f64>()?,
            v: value.v.parse::<f64>()?,
        })
    }
}

/// The raw Candlestick response.
#[derive(Deserialize, Debug)]
pub struct RawCandlestickRes {
    /// e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// candlestick.{time_frame}.{instrument_name}
    pub subscription: String,
    /// The period (e.g. M5)
    pub interval: String,
    /// Always candlestick.
    pub channel: String,
    /// [`RawCandlestick`]
    pub data: Vec<RawCandlestick>,
}

/// The processed Candlestick response.
#[derive(Debug)]
pub struct CandlestickRes {
    /// e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// candlestick.{time_frame}.{instrument_name}
    pub subscription: String,
    /// The period (e.g. M5)
    pub interval: String,
    /// Always candlestick.
    pub channel: String,
    /// [`Candlestick`]
    pub data: Vec<Candlestick>,
}

impl TryFrom<&RawCandlestickRes> for CandlestickRes {
    type Error = ApiError;

    fn try_from(value: &RawCandlestickRes) -> Result<Self, Self::Error> {
        let mut candlesticks = vec![];

        for candlestick in &value.data {
            candlesticks.push(Candlestick::try_from(candlestick)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name.clone(),
            subscription: value.subscription.clone(),
            interval: value.interval.clone(),
            channel: value.channel.clone(),
            data: candlesticks,
        })
    }
}

impl TryFrom<RawCandlestickRes> for CandlestickRes {
    type Error = ApiError;

    fn try_from(value: RawCandlestickRes) -> Result<Self, Self::Error> {
        let mut candlesticks = vec![];

        for candlestick in &value.data {
            candlesticks.push(Candlestick::try_from(candlestick)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name,
            subscription: value.subscription,
            interval: value.interval,
            channel: value.channel,
            data: candlesticks,
        })
    }
}
