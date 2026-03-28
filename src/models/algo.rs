/// Algo order (stop-loss, take-profit, trailing stop).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AlgoOrder {
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(default, rename = "orderPrice")]
    pub order_price: Option<String>,
    #[serde(default, rename = "stopPrice")]
    pub stop_price: Option<String>,
    #[serde(default, rename = "orderSize")]
    pub order_size: Option<String>,
    #[serde(default, rename = "orderValue")]
    pub order_value: Option<String>,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "orderSide")]
    pub order_side: String,
    #[serde(default, rename = "trailingRate")]
    pub trailing_rate: Option<String>,
    #[serde(default, rename = "algoType")]
    pub algo_type: Option<String>,
    #[serde(default, rename = "timeInForce")]
    pub time_in_force: Option<String>,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(default, rename = "lastActTime")]
    pub last_act_time: Option<i64>,
    #[serde(default, rename = "orderOrigTime")]
    pub order_orig_time: Option<i64>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default, rename = "orderCreateTime")]
    pub order_create_time: Option<i64>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<i32>,
    #[serde(default, rename = "errMessage")]
    pub err_message: Option<String>,
}

/// Cancel algo order result.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CancelAlgoOrderResult {
    pub accepted: Vec<String>,
    #[serde(default)]
    pub rejected: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_order_deserialize() {
        let json = r#"{
            "accountId": 12345,
            "symbol": "btcusdt",
            "orderId": 999,
            "orderPrice": "50000",
            "orderSize": "0.1",
            "orderType": "stop-loss",
            "orderSide": "sell",
            "orderStatus": "created"
        }"#;
        let order: AlgoOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "btcusdt");
        assert_eq!(order.order_type, "stop-loss");
        assert_eq!(order.order_side, "sell");
    }

    #[test]
    fn test_algo_order_with_optional_fields() {
        let json = r#"{
            "clientOrderId": "my-order",
            "accountId": 1,
            "symbol": "ethusdt",
            "orderId": 100,
            "orderType": "trailing-stop",
            "orderSide": "buy",
            "orderStatus": "created",
            "trailingRate": "5",
            "algoType": "trailing-stop"
        }"#;
        let order: AlgoOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.client_order_id.as_deref(), Some("my-order"));
        assert_eq!(order.algo_type.as_deref(), Some("trailing-stop"));
    }

    #[test]
    fn test_cancel_algo_order_result() {
        let json = r#"{
            "accepted": ["123", "456"],
            "rejected": []
        }"#;
        let result: CancelAlgoOrderResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.accepted.len(), 2);
        assert!(result.rejected.is_empty());
    }

    #[test]
    fn test_algo_order_array() {
        let json = r#"[{"accountId": 1, "symbol": "btcusdt", "orderId": 1, "orderType": "stop-loss", "orderSide": "sell", "orderStatus": "created"}]"#;
        let orders: Vec<AlgoOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 1);
    }
}
