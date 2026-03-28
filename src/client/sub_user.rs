use crate::config::Config;
use crate::http::HttpClient;
use crate::models::sub_user::*;
use crate::request::sub_user::*;

/// Client for sub-user API calls (private, auth required).
pub struct SubUserClient {
    http: HttpClient,
}

impl SubUserClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: HttpClient::new(config),
        }
    }

    /// Get sub-user list.
    pub async fn get_sub_user_list(
        &self,
        from_id: Option<i64>,
    ) -> crate::Result<Vec<User>> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(id) = from_id {
            params.insert("fromId".into(), id.to_string());
        }
        self.http
            .get_signed("/v2/sub-user/user-list", params)
            .await
    }

    /// Get sub-user state.
    pub async fn get_sub_user_state(
        &self,
        sub_uid: i64,
    ) -> crate::Result<User> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("subUid".into(), sub_uid.to_string());
        self.http
            .get_signed("/v2/sub-user/user-state", params)
            .await
    }

    /// Manage sub-user (freeze/unfreeze).
    pub async fn manage_sub_user(
        &self,
        request: &ManageSubUserRequest,
    ) -> crate::Result<serde_json::Value> {
        let body = request.to_body();
        self.http
            .post_signed(
                "/v2/sub-user/managed-sub-user",
                Default::default(),
                body,
            )
            .await
    }

    /// Get sub-user account list.
    pub async fn get_sub_user_account_list(
        &self,
        sub_uid: i64,
    ) -> crate::Result<Vec<SubUserAccount>> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("sub-uid".into(), sub_uid.to_string());
        self.http
            .get_signed("/v1/account/accounts", params)
            .await
    }

    /// Get user UID.
    pub async fn get_uid(&self) -> crate::Result<i64> {
        let result: serde_json::Value = self
            .http
            .get_signed("/v2/user/uid", Default::default())
            .await?;
        if let Some(uid) = result.as_i64() {
            Ok(uid)
        } else {
            Ok(0)
        }
    }

    /// Transfer to/from sub-user.
    pub async fn subuser_transfer(
        &self,
        request: &SubuserTransferRequest,
    ) -> crate::Result<i64> {
        let body = request.to_body();
        let result: String = self
            .http
            .post_signed(
                "/v1/subuser/transfer",
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

    /// Generate API key for sub-user.
    pub async fn api_key_generation(
        &self,
        request: &ApiKeyGenerationRequest,
    ) -> crate::Result<ApiKeyGenerationResponse> {
        let body = request.to_body();
        self.http
            .post_signed(
                "/v2/sub-user/api-key-generation",
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
    async fn test_get_sub_user_list() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/sub-user/user-list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": [
                    {"uid": 12345, "userState": "active", "subUserName": "sub1"},
                    {"uid": 12346, "userState": "active", "subUserName": "sub2"}
                ]
            })))
            .mount(&server)
            .await;

        let client = SubUserClient::new(test_config(&server.uri()));
        let users = client.get_sub_user_list(None).await.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].sub_user_name, "sub1");
    }

    #[tokio::test]
    async fn test_manage_sub_user_freeze() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v2/sub-user/managed-sub-user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {"subUid": 12345, "action": "freeze"}
            })))
            .mount(&server)
            .await;

        let client = SubUserClient::new(test_config(&server.uri()));
        let request = ManageSubUserRequest::freeze(12345);
        let result = client.manage_sub_user(&request).await.unwrap();
        assert!(result.is_object());
    }
}
