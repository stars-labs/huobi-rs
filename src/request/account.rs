/// Request for account history.
#[derive(Debug, Clone)]
pub struct AccountHistoryRequest {
    pub account_id: i64,
    pub currency: String,
    pub transact_types: Option<String>,
    pub size: Option<i32>,
}

/// Request for account ledger.
#[derive(Debug, Clone)]
pub struct AccountLedgerRequest {
    pub account_id: i64,
    pub currency: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub size: Option<i32>,
}

/// Request for account transfer.
#[derive(Debug, Clone)]
pub struct AccountTransferRequest {
    pub from_user_id: i64,
    pub from_account_type: String,
    pub from_account_id: i64,
    pub to_user_id: i64,
    pub to_account_type: String,
    pub to_account_id: i64,
    pub currency: String,
    pub amount: String,
}

/// Request for asset valuation.
#[derive(Debug, Clone)]
pub struct AssetValuationRequest {
    pub account_type: String,
    pub valuation_currency: Option<String>,
    pub sub_uid: Option<i64>,
}

impl AccountHistoryRequest {
    pub fn new(account_id: i64, currency: impl Into<String>) -> Self {
        Self {
            account_id,
            currency: currency.into(),
            transact_types: None,
            size: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("account-id".into(), self.account_id.to_string());
        params.insert("currency".into(), self.currency.clone());
        if let Some(ref tt) = self.transact_types {
            params.insert("transact-types".into(), tt.clone());
        }
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        params
    }
}

impl AccountLedgerRequest {
    pub fn new(account_id: i64, currency: impl Into<String>) -> Self {
        Self {
            account_id,
            currency: currency.into(),
            start_time: None,
            end_time: None,
            size: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("accountId".into(), self.account_id.to_string());
        params.insert("currency".into(), self.currency.clone());
        if let Some(st) = self.start_time {
            params.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = self.end_time {
            params.insert("endTime".into(), et.to_string());
        }
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        params
    }
}

impl AssetValuationRequest {
    pub fn new(account_type: impl Into<String>) -> Self {
        Self {
            account_type: account_type.into(),
            valuation_currency: None,
            sub_uid: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("accountType".into(), self.account_type.clone());
        if let Some(ref vc) = self.valuation_currency {
            params.insert("valuationCurrency".into(), vc.clone());
        }
        if let Some(uid) = self.sub_uid {
            params.insert("subUid".into(), uid.to_string());
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_history_request() {
        let req = AccountHistoryRequest::new(12345, "usdt");
        let params = req.to_params();
        assert_eq!(params.get("account-id").unwrap(), "12345");
        assert_eq!(params.get("currency").unwrap(), "usdt");
    }

    #[test]
    fn test_account_ledger_request() {
        let req = AccountLedgerRequest::new(12345, "btc");
        let params = req.to_params();
        assert_eq!(params.get("accountId").unwrap(), "12345");
    }

    #[test]
    fn test_asset_valuation_request() {
        let req = AssetValuationRequest::new("spot");
        let params = req.to_params();
        assert_eq!(params.get("accountType").unwrap(), "spot");
    }
}
