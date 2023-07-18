//! Data from the REST API.

pub mod book;
pub mod candlestick;
pub mod create_withdrawal;
pub mod currency_networks;
pub mod deposit_history;
pub mod instruments;
pub mod ticker;
pub mod trades;
pub mod withdrawal_history;

pub use book::*;
pub use candlestick::*;
pub use create_withdrawal::*;
pub use currency_networks::*;
pub use deposit_history::*;
pub use instruments::*;
pub use ticker::*;
pub use trades::*;
pub use withdrawal_history::*;
