//! Data from [private/get-trades](https://exchange-docs.crypto.com/spot/index.html#private-get-trades)

use serde::Deserialize;

/// Trade list item.
#[derive(Deserialize, Debug, Clone)]
pub struct TradeListItem {
    /// BUY, SELL.
    pub side: String,
    /// e.g. ETH_CRO, BTC_USDT.
    pub instrument_name: String,
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
    /// Client Order ID.
    pub client_order_id: Option<String>,
    /// TAKER, MAKER.
    pub liquidity_indicator: Option<String>,
}

/// Trade list.
#[derive(Deserialize, Debug)]
pub struct Trades {
    /// An array of trades.
    pub trade_list: Vec<TradeListItem>,
}
