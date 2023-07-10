//! Data from [otc_book.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#otc_book-instrument_name)

use serde::Deserialize;

use crate::prelude::ApiError;

/// The raw OTC Book data response.
///
/// Level: (
///     Price of the level,
///     Total size of the level,
///     Number of standing orders in the level,
///     Expiry time of the level (milliseconds since the Unix epoch),
///     Unique ID of the level,
/// )
#[derive(Deserialize, Debug)]
pub struct RawOtcBook {
    /// Array of level
    pub bids: Vec<(String, String, String, u64, u64)>,
    /// Array of level
    pub asks: Vec<(String, String, String, u64, u64)>,
}

/// The processed OTC Book data response.
///
/// Level: (
///     Price of the level,
///     Total size of the level,
///     Number of standing orders in the level,
///     Expiry time of the level (milliseconds since the Unix epoch),
///     Unique ID of the level,
/// )
#[derive(Debug)]
pub struct OtcBook {
    /// Array of level
    pub bids: Vec<(f64, u64, u64, u64, u64)>,
    /// Array of level
    pub asks: Vec<(f64, u64, u64, u64, u64)>,
}

impl TryFrom<&RawOtcBook> for OtcBook {
    type Error = ApiError;

    fn try_from(value: &RawOtcBook) -> Result<Self, Self::Error> {
        let mut bids = vec![];

        for bid in &value.bids {
            bids.push((
                bid.0.parse::<f64>()?,
                bid.1.parse::<u64>()?,
                bid.2.parse::<u64>()?,
                bid.3,
                bid.4,
            ));
        }

        let mut asks = vec![];

        for ask in &value.asks {
            asks.push((
                ask.0.parse::<f64>()?,
                ask.1.parse::<u64>()?,
                ask.2.parse::<u64>()?,
                ask.3,
                ask.4,
            ));
        }

        Ok(Self { bids, asks })
    }
}

/// The raw OTC Book response.
#[derive(Deserialize, Debug)]
pub struct RawOtcBookRes {
    /// otc_book
    pub channel: String,
    /// otc_book.{instrument_name}
    pub subscription: String,
    /// Same as requested instrument_name.
    pub instrument_name: String,
    /// Timestamp of book publish (milliseconds since the Unix epoch).
    pub t: Option<u64>,
    /// [`RawOtcBook`]
    pub data: Option<Vec<RawOtcBook>>,
}

/// The processed OTC Book response.
#[derive(Debug)]
pub struct OtcBookRes {
    /// otc_book
    pub channel: String,
    /// otc_book.{instrument_name}
    pub subscription: String,
    /// Same as requested instrument_name.
    pub instrument_name: String,
    /// Timestamp of book publish (milliseconds since the Unix epoch).
    pub t: Option<u64>,
    /// [`OtcBook`]
    pub data: Option<Vec<OtcBook>>,
}

impl TryFrom<&RawOtcBookRes> for OtcBookRes {
    type Error = ApiError;

    fn try_from(value: &RawOtcBookRes) -> Result<Self, Self::Error> {
        Ok(Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            instrument_name: value.instrument_name.clone(),
            t: value.t,
            data: if let Some(ref data) = value.data {
                let mut books = vec![];

                for book in data {
                    books.push(OtcBook::try_from(book)?);
                }

                Some(books)
            } else {
                None
            },
        })
    }
}

impl TryFrom<RawOtcBookRes> for OtcBookRes {
    type Error = ApiError;

    fn try_from(value: RawOtcBookRes) -> Result<Self, Self::Error> {
        Ok(Self {
            channel: value.channel,
            subscription: value.subscription,
            instrument_name: value.instrument_name,
            t: value.t,
            data: if let Some(ref data) = value.data {
                let mut books = vec![];

                for book in data {
                    books.push(OtcBook::try_from(book)?);
                }

                Some(books)
            } else {
                None
            },
        })
    }
}
