//! Data from [public/get-ticker](https://exchange-docs.crypto.com/spot/index.html#public-get-ticker)

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
    pub oi: Option<String>,
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
    pub oi: Option<f64>,
    /// 24-hour price change, null if there weren't any trades.
    pub c: Option<f64>,
    /// The current best bid price, null if there weren't any bids.
    pub b: Option<f64>,
    /// The current best ask price, null if there aren't any asks.
    pub k: Option<f64>,
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
            oi: if let Some(ref oi) = value.oi {
                Some(oi.parse::<f64>()?)
            } else {
                None
            },
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
            k: if let Some(ref k) = value.k {
                Some(k.parse::<f64>()?)
            } else {
                None
            },
            t: value.t,
        })
    }
}

/// The processed ticker response.
#[derive(Debug, Default)]
pub struct TickerRes {
    /// [`Ticker`]
    pub data: Vec<Ticker>,
}

impl TryFrom<&RawTickerRes> for TickerRes {
    type Error = ApiError;

    fn try_from(value: &RawTickerRes) -> Result<Self, Self::Error> {
        let mut data = vec![];

        for ticker_data in &value.data {
            data.push(Ticker::try_from(ticker_data)?);
        }

        Ok(Self { data })
    }
}

impl TryFrom<RawTickerRes> for TickerRes {
    type Error = ApiError;

    fn try_from(value: RawTickerRes) -> Result<Self, Self::Error> {
        let mut data = vec![];

        for ticker_data in &value.data {
            data.push(Ticker::try_from(ticker_data)?);
        }

        Ok(Self { data })
    }
}
