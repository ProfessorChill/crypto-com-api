//! Data from [public/get-trades](https://exchange-docs.crypto.com/spot/index.html#public-get-trades)

use serde::Deserialize;

/// The raw trade data response.
#[derive(Deserialize, Debug)]
pub struct RawTrade {
    /// Side ("BUY" or "SELL").
    pub s: String,
    /// Trade price.
    pub p: String,
    /// Trade quantity.
    pub q: String,
    /// Trade timestamp.
    pub t: u64,
    /// Trade ID.
    pub d: String,
    /// Instrument name e.g. BTC_CRO.
    pub i: String,
    /// Reserved. Can be ignored.
    #[serde(rename(deserialize = "dataTime"))]
    pub data_time: u64,
}

/// The raw trade response.
#[derive(Deserialize, Debug)]
pub struct RawTradesRes {
    /// [`RawTrade`]
    pub data: Vec<RawTrade>,
}

/// The processed trade data response.
#[derive(Debug, Default)]
pub struct Trade {
    /// Side ("BUY" or "SELL").
    pub s: String,
    /// Trade price.
    pub p: f64,
    /// Trade quantity.
    pub q: f64,
    /// Trade timestamp.
    pub t: u64,
    /// Trade ID.
    pub d: u64,
    /// Instrument name e.g. BTC_CRO.
    pub i: String,
    /// Reserved. Can be ignored.
    pub data_time: u64,
}

impl From<&RawTrade> for Trade {
    fn from(value: &RawTrade) -> Self {
        Self {
            s: value.s.clone(),
            p: value.p.parse::<f64>().expect("Failed to parse f64 of p"),
            q: value.q.parse::<f64>().expect("Failed to parse f64 of q"),
            t: value.t,
            d: value.d.parse::<u64>().expect("Failed to parse u64 of d"),
            i: value.i.clone(),
            data_time: value.data_time,
        }
    }
}

/// The processed trade response.
#[derive(Debug, Default)]
pub struct TradesRes {
    /// [`Trade`]
    pub data: Vec<Trade>,
}

impl From<&RawTradesRes> for TradesRes {
    fn from(value: &RawTradesRes) -> Self {
        Self {
            data: value.data.iter().map(Trade::from).collect::<Vec<Trade>>(),
        }
    }
}

impl From<RawTradesRes> for TradesRes {
    fn from(value: RawTradesRes) -> Self {
        Self {
            data: value.data.iter().map(Trade::from).collect::<Vec<Trade>>(),
        }
    }
}
