/// Request for creating a withdrawal.
#[derive(Debug, Clone)]
pub struct WithdrawCreateRequest {
    pub address: String,
    pub amount: String,
    pub currency: String,
    pub fee: Option<String>,
    pub address_tag: Option<String>,
    pub chain: Option<String>,
    pub client_order_id: Option<String>,
}

/// Request for querying deposits/withdrawals.
#[derive(Debug, Clone)]
pub struct QueryDepositWithdrawRequest {
    pub record_type: String,
    pub currency: Option<String>,
    pub size: Option<i32>,
    pub from: Option<i64>,
    pub direct: Option<String>,
}

/// Request for getting withdrawal addresses.
#[derive(Debug, Clone)]
pub struct GetWithdrawAddressRequest {
    pub currency: String,
    pub chain: Option<String>,
    pub note: Option<String>,
    pub limit: Option<i32>,
    pub from_id: Option<i64>,
}

impl WithdrawCreateRequest {
    pub fn new(
        address: impl Into<String>,
        amount: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            amount: amount.into(),
            currency: currency.into(),
            fee: None,
            address_tag: None,
            chain: None,
            client_order_id: None,
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        let mut body = serde_json::Map::new();
        body.insert(
            "address".into(),
            serde_json::Value::String(self.address.clone()),
        );
        body.insert(
            "amount".into(),
            serde_json::Value::String(self.amount.clone()),
        );
        body.insert(
            "currency".into(),
            serde_json::Value::String(self.currency.clone()),
        );
        if let Some(ref fee) = self.fee {
            body.insert("fee".into(), serde_json::Value::String(fee.clone()));
        }
        if let Some(ref tag) = self.address_tag {
            body.insert(
                "address-tag".into(),
                serde_json::Value::String(tag.clone()),
            );
        }
        if let Some(ref chain) = self.chain {
            body.insert("chain".into(), serde_json::Value::String(chain.clone()));
        }
        serde_json::Value::Object(body)
    }
}

impl QueryDepositWithdrawRequest {
    pub fn new(record_type: impl Into<String>) -> Self {
        Self {
            record_type: record_type.into(),
            currency: None,
            size: None,
            from: None,
            direct: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("type".into(), self.record_type.clone());
        if let Some(ref currency) = self.currency {
            params.insert("currency".into(), currency.clone());
        }
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        if let Some(from) = self.from {
            params.insert("from".into(), from.to_string());
        }
        if let Some(ref direct) = self.direct {
            params.insert("direct".into(), direct.clone());
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_create_request() {
        let req = WithdrawCreateRequest::new("0x1234", "100", "usdt");
        let body = req.to_body();
        assert_eq!(body["address"], "0x1234");
        assert_eq!(body["amount"], "100");
        assert_eq!(body["currency"], "usdt");
    }

    #[test]
    fn test_query_deposit_withdraw_request() {
        let req = QueryDepositWithdrawRequest::new("withdraw");
        let params = req.to_params();
        assert_eq!(params.get("type").unwrap(), "withdraw");
    }
}
