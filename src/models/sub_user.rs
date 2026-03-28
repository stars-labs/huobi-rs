/// Sub-user creation response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CreateSubUserResponse {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(default)]
    pub note: Option<String>,
    pub uid: i64,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<String>,
    #[serde(default, rename = "errMessage")]
    pub err_message: Option<String>,
}

/// Sub-user info.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct User {
    pub uid: i64,
    #[serde(rename = "userState")]
    pub user_state: String,
    #[serde(rename = "subUserName")]
    pub sub_user_name: String,
    #[serde(default)]
    pub note: Option<String>,
}

/// Account ID entry for sub-user account.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountIdEntry {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(default, rename = "accountStatus")]
    pub account_status: Option<String>,
}

/// Sub-user account.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubUserAccount {
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(default)]
    pub activation: Option<String>,
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<AccountIdEntry>,
    #[serde(default)]
    pub transferrable: Option<bool>,
}

/// API key generation response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ApiKeyGenerationResponse {
    #[serde(default)]
    pub note: Option<String>,
    #[serde(rename = "accessKey")]
    pub access_key: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    #[serde(default)]
    pub permission: Option<String>,
    #[serde(default, rename = "ipAddresses")]
    pub ip_addresses: Option<String>,
}

/// Sub-user deposit.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubUserDeposit {
    pub currency: String,
    pub amount: String,
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(rename = "depositId")]
    pub deposit_id: i64,
    #[serde(rename = "subUid")]
    pub sub_uid: i64,
    #[serde(rename = "subAccountName")]
    pub sub_account_name: String,
    #[serde(default)]
    pub state: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sub_user_response() {
        let json = r#"{"userName": "test_sub", "uid": 12345678}"#;
        let resp: CreateSubUserResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.user_name, "test_sub");
        assert_eq!(resp.uid, 12345678);
    }

    #[test]
    fn test_user_deserialize() {
        let json = r#"{"uid": 12345, "userState": "active", "subUserName": "sub1"}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.uid, 12345);
        assert_eq!(user.user_state, "active");
    }

    #[test]
    fn test_sub_user_account() {
        let json = r#"{
            "accountType": "spot",
            "accountIds": [{"accountId": 100, "subType": "trade"}]
        }"#;
        let account: SubUserAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.account_type, "spot");
        assert_eq!(account.account_ids.len(), 1);
    }

    #[test]
    fn test_api_key_generation_response() {
        let json = r#"{
            "accessKey": "test-access-key",
            "secretKey": "test-secret-key"
        }"#;
        let resp: ApiKeyGenerationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_key, "test-access-key");
        assert_eq!(resp.secret_key, "test-secret-key");
    }

    #[test]
    fn test_user_array() {
        let json = r#"[{"uid": 1, "userState": "active", "subUserName": "sub1"}]"#;
        let users: Vec<User> = serde_json::from_str(json).unwrap();
        assert_eq!(users.len(), 1);
    }
}
