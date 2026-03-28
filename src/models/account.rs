/// Account information.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub id: i64,
    #[serde(rename = "type")]
    pub account_type: String,
    pub state: String,
    #[serde(default)]
    pub subtype: Option<String>,
}

/// Account balance.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Balance {
    pub currency: String,
    #[serde(rename = "type")]
    pub balance_type: String,
    pub balance: String,
    #[serde(default)]
    pub debt: Option<String>,
    #[serde(default)]
    pub available: Option<String>,
    #[serde(default)]
    pub seq_num: Option<String>,
}

/// Account history record.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountHistory {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub currency: String,
    #[serde(rename = "transactAmt")]
    pub transact_amt: String,
    #[serde(rename = "transactType")]
    pub transact_type: String,
    #[serde(rename = "availBalance")]
    pub avail_balance: String,
    #[serde(rename = "acctBalance")]
    pub acct_balance: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    #[serde(rename = "recordId")]
    pub record_id: i64,
}

/// Account ledger entry.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountLedger {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub currency: String,
    #[serde(rename = "transactAmt")]
    pub transact_amt: String,
    #[serde(rename = "transactType")]
    pub transact_type: String,
    #[serde(default, rename = "transferType")]
    pub transfer_type: Option<String>,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    #[serde(default, rename = "nextId")]
    pub next_id: Option<i64>,
    #[serde(default, rename = "transactId")]
    pub transact_id: Option<i64>,
}

/// Account with balances.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountAndBalance {
    pub id: i64,
    #[serde(rename = "type")]
    pub account_type: String,
    pub state: String,
    #[serde(default)]
    pub symbol: Option<String>,
    pub list: Vec<Balance>,
}

/// Account transfer response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountTransferResponse {
    #[serde(rename = "transactId")]
    pub transact_id: i64,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
}

/// Asset valuation.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AssetValuation {
    pub balance: String,
    pub timestamp: i64,
}

/// Point group.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Group {
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "expiryDate")]
    pub expiry_date: i64,
    #[serde(rename = "remainAmt")]
    pub remain_amt: String,
}

/// Point account.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PointAccount {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "accountStatus")]
    pub account_status: String,
    #[serde(rename = "acctBalance")]
    pub acct_balance: String,
    #[serde(default)]
    pub group_ids: Vec<Group>,
}

/// Profit account balance (for AccountValuation).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProfitAccountBalance {
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub balance: String,
}

/// Account valuation.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountValuation {
    #[serde(rename = "totalBalance")]
    pub total_balance: String,
    #[serde(default, rename = "todayProfit")]
    pub today_profit: Option<String>,
    #[serde(default, rename = "todayProfitRate")]
    pub today_profit_rate: Option<String>,
    #[serde(default, rename = "profitAccountBalanceList")]
    pub profit_account_balance_list: Vec<ProfitAccountBalance>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_deserialize() {
        let json = r#"{"id": 12345, "type": "spot", "state": "working"}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.id, 12345);
        assert_eq!(account.account_type, "spot");
        assert_eq!(account.state, "working");
    }

    #[test]
    fn test_account_with_subtype() {
        let json = r#"{"id": 1, "type": "margin", "state": "working", "subtype": "btcusdt"}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.subtype.as_deref(), Some("btcusdt"));
    }

    #[test]
    fn test_balance_deserialize() {
        let json = r#"{"currency": "usdt", "type": "trade", "balance": "1000.00"}"#;
        let balance: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.currency, "usdt");
        assert_eq!(balance.balance_type, "trade");
        assert_eq!(balance.balance, "1000.00");
    }

    #[test]
    fn test_balance_with_available() {
        let json = r#"{"currency": "btc", "type": "frozen", "balance": "0.5", "available": "0.5"}"#;
        let balance: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.available.as_deref(), Some("0.5"));
    }

    #[test]
    fn test_account_history_deserialize() {
        let json = r#"{
            "accountId": 12345,
            "currency": "usdt",
            "transactAmt": "100.0",
            "transactType": "trade",
            "availBalance": "900.0",
            "acctBalance": "1000.0",
            "transactTime": 1609459200000,
            "recordId": 100
        }"#;
        let history: AccountHistory = serde_json::from_str(json).unwrap();
        assert_eq!(history.account_id, 12345);
        assert_eq!(history.transact_type, "trade");
    }

    #[test]
    fn test_account_ledger_deserialize() {
        let json = r#"{
            "accountId": 12345,
            "currency": "btc",
            "transactAmt": "0.1",
            "transactType": "transfer",
            "transactTime": 1609459200000
        }"#;
        let ledger: AccountLedger = serde_json::from_str(json).unwrap();
        assert_eq!(ledger.account_id, 12345);
    }

    #[test]
    fn test_account_and_balance_deserialize() {
        let json = r#"{
            "id": 12345,
            "type": "spot",
            "state": "working",
            "list": [{"currency": "usdt", "type": "trade", "balance": "1000"}]
        }"#;
        let ab: AccountAndBalance = serde_json::from_str(json).unwrap();
        assert_eq!(ab.list.len(), 1);
        assert_eq!(ab.list[0].currency, "usdt");
    }

    #[test]
    fn test_account_transfer_response() {
        let json = r#"{"transactId": 999, "transactTime": 1609459200000}"#;
        let resp: AccountTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.transact_id, 999);
    }

    #[test]
    fn test_asset_valuation() {
        let json = r#"{"balance": "50000.0", "timestamp": 1609459200000}"#;
        let val: AssetValuation = serde_json::from_str(json).unwrap();
        assert_eq!(val.balance, "50000.0");
    }

    #[test]
    fn test_account_valuation() {
        let json = r#"{"totalBalance": "100000.0"}"#;
        let val: AccountValuation = serde_json::from_str(json).unwrap();
        assert_eq!(val.total_balance, "100000.0");
    }

    #[test]
    fn test_account_array_deserialize() {
        let json = r#"[{"id": 1, "type": "spot", "state": "working"}, {"id": 2, "type": "margin", "state": "working"}]"#;
        let accounts: Vec<Account> = serde_json::from_str(json).unwrap();
        assert_eq!(accounts.len(), 2);
    }
}
