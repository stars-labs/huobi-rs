use crate::config::Config;
use crate::http::HttpClient;

/// Client for cross margin API calls (private, auth required).
pub struct CrossMarginClient {
    http: HttpClient,
}

impl CrossMarginClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Transfer in to cross margin.
    pub async fn transfer_in(
        &self,
        request: &crate::request::margin::CrossMarginTransferOrApplyRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed(
                "/v1/cross-margin/transfer-in",
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

    /// Transfer out from cross margin.
    pub async fn transfer_out(
        &self,
        request: &crate::request::margin::CrossMarginTransferOrApplyRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed(
                "/v1/cross-margin/transfer-out",
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

    /// Apply for cross margin loan.
    pub async fn margin_orders(
        &self,
        request: &crate::request::margin::CrossMarginTransferOrApplyRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed("/v1/cross-margin/orders", Default::default(), body)
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse loan id: {}", result),
            }
        })
    }

    /// Repay cross margin loan.
    pub async fn repay(&self, order_id: i64, amount: &str) -> crate::Result<serde_json::Value> {
        let path = format!("/v1/cross-margin/orders/{}/repay", order_id);
        let body = serde_json::json!({ "amount": amount });
        self.http
            .post_signed(&path, Default::default(), body)
            .await
    }

    /// Get cross margin loan orders.
    pub async fn get_loan_orders(
        &self,
        request: &crate::request::margin::CrossMarginLoanOrdersRequest,
    ) -> crate::Result<Vec<crate::models::margin::CrossMarginLoanOrder>> {
        let params = request.to_params();
        self.http
            .get_signed("/v1/cross-margin/loan-orders", params)
            .await
    }

    /// Get cross margin balance.
    pub async fn get_balance(
        &self,
        sub_uid: i64,
    ) -> crate::Result<crate::models::margin::CrossMarginBalance> {
        let mut params = std::collections::BTreeMap::new();
        if sub_uid > 0 {
            params.insert("sub-uid".into(), sub_uid.to_string());
        }
        self.http
            .get_signed("/v1/cross-margin/accounts/balance", params)
            .await
    }

    /// Get loan info (interest rate, limits).
    pub async fn get_loan_info(
        &self,
    ) -> crate::Result<Vec<crate::models::margin::LoanCurrencyInfo>> {
        let result: serde_json::Value = self
            .http
            .get_signed("/v1/cross-margin/loan-info", Default::default())
            .await?;
        if let Some(currencies) = result.get("currencies") {
            Ok(serde_json::from_value(currencies.clone())?)
        } else {
            Ok(vec![])
        }
    }

    /// General repayment (supports both cross and isolated).
    pub async fn general_repay(
        &self,
        repay_id: Option<i64>,
    ) -> crate::Result<Vec<crate::models::margin::CrossMarginGeneralRepay>> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(id) = repay_id {
            params.insert("repayId".into(), id.to_string());
        }
        self.http
            .get_signed("/v2/account/repayment", params)
            .await
    }

    /// General repayment record query.
    pub async fn general_repay_records(
        &self,
        repay_id: Option<i64>,
    ) -> crate::Result<Vec<crate::models::margin::CrossMarginRepayLoanRecord>> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(id) = repay_id {
            params.insert("repayId".into(), id.to_string());
        }
        self.http
            .get_signed("/v2/account/repayment-record", params)
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
    async fn test_cross_margin_transfer_in() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/cross-margin/transfer-in"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "12345"
            })))
            .mount(&server)
            .await;

        let client = CrossMarginClient::new(test_config(&server.uri()));
        let request =
            crate::request::margin::CrossMarginTransferOrApplyRequest::new("usdt", "1000");
        let id = client.transfer_in(&request).await.unwrap();
        assert_eq!(id, 12345);
    }

    #[tokio::test]
    async fn test_get_cross_margin_balance() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/cross-margin/accounts/balance"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "id": 12345,
                    "state": "working",
                    "type": "cross-margin",
                    "list": [{"currency": "usdt", "type": "trade", "balance": "10000"}]
                }
            })))
            .mount(&server)
            .await;

        let client = CrossMarginClient::new(test_config(&server.uri()));
        let balance = client.get_balance(0).await.unwrap();
        assert_eq!(balance.id, 12345);
        assert_eq!(balance.list.len(), 1);
    }
}
