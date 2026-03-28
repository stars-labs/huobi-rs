use crate::error::HuobiError;
use crate::Result;
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

/// A simple WebSocket client for Huobi.
pub struct WsClient {
    url: String,
    access_key: Option<String>,
    secret_key: Option<String>,
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl WsClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            access_key: None,
            secret_key: None,
        }
    }

    pub fn with_auth(mut self, access_key: String, secret_key: String) -> Self {
        self.access_key = Some(access_key);
        self.secret_key = Some(secret_key);
        self
    }

    /// Connect to the WebSocket server.
    pub async fn connect(&self) -> Result<WsStream> {
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .map_err(|e| HuobiError::WebSocket(e.to_string()))?;
        Ok(ws_stream)
    }

    /// Subscribe to a channel and receive messages via a channel.
    pub async fn subscribe<T: DeserializeOwned + Send + 'static>(
        &self,
        channel: &str,
    ) -> Result<mpsc::UnboundedReceiver<T>> {
        let (tx, rx) = mpsc::unbounded_channel();
        let channel = channel.to_string();
        let url = self.url.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::run_subscription(url, &channel, tx).await {
                tracing::error!("WebSocket subscription error: {}", e);
            }
        });

        Ok(rx)
    }

    /// Subscribe with callback (like the C++ SDK pattern).
    pub async fn subscribe_with_callback<T, F>(
        &self,
        channel: &str,
        callback: F,
    ) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        F: Fn(T) + Send + 'static,
    {
        let mut rx = self.subscribe::<T>(channel).await?;
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                callback(msg);
            }
        });
        Ok(())
    }

    async fn run_subscription<T: DeserializeOwned>(
        url: String,
        channel: &str,
        tx: mpsc::UnboundedSender<T>,
    ) -> Result<()> {
        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| HuobiError::WebSocket(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();

        // Send subscription message
        let sub_msg = serde_json::json!({
            "sub": channel
        });
        write
            .send(Message::Text(sub_msg.to_string().into()))
            .await
            .map_err(|e| HuobiError::WebSocket(e.to_string()))?;

        // Process incoming messages
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Skip ping/pong and subscription confirmations
                    if text.contains("ping") {
                        let pong = text.replace("ping", "pong");
                        let _ = write.send(Message::Text(pong.into())).await;
                        continue;
                    }
                    if let Ok(data) = serde_json::from_str::<T>(&text) {
                        if tx.send(data).is_err() {
                            break;
                        }
                    }
                }
                Ok(Message::Binary(bin)) => {
                    if let Ok(text) = String::from_utf8(bin.to_vec()) {
                        if let Ok(data) = serde_json::from_str::<T>(&text) {
                            if tx.send(data).is_err() {
                                break;
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    return Err(HuobiError::WebSocket(e.to_string()));
                }
                _ => {}
            }
        }

        Ok(())
    }
}

/// Simplified WebSocket response wrapper.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WsResponse<T> {
    pub ch: Option<String>,
    pub ts: Option<i64>,
    pub tick: Option<T>,
    pub data: Option<T>,
    pub status: Option<String>,
    pub rep: Option<String>,
    pub subbed: Option<String>,
    pub unsubbed: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_client_creation() {
        let client = WsClient::new("wss://api.huobi.pro/ws");
        assert_eq!(client.url, "wss://api.huobi.pro/ws");
        assert!(client.access_key.is_none());
    }

    #[test]
    fn test_ws_client_with_auth() {
        let client =
            WsClient::new("wss://api.huobi.pro/ws").with_auth("my-key".into(), "my-secret".into());
        assert_eq!(client.access_key.as_deref(), Some("my-key"));
        assert_eq!(client.secret_key.as_deref(), Some("my-secret"));
    }

    #[test]
    fn test_ws_response_deserialize() {
        let json = r#"{
            "ch": "market.btcusdt.kline.1min",
            "ts": 1609459200000,
            "tick": {"id": 1, "open": "50000.0"}
        }"#;

        let resp: WsResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.ch.as_deref(), Some("market.btcusdt.kline.1min"));
        assert_eq!(resp.ts, Some(1609459200000));
        assert!(resp.tick.is_some());
    }
}
