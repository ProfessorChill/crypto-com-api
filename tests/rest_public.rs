#![cfg(feature = "rest")]

use anyhow::Result;
use crypto_com_api::{
    rest::public::{get_book, get_candlestick, get_instruments, get_ticker, get_trades},
    utils::config::Config,
};

fn create_config() -> Config {
    Config {
        rest_url: Some(
            url::Url::parse("https://uat-api.3ona.co/v2/")
                .expect("UAS Rest API Sandbox Invalid URL"),
        ),
        ..Default::default()
    }
}

#[tokio::test]
async fn rest_get_book() -> Result<()> {
    let config = create_config();

    let _ = get_book(&config, "BTC_USDT".to_owned(), 50).await?;

    Ok(())
}

#[tokio::test]
async fn rest_get_candlestick() -> Result<()> {
    let config = create_config();

    let _ = get_candlestick(&config, "BTC_USDT".to_owned(), "5m".to_owned()).await?;

    Ok(())
}

#[tokio::test]
async fn rest_get_instruments() -> Result<()> {
    let config = create_config();

    let _ = get_instruments(&config).await?;

    Ok(())
}

#[tokio::test]
async fn rest_get_ticker() -> Result<()> {
    let config = create_config();

    let _ = get_ticker(&config, Some("BTC_USDT".to_owned())).await?;

    Ok(())
}

#[tokio::test]
async fn rest_get_trades() -> Result<()> {
    let config = create_config();

    let _ = get_trades(&config, Some("BTC_USDT".to_owned())).await?;

    Ok(())
}
