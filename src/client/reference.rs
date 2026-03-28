use crate::config::Config;
use crate::http::HttpClient;
use crate::models::reference::*;
use crate::request::reference::ReferenceCurrenciesRequest;

/// Client for reference data (public, no auth required).
pub struct ReferenceClient {
    http: HttpClient,
}

impl ReferenceClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Get exchange timestamp.
    pub async fn get_timestamp(&self) -> crate::Result<i64> {
        self.http.get("/v1/common/timestamp", Default::default()).await
    }

    /// Get all trading symbols.
    pub async fn get_symbols(&self) -> crate::Result<Vec<Symbol>> {
        self.http.get("/v1/common/symbols", Default::default()).await
    }

    /// Get all currencies.
    pub async fn get_currencies(&self) -> crate::Result<Vec<String>> {
        self.http.get("/v1/common/currencys", Default::default()).await
    }

    /// Get currency and chain info.
    pub async fn get_reference_currencies(
        &self,
        request: &ReferenceCurrenciesRequest,
    ) -> crate::Result<Vec<ReferenceCurrencies>> {
        let params = request.to_params();
        self.http.get("/v2/reference/currencies", params).await
    }

    /// Get market status.
    pub async fn get_market_status(&self) -> crate::Result<crate::models::market::MarketStatus> {
        self.http.get("/v1/market-status", Default::default()).await
    }

    /// Get market symbols.
    pub async fn get_market_symbols(&self) -> crate::Result<Vec<MarketSymbols>> {
        self.http.get("/v1/settings/common/symbols", Default::default()).await
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
    async fn test_get_timestamp() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/common/timestamp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": 1609459200000i64
            })))
            .mount(&server)
            .await;

        let client = ReferenceClient::new(test_config(&server.uri()));
        let ts = client.get_timestamp().await.unwrap();
        assert_eq!(ts, 1609459200000i64);
    }

    #[tokio::test]
    async fn test_get_symbols() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/common/symbols"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "symbol": "btcusdt",
                        "state": "online",
                        "base-currency": "btc",
                        "quote-currency": "usdt"
                    },
                    {
                        "symbol": "ethusdt",
                        "state": "online",
                        "base-currency": "eth",
                        "quote-currency": "usdt"
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = ReferenceClient::new(test_config(&server.uri()));
        let symbols = client.get_symbols().await.unwrap();
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].symbol, "btcusdt");
        assert_eq!(symbols[0].base_currency, "btc");
    }

    #[tokio::test]
    async fn test_get_currencies() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/common/currencys"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": ["btc", "usdt", "eth"]
            })))
            .mount(&server)
            .await;

        let client = ReferenceClient::new(test_config(&server.uri()));
        let currencies = client.get_currencies().await.unwrap();
        assert_eq!(currencies.len(), 3);
        assert!(currencies.contains(&"btc".to_string()));
    }

    #[tokio::test]
    async fn test_get_reference_currencies() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/reference/currencies"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "currency": "usdt",
                        "instStatus": "normal",
                        "chains": [
                            {"chain": "trc20usdt", "depositStatus": "allowed"},
                            {"chain": "erc20usdt", "depositStatus": "allowed"}
                        ]
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = ReferenceClient::new(test_config(&server.uri()));
        let request = ReferenceCurrenciesRequest::new();
        let result = client.get_reference_currencies(&request).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].currency, "usdt");
        assert_eq!(result[0].chains.len(), 2);
    }
}
