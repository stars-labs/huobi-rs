/// Deposit address.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DepositAddress {
    pub currency: String,
    pub address: String,
    #[serde(default, rename = "addressTag")]
    pub address_tag: Option<String>,
    #[serde(default)]
    pub chain: Option<String>,
}

/// Withdrawal quota info.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WithdrawQuota {
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(rename = "maxWithdrawAmt")]
    pub max_withdraw_amt: String,
    #[serde(rename = "withdrawQuotaPerDay")]
    pub withdraw_quota_per_day: String,
    #[serde(rename = "remainWithdrawQuotaPerDay")]
    pub remain_withdraw_quota_per_day: String,
    #[serde(rename = "withdrawQuotaPerYear")]
    pub withdraw_quota_per_year: String,
    #[serde(rename = "remainWithdrawQuotaPerYear")]
    pub remain_withdraw_quota_per_year: String,
    #[serde(rename = "withdrawQuotaTotal")]
    pub withdraw_quota_total: String,
    #[serde(rename = "remainWithdrawQuotaTotal")]
    pub remain_withdraw_quota_total: String,
}

/// Deposit or withdrawal record.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DepositWithdraw {
    pub id: i64,
    #[serde(rename = "type")]
    pub record_type: String,
    pub currency: String,
    #[serde(default, rename = "txHash")]
    pub tx_hash: Option<String>,
    pub amount: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default, rename = "addressTag")]
    pub address_tag: Option<String>,
    #[serde(default)]
    pub fee: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub state: String,
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<String>,
    #[serde(default, rename = "errMessage")]
    pub err_message: Option<String>,
}

/// Withdrawal address response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetWithdrawAddressResponse {
    pub currency: String,
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default, rename = "addressTag")]
    pub address_tag: Option<String>,
    pub address: String,
}

/// Withdrawal by client order ID response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetWithdrawClientOrderIdResponse {
    pub address: String,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(default, rename = "addressTag")]
    pub address_tag: Option<String>,
    pub amount: f64,
    #[serde(rename = "blockchainConfirm")]
    pub blockchain_confirm: i32,
    #[serde(default)]
    pub chain: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub currency: String,
    #[serde(default, rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(default, rename = "errorMsg")]
    pub error_msg: Option<String>,
    pub fee: f64,
    pub id: i64,
    #[serde(default, rename = "requestId")]
    pub request_id: Option<String>,
    pub state: String,
    #[serde(default, rename = "txHash")]
    pub tx_hash: Option<String>,
    #[serde(rename = "type")]
    pub record_type: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "walletConfirm")]
    pub wallet_confirm: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_address_deserialize() {
        let json = r#"{
            "currency": "usdt",
            "address": "0x1234567890abcdef"
        }"#;
        let addr: DepositAddress = serde_json::from_str(json).unwrap();
        assert_eq!(addr.currency, "usdt");
        assert_eq!(addr.address, "0x1234567890abcdef");
    }

    #[test]
    fn test_deposit_address_with_tag_and_chain() {
        let json = r#"{
            "currency": "xrp",
            "address": "rXXX",
            "addressTag": "12345",
            "chain": "xrp"
        }"#;
        let addr: DepositAddress = serde_json::from_str(json).unwrap();
        assert_eq!(addr.address_tag.as_deref(), Some("12345"));
        assert_eq!(addr.chain.as_deref(), Some("xrp"));
    }

    #[test]
    fn test_withdraw_quota_deserialize() {
        let json = r#"{
            "maxWithdrawAmt": "10000",
            "withdrawQuotaPerDay": "5000",
            "remainWithdrawQuotaPerDay": "4000",
            "withdrawQuotaPerYear": "100000",
            "remainWithdrawQuotaPerYear": "90000",
            "withdrawQuotaTotal": "200000",
            "remainWithdrawQuotaTotal": "180000"
        }"#;
        let quota: WithdrawQuota = serde_json::from_str(json).unwrap();
        assert_eq!(quota.max_withdraw_amt, "10000");
    }

    #[test]
    fn test_deposit_withdraw_deserialize() {
        let json = r#"{
            "id": 100,
            "type": "withdraw",
            "currency": "usdt",
            "amount": "500",
            "createdAt": 1609459200000,
            "updatedAt": 1609459300000,
            "state": "confirmed"
        }"#;
        let record: DepositWithdraw = serde_json::from_str(json).unwrap();
        assert_eq!(record.record_type, "withdraw");
        assert_eq!(record.amount, "500");
        assert_eq!(record.state, "confirmed");
    }

    #[test]
    fn test_deposit_record() {
        let json = r#"{
            "id": 200,
            "type": "deposit",
            "currency": "btc",
            "txHash": "0xabc",
            "amount": "0.5",
            "createdAt": 1609459200000,
            "updatedAt": 1609459200000,
            "state": "safe"
        }"#;
        let record: DepositWithdraw = serde_json::from_str(json).unwrap();
        assert_eq!(record.record_type, "deposit");
        assert_eq!(record.tx_hash.as_deref(), Some("0xabc"));
    }

    #[test]
    fn test_get_withdraw_address_response() {
        let json = r#"{
            "currency": "usdt",
            "address": "0x1234"
        }"#;
        let resp: GetWithdrawAddressResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.currency, "usdt");
    }

    #[test]
    fn test_deposit_address_array() {
        let json = r#"[{"currency": "btc", "address": "bc1xxx"}]"#;
        let addrs: Vec<DepositAddress> = serde_json::from_str(json).unwrap();
        assert_eq!(addrs.len(), 1);
    }
}
