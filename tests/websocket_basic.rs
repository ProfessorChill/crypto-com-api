#![cfg(feature = "websocket")]

use anyhow::Result;
use crypto_com_api::{
    controller::{Controller, ControllerBuilder, MarketWs, NoUserWs},
    websocket::{actions::Subscribe, WebsocketData},
};

async fn get_controller_sandbox() -> Result<Controller<NoUserWs, MarketWs>> {
    let websocket_user_url = url::Url::parse("wss://uat-stream.3ona.co/v2/market")
        .expect("UAT Sandbox Market API Invalid URL");

    Ok(ControllerBuilder::new()
        .with_market_websocket(websocket_user_url)
        .await?
        .build())
}

#[tokio::test]
async fn handshake() -> Result<()> {
    let controller = get_controller_sandbox().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };

        match res {
            WebsocketData::MarketHandshake => Ok(true),
            _ => Ok(false),
        }
    });

    let result = join_handle.await?;
    assert!(result.is_ok(), "handshake failed!");

    Ok(())
}

#[tokio::test]
async fn book_subscription() -> Result<()> {
    let mut controller = get_controller_sandbox().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };

        match res {
            WebsocketData::Book(_book) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_market_action(Box::new(Subscribe {
            channels: vec!["book.BTCUSD-PERP".to_string()],
        }))
        .await?;

    let result = join_handle.await?;
    assert!(result.is_ok(), "book subscription failed!");

    Ok(())
}

#[tokio::test]
async fn ticker_subscription() -> Result<()> {
    let mut controller = get_controller_sandbox().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };

        match res {
            WebsocketData::Ticker(_ticker) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_market_action(Box::new(Subscribe {
            channels: vec!["ticker.BTCUSD-PERP".to_string()],
        }))
        .await?;

    let result = join_handle.await?;
    assert!(result.is_ok(), "ticker subscription failed!");

    Ok(())
}

#[tokio::test]
async fn trade_subscription() -> Result<()> {
    let mut controller = get_controller_sandbox().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };

        match res {
            WebsocketData::Trade(_trade) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_market_action(Box::new(Subscribe {
            channels: vec!["trade.BTCUSD-PERP".to_string()],
        }))
        .await?;

    let result = join_handle.await?;
    assert!(result.is_ok(), "trade subscription failed!");

    Ok(())
}

#[tokio::test]
async fn candlestick_subscription() -> Result<()> {
    let mut controller = get_controller_sandbox().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };

        match res {
            WebsocketData::Candlestick(_candlestick) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_market_action(Box::new(Subscribe {
            channels: vec!["candlestick.M1.BTCUSD-PERP".to_string()],
        }))
        .await?;

    let result = join_handle.await?;
    assert!(result.is_ok(), "candlestick subscription failed!");

    Ok(())
}

#[tokio::test]
async fn otc_book_subscription() -> Result<()> {
    let mut controller = get_controller_sandbox().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };

        match res {
            WebsocketData::OtcBook(_otc_book) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_market_action(Box::new(Subscribe {
            channels: vec!["otc_book.BTC_USDT".to_string()],
        }))
        .await?;

    let result = join_handle.await?;
    assert!(result.is_ok(), "otc_book subscription failed!");

    Ok(())
}
