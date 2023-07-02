//! Controller crate.

use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

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
    /// User actions sender, this is used to send actions to the user API.
    pub user_actions_tx: Option<ActionStoreSender>,
    /// Data action sender.
    pub data_tx: DataSender,
    /// Data reciever.
    pub data_rx: DataReciever,
    _mark_user_ws: PhantomData<U>,
    _mark_market_ws: PhantomData<M>,
}

/// The builder for the [`Controller`] that contains all the threading and readers.
#[derive(Debug)]
pub struct ControllerBuilder<A, U, M> {
    /// Application config.
    pub config: Config,
    /// Market actions sender, this is used to send actions to the market API.
    pub market_actions_tx: Option<ActionStoreSender>,
    /// User actions sender, this is used to send actions to the user API.
    pub user_actions_tx: Option<ActionStoreSender>,
    /// Data action sender.
    pub data_tx: DataSender,
    /// Data reciever.
    pub data_rx: DataReciever,
    _mark_auth: PhantomData<A>,
    _mark_user_ws: PhantomData<U>,
    _mark_market_ws: PhantomData<M>,
}

impl ControllerBuilder<NoAuth, NoUserWs, NoMarketWs> {
    /// The base controller builder with no actions.
    pub fn new() -> Self {
        let (data_tx, data_rx) = futures_channel::mpsc::unbounded::<WebsocketData>();

        Self {
            config: Config::default(),
            market_actions_tx: None,
            user_actions_tx: None,
            data_rx: Arc::new(Mutex::new(data_rx)),
            data_tx: Arc::new(Mutex::new(data_tx)),
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        }
    }
}

impl<A, U, M> ControllerBuilder<A, U, M> {
    /// With authorization (api_key, secret_key), required for user websocket.
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
            user_actions_tx: self.user_actions_tx,
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        }
    }

    /// With the Market Websocket.
    pub async fn with_market_websocket(
        mut self,
        url: url::Url,
    ) -> Result<ControllerBuilder<A, U, MarketWs>> {
        self.config.websocket_market_api = Some(url);
        let market_tx_arc =
            market_api::initialize_market_stream(&self.config, self.data_tx.clone()).await?;
        let market_actions_tx = market_api::initialize_market_actions(market_tx_arc.clone()).await;

        Ok(ControllerBuilder {
            config: self.config,
            market_actions_tx: Some(Arc::new(Mutex::new(market_actions_tx))),
            user_actions_tx: self.user_actions_tx,
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_auth: PhantomData,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        })
    }
}

impl<Auth, U, M> ControllerBuilder<Auth, U, M> {
    /// With the User Websocket, requires api_key and secret_key [`ControllerBuilder::with_auth`].
    pub async fn with_user_websocket(
        mut self,
        url: url::Url,
    ) -> Result<ControllerBuilder<Auth, UserWs, M>> {
        self.config.websocket_user_api = Some(url);
        let user_tx_arc =
            user_api::initialize_user_stream(&self.config, self.data_tx.clone()).await?;
        let user_actions_tx = user_api::initialize_user_actions(user_tx_arc.clone()).await;

        Ok(ControllerBuilder {
            config: self.config,
            market_actions_tx: self.market_actions_tx,
            user_actions_tx: Some(Arc::new(Mutex::new(user_actions_tx))),
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
    pub fn build(self) -> Controller<U, M> {
        Controller {
            config: self.config,
            current_id: 0,
            market_actions_tx: self.market_actions_tx,
            user_actions_tx: self.user_actions_tx,
            data_tx: self.data_tx,
            data_rx: self.data_rx,
            _mark_user_ws: PhantomData,
            _mark_market_ws: PhantomData,
        }
    }
}

impl<UserWs, W> Controller<UserWs, W> {
    /// Push an action to the user websocket and increment the current ID to prevent duplicates.
    pub async fn push_user_action(&mut self, action: Box<dyn Action>) -> Result<()> {
        if let Some(user_actions_tx) = &self.user_actions_tx {
            let user_actions = user_actions_tx.lock().await;

            user_actions.unbounded_send(ActionStore {
                id: self.current_id,
                action,
            })?;

            self.current_id += 1;
        }

        Ok(())
    }
}

impl<U, MarketWs> Controller<U, MarketWs> {
    /// Push an action to the market websocket and increment the current ID to prevent duplicates;
    pub async fn push_market_action(&mut self, action: Box<dyn Action>) -> Result<()> {
        if let Some(market_actions_tx) = &self.market_actions_tx {
            let market_actions = market_actions_tx.lock().await;

            market_actions.unbounded_send(ActionStore {
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
    pub fn get_data_reader(&self) -> DataReciever {
        self.data_rx.clone()
    }
}
