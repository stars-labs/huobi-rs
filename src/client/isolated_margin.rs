use crate::config::Config;
use crate::http::HttpClient;

/// Client for isolated margin API calls (private, auth required).
pub struct IsolatedMarginClient {
    http: HttpClient,
}

impl IsolatedMarginClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Transfer in to isolated margin.
    pub async fn transfer_in(
        &self,
        request: &crate::request::margin::IsolatedMarginTransferOrApplyRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed(
                "/v1/isolated-margin/transfer-in",
                Default::default(),
                body,
            )
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse transfer id: {}", result),
            }
        })
    }

    /// Transfer out from isolated margin.
    pub async fn transfer_out(
        &self,
        request: &crate::request::margin::IsolatedMarginTransferOrApplyRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed(
                "/v1/isolated-margin/transfer-out",
                Default::default(),
                body,
            )
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse transfer id: {}", result),
            }
        })
    }

    /// Apply for isolated margin loan.
    pub async fn margin_orders(
        &self,
        request: &crate::request::margin::IsolatedMarginTransferOrApplyRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed(
                "/v1/isolated-margin/orders",
                Default::default(),
                body,
            )
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse loan id: {}", result),
            }
        })
    }

    /// Repay isolated margin loan.
    pub async fn repay(&self, order_id: i64, amount: &str) -> crate::Result<i64> {
        let path = format!("/v1/isolated-margin/orders/{}/repay", order_id);
        let body = serde_json::json!({ "amount": amount });
        let result: String = self
            .http
            .post_signed(&path, Default::default(), body)
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse repay id: {}", result),
            }
        })
    }

    /// Get isolated margin loan info.
    pub async fn get_loan_info(
        &self,
        symbols: &str,
    ) -> crate::Result<Vec<crate::models::margin::IsolatedMarginLoanInfo>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbols".into(), symbols.to_string());
        self.http
            .get_signed("/v1/isolated-margin/loan-info", params)
            .await
    }

    /// Get isolated margin loan orders.
    pub async fn get_loan_orders(
        &self,
        request: &crate::request::margin::IsolatedMarginLoanOrdersRequest,
    ) -> crate::Result<Vec<crate::models::margin::IsolatedMarginLoanOrder>> {
        let params = request.to_params();
        self.http
            .get_signed("/v1/isolated-margin/loan-orders", params)
            .await
    }

    /// Get isolated margin balance.
    pub async fn get_balance(
        &self,
        symbols: &str,
    ) -> crate::Result<Vec<crate::models::margin::IsolatedMarginBalance>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbols".into(), symbols.to_string());
        self.http
            .get_signed("/v1/isolated-margin/accounts/balance", params)
            .await
    }

    /// Get margin limit.
    pub async fn get_margin_limit(
        &self,
        currency: &str,
    ) -> crate::Result<Vec<crate::models::margin::MarginLimitResponse>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("currency".into(), currency.to_string());
        self.http
            .get_signed("/v1/isolated-margin/query-currencies", params)
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
    async fn test_isolated_margin_transfer_in() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/isolated-margin/transfer-in"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "54321"
            })))
            .mount(&server)
            .await;

        let client = IsolatedMarginClient::new(test_config(&server.uri()));
        let request = crate::request::margin::IsolatedMarginTransferOrApplyRequest::new(
            "btcusdt", "usdt", "500", "transfer",
        );
        let id = client.transfer_in(&request).await.unwrap();
        assert_eq!(id, 54321);
    }

    #[tokio::test]
    async fn test_get_isolated_margin_balance() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/isolated-margin/accounts/balance"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "id": 12345,
                        "symbol": "btcusdt",
                        "state": "working",
                        "type": "isolated-margin",
                        "riskRate": "2.5",
                        "list": [{"currency": "usdt", "type": "trade", "balance": "5000"}]
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = IsolatedMarginClient::new(test_config(&server.uri()));
        let balances = client.get_balance("btcusdt").await.unwrap();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].symbol, "btcusdt");
        assert_eq!(balances[0].risk_rate, "2.5");
    }
}
