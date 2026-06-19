//! # huobi-rs
//!
//! An async Rust SDK for the [Huobi/HTX](https://www.htx.com/) Spot API v3,
//! covering REST and WebSocket endpoints for market data, trading, accounts,
//! wallets, margin, algo orders, and sub-user management.
//!
//! ## Quick start
//!
//! Public market data requires no API keys:
//!
//! ```no_run
//! use huobi_rs::{Config, MarketClient};
//! use huobi_rs::request::market::TickerRequest;
//!
//! # async fn run() -> huobi_rs::Result<()> {
//! let client = MarketClient::new(Config::public());
//! let ticker = client.get_ticker(&TickerRequest::new("btcusdt")).await?;
//! println!("BTC/USDT last price: {}", ticker.close);
//! # Ok(())
//! # }
//! ```
//!
//! Private endpoints need credentials supplied via [`Config::new`]:
//!
//! ```no_run
//! use huobi_rs::{Config, AccountClient};
//!
//! # async fn run() -> huobi_rs::Result<()> {
//! let config = Config::new("access-key", "secret-key");
//! let accounts = AccountClient::new(config).get_accounts().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Clients
//!
//! Each API domain has a dedicated client constructed from a [`Config`]:
//! [`AccountClient`], [`MarketClient`], [`TradeClient`], [`WalletClient`],
//! [`ReferenceClient`], [`AlgoClient`], [`CrossMarginClient`],
//! [`IsolatedMarginClient`], [`SubUserClient`], and the WebSocket clients
//! [`WebSocketMarketClient`], [`WebSocketOrdersClient`],
//! [`WebSocketTradeClient`], and [`WebSocketAssetClient`].
//!
//! Errors are reported through [`HuobiError`], and all fallible calls return
//! the crate-level [`Result`] alias.

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod http;
pub mod models;
pub mod request;
pub mod ws;

pub use client::{
    AccountClient, AlgoClient, CrossMarginClient, IsolatedMarginClient, MarketClient,
    ReferenceClient, SubUserClient, TradeClient, WalletClient, WebSocketAssetClient,
    WebSocketMarketClient, WebSocketOrdersClient, WebSocketTradeClient,
};
pub use config::Config;
pub use error::HuobiError;

/// Result type alias using HuobiError
pub type Result<T> = std::result::Result<T, HuobiError>;
