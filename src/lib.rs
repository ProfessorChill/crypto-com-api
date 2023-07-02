#![allow(
    clippy::module_name_repetitions,
    clippy::cast_sign_loss,
    rustdoc::broken_intra_doc_links
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(unused_must_use)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(loom, allow(dead_code, unreachable_pub))]

//! A crypto.com API system for both REST and `WebSocket` in accordance to the
//! [Crypto.com docs](https://exchange-docs.crypto.com/spot/index.html).
//!
//! To start using the Websocket API, refer to [`crate::controller::Controller::new`]
//!
//! To start using the REST API, refer to [`crate::rest`]

pub mod api_request;
pub mod api_response;
#[cfg(feature = "websocket")]
pub mod controller;
pub mod error;
pub mod prelude;
#[cfg(feature = "rest")]
pub mod rest;
pub mod utils;
#[cfg(feature = "websocket")]
pub mod websocket;
