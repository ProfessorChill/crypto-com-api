//! Data from [user.trade.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#user-trade-instrument_name)

use serde::Deserialize;

use crate::prelude::ApiError;

/// Raw user trade response data.
#[derive(Deserialize, Debug)]
pub struct RawUserTrade {
    /// BUY, SELL.
    pub side: String,
    /// Trade fee.
    pub fee: f64,
    /// Trade ID.
    pub trade_id: String,
    /// Trade creation time.
    pub create_time: u64,
    /// Executed trade price.
    pub traded_price: f64,
    /// Executed trade quantity.
    pub traded_quantity: f64,
    /// Currency used for the fees (e.g. CRO).
    pub fee_currency: String,
    /// Order ID.
    pub order_id: String,
}

/// Raw user trade response.
#[derive(Deserialize, Debug)]
pub struct RawUserTradeRes {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// user.trade.{instrument_name} -- even in the all case.
    pub subscription: String,
    /// user.trade
    pub channel: String,
    /// [`RawUserTrade`]
    pub data: Vec<RawUserTrade>,
}

/// Processed version of [`RawUserTrade`].
#[derive(Debug)]
pub struct UserTrade {
    /// BUY, SELL.
    pub side: String,
    /// Trade fee.
    pub fee: f64,
    /// Trade ID.
    pub trade_id: u64,
    /// Trade creation time.
    pub create_time: u64,
    /// Executed trade price.
    pub traded_price: f64,
    /// Executed trade quantity.
    pub traded_quantity: f64,
    /// Currency used for the fees (e.g. CRO).
    pub fee_currency: String,
    /// Order ID.
    pub order_id: u64,
}

impl TryFrom<&RawUserTrade> for UserTrade {
    type Error = ApiError;

    fn try_from(value: &RawUserTrade) -> Result<Self, Self::Error> {
        Ok(Self {
            side: value.side.clone(),
            fee: value.fee,
            trade_id: value.trade_id.parse::<u64>()?,
            create_time: value.create_time,
            traded_price: value.traded_price,
            traded_quantity: value.traded_quantity,
            fee_currency: value.fee_currency.clone(),
            order_id: value.order_id.parse::<u64>()?,
        })
    }
}

/// Processed version of [`RawUserTradeRes`].
#[derive(Debug)]
pub struct UserTradeRes {
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
    /// user.trade.{instrument_name} -- even in the all case.
    pub subscription: String,
    /// user.trade
    pub channel: String,
    /// [`UserTrade`]
    pub data: Vec<UserTrade>,
}

impl TryFrom<&RawUserTradeRes> for UserTradeRes {
    type Error = ApiError;

    fn try_from(value: &RawUserTradeRes) -> Result<Self, Self::Error> {
        let mut trades = vec![];

        for trade in &value.data {
            trades.push(UserTrade::try_from(trade)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name.clone(),
            subscription: value.subscription.clone(),
            channel: value.channel.clone(),
            data: trades,
        })
    }
}

impl TryFrom<RawUserTradeRes> for UserTradeRes {
    type Error = ApiError;

    fn try_from(value: RawUserTradeRes) -> Result<Self, Self::Error> {
        let mut trades = vec![];

        for trade in &value.data {
            trades.push(UserTrade::try_from(trade)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name,
            subscription: value.subscription,
            channel: value.channel,
            data: trades,
        })
    }
}
