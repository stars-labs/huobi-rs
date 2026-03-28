use crate::config::Config;
use crate::http::HttpClient;
use crate::models::trade::*;
use crate::request::trade::*;

/// Client for trading API calls (private, auth required).
pub struct TradeClient {
    http: HttpClient,
}

impl TradeClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Place an order.
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed("/v1/order/orders/place", request.to_params(), body)
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse order id: {}", result),
            }
        })
    }

    /// Submit cancel order by order ID.
    pub async fn submit_cancel_order(&self, order_id: i64) -> crate::Result<String> {
        let path = format!("/v1/order/orders/{}/submitcancel", order_id);
        self.http
            .post_signed_empty(&path, Default::default())
            .await
    }

    /// Cancel order by client order ID.
    pub async fn submit_cancel_client_order(
        &self,
        client_order_id: &str,
    ) -> crate::Result<serde_json::Value> {
        let path = format!("/v1/order/orders/submitCancelClientOrder?clientOrderId={}", client_order_id);
        self.http
            .post_signed_empty(&path, Default::default())
            .await
    }

    /// Get open orders.
    pub async fn get_open_orders(
        &self,
        request: &OpenOrdersRequest,
    ) -> crate::Result<Vec<OpenOrder>> {
        let params = request.to_params();
        self.http.get_signed("/v1/order/openOrders", params).await
    }

    /// Batch cancel open orders.
    pub async fn batch_cancel_open_orders(
        &self,
        request: &BatchCancelOpenOrdersRequest,
    ) -> crate::Result<BatchCancelOpenOrders> {
        let params = request.to_params();
        self.http
            .post_signed_empty("/v1/order/orders/batchCancelOpenOrders", params)
            .await
    }

    /// Get order info by order ID.
    pub async fn get_order(&self, order_id: i64) -> crate::Result<Order> {
        let path = format!("/v1/order/orders/{}", order_id);
        self.http
            .get_signed(&path, Default::default())
            .await
    }

    /// Get order by client order ID.
    pub async fn get_client_order(
        &self,
        client_order_id: &str,
    ) -> crate::Result<Order> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("clientOrderId".into(), client_order_id.to_string());
        self.http
            .get_signed("/v1/order/orders/getClientOrder", params)
            .await
    }

    /// Get order match results.
    pub async fn get_matchresults(&self, order_id: i64) -> crate::Result<Vec<MatchResult>> {
        let path = format!("/v1/order/orders/{}/matchresults", order_id);
        self.http.get_signed(&path, Default::default()).await
    }

    /// Get orders with filters.
    pub async fn get_orders(&self, request: &GetOrdersRequest) -> crate::Result<Vec<Order>> {
        let params = request.to_params();
        self.http.get_signed("/v1/order/orders", params).await
    }

    /// Get historical orders.
    pub async fn get_orders_history(
        &self,
        request: &OrdersHistoryRequest,
    ) -> crate::Result<Vec<Order>> {
        let params = request.to_params();
        self.http
            .get_signed("/v1/order/orders/history", params)
            .await
    }

    /// Get transaction fee rate.
    pub async fn get_transact_fee_rate(
        &self,
        symbols: &str,
    ) -> crate::Result<Vec<TransactFeeRate>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbols".into(), symbols.to_string());
        self.http
            .get_signed("/v1/fee/fee-rate/get", params)
            .await
    }

    /// Cancel all after timeout (dead man's switch).
    pub async fn cancel_all_after(&self, timeout_secs: i32) -> crate::Result<CancelAllAfterResponse> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("timeout".into(), timeout_secs.to_string());
        self.http
            .post_signed_empty("/v1/order/orders/cancelAllAfter", params)
            .await
    }

    /// Place batch orders.
    pub async fn batch_orders(
        &self,
        orders: &[PlaceOrderRequest],
    ) -> crate::Result<Vec<PlaceOrderResponse>> {
        let body: Vec<serde_json::Value> = orders.iter().map(|o| o.to_body()).collect();
        self.http
            .post_signed(
                "/v1/order/batch-orders",
                Default::default(),
                serde_json::to_value(&body)?,
            )
            .await
    }

    /// Get historical match results.
    pub async fn get_matchresults_history(
        &self,
        request: &MatchresultsHistoryRequest,
    ) -> crate::Result<Vec<MatchResult>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), request.symbol.clone());
        if let Some(ref types) = request.types {
            params.insert("types".into(), types.clone());
        }
        if let Some(ref direct) = request.direct {
            params.insert("direct".into(), direct.clone());
        }
        if let Some(ref start) = request.start_date {
            params.insert("start-date".into(), start.clone());
        }
        if let Some(ref end) = request.end_date {
            params.insert("end-date".into(), end.clone());
        }
        if let Some(from) = request.from {
            params.insert("from".into(), from.to_string());
        }
        if let Some(size) = request.size {
            params.insert("size".into(), size.to_string());
        }
        self.http
            .get_signed("/v1/order/matchresults", params)
            .await
    }

    /// Batch cancel orders.
    pub async fn batch_cancel_orders(
        &self,
        request: &BatchCancelOrdersRequest,
    ) -> crate::Result<BatchCancelOrders> {
        let body = request.to_body();
        self.http
            .post_signed(
                "/v1/order/orders/batchCancel",
                Default::default(),
                body,
            )
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
    async fn test_place_order() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/order/orders/place"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "12345678"
            })))
            .mount(&server)
            .await;

        let client = TradeClient::new(test_config(&server.uri()));
        let request = PlaceOrderRequest::new(12345, "btcusdt", "buy-limit", "0.1")
            .with_price("50000.0");
        let order_id = client.place_order(&request).await.unwrap();
        assert_eq!(order_id, 12345678);
    }

    #[tokio::test]
    async fn test_submit_cancel_order() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/order/orders/12345/submitcancel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "12345"
            })))
            .mount(&server)
            .await;

        let client = TradeClient::new(test_config(&server.uri()));
        let result = client.submit_cancel_order(12345).await.unwrap();
        assert_eq!(result, "12345");
    }

    #[tokio::test]
    async fn test_get_order() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/order/orders/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "id": 12345,
                    "accountId": 100,
                    "symbol": "btcusdt",
                    "amount": "0.1",
                    "price": "50000.0",
                    "createdAt": 1609459200000i64,
                    "type": "buy-limit",
                    "filledAmount": "0.0",
                    "filledCashAmount": "0.0",
                    "filledFees": "0.0",
                    "state": "submitted"
                }
            })))
            .mount(&server)
            .await;

        let client = TradeClient::new(test_config(&server.uri()));
        let order = client.get_order(12345).await.unwrap();
        assert_eq!(order.id, 12345);
        assert_eq!(order.symbol, "btcusdt");
        assert_eq!(order.order_type, "buy-limit");
    }

    #[tokio::test]
    async fn test_get_open_orders() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/order/openOrders"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "id": 12345,
                        "accountId": 100,
                        "symbol": "btcusdt",
                        "amount": "0.1",
                        "createdAt": 1609459200000i64,
                        "type": "buy-limit",
                        "filledAmount": "0.0",
                        "filledCashAmount": "0.0",
                        "filledFees": "0.0",
                        "state": "submitted"
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = TradeClient::new(test_config(&server.uri()));
        let request = OpenOrdersRequest::new(12345);
        let orders = client.get_open_orders(&request).await.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].symbol, "btcusdt");
    }

    #[tokio::test]
    async fn test_batch_orders() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/order/batch-orders"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {"orderId": 111},
                    {"orderId": 222}
                ]
            })))
            .mount(&server)
            .await;

        let client = TradeClient::new(test_config(&server.uri()));
        let orders = vec![
            PlaceOrderRequest::new(12345, "btcusdt", "buy-limit", "0.1").with_price("50000"),
            PlaceOrderRequest::new(12345, "ethusdt", "buy-limit", "1.0").with_price("3000"),
        ];
        let results = client.batch_orders(&orders).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].order_id, 111);
    }

    #[tokio::test]
    async fn test_batch_cancel_orders() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/order/orders/batchCancel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "success": ["111", "222"],
                    "failed": []
                }
            })))
            .mount(&server)
            .await;

        let client = TradeClient::new(test_config(&server.uri()));
        let request = BatchCancelOrdersRequest {
            order_ids: vec![111, 222],
            client_order_ids: vec![],
        };
        let result = client.batch_cancel_orders(&request).await.unwrap();
        assert_eq!(result.success.len(), 2);
        assert!(result.failed.is_empty());
    }
}
