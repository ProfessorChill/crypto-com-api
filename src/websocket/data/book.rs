//! Data from [book.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#book-instrument_name)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw book data response.
///
/// Level: (
///     Price of the level,
///     Total size of the level,
///     Number of standing orders in the level,
/// )
#[derive(Deserialize, Debug)]
pub struct RawBook {
    /// Array of level.
    pub bids: Vec<(String, String, String)>,
    /// Array of level.
    pub asks: Vec<(String, String, String)>,
    /// Epoch millis of last book update.
    pub tt: u64,
    /// Epoch millis of message publish.
    pub t: u64,
    /// Update sequence.
    pub u: u64,
    /// Internal use only.
    pub cs: i64,
}

/// The processed data response.
///
/// Level: (
///     Price of the level,
///     Total size of the level,
///     Number of standing orders in the level,
/// )
#[derive(Debug, Clone)]
pub struct Book {
    /// Array of level.
    pub bids: Vec<(f64, f64, u64)>,
    /// Array of level.
    pub asks: Vec<(f64, f64, u64)>,
    /// Epoch millis of last book update.
    pub tt: u64,
    /// Epoch millis of message publish.
    pub t: u64,
    /// Update sequence.
    pub u: u64,
    /// Internal use only.
    pub cs: i64,
}

impl TryFrom<&RawBook> for Book {
    type Error = ApiError;

    fn try_from(value: &RawBook) -> Result<Self, Self::Error> {
        let mut bids = vec![];

        for bid in &value.bids {
            bids.push((
                bid.0.parse::<f64>()?,
                bid.1.parse::<f64>()?,
                bid.2.parse::<u64>()?,
            ));
        }

        let mut asks = vec![];

        for ask in &value.asks {
            asks.push((
                ask.0.parse::<f64>()?,
                ask.1.parse::<f64>()?,
                ask.2.parse::<u64>()?,
            ));
        }

        Ok(Self {
            bids,
            asks,
            tt: value.tt,
            t: value.t,
            u: value.u,
            cs: value.cs,
        })
    }
}

/// The raw book response.
#[derive(Deserialize, Debug)]
pub struct RawBookRes {
    /// Same as requested instrument_name.
    pub instrument_name: String,
    /// Same as requested channel.
    pub subscription: String,
    /// book
    pub channel: String,
    /// Default 50.
    pub depth: u64,
    /// [`RawBook`]
    pub data: Vec<RawBook>,
}

/// The processed book response.
#[derive(Debug)]
pub struct BookRes {
    /// Same as requested instrument_name.
    pub instrument_name: String,
    /// Same as requested channel.
    pub subscription: String,
    /// book
    pub channel: String,
    /// Default 50.
    pub depth: u64,
    /// [`Book`]
    pub data: Vec<Book>,
}

impl TryFrom<&RawBookRes> for BookRes {
    type Error = ApiError;

    fn try_from(value: &RawBookRes) -> Result<Self, Self::Error> {
        let mut books = vec![];

        for raw_book in &value.data {
            books.push(Book::try_from(raw_book)?);
        }

        Ok(Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            data: books,
            instrument_name: value.instrument_name.clone(),
            depth: value.depth,
        })
    }
}

impl TryFrom<RawBookRes> for BookRes {
    type Error = ApiError;

    fn try_from(value: RawBookRes) -> Result<Self, Self::Error> {
        let mut books = vec![];

        for raw_book in &value.data {
            books.push(Book::try_from(raw_book)?);
        }

        Ok(Self {
            channel: value.channel,
            subscription: value.subscription,
            data: books,
            instrument_name: value.instrument_name,
            depth: value.depth,
        })
    }
}
