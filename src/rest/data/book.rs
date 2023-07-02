//! Data from [public/get-book](https://exchange-docs.crypto.com/spot/index.html#public-get-book)

use serde::Deserialize;

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

impl From<&RawBook> for Book {
    fn from(value: &RawBook) -> Self {
        Self {
            bids: value
                .bids
                .iter()
                .map(|bid| {
                    (
                        bid.0.parse::<f64>().expect("Failed to parse f64 of price"),
                        bid.1
                            .parse::<f64>()
                            .expect("Failed to parse f64 of quantity"),
                        bid.2
                            .parse::<u64>()
                            .expect("Failed to parse u64 of number of orderes"),
                    )
                })
                .collect::<Vec<(f64, f64, u64)>>(),
            asks: value
                .asks
                .iter()
                .map(|ask| {
                    (
                        ask.0.parse::<f64>().expect("Failed to parse f64 of price"),
                        ask.1
                            .parse::<f64>()
                            .expect("Failed to parse f64 of quantity"),
                        ask.2
                            .parse::<u64>()
                            .expect("Failed to parse u64 of number of orderes"),
                    )
                })
                .collect::<Vec<(f64, f64, u64)>>(),
            t: value.t,
        }
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

impl From<&RawBookRes> for BookRes {
    fn from(value: &RawBookRes) -> Self {
        Self {
            instrument_name: value.instrument_name.clone(),
            depth: value.depth,
            data: value.data.iter().map(Book::from).collect::<Vec<Book>>(),
        }
    }
}
