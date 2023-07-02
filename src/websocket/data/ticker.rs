//! Data from [ticker.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#ticker-instrument_name)

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
    /// The current best bid price, null if there aren't any bids.
    pub b: Option<String>,
    /// The current best bid size, null if there aren't any bids.
    pub bs: Option<String>,
    /// The current best ask price, null if there aren't any asks.
    pub k: Option<String>,
    /// The current best ask size, null if there aren't any asks.
    pub ks: Option<String>,
    /// Trade timestamp.
    pub t: u64,
}

/// The raw ticker response.
#[derive(Deserialize, Debug)]
pub struct RawTickerRes {
    /// Always ticker.
    pub channel: String,
    /// ticker.{instrument_name}
    pub subscription: String,
    /// [`RawTicker`]
    pub data: Vec<RawTicker>,
    /// e.g. BTCUSD-PERP.
    pub instrument_name: String,
}

/// The ticker data response.
#[derive(Debug)]
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
    /// The current best bid price, null if there aren't any bids.
    pub b: Option<f64>,
    /// The current best bid size, null if there aren't any bids.
    pub bs: Option<f64>,
    /// The current best ask price, null if there aren't any asks.
    pub k: Option<f64>,
    /// The current best ask size, null if there aren't any asks.
    pub ks: Option<f64>,
    /// Trade timestamp.
    pub t: u64,
}

impl From<&RawTicker> for Ticker {
    fn from(value: &RawTicker) -> Self {
        Self {
            h: value.h.parse::<f64>().expect("Unable to parse f64 from h"),
            l: value
                .l
                .as_ref()
                .map(|l| l.parse::<f64>().expect("Unable to parse f64 from l")),
            a: value
                .a
                .as_ref()
                .map(|a| a.parse::<f64>().expect("Unable to parse f64 from a")),
            i: value.i.clone(),
            v: value.v.parse::<f64>().expect("Unable to parse f64 from v"),
            vv: value
                .vv
                .parse::<f64>()
                .expect("Unable to parse f64 from vv"),
            oi: value
                .oi
                .parse::<f64>()
                .expect("Unable to parse f64 from oi"),
            c: value
                .c
                .as_ref()
                .map(|c| c.parse::<f64>().expect("Unable to parse f64 from c")),
            b: value
                .b
                .as_ref()
                .map(|b| b.parse::<f64>().expect("Unable to parse f64 from b")),
            bs: value
                .bs
                .as_ref()
                .map(|bs| bs.parse::<f64>().expect("Unable to parse f64 from bs")),
            k: value
                .k
                .as_ref()
                .map(|k| k.parse::<f64>().expect("Unable to parse f64 from k")),
            ks: value
                .ks
                .as_ref()
                .map(|ks| ks.parse::<f64>().expect("Unable to parse f64 from ks")),
            t: value.t,
        }
    }
}

/// The ticker response.
#[derive(Debug)]
pub struct TickerRes {
    /// Always ticker.
    pub channel: String,
    /// ticker.{instrument_name}
    pub subscription: String,
    /// [`Ticker`]
    pub data: Vec<Ticker>,
    /// e.g. BTCUSD-PERP.
    pub instrument_name: String,
}

impl From<&RawTickerRes> for TickerRes {
    fn from(value: &RawTickerRes) -> Self {
        Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            data: value.data.iter().map(Ticker::from).collect::<Vec<Ticker>>(),
            instrument_name: value.instrument_name.clone(),
        }
    }
}

impl From<RawTickerRes> for TickerRes {
    fn from(value: RawTickerRes) -> Self {
        Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            data: value.data.iter().map(Ticker::from).collect::<Vec<Ticker>>(),
            instrument_name: value.instrument_name.clone(),
        }
    }
}
