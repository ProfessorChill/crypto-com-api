//! Data from [public/get-ticker](https://exchange-docs.crypto.com/spot/index.html#public-get-ticker)

use serde::Deserialize;

/// The raw ticker data response.
#[derive(Deserialize, Debug)]
pub struct RawTicker {
    /// Price of the 24h highest trade.
    pub h: String,
    /// Price of the 24h lowest trade, null if there weren't any trades.
    pub l: Option<String>,
    /// The price of the latest trade, null if there weren't any trades.
    pub a: Option<String>,
    /// Instrument name.
    pub i: String,
    /// The total 24h traded volume.
    pub v: String,
    /// The total 24h traded volume value (in USD).
    pub vv: String,
    /// The open interest.
    pub oi: String,
    /// 24-hour price change, null if there weren't any trades.
    pub c: Option<String>,
    /// The current best bid price, null if there weren't any bids.
    pub b: Option<String>,
    /// The current best ask price, null if there aren't any asks.
    pub k: Option<String>,
    /// Trade timestamp.
    pub t: u64,
}

/// The raw ticker response.
#[derive(Deserialize, Debug)]
pub struct RawTickerRes {
    /// [`RawTicker`]
    pub data: Vec<RawTicker>,
}

/// The processed ticker response.
#[derive(Debug, Default)]
pub struct Ticker {
    /// Price of the 24h highest trade.
    pub h: f64,
    /// Price of the 24h lowest trade, null if there weren't any trades.
    pub l: Option<f64>,
    /// The price of the latest trade, null if there weren't any trades.
    pub a: Option<f64>,
    /// Instrument name.
    pub i: String,
    /// The total 24h traded volume.
    pub v: f64,
    /// The total 24h traded volume value (in USD).
    pub vv: f64,
    /// The open interest.
    pub oi: f64,
    /// 24-hour price change, null if there weren't any trades.
    pub c: Option<f64>,
    /// The current best bid price, null if there weren't any bids.
    pub b: Option<f64>,
    /// The current best ask price, null if there aren't any asks.
    pub k: Option<f64>,
    /// Trade timestamp.
    pub t: u64,
}

impl From<&RawTicker> for Ticker {
    fn from(value: &RawTicker) -> Self {
        Self {
            h: value.h.parse::<f64>().expect("Failed to parse f64 from h"),
            l: value
                .l
                .as_ref()
                .map(|l| l.parse::<f64>().expect("Failed to parse f64 from l")),
            a: value
                .a
                .as_ref()
                .map(|a| a.parse::<f64>().expect("Failed to parse f64 from a")),
            i: value.i.clone(),
            v: value.v.parse::<f64>().expect("Failed to parse f64 from v"),
            vv: value
                .vv
                .parse::<f64>()
                .expect("Failed to parse f64 from vv"),
            oi: value
                .oi
                .parse::<f64>()
                .expect("Failed to parse f64 from oi"),
            c: value
                .c
                .as_ref()
                .map(|c| c.parse::<f64>().expect("Failed to parse f64 from c")),
            b: value
                .b
                .as_ref()
                .map(|b| b.parse::<f64>().expect("Failed to parse f64 from b")),
            k: value
                .k
                .as_ref()
                .map(|k| k.parse::<f64>().expect("Failed to parse f64 from k")),
            t: value.t,
        }
    }
}

/// The processed ticker response.
#[derive(Debug, Default)]
pub struct TickerRes {
    /// [`Ticker`]
    pub data: Vec<Ticker>,
}

impl From<&RawTickerRes> for TickerRes {
    fn from(value: &RawTickerRes) -> Self {
        Self {
            data: value.data.iter().map(Ticker::from).collect::<Vec<Ticker>>(),
        }
    }
}

impl From<RawTickerRes> for TickerRes {
    fn from(value: RawTickerRes) -> Self {
        Self {
            data: value.data.iter().map(Ticker::from).collect::<Vec<Ticker>>(),
        }
    }
}
