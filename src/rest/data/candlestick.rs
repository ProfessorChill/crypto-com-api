//! Data from [public/get-candlestick](https://exchange-docs.crypto.com/spot/index.html#public-get-candlestick)

use serde::Deserialize;

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

impl From<&RawCandlestick> for Candlestick {
    fn from(value: &RawCandlestick) -> Self {
        Self {
            t: value.t,
            o: value.o.parse::<f64>().expect("Failed to parse f64 of o"),
            h: value.h.parse::<f64>().expect("Failed to parse f64 of h"),
            l: value.l.parse::<f64>().expect("Failed to parse f64 of l"),
            c: value.c.parse::<f64>().expect("Failed to parse f64 of c"),
            v: value.v.parse::<f64>().expect("Failed to parse f64 of v"),
        }
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

impl From<&RawCandlestickRes> for CandlestickRes {
    fn from(value: &RawCandlestickRes) -> Self {
        Self {
            instrument_name: value.instrument_name.clone(),
            interval: value.interval.clone(),
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
            instrument_name: value.instrument_name,
            interval: value.interval,
            data: value
                .data
                .iter()
                .map(Candlestick::from)
                .collect::<Vec<Candlestick>>(),
        }
    }
}
