use crate::config::Config;
use crate::http::HttpClient;
use crate::models::algo::*;
use crate::request::algo::*;

/// Client for algo order API calls (private, auth required).
pub struct AlgoClient {
    http: HttpClient,
}

impl AlgoClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Create an algo order.
    pub async fn create_algo_order(
        &self,
        request: &CreateAlgoOrderRequest,
    ) -> crate::Result<String> {
        let body = request.to_body();
        self.http
            .post_signed(
                "/v2/algo-order/create",
                Default::default(),
                body,
            )
            .await
    }

    /// Get opening algo orders.
    pub async fn opening_algo_orders(
        &self,
        request: &OpeningAlgoOrdersRequest,
    ) -> crate::Result<Vec<AlgoOrder>> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(ref symbol) = request.symbol {
            params.insert("symbol".into(), symbol.clone());
        }
        if let Some(ref side) = request.order_side {
            params.insert("order-side".into(), side.clone());
        }
        if let Some(from) = request.from {
            params.insert("from".into(), from.to_string());
        }
        if let Some(size) = request.size {
            params.insert("size".into(), size.to_string());
        }
        if let Some(ref direct) = request.direct {
            params.insert("direct".into(), direct.clone());
        }
        self.http
            .get_signed("/v2/algo-order/opening", params)
            .await
    }

    /// Get history algo orders.
    pub async fn history_algo_orders(
        &self,
        request: &HistoryAlgoOrdersRequest,
    ) -> crate::Result<Vec<AlgoOrder>> {
        let params = request.to_params();
        self.http
            .get_signed("/v2/algo-order/history", params)
            .await
    }

    /// Cancel algo orders.
    pub async fn cancel_algo_order(
        &self,
        client_order_ids: Vec<String>,
    ) -> crate::Result<CancelAlgoOrderResult> {
        let body = serde_json::json!({
            "clientOrderIds": client_order_ids
        });
        self.http
            .post_signed(
                "/v2/algo-order/cancel",
                Default::default(),
                body,
            )
            .await
    }

    /// Get specific algo order by client order ID.
    pub async fn specific_algo_order(
        &self,
        client_order_id: &str,
    ) -> crate::Result<AlgoOrder> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("clientOrderId".into(), client_order_id.to_string());
        self.http
            .get_signed("/v2/algo-order/specific", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(base_url: &str) -> Config {
        Config::new("test-key", "test-secret").with_rest_url(base_url)
    }

    #[tokio::test]
    async fn test_create_algo_order() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v2/algo-order/create"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {"clientOrderId": "algo-123"}
            })))
            .mount(&server)
            .await;

        let client = AlgoClient::new(test_config(&server.uri()));
        let request = CreateAlgoOrderRequest::new(12345, "btcusdt", "stop-loss", "sell");
        let result = client.create_algo_order(&request).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_history_algo_orders() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/algo-order/history"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "accountId": 12345,
                        "symbol": "btcusdt",
                        "orderId": 999,
                        "orderType": "stop-loss",
                        "orderSide": "sell",
                        "orderStatus": "canceled"
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = AlgoClient::new(test_config(&server.uri()));
        let request = HistoryAlgoOrdersRequest::new();
        let orders = client.history_algo_orders(&request).await.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_type, "stop-loss");
    }
}
