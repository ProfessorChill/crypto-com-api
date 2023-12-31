//! Data from [public/get-instruments](https://exchange-docs.crypto.com/spot/index.html#public-get-instruments)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw instrument data response.
#[derive(Deserialize, Debug)]
pub struct RawInstrument {
    /// e.g. BTC_USDT.
    pub instrument_name: String,
    /// e.g. USDT.
    pub quote_currency: String,
    /// e.g. BTC.
    pub base_currency: String,
    /// Maximum decimal places for specifying price.
    pub price_decimals: u8,
    /// Maximum decimal places for specifying quantity.
    pub quantity_decimals: u8,
    /// always false.
    pub margin_trading_enabled: bool,
    /// always false.
    pub margin_trading_enabled_5x: bool,
    /// always false.
    pub margin_trading_enabled_10x: bool,
    /// Maximum quantity.
    pub max_quantity: String,
    /// Minimum quantity.
    pub min_quantity: String,
    /// Maximum price.
    pub max_price: String,
    /// Minimum price.
    pub min_price: String,
    /// Instrument last update time (Unix timestamp).
    pub last_update_date: u64,
    /// Quantity tick size.
    pub quantity_tick_size: String,
    /// Price tick size.
    pub price_tick_size: String,
}

/// The raw instrument response.
#[derive(Deserialize, Debug)]
pub struct RawInstrumentsRes {
    /// [`RawInstrument`]
    pub instruments: Vec<RawInstrument>,
}

/// The processed instrument data response.
#[derive(Debug, Default)]
pub struct Instrument {
    /// e.g. BTC_USDT.
    pub instrument_name: String,
    /// e.g. USDT.
    pub quote_currency: String,
    /// e.g. BTC.
    pub base_currency: String,
    /// Maximum decimal places for specifying price.
    pub price_decimals: u8,
    /// Maximum decimal places for specifying quantity.
    pub quantity_decimals: u8,
    /// always false.
    pub margin_trading_enabled: bool,
    /// always false.
    pub margin_trading_enabled_5x: bool,
    /// always false.
    pub margin_trading_enabled_10x: bool,
    /// Maximum quantity.
    pub max_quantity: f64,
    /// Minimum quantity.
    pub min_quantity: f64,
    /// Maximum price.
    pub max_price: f64,
    /// Minimum price.
    pub min_price: f64,
    /// Instrument last update time (Unix timestamp).
    pub last_update_date: u64,
    /// Quantity tick size.
    pub quantity_tick_size: f64,
    /// Price tick size.
    pub price_tick_size: f64,
}

impl TryFrom<&RawInstrument> for Instrument {
    type Error = ApiError;

    fn try_from(value: &RawInstrument) -> Result<Self, Self::Error> {
        Ok(Self {
            instrument_name: value.instrument_name.clone(),
            quote_currency: value.quote_currency.clone(),
            base_currency: value.base_currency.clone(),
            price_decimals: value.price_decimals,
            quantity_decimals: value.quantity_decimals,
            margin_trading_enabled: value.margin_trading_enabled,
            margin_trading_enabled_5x: value.margin_trading_enabled_5x,
            margin_trading_enabled_10x: value.margin_trading_enabled_10x,
            max_quantity: value.max_quantity.parse::<f64>()?,
            min_quantity: value.min_quantity.parse::<f64>()?,
            max_price: value.max_price.parse::<f64>()?,
            min_price: value.min_price.parse::<f64>()?,
            last_update_date: value.last_update_date,
            quantity_tick_size: value.quantity_tick_size.parse::<f64>()?,
            price_tick_size: value.price_tick_size.parse::<f64>()?,
        })
    }
}

/// The instrument response.
#[derive(Debug, Default)]
pub struct InstrumentsRes {
    /// [`Instrument`]
    pub instruments: Vec<Instrument>,
}

impl TryFrom<&RawInstrumentsRes> for InstrumentsRes {
    type Error = ApiError;

    fn try_from(value: &RawInstrumentsRes) -> Result<Self, Self::Error> {
        let mut instruments = vec![];

        for instrument in &value.instruments {
            instruments.push(Instrument::try_from(instrument)?);
        }

        Ok(Self { instruments })
    }
}

impl TryFrom<RawInstrumentsRes> for InstrumentsRes {
    type Error = ApiError;

    fn try_from(value: RawInstrumentsRes) -> Result<Self, Self::Error> {
        let mut instruments = vec![];

        for instrument in &value.instruments {
            instruments.push(Instrument::try_from(instrument)?);
        }

        Ok(Self { instruments })
    }
}
