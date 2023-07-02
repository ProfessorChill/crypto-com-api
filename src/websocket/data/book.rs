//! Data from [book.{instrument_name}](https://exchange-docs.crypto.com/spot/index.html#book-instrument_name)

use serde::Deserialize;

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
#[derive(Debug)]
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

impl From<&RawBook> for Book {
    fn from(value: &RawBook) -> Self {
        Self {
            bids: value
                .bids
                .iter()
                .map(|bid| {
                    (
                        bid.0
                            .parse::<f64>()
                            .expect("Failed to parse f64 of price of the level"),
                        bid.1
                            .parse::<f64>()
                            .expect("Failed to parse f64 of total size of the level"),
                        bid.2.parse::<u64>().expect(
                            "Failed to parse u64 of number of standing orders in the level",
                        ),
                    )
                })
                .collect::<Vec<(f64, f64, u64)>>(),
            asks: value
                .asks
                .iter()
                .map(|ask| {
                    (
                        ask.0
                            .parse::<f64>()
                            .expect("Failed to parse f64 of price of the level"),
                        ask.1
                            .parse::<f64>()
                            .expect("Failed to parse f64 of total size of the level"),
                        ask.2.parse::<u64>().expect(
                            "Failed to parse u64 of number of standing orders in the level",
                        ),
                    )
                })
                .collect::<Vec<(f64, f64, u64)>>(),
            tt: value.tt,
            t: value.t,
            u: value.u,
            cs: value.cs,
        }
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

impl From<&RawBookRes> for BookRes {
    fn from(value: &RawBookRes) -> Self {
        Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            data: value.data.iter().map(Book::from).collect::<Vec<Book>>(),
            instrument_name: value.instrument_name.clone(),
            depth: value.depth,
        }
    }
}

impl From<RawBookRes> for BookRes {
    fn from(value: RawBookRes) -> Self {
        Self {
            channel: value.channel.clone(),
            subscription: value.subscription.clone(),
            data: value.data.iter().map(Book::from).collect::<Vec<Book>>(),
            instrument_name: value.instrument_name.clone(),
            depth: value.depth,
        }
    }
}
