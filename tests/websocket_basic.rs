#![cfg(feature = "websocket")]

use anyhow::Result;
use crypto_com_api::{
    controller::{Controller, ControllerBuilder, MarketWs, NoUserWs},
    websocket::{actions::Subscribe, WebsocketData},
};
use futures_util::StreamExt;

async fn get_controller_sandbox() -> Result<Controller<NoUserWs, MarketWs>> {
    let websocket_user_url = url::Url::parse("wss://uat-stream.3ona.co/v2/market")
        .expect("UAT Sandbox Market API Invalid URL");

    Ok(ControllerBuilder::new()
        .with_market_websocket(websocket_user_url)
        .await?
        .build())
}

const DATA_SUCCESS_LIMIT: usize = 10;

#[tokio::test]
async fn handshake() -> Result<()> {
    let controller = get_controller_sandbox().await?;
    let data_rx_arc = controller.get_data_reader();

    let mut data_till_fail = 0;

    let join_handle: tokio::task::JoinHandle<Result<()>> = {
        let data_rx_arc = data_rx_arc.clone();

        tokio::spawn(async move {
            let mut data_rx = data_rx_arc.lock().await;

            while let Some(data) = data_rx.next().await {
                if data_till_fail >= 10 {
                    panic!("Handshake never arrived.");
                }

                match data {
                    WebsocketData::MarketHandshake => return Ok(()),
                    _ => {}
                }

                data_till_fail += 1;
            }

            Ok(())
        })
    };

    let result = join_handle.await?;
    assert!(result.is_ok(), "handshake failed!");

    Ok(())
}

#[tokio::test]
async fn book_subscription() -> Result<()> {
    let mut controller = get_controller_sandbox().await?;
    let data_rx_arc = controller.get_data_reader();

    let mut data_success = 0;

    let join_handle: tokio::task::JoinHandle<Result<()>> = {
        let data_rx_arc = data_rx_arc.clone();

        tokio::spawn(async move {
            let mut data_rx = data_rx_arc.lock().await;

            while let Some(data) = data_rx.next().await {
                match data {
                    WebsocketData::Book(_book) => data_success += 1,
                    _ => {}
                }

                if data_success > DATA_SUCCESS_LIMIT {
                    return Ok(());
                }
            }

            Ok(())
        })
    };

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
    let data_rx_arc = controller.get_data_reader();

    let mut data_success = 0;

    let join_handle: tokio::task::JoinHandle<Result<()>> = {
        let data_rx_arc = data_rx_arc.clone();

        tokio::spawn(async move {
            let mut data_rx = data_rx_arc.lock().await;

            while let Some(data) = data_rx.next().await {
                match data {
                    WebsocketData::Ticker(_ticker) => data_success += 1,
                    _ => {}
                }

                if data_success > DATA_SUCCESS_LIMIT {
                    return Ok(());
                }
            }

            Ok(())
        })
    };

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
    let data_rx_arc = controller.get_data_reader();

    let mut data_success = 0;

    let join_handle: tokio::task::JoinHandle<Result<()>> = {
        let data_rx_arc = data_rx_arc.clone();

        tokio::spawn(async move {
            let mut data_rx = data_rx_arc.lock().await;

            while let Some(data) = data_rx.next().await {
                match data {
                    WebsocketData::Trade(_trade) => data_success += 1,
                    _ => {}
                }

                if data_success > DATA_SUCCESS_LIMIT {
                    return Ok(());
                }
            }

            Ok(())
        })
    };

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
    let data_rx_arc = controller.get_data_reader();

    let mut data_success = 0;

    let join_handle: tokio::task::JoinHandle<Result<()>> = {
        let data_rx_arc = data_rx_arc.clone();

        tokio::spawn(async move {
            let mut data_rx = data_rx_arc.lock().await;

            while let Some(data) = data_rx.next().await {
                match data {
                    WebsocketData::Candlestick(_candlestick) => data_success += 1,
                    _ => {}
                }

                if data_success > DATA_SUCCESS_LIMIT {
                    return Ok(());
                }
            }

            Ok(())
        })
    };

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
    let data_rx_arc = controller.get_data_reader();

    let join_handle: tokio::task::JoinHandle<Result<()>> = {
        let data_rx_arc = data_rx_arc.clone();

        tokio::spawn(async move {
            let mut data_rx = data_rx_arc.lock().await;

            while let Some(data) = data_rx.next().await {
                match data {
                    WebsocketData::OtcBook(_otc_book) => return Ok(()),
                    _ => {}
                }
            }

            Ok(())
        })
    };

    controller
        .push_market_action(Box::new(Subscribe {
            channels: vec!["otc_book.BTC_USDT".to_string()],
        }))
        .await?;

    let result = join_handle.await?;
    assert!(result.is_ok(), "otc_book subscription failed!");

    Ok(())
}
