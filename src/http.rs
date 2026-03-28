use crate::config::Config;
use crate::error::HuobiError;
use crate::auth;
use crate::Result;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

/// Wrapper for Huobi API response.
#[derive(Debug, serde::Deserialize)]
pub struct ApiResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(default)]
    pub err_code: Option<String>,
    #[serde(default)]
    pub err_msg: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// HTTP client for making REST API calls.
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    config: Config,
}

impl HttpClient {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("failed to build reqwest client");

        Self { client, config }
    }

    /// Perform an authenticated GET request.
    pub async fn get_signed<T: DeserializeOwned>(
        &self,
        path: &str,
        params: BTreeMap<String, String>,
    ) -> Result<T> {
        let host = self.extract_host()?;
        let auth_params = auth::create_auth_params(
            self.config.access_key.as_deref().ok_or_else(|| {
                HuobiError::Auth("access_key required for signed requests".into())
            })?,
            self.config.secret_key.as_deref().ok_or_else(|| {
                HuobiError::Auth("secret_key required for signed requests".into())
            })?,
            "GET",
            &host,
            path,
            params,
        )?;

        let url = auth::build_url(&self.config.rest_url, path, &auth_params)?;
        self.do_get(url).await
    }

    /// Perform a public GET request (no auth).
    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: BTreeMap<String, String>,
    ) -> Result<T> {
        let url = auth::build_url(&self.config.rest_url, path, &params)?;
        self.do_get(url).await
    }

    /// Perform an authenticated POST request.
    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        path: &str,
        params: BTreeMap<String, String>,
        body: serde_json::Value,
    ) -> Result<T> {
        let host = self.extract_host()?;
        let auth_params = auth::create_auth_params(
            self.config.access_key.as_deref().ok_or_else(|| {
                HuobiError::Auth("access_key required for signed requests".into())
            })?,
            self.config.secret_key.as_deref().ok_or_else(|| {
                HuobiError::Auth("secret_key required for signed requests".into())
            })?,
            "POST",
            &host,
            path,
            params,
        )?;

        let url = auth::build_url(&self.config.rest_url, path, &auth_params)?;
        self.do_post(url, body).await
    }

    /// Perform an authenticated POST request with empty body.
    pub async fn post_signed_empty<T: DeserializeOwned>(
        &self,
        path: &str,
        params: BTreeMap<String, String>,
    ) -> Result<T> {
        self.post_signed(path, params, serde_json::Value::Object(Default::default()))
            .await
    }

    async fn do_get<T: DeserializeOwned>(&self, url: url::Url) -> Result<T> {
        let resp = self.client.get(url).send().await?;
        let text = resp.text().await?;
        self.parse_response(&text)
    }

    async fn do_post<T: DeserializeOwned>(
        &self,
        url: url::Url,
        body: serde_json::Value,
    ) -> Result<T> {
        let resp = self.client.post(url).json(&body).send().await?;
        let text = resp.text().await?;
        self.parse_response(&text)
    }

    fn parse_response<T: DeserializeOwned>(&self, text: &str) -> Result<T> {
        let raw: ApiResponse = serde_json::from_str(text)?;
        if raw.status == "error" {
            return Err(HuobiError::Api {
                code: raw.err_code.unwrap_or_else(|| "unknown".into()),
                message: raw.err_msg.unwrap_or_else(|| text.to_string()),
            });
        }
        // Try to deserialize the "data" field
        let data = raw.data.unwrap_or(serde_json::Value::Null);
        Ok(serde_json::from_value(data)?)
    }

    fn extract_host(&self) -> Result<String> {
        let url = url::Url::parse(&self.config.rest_url)?;
        Ok(url.host_str().unwrap_or("api.huobi.pro").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_config(base_url: &str) -> Config {
        Config::new("test-key", "test-secret").with_rest_url(base_url)
    }

    async fn setup_mock() -> MockServer {
        MockServer::start().await
    }

    #[tokio::test]
    async fn test_get_public_success() {
        let server = setup_mock().await;

        Mock::given(method("GET"))
            .and(path("/v1/common/timestamp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": 1609459200000i64
            })))
            .mount(&server)
            .await;

        let config = Config::default().with_rest_url(server.uri());
        let client = HttpClient::new(config);

        let result: i64 = client
            .get("/v1/common/timestamp", BTreeMap::new())
            .await
            .unwrap();
        assert_eq!(result, 1609459200000i64);
    }

    #[tokio::test]
    async fn test_get_with_params() {
        let server = setup_mock().await;

        Mock::given(method("GET"))
            .and(path("/v1/market/kline"))
            .and(query_param("symbol", "btcusdt"))
            .and(query_param("period", "1min"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [{"id": 1, "open": "50000.0"}]
            })))
            .mount(&server)
            .await;

        let config = Config::default().with_rest_url(server.uri());
        let client = HttpClient::new(config);

        let mut params = BTreeMap::new();
        params.insert("symbol".into(), "btcusdt".into());
        params.insert("period".into(), "1min".into());

        let result: serde_json::Value = client.get("/v1/market/kline", params).await.unwrap();
        assert!(result.is_array());
    }

    #[tokio::test]
    async fn test_get_api_error() {
        let server = setup_mock().await;

        Mock::given(method("GET"))
            .and(path("/v1/bad"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "error",
                "err_code": "bad-request",
                "err_msg": "invalid parameter"
            })))
            .mount(&server)
            .await;

        let config = Config::default().with_rest_url(server.uri());
        let client = HttpClient::new(config);

        let result: std::result::Result<serde_json::Value, _> =
            client.get("/v1/bad", BTreeMap::new()).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("bad-request"));
    }

    #[tokio::test]
    async fn test_get_signed_includes_signature() {
        let server = setup_mock().await;

        // The signed request should include signature params
        Mock::given(method("GET"))
            .and(path("/v1/account/accounts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [{"id": 12345, "type": "spot"}]
            })))
            .mount(&server)
            .await;

        let config = test_config(&server.uri());
        let client = HttpClient::new(config);

        let result: serde_json::Value = client
            .get_signed("/v1/account/accounts", BTreeMap::new())
            .await
            .unwrap();
        assert!(result.is_array());
    }

    #[tokio::test]
    async fn test_post_signed() {
        let server = setup_mock().await;

        Mock::given(method("POST"))
            .and(path("/v1/order/orders/place"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": "12345678"
            })))
            .mount(&server)
            .await;

        let config = test_config(&server.uri());
        let client = HttpClient::new(config);

        let result: serde_json::Value = client
            .post_signed(
                "/v1/order/orders/place",
                BTreeMap::new(),
                serde_json::json!({"symbol": "btcusdt"}),
            )
            .await
            .unwrap();
        assert_eq!(result, "12345678");
    }

    #[tokio::test]
    async fn test_get_signed_without_credentials_fails() {
        let server = setup_mock().await;
        let config = Config::default().with_rest_url(server.uri());
        let client = HttpClient::new(config);

        let result: std::result::Result<serde_json::Value, _> = client
            .get_signed("/v1/account/accounts", BTreeMap::new())
            .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("access_key required"));
    }
}
