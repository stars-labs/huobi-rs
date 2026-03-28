mod algo;
mod cross_margin;
mod isolated_margin;
mod market;
mod reference;
mod sub_user;
mod trade;
mod wallet;
mod ws_asset;
mod ws_market;
mod ws_orders;
mod ws_trade;

pub use algo::AlgoClient;
pub use cross_margin::CrossMarginClient;
pub use isolated_margin::IsolatedMarginClient;
pub use market::MarketClient;
pub use reference::ReferenceClient;
pub use sub_user::SubUserClient;
pub use trade::TradeClient;
pub use wallet::WalletClient;
pub use ws_asset::WebSocketAssetClient;
pub use ws_market::WebSocketMarketClient;
pub use ws_orders::WebSocketOrdersClient;
pub use ws_trade::WebSocketTradeClient;

use crate::config::Config;
use crate::http::HttpClient;

/// Account client for account-related API calls.
pub struct AccountClient {
    http: HttpClient,
}

impl AccountClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Get all accounts.
    pub async fn get_accounts(
        &self,
    ) -> crate::Result<Vec<crate::models::account::Account>> {
        self.http
            .get_signed("/v1/account/accounts", std::collections::BTreeMap::new())
            .await
    }

    /// Get account balance.
    pub async fn get_balance(
        &self,
        account_id: i64,
    ) -> crate::Result<Vec<crate::models::account::Balance>> {
        let path = format!("/v1/account/accounts/{}/balance", account_id);
        let data: serde_json::Value = self
            .http
            .get_signed(&path, std::collections::BTreeMap::new())
            .await?;
        // The balance data is nested under "list"
        if let Some(list) = data.get("list") {
            Ok(serde_json::from_value(list.clone())?)
        } else {
            Ok(vec![])
        }
    }

    /// Get account history.
    pub async fn get_history(
        &self,
        request: &crate::request::account::AccountHistoryRequest,
    ) -> crate::Result<Vec<crate::models::account::AccountHistory>> {
        let path = format!("/v1/account/accounts/{}/history", request.account_id);
        let mut params = std::collections::BTreeMap::new();
        params.insert("currency".into(), request.currency.clone());
        if let Some(ref tt) = request.transact_types {
            params.insert("transact-types".into(), tt.clone());
        }
        if let Some(size) = request.size {
            params.insert("size".into(), size.to_string());
        }
        self.http.get_signed(&path, params).await
    }

    /// Get account ledger.
    pub async fn get_ledger(
        &self,
        request: &crate::request::account::AccountLedgerRequest,
    ) -> crate::Result<Vec<crate::models::account::AccountLedger>> {
        let params = request.to_params();
        self.http.get_signed("/v2/account/ledger", params).await
    }

    /// Transfer between accounts.
    pub async fn account_transfer(
        &self,
        request: &crate::request::account::AccountTransferRequest,
    ) -> crate::Result<crate::models::account::AccountTransferResponse> {
        let body = serde_json::json!({
            "fromUserId": request.from_user_id,
            "fromAccountType": request.from_account_type,
            "fromAccountId": request.from_account_id,
            "toUserId": request.to_user_id,
            "toAccountType": request.to_account_type,
            "toAccountId": request.to_account_id,
            "currency": request.currency,
            "amount": request.amount,
        });
        self.http
            .post_signed(
                "/v1/account/transfer",
                std::collections::BTreeMap::new(),
                body,
            )
            .await
    }

    /// Get asset valuation.
    pub async fn get_asset_valuation(
        &self,
        request: &crate::request::account::AssetValuationRequest,
    ) -> crate::Result<crate::models::account::AssetValuation> {
        let params = request.to_params();
        self.http
            .get_signed("/v2/account/asset-valuation", params)
            .await
    }

    /// Get point account balance.
    pub async fn get_point_account(
        &self,
        sub_uid: i64,
    ) -> crate::Result<crate::models::account::PointAccount> {
        let mut params = std::collections::BTreeMap::new();
        if sub_uid > 0 {
            params.insert("subUid".into(), sub_uid.to_string());
        }
        self.http
            .get_signed("/v2/point/account", params)
            .await
    }

    /// Transfer points.
    pub async fn point_transfer(
        &self,
        to_uid: i64,
        group_id: &str,
        amount: &str,
    ) -> crate::Result<i64> {
        let body = serde_json::json!({
            "toUid": to_uid,
            "groupId": group_id,
            "amount": amount,
        });
        let result: String = self
            .http
            .post_signed("/v2/point/transfer", std::collections::BTreeMap::new(), body)
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse transfer id: {}", result),
            }
        })
    }

    /// Transfer between spot and futures accounts.
    pub async fn futures_transfer(
        &self,
        currency: &str,
        amount: &str,
        transfer_type: &str,
    ) -> crate::Result<i64> {
        let body = serde_json::json!({
            "currency": currency,
            "amount": amount,
            "type": transfer_type,
        });
        let result: String = self
            .http
            .post_signed("/v1/futures/transfer", std::collections::BTreeMap::new(), body)
            .await?;
        result.parse::<i64>().map_err(|_| {
            crate::HuobiError::Api {
                code: "parse".into(),
                message: format!("failed to parse transfer id: {}", result),
            }
        })
    }

    /// Get account valuation (total assets across accounts).
    pub async fn get_account_valuation(
        &self,
    ) -> crate::Result<crate::models::account::AccountValuation> {
        self.http
            .get_signed("/v2/account/valuation", std::collections::BTreeMap::new())
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
    async fn test_get_accounts() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/account/accounts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {"id": 12345, "type": "spot", "state": "working"},
                    {"id": 12346, "type": "margin", "state": "working"}
                ]
            })))
            .mount(&server)
            .await;

        let client = AccountClient::new(test_config(&server.uri()));
        let accounts = client.get_accounts().await.unwrap();
        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].account_type, "spot");
    }

    #[tokio::test]
    async fn test_get_balance() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/account/accounts/12345/balance"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "id": 12345,
                    "type": "spot",
                    "state": "working",
                    "list": [
                        {"currency": "usdt", "type": "trade", "balance": "10000.00"},
                        {"currency": "btc", "type": "trade", "balance": "0.5"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let client = AccountClient::new(test_config(&server.uri()));
        let balances = client.get_balance(12345).await.unwrap();
        assert_eq!(balances.len(), 2);
        assert_eq!(balances[0].currency, "usdt");
    }

    #[tokio::test]
    async fn test_account_transfer() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/account/transfer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {"transactId": 999, "transactTime": 1609459200000i64}
            })))
            .mount(&server)
            .await;

        let client = AccountClient::new(test_config(&server.uri()));
        let request = crate::request::account::AccountTransferRequest {
            from_user_id: 1,
            from_account_type: "spot".into(),
            from_account_id: 12345,
            to_user_id: 1,
            to_account_type: "margin".into(),
            to_account_id: 12346,
            currency: "usdt".into(),
            amount: "100".into(),
        };
        let resp = client.account_transfer(&request).await.unwrap();
        assert_eq!(resp.transact_id, 999);
    }

    #[tokio::test]
    async fn test_get_point_account() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/point/account"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {
                    "accountId": 99999,
                    "accountStatus": "working",
                    "acctBalance": "1000"
                }
            })))
            .mount(&server)
            .await;

        let client = AccountClient::new(test_config(&server.uri()));
        let point = client.get_point_account(0).await.unwrap();
        assert_eq!(point.account_id, 99999);
        assert_eq!(point.acct_balance, "1000");
    }

    #[tokio::test]
    async fn test_futures_transfer() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/futures/transfer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "55555"
            })))
            .mount(&server)
            .await;

        let client = AccountClient::new(test_config(&server.uri()));
        let id = client.futures_transfer("usdt", "100", "futures-to-spot").await.unwrap();
        assert_eq!(id, 55555);
    }
}
