/// Request for creating an algo order.
#[derive(Debug, Clone)]
pub struct CreateAlgoOrderRequest {
    pub account_id: i64,
    pub symbol: String,
    pub order_type: String,
    pub order_side: String,
    pub order_price: Option<String>,
    pub order_size: Option<String>,
    pub order_value: Option<String>,
    pub time_in_force: Option<String>,
    pub stop_price: Option<String>,
    pub trailing_rate: Option<String>,
    pub client_order_id: Option<String>,
    pub source: Option<String>,
}

/// Request for opening algo orders.
#[derive(Debug, Clone)]
pub struct OpeningAlgoOrdersRequest {
    pub symbol: Option<String>,
    pub order_side: Option<String>,
    pub from: Option<i64>,
    pub size: Option<i32>,
    pub direct: Option<String>,
}

/// Request for history algo orders.
#[derive(Debug, Clone)]
pub struct HistoryAlgoOrdersRequest {
    pub symbol: Option<String>,
    pub order_side: Option<String>,
    pub order_type: Option<String>,
    pub order_status: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from: Option<i64>,
    pub size: Option<i32>,
    pub direct: Option<String>,
}

impl CreateAlgoOrderRequest {
    pub fn new(
        account_id: i64,
        symbol: impl Into<String>,
        order_type: impl Into<String>,
        order_side: impl Into<String>,
    ) -> Self {
        Self {
            account_id,
            symbol: symbol.into(),
            order_type: order_type.into(),
            order_side: order_side.into(),
            order_price: None,
            order_size: None,
            order_value: None,
            time_in_force: None,
            stop_price: None,
            trailing_rate: None,
            client_order_id: None,
            source: None,
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        let mut body = serde_json::json!({
            "account-id": self.account_id,
            "symbol": self.symbol,
            "order-type": self.order_type,
            "order-side": self.order_side,
        });
        if let Some(ref price) = self.order_price {
            body["order-price"] = serde_json::Value::String(price.clone());
        }
        if let Some(ref size) = self.order_size {
            body["order-size"] = serde_json::Value::String(size.clone());
        }
        if let Some(ref value) = self.order_value {
            body["order-value"] = serde_json::Value::String(value.clone());
        }
        if let Some(ref sp) = self.stop_price {
            body["stop-price"] = serde_json::Value::String(sp.clone());
        }
        if let Some(ref id) = self.client_order_id {
            body["client-order-id"] = serde_json::Value::String(id.clone());
        }
        body
    }
}

impl Default for HistoryAlgoOrdersRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryAlgoOrdersRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            order_side: None,
            order_type: None,
            order_status: None,
            start_time: None,
            end_time: None,
            from: None,
            size: None,
            direct: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(ref symbol) = self.symbol {
            params.insert("symbol".into(), symbol.clone());
        }
        if let Some(ref side) = self.order_side {
            params.insert("order-side".into(), side.clone());
        }
        if let Some(ref order_type) = self.order_type {
            params.insert("order-type".into(), order_type.clone());
        }
        if let Some(ref status) = self.order_status {
            params.insert("order-status".into(), status.clone());
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
    fn test_create_algo_order_request() {
        let req = CreateAlgoOrderRequest::new(12345, "btcusdt", "stop-loss", "sell");
        let body = req.to_body();
        assert_eq!(body["account-id"], 12345);
        assert_eq!(body["order-type"], "stop-loss");
    }

    #[test]
    fn test_history_algo_orders_request() {
        let req = HistoryAlgoOrdersRequest::new();
        let params = req.to_params();
        assert!(params.is_empty());
    }
}
