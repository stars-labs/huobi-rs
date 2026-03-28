use crate::config::Config;
use crate::models::websocket::OrdersUpdate;
use crate::ws::WsClient;

/// WebSocket orders client (private, v2 API, auth required).
pub struct WebSocketOrdersClient {
    ws: WsClient,
}

impl WebSocketOrdersClient {
    pub fn new(config: &Config) -> Self {
        let url = format!("{}/ws/v2", config.ws_url);
        let ws = match (&config.access_key, &config.secret_key) {
            (Some(ak), Some(sk)) => WsClient::new(url).with_auth(ak.clone(), sk.clone()),
            _ => WsClient::new(url),
        };
        Self { ws }
    }

    /// Subscribe to order updates for a symbol.
    pub async fn sub_orders<F>(
        &self,
        symbol: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(OrdersUpdate) + Send + 'static,
    {
        let channel = format!("orders#{}", symbol);
        self.ws
            .subscribe_with_callback::<OrdersUpdate, F>(&channel, callback)
            .await
    }

    /// Subscribe to all order updates.
    pub async fn sub_orders_all<F>(
        &self,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(OrdersUpdate) + Send + 'static,
    {
        self.ws
            .subscribe_with_callback::<OrdersUpdate, F>("orders", callback)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_orders_client_creation() {
        let config = Config::new("test-key", "test-secret");
        let _client = WebSocketOrdersClient::new(&config);
    }

    #[test]
    fn test_ws_orders_client_public() {
        let config = Config::public();
        let _client = WebSocketOrdersClient::new(&config);
    }
}
