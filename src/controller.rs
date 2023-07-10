//! Controller crate.

use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::api_response::ApiResponse;
use crate::prelude::{ActionStoreSender, DataReciever, DataSender};
use crate::utils::action::{Action, ActionStore};
use crate::utils::config::Config;
use crate::websocket::{market_api, user_api, WebsocketData};

/// No auth keys state.
#[derive(Debug)]
pub struct NoAuth;
/// Auth keys state.
#[derive(Debug)]
pub struct Auth;

/// No user websocket state.
#[derive(Debug)]
pub struct NoUserWs;
/// User websocket state.
#[derive(Debug)]
pub struct UserWs;

/// No market websocket state.
#[derive(Debug)]
pub struct NoMarketWs;
/// Market websocket state.
#[derive(Debug)]
pub struct MarketWs;

/// The main controller of the application, this contains all of the information, threads, and the
/// current ID used.
#[derive(Debug)]
pub struct Controller<U, M> {
    /// Application config.
    pub config: Config,
    /// Current ID, we don't want to use duplicates so a current count is stored.
    pub current_id: u64,
    /// Market actions sender, this is used to send actions to the market API.
    pub market_actions_tx: Option<ActionStoreSender>,
    /// Market actions `JoinHandle`.
    pub market_actions_handle: Option<JoinHandle<Result<()>>>,
    /// Market stream `JoinHandle`.
    pub market_stream_handle: Option<JoinHandle<Result<()>>>,
    /// User actions sender, this is used to send actions to the user API.
    pub user_actions_tx: Option<ActionStoreSender>,
    /// User actions `JoinHandle`.
    pub user_actions_handle: Option<JoinHandle<Result<()>>>,
    /// User stream `JoinHandle`.
    pub user_stream_handle: Option<JoinHandle<Result<()>>>,
    /// Data action sender.
    pub data_tx: DataSender,
    /// Data reciever.
    pub data_rx: DataReciever,
    /// Marker for user websocket.
    _mark_user_ws: PhantomData<U>,
    /// Marker for market websocket.
    _mark_market_ws: PhantomData<M>,
}

/// The builder for the [`Controller`] that contains all the threading and readers.
#[derive(Debug)]
pub struct ControllerBuilder<A, U, M> {
    /// Application config.
    pub config: Config,
    /// Market actions sender, this is used to send actions to the market API.
    pub market_actions_tx: Option<ActionStoreSender>,
    /// Market actions `JoinHandle`.
    pub market_actions_handle: Option<JoinHandle<Result<()>>>,
    /// Market stream `JoinHandle`.
    pub market_stream_handle: Option<JoinHandle<Result<()>>>,
    /// User actions sender, this is used to send actions to the user API.
    pub user_actions_tx: Option<ActionStoreSender>,
    /// User actions `JoinHandle`.
    pub user_actions_handle: Option<JoinHandle<Result<()>>>,
    /// User stream `JoinHandle`.
    pub user_stream_handle: Option<JoinHandle<Result<()>>>,
    /// Data action sender.
    pub data_tx: DataSender,
    /// Data reciever.
    pub data_rx: DataReciever,
    /// Marker for authorization.
    _mark_auth: PhantomData<A>,
    /// Marker for user websocket.
    _mark_user_ws: PhantomData<U>,
    /// Market for market websocket.
    _mark_market_ws: PhantomData<M>,
}

impl Default for ControllerBuilder<NoAuth, NoUserWs, NoMarketWs> {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerBuilder<NoAuth, NoUserWs, NoMarketWs> {
    /// The base controller builder with no actions.
    #[must_use]
    pub fn new() -> Self {
        let (data_tx, data_rx) = futures_channel::mpsc::unbounded::<ApiResponse<WebsocketData>>();

        Self {
            config: Config::default(),
            market_actions_tx: None,
            market_actions_handle: None,
            market_stream_handle: None,
            user_actions_tx: None,
            user_actions_handle: None,
            user_stream_handle: None,
            data_rx: Arc::new(Mutex::new(data_rx)),
            data_tx: Arc::new(Mutex::new(data_tx)),
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        }
    }
}

impl<A, U, M> ControllerBuilder<A, U, M> {
    /// With authorization (`api_key`, `secret_key`), required for user websocket.
    pub fn with_auth(
        mut self,
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> ControllerBuilder<Auth, U, M> {
        self.config.api_key = Some(api_key.into());
        self.config.secret_key = Some(secret_key.into());

        ControllerBuilder {
            config: self.config,
            market_actions_tx: self.market_actions_tx,
            market_actions_handle: self.market_actions_handle,
            market_stream_handle: self.market_stream_handle,
            user_actions_tx: self.user_actions_tx,
            user_actions_handle: self.user_actions_handle,
            user_stream_handle: self.user_stream_handle,
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        }
    }

    /// With the Market Websocket.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `initialize_market_stream` fails.
    pub async fn with_market_websocket(
        mut self,
        url: url::Url,
    ) -> Result<ControllerBuilder<A, U, MarketWs>> {
        self.config.websocket_market_api = Some(url);
        let (market_stream_handle, market_tx_arc) =
            market_api::initialize_market_stream(&self.config, Arc::clone(&self.data_tx)).await?;
        let (market_join_handle, market_actions_tx) =
            market_api::initialize_market_actions(Arc::clone(&market_tx_arc)).await;

        Ok(ControllerBuilder {
            config: self.config,
            market_actions_tx: Some(Arc::new(Mutex::new(market_actions_tx))),
            market_actions_handle: Some(market_join_handle),
            market_stream_handle: Some(market_stream_handle),
            user_actions_tx: self.user_actions_tx,
            user_actions_handle: self.user_actions_handle,
            user_stream_handle: self.user_stream_handle,
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        })
    }
}

impl<Auth, U, M> ControllerBuilder<Auth, U, M> {
    /// With the User Websocket, requires `api_key` and `secret_key` [`ControllerBuilder::with_auth`].
    pub async fn with_user_websocket(
        mut self,
        url: url::Url,
    ) -> Result<ControllerBuilder<Auth, UserWs, M>> {
        self.config.websocket_user_api = Some(url);
        let (user_stream_handle, user_tx_arc) =
            user_api::initialize_user_stream(&self.config, Arc::clone(&self.data_tx)).await?;
        let (user_actions_handle, user_actions_tx) =
            user_api::initialize_user_actions(Arc::clone(&user_tx_arc)).await;

        Ok(ControllerBuilder {
            config: self.config,
            market_actions_tx: self.market_actions_tx,
            market_actions_handle: self.market_actions_handle,
            market_stream_handle: self.market_stream_handle,
            user_actions_tx: Some(Arc::new(Mutex::new(user_actions_tx))),
            user_actions_handle: Some(user_actions_handle),
            user_stream_handle: Some(user_stream_handle),
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        })
    }
}

impl<A, U, M> ControllerBuilder<A, U, M> {
    /// Build a controller with auth and a user websocket but no market websocket.
    #[must_use]
    pub fn build(self) -> Controller<U, M> {
        Controller {
            config: self.config,
            current_id: 0,
            market_actions_tx: self.market_actions_tx,
            market_actions_handle: self.market_actions_handle,
            market_stream_handle: self.market_stream_handle,
            user_actions_tx: self.user_actions_tx,
            user_actions_handle: self.user_actions_handle,
            user_stream_handle: self.user_stream_handle,
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        }
    }
}

impl<UserWs, W> Controller<UserWs, W> {
    /// Push an action to the user websocket and increment the current ID to prevent duplicates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `unbounded_send` fails.
    pub async fn push_user_action(&mut self, action: Box<dyn Action>) -> Result<()> {
        if let Some(ref user_actions_tx) = self.user_actions_tx {
            user_actions_tx.lock().await.unbounded_send(ActionStore {
                id: self.current_id,
                action,
            })?;

            self.current_id += 1;
        }

        Ok(())
    }
}

impl<U, MarketWs> Controller<U, MarketWs> {
    /// Push an action to the market websocket and increment the current ID to prevent duplicates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `unbounded_send` fails.
    pub async fn push_market_action(&mut self, action: Box<dyn Action>) -> Result<()> {
        if let Some(ref market_actions_tx) = self.market_actions_tx {
            market_actions_tx.lock().await.unbounded_send(ActionStore {
                id: self.current_id,
                action,
            })?;

            self.current_id += 1;
        }

        Ok(())
    }
}

impl<U, W> Controller<U, W> {
    /// Get a clone of the data reader.
    #[must_use]
    pub fn get_data_reader(&self) -> DataReciever {
        Arc::clone(&self.data_rx)
    }

    /// Create a data listener.
    ///
    /// In order to use this function you must pass a lambda that returns `Ok(false)` to continue
    /// processing data, or `Ok(true)` to break the loop and stop processing data.
    pub fn listen<F>(&mut self, mut async_fn: F) -> JoinHandle<Result<()>>
    where
        F: FnMut(ApiResponse<WebsocketData>) -> Result<bool> + Send + 'static,
    {
        let data_rx_arc = self.get_data_reader();
        let market_actions_handle = self.market_actions_handle.take();
        let market_stream_handle = self.market_stream_handle.take();
        let user_actions_handle = self.user_actions_handle.take();
        let user_stream_handle = self.user_stream_handle.take();

        tokio::spawn(async move {
            let listener_join_handle = tokio::spawn(async move {
                let mut data_rx = data_rx_arc.lock().await;

                while let Some(data) = data_rx.next().await {
                    if async_fn(data)? {
                        break;
                    }
                }

                Ok(())
            });

            match (
                market_actions_handle,
                market_stream_handle,
                user_actions_handle,
                user_stream_handle,
            ) {
                (
                    Some(market_actions_handle),
                    Some(market_stream_handle),
                    Some(user_actions_handle),
                    Some(user_stream_handle),
                ) => tokio::select! {
                    listener = listener_join_handle => listener,
                    market_actions = market_actions_handle => market_actions,
                    market_stream = market_stream_handle => market_stream,
                    user_actions = user_actions_handle => user_actions,
                    user_stream = user_stream_handle => user_stream,
                }?,
                (Some(market_actions_handle), Some(market_stream_handle), None, None) => {
                    tokio::select! {
                        listener = listener_join_handle => listener,
                        market_actions = market_actions_handle => market_actions,
                        market_stream = market_stream_handle => market_stream,
                    }?
                }
                (None, None, Some(user_actions_handle), Some(user_stream_handle)) => {
                    tokio::select! {
                        listener = listener_join_handle => listener,
                        user_actions = user_actions_handle => user_actions,
                        user_stream = user_stream_handle => user_stream,
                    }?
                }
                _ => listener_join_handle.await?,
            }
        })
    }
}
