//! Data from [public/get-book](https://exchange-docs.crypto.com/spot/index.html#public-get-book)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw book data response.
///
/// Level: (
///     Price,
///     Quantity,
///     Number of Orders,
/// )
#[derive(Deserialize, Debug)]
pub struct RawBook {
    /// Array of level.
    pub bids: Vec<(String, String, String)>,
    /// Array of level.
    pub asks: Vec<(String, String, String)>,
    /// Timestamp of the data.
    pub t: Option<u64>,
}

/// The raw book response.
#[derive(Deserialize, Debug)]
pub struct RawBookRes {
    /// e.g. BTC_USDT, ETH_CRO, etc.
    pub instrument_name: String,
    /// Number of bids and asks to return (up to 50).
    pub depth: u64,
    /// [`RawBook`]
    pub data: Vec<RawBook>,
}

/// The processed data response.
///
/// Level: (
///     Price,
///     Quantity,
///     Number of Orders,
/// )
#[derive(Debug, Default)]
pub struct Book {
    /// Array of level.
    pub bids: Vec<(f64, f64, u64)>,
    /// Array of level.
    pub asks: Vec<(f64, f64, u64)>,
    /// Timestamp of the data.
    pub t: Option<u64>,
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
            t: value.t,
        })
    }
}

/// The processed book response.
#[derive(Debug, Default)]
pub struct BookRes {
    /// e.g. BTC_USDT, ETH_CRO, etc.
    pub instrument_name: String,
    /// Number of bids and asks to return (up to 50).
    pub depth: u64,
    /// [`Book`]
    pub data: Vec<Book>,
}

impl TryFrom<&RawBookRes> for BookRes {
    type Error = ApiError;

    fn try_from(value: &RawBookRes) -> Result<Self, Self::Error> {
        let mut books = vec![];

        for book in &value.data {
            books.push(Book::try_from(book)?);
        }

        Ok(Self {
            instrument_name: value.instrument_name.clone(),
            depth: value.depth,
            data: books,
        })
    }
}
