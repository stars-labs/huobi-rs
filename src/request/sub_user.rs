/// Request for creating sub-users.
#[derive(Debug, Clone)]
pub struct SubUserCreationRequest {
    pub user_password: String,
    pub note: String,
}

/// Request for setting sub-user tradable market.
#[derive(Debug, Clone)]
pub struct SubUserTradableMarketRequest {
    pub sub_uid: i64,
    pub tradable: bool,
}

/// Request for sub-user transfer.
#[derive(Debug, Clone)]
pub struct SubuserTransferRequest {
    pub sub_uid: i64,
    pub currency: String,
    pub amount: String,
    pub transfer_type: String,
}

/// Request for managing sub-users.
#[derive(Debug, Clone)]
pub struct ManageSubUserRequest {
    pub sub_uid: i64,
    pub action: String,
}

/// Request for generating API key.
#[derive(Debug, Clone)]
pub struct ApiKeyGenerationRequest {
    pub sub_uid: i64,
    pub note: String,
    pub permission: Option<String>,
    pub ip_addresses: Option<String>,
}

impl SubUserCreationRequest {
    pub fn new(user_password: impl Into<String>, note: impl Into<String>) -> Self {
        Self {
            user_password: user_password.into(),
            note: note.into(),
        }
    }
}

impl SubuserTransferRequest {
    pub fn new(
        sub_uid: i64,
        currency: impl Into<String>,
        amount: impl Into<String>,
        transfer_type: impl Into<String>,
    ) -> Self {
        Self {
            sub_uid,
            currency: currency.into(),
            amount: amount.into(),
            transfer_type: transfer_type.into(),
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        serde_json::json!({
            "sub-uid": self.sub_uid,
            "currency": self.currency,
            "amount": self.amount,
            "type": self.transfer_type,
        })
    }
}

impl ManageSubUserRequest {
    pub fn freeze(sub_uid: i64) -> Self {
        Self {
            sub_uid,
            action: "freeze".into(),
        }
    }

    pub fn unfreeze(sub_uid: i64) -> Self {
        Self {
            sub_uid,
            action: "unfreeze".into(),
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        serde_json::json!({
            "subUid": self.sub_uid,
            "action": self.action,
        })
    }
}

impl ApiKeyGenerationRequest {
    pub fn new(sub_uid: i64, note: impl Into<String>) -> Self {
        Self {
            sub_uid,
            note: note.into(),
            permission: None,
            ip_addresses: None,
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        serde_json::json!({
            "subUid": self.sub_uid,
            "note": self.note,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_user_creation_request() {
        let req = SubUserCreationRequest::new("Pass1234", "test sub-user");
        assert_eq!(req.note, "test sub-user");
    }

    #[test]
    fn test_manage_sub_user_request() {
        let freeze = ManageSubUserRequest::freeze(12345);
        assert_eq!(freeze.action, "freeze");
        let unfreeze = ManageSubUserRequest::unfreeze(12345);
        assert_eq!(unfreeze.action, "unfreeze");
    }

    #[test]
    fn test_subuser_transfer_request() {
        let req = SubuserTransferRequest::new(12345, "usdt", "100", "master-transfer-in");
        let body = req.to_body();
        assert_eq!(body["sub-uid"], 12345);
    }
}
