/// Request for transferring to/from isolated margin.
#[derive(Debug, Clone)]
pub struct IsolatedMarginTransferOrApplyRequest {
    pub symbol: String,
    pub currency: String,
    pub amount: String,
    pub transfer_type: String,
}

/// Request for isolated margin balance.
#[derive(Debug, Clone)]
pub struct IsolatedMarginBalanceRequest {
    pub symbols: String,
}

/// Request for isolated margin loan orders.
#[derive(Debug, Clone)]
pub struct IsolatedMarginLoanOrdersRequest {
    pub symbol: String,
    pub currency: Option<String>,
    pub state: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from: Option<i64>,
    pub size: Option<i32>,
    pub direct: Option<String>,
}

/// Request for transferring to/from cross margin.
#[derive(Debug, Clone)]
pub struct CrossMarginTransferOrApplyRequest {
    pub currency: String,
    pub amount: String,
}

/// Request for cross margin loan orders.
#[derive(Debug, Clone)]
pub struct CrossMarginLoanOrdersRequest {
    pub currency: Option<String>,
    pub state: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from: Option<i64>,
    pub size: Option<i32>,
    pub direct: Option<String>,
}

impl IsolatedMarginTransferOrApplyRequest {
    pub fn new(
        symbol: impl Into<String>,
        currency: impl Into<String>,
        amount: impl Into<String>,
        transfer_type: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            currency: currency.into(),
            amount: amount.into(),
            transfer_type: transfer_type.into(),
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        serde_json::json!({
            "symbol": self.symbol,
            "currency": self.currency,
            "amount": self.amount,
            "type": self.transfer_type,
        })
    }
}

impl CrossMarginTransferOrApplyRequest {
    pub fn new(currency: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            amount: amount.into(),
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        serde_json::json!({
            "currency": self.currency,
            "amount": self.amount,
        })
    }
}

impl IsolatedMarginLoanOrdersRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            currency: None,
            state: None,
            start_time: None,
            end_time: None,
            from: None,
            size: None,
            direct: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        if let Some(ref currency) = self.currency {
            params.insert("currency".into(), currency.clone());
        }
        if let Some(ref state) = self.state {
            params.insert("state".into(), state.clone());
        }
        if let Some(st) = self.start_time {
            params.insert("start-time".into(), st.to_string());
        }
        if let Some(et) = self.end_time {
            params.insert("end-time".into(), et.to_string());
        }
        if let Some(from) = self.from {
            params.insert("from".into(), from.to_string());
        }
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        if let Some(ref direct) = self.direct {
            params.insert("direct".into(), direct.clone());
        }
        params
    }
}

impl Default for CrossMarginLoanOrdersRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossMarginLoanOrdersRequest {
    pub fn new() -> Self {
        Self {
            currency: None,
            state: None,
            start_time: None,
            end_time: None,
            from: None,
            size: None,
            direct: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(ref currency) = self.currency {
            params.insert("currency".into(), currency.clone());
        }
        if let Some(ref state) = self.state {
            params.insert("state".into(), state.clone());
        }
        if let Some(st) = self.start_time {
            params.insert("start-time".into(), st.to_string());
        }
        if let Some(et) = self.end_time {
            params.insert("end-time".into(), et.to_string());
        }
        if let Some(from) = self.from {
            params.insert("from".into(), from.to_string());
        }
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
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
    fn test_isolated_margin_transfer_request() {
        let req = IsolatedMarginTransferOrApplyRequest::new("btcusdt", "usdt", "100", "transfer");
        let body = req.to_body();
        assert_eq!(body["symbol"], "btcusdt");
        assert_eq!(body["currency"], "usdt");
    }

    #[test]
    fn test_cross_margin_transfer_request() {
        let req = CrossMarginTransferOrApplyRequest::new("usdt", "500");
        let body = req.to_body();
        assert_eq!(body["amount"], "500");
    }
}
