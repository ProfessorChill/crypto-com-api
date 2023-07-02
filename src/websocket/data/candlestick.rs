//! Data from [candlestick.{time_frame}.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#candlestick-time_frame-instrument_name)

use serde::Deserialize;

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
#[derive(Debug)]
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

impl From<&RawCandlestick> for Candlestick {
    fn from(value: &RawCandlestick) -> Self {
        Self {
            t: value.t,
            ut: value.ut,
            o: value.o.parse::<f64>().expect("Failed to parse f64 of o"),
            h: value.h.parse::<f64>().expect("Failed to parse f64 of h"),
            l: value.l.parse::<f64>().expect("Failed to parse f64 of l"),
            c: value.c.parse::<f64>().expect("Failed to parse f64 of c"),
            v: value.v.parse::<f64>().expect("Failed to parse f64 of v"),
        }
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

impl From<&RawCandlestickRes> for CandlestickRes {
    fn from(value: &RawCandlestickRes) -> Self {
        Self {
            instrument_name: value.instrument_name.clone(),
            subscription: value.subscription.clone(),
            interval: value.interval.clone(),
            channel: value.channel.clone(),
            data: value
                .data
                .iter()
                .map(Candlestick::from)
                .collect::<Vec<Candlestick>>(),
        }
    }
}

impl From<RawCandlestickRes> for CandlestickRes {
    fn from(value: RawCandlestickRes) -> Self {
        Self {
            instrument_name: value.instrument_name.clone(),
            subscription: value.subscription.clone(),
            interval: value.interval.clone(),
            channel: value.channel.clone(),
            data: value
                .data
                .iter()
                .map(Candlestick::from)
                .collect::<Vec<Candlestick>>(),
        }
    }
}
