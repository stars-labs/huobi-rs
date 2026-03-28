use crate::config::Config;
use crate::http::HttpClient;
use crate::models::wallet::*;
use crate::request::wallet::*;

/// Client for wallet API calls (private, auth required).
pub struct WalletClient {
    http: HttpClient,
}

impl WalletClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Get deposit address.
    pub async fn get_deposit_address(
        &self,
        currency: &str,
    ) -> crate::Result<Vec<DepositAddress>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("currency".into(), currency.to_string());
        self.http
            .get_signed("/v2/account/deposit/address", params)
            .await
    }

    /// Get withdrawal quota.
    pub async fn get_withdraw_quota(
        &self,
        currency: &str,
    ) -> crate::Result<Vec<WithdrawQuota>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("currency".into(), currency.to_string());
        self.http
            .get_signed("/v2/account/withdraw/quota", params)
            .await
    }

    /// Create withdrawal.
    pub async fn withdraw_create(
        &self,
        request: &WithdrawCreateRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed("/v1/dw/withdraw/api/create", Default::default(), body)
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse withdraw id: {}", result),
            }
        })
    }

    /// Cancel withdrawal.
    pub async fn withdraw_cancel(&self, withdraw_id: i64) -> crate::Result<i64> {
        let path = format!("/v1/dw/withdraw-virtual/{}/cancel", withdraw_id);
        let result: String = self
            .http
            .post_signed_empty(&path, Default::default())
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse result: {}", result),
            }
        })
    }

    /// Query deposit/withdraw records.
    pub async fn query_deposit_withdraw(
        &self,
        request: &QueryDepositWithdrawRequest,
    ) -> crate::Result<Vec<DepositWithdraw>> {
        let params = request.to_params();
        self.http
            .get_signed("/v1/query/deposit-withdraw", params)
            .await
    }

    /// Get withdrawal addresses.
    pub async fn get_withdraw_address(
        &self,
        request: &GetWithdrawAddressRequest,
    ) -> crate::Result<Vec<GetWithdrawAddressResponse>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("currency".into(), request.currency.clone());
        if let Some(ref chain) = request.chain {
            params.insert("chain".into(), chain.clone());
        }
        if let Some(limit) = request.limit {
            params.insert("limit".into(), limit.to_string());
        }
        self.http
            .get_signed("/v2/sub-user/query-deposit", params)
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
    async fn test_get_deposit_address() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/account/deposit/address"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {"currency": "usdt", "address": "0x1234567890abcdef", "chain": "trc20usdt"}
                ]
            })))
            .mount(&server)
            .await;

        let client = WalletClient::new(test_config(&server.uri()));
        let addrs = client.get_deposit_address("usdt").await.unwrap();
        assert_eq!(addrs.len(), 1);
        assert_eq!(addrs[0].address, "0x1234567890abcdef");
    }

    #[tokio::test]
    async fn test_withdraw_create() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/dw/withdraw/api/create"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "99887766"
            })))
            .mount(&server)
            .await;

        let client = WalletClient::new(test_config(&server.uri()));
        let request = WithdrawCreateRequest::new("0x1234", "100", "usdt");
        let id = client.withdraw_create(&request).await.unwrap();
        assert_eq!(id, 99887766);
    }

    #[tokio::test]
    async fn test_query_deposit_withdraw() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/query/deposit-withdraw"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {
                        "id": 100,
                        "type": "withdraw",
                        "currency": "usdt",
                        "amount": "500",
                        "createdAt": 1609459200000i64,
                        "updatedAt": 1609459300000i64,
                        "state": "confirmed"
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = WalletClient::new(test_config(&server.uri()));
        let request = QueryDepositWithdrawRequest::new("withdraw");
        let records = client.query_deposit_withdraw(&request).await.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].record_type, "withdraw");
    }
}
