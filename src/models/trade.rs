/// Open order.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OpenOrder {
    pub id: i64,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub amount: String,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(rename = "filledAmount")]
    pub filled_amount: String,
    #[serde(rename = "filledCashAmount")]
    pub filled_cash_amount: String,
    #[serde(rename = "filledFees")]
    pub filled_fees: String,
    #[serde(default)]
    pub source: Option<String>,
    pub state: String,
    #[serde(default, rename = "stopPrice")]
    pub stop_price: Option<String>,
    #[serde(default, rename = "operator")]
    pub operator: Option<String>,
}

/// Order (includes cancellation info).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub id: i64,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub amount: String,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(default, rename = "canceledAt")]
    pub canceled_at: Option<i64>,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(rename = "filledAmount")]
    pub filled_amount: String,
    #[serde(rename = "filledCashAmount")]
    pub filled_cash_amount: String,
    #[serde(rename = "filledFees")]
    pub filled_fees: String,
    #[serde(default)]
    pub source: Option<String>,
    pub state: String,
    #[serde(default, rename = "stopPrice")]
    pub stop_price: Option<String>,
    #[serde(default, rename = "operator")]
    pub operator: Option<String>,
}

/// Order match result.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MatchResult {
    pub id: i64,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "tradeId")]
    pub trade_id: i64,
    #[serde(rename = "matchId")]
    pub match_id: i64,
    pub symbol: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(default)]
    pub source: Option<String>,
    pub price: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "filledAmount")]
    pub filled_amount: String,
    #[serde(rename = "filledFees")]
    pub filled_fees: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default, rename = "filledPoints")]
    pub filled_points: Option<String>,
    #[serde(default, rename = "feeDeductCurrency")]
    pub fee_deduct_currency: Option<String>,
    #[serde(default, rename = "feeCurrency")]
    pub fee_currency: Option<String>,
    #[serde(default, rename = "feeDeductState")]
    pub fee_deduct_state: Option<String>,
}

/// Place order response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<String>,
    #[serde(default, rename = "errMsg")]
    pub err_msg: Option<String>,
}

/// Batch cancel open orders response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BatchCancelOpenOrders {
    #[serde(rename = "successCount")]
    pub success_count: i32,
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
    #[serde(default, rename = "nextId")]
    pub next_id: Option<i64>,
}

/// Failed cancel order entry.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FailedOrder {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<String>,
    #[serde(default, rename = "errMsg")]
    pub err_msg: Option<String>,
    #[serde(default, rename = "orderState")]
    pub order_state: Option<String>,
}

/// Batch cancel orders response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BatchCancelOrders {
    pub success: Vec<String>,
    #[serde(default)]
    pub failed: Vec<FailedOrder>,
}

/// Transaction fee rate.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TransactFeeRate {
    pub symbol: String,
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
    #[serde(default, rename = "actualMakerRate")]
    pub actual_maker_rate: Option<String>,
    #[serde(default, rename = "actualTakerRate")]
    pub actual_taker_rate: Option<String>,
}

/// Cancel all after response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CancelAllAfterResponse {
    #[serde(rename = "currentTime")]
    pub current_time: i64,
    #[serde(rename = "triggerTime")]
    pub trigger_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_order_deserialize() {
        let json = r#"{
            "id": 12345,
            "accountId": 100,
            "symbol": "btcusdt",
            "amount": "0.1",
            "price": "50000.0",
            "createdAt": 1609459200000,
            "type": "buy-limit",
            "filledAmount": "0.0",
            "filledCashAmount": "0.0",
            "filledFees": "0.0",
            "state": "submitted"
        }"#;
        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 12345);
        assert_eq!(order.symbol, "btcusdt");
        assert_eq!(order.order_type, "buy-limit");
    }

    #[test]
    fn test_order_deserialize() {
        let json = r#"{
            "id": 12345,
            "accountId": 100,
            "symbol": "ethusdt",
            "amount": "1.0",
            "price": "3000.0",
            "createdAt": 1609459200000,
            "type": "sell-limit",
            "filledAmount": "0.5",
            "filledCashAmount": "1500.0",
            "filledFees": "0.75",
            "state": "partial-filled"
        }"#;
        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "ethusdt");
        assert_eq!(order.filled_amount, "0.5");
        assert!(order.canceled_at.is_none());
    }

    #[test]
    fn test_order_with_cancel() {
        let json = r#"{
            "id": 1, "accountId": 100, "symbol": "btcusdt", "amount": "1",
            "createdAt": 100, "type": "buy-limit", "filledAmount": "0",
            "filledCashAmount": "0", "filledFees": "0", "state": "canceled",
            "canceledAt": 200
        }"#;
        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.canceled_at, Some(200));
        assert_eq!(order.state, "canceled");
    }

    #[test]
    fn test_match_result_deserialize() {
        let json = r#"{
            "id": 100,
            "orderId": 12345,
            "tradeId": 500,
            "matchId": 999,
            "symbol": "btcusdt",
            "type": "buy-limit",
            "price": "50000.0",
            "createdAt": 1609459200000,
            "filledAmount": "0.1",
            "filledFees": "0.5"
        }"#;
        let result: MatchResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.order_id, 12345);
        assert_eq!(result.price, "50000.0");
    }

    #[test]
    fn test_place_order_response() {
        let json = r#"{"orderId": 12345678}"#;
        let resp: PlaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.order_id, 12345678);
    }

    #[test]
    fn test_place_order_response_with_error() {
        let json = r#"{"orderId": 0, "errCode": "1001", "errMsg": "insufficient balance"}"#;
        let resp: PlaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.err_code.as_deref(), Some("1001"));
    }

    #[test]
    fn test_batch_cancel_open_orders() {
        let json = r#"{"successCount": 3, "failedCount": 0}"#;
        let resp: BatchCancelOpenOrders = serde_json::from_str(json).unwrap();
        assert_eq!(resp.success_count, 3);
        assert_eq!(resp.failed_count, 0);
    }

    #[test]
    fn test_batch_cancel_orders() {
        let json = r#"{
            "success": ["123", "456"],
            "failed": [{"orderId": "789", "errCode": "error"}]
        }"#;
        let resp: BatchCancelOrders = serde_json::from_str(json).unwrap();
        assert_eq!(resp.success.len(), 2);
        assert_eq!(resp.failed.len(), 1);
        assert_eq!(resp.failed[0].order_id, "789");
    }

    #[test]
    fn test_transact_fee_rate() {
        let json = r#"{
            "symbol": "btcusdt",
            "makerFeeRate": "0.002",
            "takerFeeRate": "0.002"
        }"#;
        let rate: TransactFeeRate = serde_json::from_str(json).unwrap();
        assert_eq!(rate.symbol, "btcusdt");
        assert_eq!(rate.maker_fee_rate, "0.002");
    }

    #[test]
    fn test_cancel_all_after_response() {
        let json = r#"{"currentTime": 1000, "triggerTime": 2000}"#;
        let resp: CancelAllAfterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.current_time, 1000);
        assert_eq!(resp.trigger_time, 2000);
    }

    #[test]
    fn test_order_array_deserialize() {
        let json = r#"[{"id": 1, "accountId": 1, "symbol": "btcusdt", "amount": "1", "createdAt": 1, "type": "buy-limit", "filledAmount": "0", "filledCashAmount": "0", "filledFees": "0", "state": "submitted"}]"#;
        let orders: Vec<Order> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 1);
    }
}
