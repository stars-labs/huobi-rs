use crate::config::Config;
use crate::models::websocket::AccountsUpdate;
use crate::ws::WsClient;

/// WebSocket asset client (private, v2 API, auth required).
pub struct WebSocketAssetClient {
    ws: WsClient,
}

impl WebSocketAssetClient {
    pub fn new(config: &Config) -> Self {
        let url = format!("{}/ws/v2", config.ws_url);
        let ws = match (&config.access_key, &config.secret_key) {
            (Some(ak), Some(sk)) => WsClient::new(url).with_auth(ak.clone(), sk.clone()),
            _ => WsClient::new(url),
        };
        Self { ws }
    }

    /// Subscribe to account changes.
    pub async fn sub_accounts<F>(
        &self,
        account_id: i64,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(AccountsUpdate) + Send + 'static,
    {
        let channel = format!("accounts.update#{}", account_id);
        self.ws
            .subscribe_with_callback::<AccountsUpdate, F>(&channel, callback)
            .await
    }

    /// Subscribe to all account changes.
    pub async fn sub_accounts_all<F>(
        &self,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(AccountsUpdate) + Send + 'static,
    {
        self.ws
            .subscribe_with_callback::<AccountsUpdate, F>("accounts.update", callback)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_asset_client_creation() {
        let config = Config::new("test-key", "test-secret");
        let _client = WebSocketAssetClient::new(&config);
    }

    #[test]
    fn test_ws_asset_client_public() {
        let config = Config::public();
        let _client = WebSocketAssetClient::new(&config);
    }
}
