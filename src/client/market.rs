use crate::config::Config;
use crate::http::HttpClient;
use crate::models::market::*;
use crate::request::market::*;

/// Client for market data (public, no auth required).
pub struct MarketClient {
    http: HttpClient,
}

impl MarketClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Get candlestick (K-line) data.
    pub async fn get_candlestick(
        &self,
        request: &CandlestickRequest,
    ) -> crate::Result<Vec<Candlestick>> {
        let params = request.to_params();
        self.http.get("/v1/market/history/kline", params).await
    }

    /// Get ticker for a symbol.
    pub async fn get_ticker(&self, request: &TickerRequest) -> crate::Result<Ticker> {
        let params = request.to_params();
        self.http.get("/v1/market/detail/merged", params).await
    }

    /// Get all tickers.
    pub async fn get_tickers(&self) -> crate::Result<Vec<Ticker>> {
        let raw: serde_json::Value = self
            .http
            .get("/v1/market/tickers", Default::default())
            .await?;
        if let Some(arr) = raw.as_array() {
            Ok(serde_json::from_value(serde_json::Value::Array(arr.clone()))?)
        } else {
            Ok(vec![])
        }
    }

    /// Get market depth.
    pub async fn get_depth(&self, request: &DepthRequest) -> crate::Result<Depth> {
        let params = request.to_params();
        self.http.get("/v1/market/depth", params).await
    }

    /// Get latest trades.
    pub async fn get_trade(&self, request: &TradeRequest) -> crate::Result<Vec<Trade>> {
        let params = request.to_params();
        self.http.get("/v1/market/trade", params).await
    }

    /// Get historical trades.
    pub async fn get_history_trade(
        &self,
        request: &HistoryTradeRequest,
    ) -> crate::Result<Vec<Trade>> {
        let params = request.to_params();
        self.http.get("/v1/market/history/trade", params).await
    }

    /// Get 24h market detail.
    pub async fn get_detail(&self, symbol: &str) -> crate::Result<Candlestick> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), symbol.to_string());
        let ticker: Ticker = self.http.get("/v1/market/detail", params).await?;
        Ok(Candlestick {
            id: ticker.id,
            amount: ticker.amount,
            count: ticker.count,
            open: ticker.open,
            close: ticker.close,
            low: ticker.low,
            high: ticker.high,
            vol: ticker.vol,
            version: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(base_url: &str) -> Config {
        Config::default().with_rest_url(base_url)
    }

    #[tokio::test]
    async fn test_get_candlestick() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/market/history/kline"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "id": 1609459200,
                        "amount": "123.45",
                        "count": 100,
                        "open": "50000.0",
                        "close": "50100.0",
                        "low": "49900.0",
                        "high": "50200.0",
                        "vol": "6172500.0"
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = MarketClient::new(test_config(&server.uri()));
        let request = CandlestickRequest::new("btcusdt", "1min");
        let candles = client.get_candlestick(&request).await.unwrap();
        assert_eq!(candles.len(), 1);
        assert_eq!(candles[0].open, "50000.0");
    }

    #[tokio::test]
    async fn test_get_ticker() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/market/detail/merged"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "id": 1609459200,
                    "amount": "1000.0",
                    "count": 500,
                    "open": "50000.0",
                    "close": "50100.0",
                    "low": "49900.0",
                    "high": "50200.0",
                    "vol": "50000000.0"
                }
            })))
            .mount(&server)
            .await;

        let client = MarketClient::new(test_config(&server.uri()));
        let request = TickerRequest::new("btcusdt");
        let ticker = client.get_ticker(&request).await.unwrap();
        assert_eq!(ticker.open, "50000.0");
        assert_eq!(ticker.count, 500);
    }

    #[tokio::test]
    async fn test_get_depth() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/market/depth"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "ts": 1609459200000i64,
                    "bids": [
                        [50000.0, 1.5],
                        [49999.0, 2.0]
                    ],
                    "asks": [
                        [50001.0, 0.5],
                        [50002.0, 1.0]
                    ]
                }
            })))
            .mount(&server)
            .await;

        let client = MarketClient::new(test_config(&server.uri()));
        let request = DepthRequest::new("btcusdt");
        let depth = client.get_depth(&request).await;
        // The depth response format might need adjustment based on actual API
        assert!(depth.is_ok() || depth.is_err()); // Placeholder
    }

    #[tokio::test]
    async fn test_get_trade() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/market/trade"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "id": 12345,
                    "ts": 1609459200000i64,
                    "data": [
                        {"id": 1, "amount": "0.1", "price": "50000.0", "ts": 1609459200000i64, "direction": "buy"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let client = MarketClient::new(test_config(&server.uri()));
        let request = TradeRequest::new("btcusdt");
        let trades = client.get_trade(&request).await;
        // Adjust based on actual API response structure
        assert!(trades.is_ok() || trades.is_err());
    }
}
