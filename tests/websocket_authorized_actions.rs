#![cfg(all(feature = "websocket", feature = "test_authorized"))]

use anyhow::Result as AnyhowResult;
use crypto_com_api::websocket::actions::spot_trading_api::{
    GetAccountSummary, GetOpenOrders, GetOrderHistory, GetTrades, Paginated,
};
use crypto_com_api::websocket::actions::Auth;
use crypto_com_api::{
    controller::{Controller, ControllerBuilder, NoMarketWs, UserWs},
    prelude::ApiError,
    websocket::WebsocketData,
};

async fn get_controller() -> AnyhowResult<Controller<UserWs, NoMarketWs>> {
    let api_key = std::env::var("API_KEY").expect("API_KEY is not set in .env");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY is not set in .env");
    let websocket_user_api = std::env::var("WEBSOCKET_USER_API_ROOT_V2")
        .expect("WEBSOCKET_USER_API_ROOT_V2 is not set in .env");

    let websocket_user_api = url::Url::parse(&websocket_user_api)
        .expect("WEBSOCKET_USER_API_ROOT_v2 does not appear to be a valid URL");

    Ok(ControllerBuilder::new()
        .with_auth(api_key.clone(), secret_key.clone())
        .with_user_websocket(websocket_user_api)
        .await?
        .build())
}

#[tokio::test]
async fn auth() -> AnyhowResult<()> {
    dotenv::dotenv()?;

    let mut controller = get_controller().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };
        let code = data.code.unwrap_or(0);

        match res {
            WebsocketData::Auth => {
                if code > 0 {
                    anyhow::bail!(ApiError::AuthFail(code));
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    });

    controller
        .push_user_action(Box::new(Auth {
            api_key: std::env::var("API_KEY")?,
            secret_key: std::env::var("SECRET_KEY")?,
        }))
        .await?;

    let result = join_handle.await?;
    match result {
        Ok(()) => Ok(()),
        Err(err) => panic!("{err}"),
    }
}

#[tokio::test]
async fn get_account_summary() -> AnyhowResult<()> {
    dotenv::dotenv()?;

    let mut controller = get_controller().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };
        let code = data.code.unwrap_or(0);

        match res {
            WebsocketData::Auth => {
                if code > 0 {
                    anyhow::bail!(ApiError::AuthFail(code))
                }

                Ok(false)
            }
            WebsocketData::GetAccountSummary(_account_summary) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_user_action(Box::new(Auth {
            api_key: std::env::var("API_KEY")?,
            secret_key: std::env::var("SECRET_KEY")?,
        }))
        .await?;

    controller
        .push_user_action(Box::new(GetAccountSummary {
            currency: Some("CRO".to_string()),
        }))
        .await?;

    let result = join_handle.await?;
    match result {
        Ok(()) => Ok(()),
        Err(err) => panic!("{err}"),
    }
}

#[tokio::test]
async fn get_order_history() -> AnyhowResult<()> {
    dotenv::dotenv()?;

    let mut controller = get_controller().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };
        let code = data.code.unwrap_or(0);

        match res {
            WebsocketData::Auth => {
                if code > 0 {
                    anyhow::bail!(ApiError::AuthFail(code))
                }

                Ok(false)
            }
            WebsocketData::GetOrderHistory(_order_history) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_user_action(Box::new(Auth {
            api_key: std::env::var("API_KEY")?,
            secret_key: std::env::var("SECRET_KEY")?,
        }))
        .await?;

    controller
        .push_user_action(Box::new(GetOrderHistory(Paginated {
            instrument_name: None,
            start_ts: None,
            end_ts: None,
            page_size: None,
            page: None,
        })))
        .await?;

    let result = join_handle.await?;
    match result {
        Ok(()) => Ok(()),
        Err(err) => panic!("{err}"),
    }
}

#[tokio::test]
async fn get_open_orders() -> AnyhowResult<()> {
    dotenv::dotenv()?;

    let mut controller = get_controller().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };
        let code = data.code.unwrap_or(0);

        match res {
            WebsocketData::Auth => {
                if code > 0 {
                    anyhow::bail!(ApiError::AuthFail(code))
                }

                Ok(false)
            }
            WebsocketData::GetOpenOrders(_open_orders) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_user_action(Box::new(Auth {
            api_key: std::env::var("API_KEY")?,
            secret_key: std::env::var("SECRET_KEY")?,
        }))
        .await?;

    controller
        .push_user_action(Box::new(GetOpenOrders {
            instrument_name: None,
            page_size: None,
            page: None,
        }))
        .await?;

    let result = join_handle.await?;
    match result {
        Ok(()) => Ok(()),
        Err(err) => panic!("{err}"),
    }
}

#[tokio::test]
async fn get_trades() -> AnyhowResult<()> {
    dotenv::dotenv()?;

    let mut controller = get_controller().await?;

    let join_handle = controller.listen(move |data| {
        let Some(res) = &data.result else {
            return Ok(false);
        };
        let code = data.code.unwrap_or(0);

        match res {
            WebsocketData::Auth => {
                if code > 0 {
                    anyhow::bail!(ApiError::AuthFail(code))
                } else {
                    Ok(false)
                }
            }
            WebsocketData::GetTrades(_trades) => Ok(true),
            _ => Ok(false),
        }
    });

    controller
        .push_user_action(Box::new(Auth {
            api_key: std::env::var("API_KEY")?,
            secret_key: std::env::var("SECRET_KEY")?,
        }))
        .await?;

    controller
        .push_user_action(Box::new(GetTrades(Paginated {
            instrument_name: None,
            start_ts: None,
            end_ts: None,
            page_size: None,
            page: None,
        })))
        .await?;

    let result = join_handle.await?;
    match result {
        Ok(()) => Ok(()),
        Err(err) => panic!("{err}"),
    }
}
