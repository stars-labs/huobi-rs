/// Isolated margin loan info - currency detail.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LoanCurrencyInfo {
    pub currency: String,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    #[serde(rename = "minLoanAmt")]
    pub min_loan_amt: String,
    #[serde(rename = "maxLoanAmt")]
    pub max_loan_amt: String,
    #[serde(rename = "loanableAmt")]
    pub loanable_amt: String,
    #[serde(default, rename = "actualRate")]
    pub actual_rate: Option<String>,
}

/// Isolated margin loan info.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct IsolatedMarginLoanInfo {
    pub symbol: String,
    pub currencies: Vec<LoanCurrencyInfo>,
}

/// Isolated margin loan order.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct IsolatedMarginLoanOrder {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub symbol: String,
    pub currency: String,
    #[serde(rename = "loanAmount")]
    pub loan_amount: String,
    #[serde(rename = "loanBalance")]
    pub loan_balance: String,
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
    #[serde(rename = "interestAmount")]
    pub interest_amount: String,
    #[serde(rename = "interestBalance")]
    pub interest_balance: String,
    pub state: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "accruedAt")]
    pub accrued_at: i64,
    #[serde(default, rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

/// Isolated margin balance.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct IsolatedMarginBalance {
    pub id: i64,
    pub symbol: String,
    pub state: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(rename = "riskRate")]
    pub risk_rate: String,
    #[serde(default, rename = "flPrice")]
    pub fl_price: Option<String>,
    #[serde(default, rename = "flType")]
    pub fl_type: Option<String>,
    pub list: Vec<super::account::Balance>,
}

/// Cross margin loan order.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CrossMarginLoanOrder {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub currency: String,
    #[serde(rename = "loanAmount")]
    pub loan_amount: String,
    #[serde(rename = "loanBalance")]
    pub loan_balance: String,
    #[serde(rename = "interestAmount")]
    pub interest_amount: String,
    #[serde(rename = "interestBalance")]
    pub interest_balance: String,
    pub state: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "accruedAt")]
    pub accrued_at: i64,
    #[serde(default, rename = "filledPoints")]
    pub filled_points: Option<String>,
    #[serde(default, rename = "filledHt")]
    pub filled_ht: Option<String>,
}

/// Cross margin balance.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CrossMarginBalance {
    pub id: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(default, rename = "acctBalanceSum")]
    pub acct_balance_sum: Option<String>,
    #[serde(default, rename = "debtBalanceSum")]
    pub debt_balance_sum: Option<String>,
    #[serde(default, rename = "riskRate")]
    pub risk_rate: Option<String>,
    pub list: Vec<super::account::Balance>,
}

/// Margin limit response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MarginLimitResponse {
    pub currency: String,
    #[serde(rename = "maxHoldings")]
    pub max_holdings: String,
}

/// Cross margin general repay response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CrossMarginGeneralRepay {
    #[serde(rename = "repayId")]
    pub repay_id: i64,
    #[serde(rename = "repayTime")]
    pub repay_time: i64,
}

/// Transact entry in repay record.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RepayTransact {
    #[serde(rename = "transactId")]
    pub transact_id: i64,
    #[serde(rename = "repaidPrincipal")]
    pub repaid_principal: String,
    #[serde(rename = "repaidInterest")]
    pub repaid_interest: String,
    #[serde(default, rename = "paidHt")]
    pub paid_ht: Option<String>,
    #[serde(default, rename = "paidPoint")]
    pub paid_point: Option<String>,
}

/// Cross margin repay loan record.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CrossMarginRepayLoanRecord {
    #[serde(rename = "repayId")]
    pub repay_id: i64,
    #[serde(rename = "repayTime")]
    pub repay_time: i64,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub currency: String,
    #[serde(rename = "repaidAmount")]
    pub repaid_amount: String,
    #[serde(default, rename = "transactIds")]
    pub transact_ids: Vec<RepayTransact>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_currency_info() {
        let json = r#"{
            "currency": "usdt",
            "interestRate": "0.001",
            "minLoanAmt": "10",
            "maxLoanAmt": "100000",
            "loanableAmt": "50000"
        }"#;
        let info: LoanCurrencyInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.currency, "usdt");
        assert_eq!(info.interest_rate, "0.001");
    }

    #[test]
    fn test_isolated_margin_loan_info() {
        let json = r#"{
            "symbol": "btcusdt",
            "currencies": [{"currency": "btc", "interestRate": "0.001", "minLoanAmt": "0.001", "maxLoanAmt": "10", "loanableAmt": "5"}]
        }"#;
        let info: IsolatedMarginLoanInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.symbol, "btcusdt");
        assert_eq!(info.currencies.len(), 1);
    }

    #[test]
    fn test_isolated_margin_loan_order() {
        let json = r#"{
            "id": 100,
            "userId": 12345,
            "accountId": 67890,
            "symbol": "btcusdt",
            "currency": "usdt",
            "loanAmount": "1000",
            "loanBalance": "900",
            "interestRate": "0.001",
            "interestAmount": "10",
            "interestBalance": "5",
            "state": "accrual",
            "createdAt": 1609459200000,
            "accruedAt": 1609459200000
        }"#;
        let order: IsolatedMarginLoanOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 100);
        assert_eq!(order.state, "accrual");
    }

    #[test]
    fn test_cross_margin_balance() {
        let json = r#"{
            "id": 12345,
            "state": "working",
            "type": "cross-margin",
            "list": [{"currency": "usdt", "type": "trade", "balance": "10000"}]
        }"#;
        let balance: CrossMarginBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.id, 12345);
        assert_eq!(balance.list.len(), 1);
    }

    #[test]
    fn test_cross_margin_loan_order() {
        let json = r#"{
            "id": 100,
            "userId": 12345,
            "accountId": 67890,
            "currency": "usdt",
            "loanAmount": "5000",
            "loanBalance": "4500",
            "interestAmount": "50",
            "interestBalance": "25",
            "state": "accrual",
            "createdAt": 1609459200000,
            "accruedAt": 1609459200000
        }"#;
        let order: CrossMarginLoanOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.currency, "usdt");
    }

    #[test]
    fn test_cross_margin_general_repay() {
        let json = r#"{"repayId": 999, "repayTime": 1609459200000}"#;
        let repay: CrossMarginGeneralRepay = serde_json::from_str(json).unwrap();
        assert_eq!(repay.repay_id, 999);
    }
}
