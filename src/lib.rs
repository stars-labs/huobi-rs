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
