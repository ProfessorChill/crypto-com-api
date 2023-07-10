//! Data from [ticker.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#ticker-instrument_name)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw ticker data response.
#[derive(Deserialize, Debug)]
pub struct RawTicker {
    /// Price of the 24h highest trade.
    pub h: Option<String>,
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
    pub h: Option<f64>,
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

impl TryFrom<&RawTicker> for Ticker {
    type Error = ApiError;

    fn try_from(value: &RawTicker) -> Result<Self, Self::Error> {
        Ok(Self {
            h: if let Some(ref h) = value.h {
                Some(h.parse::<f64>()?)
            } else {
                None
            },
            l: if let Some(ref l) = value.l {
                Some(l.parse::<f64>()?)
            } else {
                None
            },
            a: if let Some(ref a) = value.a {
                Some(a.parse::<f64>()?)
            } else {
                None
            },
            i: value.i.clone(),
            v: value.v.parse::<f64>()?,
            vv: value.vv.parse::<f64>()?,
            oi: value.oi.parse::<f64>()?,
            c: if let Some(ref c) = value.c {
                Some(c.parse::<f64>()?)
            } else {
                None
            },
            b: if let Some(ref b) = value.b {
                Some(b.parse::<f64>()?)
            } else {
                None
            },
            bs: if let Some(ref bs) = value.bs {
                Some(bs.parse::<f64>()?)
            } else {
                None
            },
            k: if let Some(ref k) = value.k {
                Some(k.parse::<f64>()?)
            } else {
                None
            },
            ks: if let Some(ref ks) = value.ks {
                Some(ks.parse::<f64>()?)
            } else {
                None
            },
            t: value.t,
        })
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

impl TryFrom<&RawTickerRes> for TickerRes {
    type Error = ApiError;

    fn try_from(value: &RawTickerRes) -> Result<Self, Self::Error> {
        let mut tickers = vec![];

        for ticker in &value.data {
            tickers.push(Ticker::try_from(ticker)?);
        }

        Ok(Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            data: tickers,
            instrument_name: value.instrument_name.clone(),
        })
    }
}

impl TryFrom<RawTickerRes> for TickerRes {
    type Error = ApiError;

    fn try_from(value: RawTickerRes) -> Result<Self, Self::Error> {
        let mut tickers = vec![];

        for ticker in &value.data {
            tickers.push(Ticker::try_from(ticker)?);
        }

        Ok(Self {
            channel: value.channel,
            subscription: value.subscription,
            data: tickers,
            instrument_name: value.instrument_name,
        })
    }
}
