use crate::config::Config;
use crate::models::websocket::TradeClearing;
use crate::ws::WsClient;

/// WebSocket trade client (private, v2 API, auth required).
pub struct WebSocketTradeClient {
    ws: WsClient,
}

impl WebSocketTradeClient {
    pub fn new(config: &Config) -> Self {
        let url = format!("{}/ws/v2", config.ws_url);
        let ws = match (&config.access_key, &config.secret_key) {
            (Some(ak), Some(sk)) => WsClient::new(url).with_auth(ak.clone(), sk.clone()),
            _ => WsClient::new(url),
        };
        Self { ws }
    }

    /// Subscribe to trade clearing updates for a symbol.
    pub async fn sub_trade_clearing<F>(
        &self,
        symbol: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(TradeClearing) + Send + 'static,
    {
        let channel = format!("trade.clearing#{}", symbol);
        self.ws
            .subscribe_with_callback::<TradeClearing, F>(&channel, callback)
            .await
    }

    /// Subscribe to all trade clearing updates.
    pub async fn sub_trade_clearing_all<F>(
        &self,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(TradeClearing) + Send + 'static,
    {
        self.ws
            .subscribe_with_callback::<TradeClearing, F>("trade.clearing", callback)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_trade_client_creation() {
        let config = Config::new("test-key", "test-secret");
        let _client = WebSocketTradeClient::new(&config);
    }

    #[test]
    fn test_ws_trade_client_public() {
        let config = Config::public();
        let _client = WebSocketTradeClient::new(&config);
    }
}
