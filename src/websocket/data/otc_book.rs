//! Data from [otc_book.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#otc_book-instrument_name)

use serde::Deserialize;

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

impl From<&RawOtcBook> for OtcBook {
    fn from(value: &RawOtcBook) -> Self {
        Self {
            bids: value
                .bids
                .iter()
                .map(|bid| {
                    (
                        bid.0
                            .parse::<f64>()
                            .expect("Failed to parse price of the level"),
                        bid.1
                            .parse::<u64>()
                            .expect("Failed to parse total size of the level"),
                        bid.2
                            .parse::<u64>()
                            .expect("Failed to parse number of standing orders in the level"),
                        bid.3,
                        bid.4,
                    )
                })
                .collect::<Vec<(f64, u64, u64, u64, u64)>>(),
            asks: value
                .asks
                .iter()
                .map(|ask| {
                    (
                        ask.0
                            .parse::<f64>()
                            .expect("Failed to parse price of the level"),
                        ask.1
                            .parse::<u64>()
                            .expect("Failed to parse total size of the level"),
                        ask.2
                            .parse::<u64>()
                            .expect("Failed to parse number of standing orders in the level"),
                        ask.3,
                        ask.4,
                    )
                })
                .collect::<Vec<(f64, u64, u64, u64, u64)>>(),
        }
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

impl From<&RawOtcBookRes> for OtcBookRes {
    fn from(value: &RawOtcBookRes) -> Self {
        Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            instrument_name: value.instrument_name.clone(),
            t: value.t,
            data: value
                .data
                .as_ref()
                .map(|data| data.iter().map(OtcBook::from).collect::<Vec<OtcBook>>()),
        }
    }
}

impl From<RawOtcBookRes> for OtcBookRes {
    fn from(value: RawOtcBookRes) -> Self {
        Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            instrument_name: value.instrument_name.clone(),
            t: value.t,
            data: value
                .data
                .map(|data| data.iter().map(OtcBook::from).collect::<Vec<OtcBook>>()),
        }
    }
}
