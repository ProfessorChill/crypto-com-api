//! Data from [trade.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#trade-instrument_name)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw trade response data.
#[derive(Deserialize, Debug)]
pub struct RawTrade {
    /// Side (buy or sell).
    pub s: String,
    /// Trade price.
    pub p: String,
    /// Trade quantity.
    pub q: String,
    /// Trade timestamp.
    pub t: u64,
    /// Trade ID.
    pub d: String,
    /// Instrument name.
    pub i: String,
}

/// The procesed trade response data.
#[derive(Debug)]
pub struct Trade {
    /// Side (buy or sell).
    pub s: String,
    /// Trade price.
    pub p: f64,
    /// Trade quantity.
    pub q: f64,
    /// Trade timestamp.
    pub t: u64,
    /// Trade ID.
    pub d: String,
    /// Instrument name.
    pub i: String,
}

impl TryFrom<&RawTrade> for Trade {
    type Error = ApiError;

    fn try_from(value: &RawTrade) -> Result<Self, Self::Error> {
        Ok(Self {
            s: value.s.clone(),
            p: value.p.parse::<f64>()?,
            q: value.q.parse::<f64>()?,
            t: value.t,
            d: value.d.clone(),
            i: value.i.clone(),
        })
    }
}

/// The raw trade response.
#[derive(Deserialize, Debug)]
pub struct RawTradeRes {
    /// e.g. BTCUSD-PERP.
    pub instrument_name: String,
    /// trade.{instrument_name}
    pub subscription: String,
    /// Always trade.
    pub channel: String,
    /// [`RawTrade`]
    pub data: Vec<RawTrade>,
}

/// The processed trade response.
#[derive(Debug)]
pub struct TradeRes {
    /// e.g. BTCUSD-PERP.
    pub instrument_name: String,
    /// trade.{instrument_name}
    pub subscription: String,
    /// Always trade.
    pub channel: String,
    /// [`Trade`]
    pub data: Vec<Trade>,
}

impl TryFrom<&RawTradeRes> for TradeRes {
    type Error = ApiError;

    fn try_from(value: &RawTradeRes) -> Result<Self, Self::Error> {
        let mut trades = vec![];

        for trade in &value.data {
            trades.push(Trade::try_from(trade)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name.clone(),
            subscription: value.subscription.clone(),
            channel: value.channel.clone(),
            data: trades,
        })
    }
}

impl TryFrom<RawTradeRes> for TradeRes {
    type Error = ApiError;

    fn try_from(value: RawTradeRes) -> Result<Self, Self::Error> {
        let mut trades = vec![];

        for trade in &value.data {
            trades.push(Trade::try_from(trade)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name,
            subscription: value.subscription,
            channel: value.channel,
            data: trades,
        })
    }
}
