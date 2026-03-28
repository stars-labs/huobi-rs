/// Request for placing an order.
#[derive(Debug, Clone)]
pub struct PlaceOrderRequest {
    pub account_id: i64,
    pub symbol: String,
    pub order_type: String,
    pub amount: String,
    pub price: Option<String>,
    pub client_order_id: Option<String>,
    pub stop_price: Option<String>,
    pub source: Option<String>,
    pub operator: Option<String>,
}

/// Request for open orders.
#[derive(Debug, Clone)]
pub struct OpenOrdersRequest {
    pub account_id: i64,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub size: Option<i32>,
    pub from: Option<i64>,
    pub direct: Option<String>,
    pub types: Option<String>,
}

/// Request for getting orders.
#[derive(Debug, Clone)]
pub struct GetOrdersRequest {
    pub symbol: String,
    pub states: String,
    pub types: Option<String>,
    pub direct: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from: Option<i64>,
    pub size: Option<i32>,
}

/// Request for historical orders.
#[derive(Debug, Clone)]
pub struct OrdersHistoryRequest {
    pub symbol: String,
    pub direct: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub size: Option<i32>,
}

/// Request for historical match results.
#[derive(Debug, Clone)]
pub struct MatchresultsHistoryRequest {
    pub symbol: String,
    pub types: Option<String>,
    pub direct: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub from: Option<i64>,
    pub size: Option<i32>,
}

/// Request for batch cancel open orders.
#[derive(Debug, Clone)]
pub struct BatchCancelOpenOrdersRequest {
    pub account_id: i64,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub types: Option<String>,
    pub size: Option<i32>,
}

/// Request for batch cancel orders.
#[derive(Debug, Clone)]
pub struct BatchCancelOrdersRequest {
    pub order_ids: Vec<i64>,
    pub client_order_ids: Vec<String>,
}

impl PlaceOrderRequest {
    pub fn new(
        account_id: i64,
        symbol: impl Into<String>,
        order_type: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            account_id,
            symbol: symbol.into(),
            order_type: order_type.into(),
            amount: amount.into(),
            price: None,
            client_order_id: None,
            stop_price: None,
            source: None,
            operator: None,
        }
    }

    pub fn with_price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn with_client_order_id(mut self, id: impl Into<String>) -> Self {
        self.client_order_id = Some(id.into());
        self
    }

    pub fn to_body(&self) -> serde_json::Value {
        let mut body = serde_json::Map::new();
        body.insert(
            "account-id".into(),
            serde_json::Value::Number(self.account_id.into()),
        );
        body.insert(
            "symbol".into(),
            serde_json::Value::String(self.symbol.clone()),
        );
        body.insert(
            "type".into(),
            serde_json::Value::String(self.order_type.clone()),
        );
        body.insert(
            "amount".into(),
            serde_json::Value::String(self.amount.clone()),
        );
        if let Some(ref price) = self.price {
            body.insert("price".into(), serde_json::Value::String(price.clone()));
        }
        if let Some(ref id) = self.client_order_id {
            body.insert(
                "client-order-id".into(),
                serde_json::Value::String(id.clone()),
            );
        }
        if let Some(ref sp) = self.stop_price {
            body.insert("stop-price".into(), serde_json::Value::String(sp.clone()));
        }
        if let Some(ref op) = self.operator {
            body.insert("operator".into(), serde_json::Value::String(op.clone()));
        }
        serde_json::Value::Object(body)
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        std::collections::BTreeMap::new()
    }
}

impl OpenOrdersRequest {
    pub fn new(account_id: i64) -> Self {
        Self {
            account_id,
            symbol: None,
            side: None,
            size: None,
            from: None,
            direct: None,
            types: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("account-id".into(), self.account_id.to_string());
        if let Some(ref symbol) = self.symbol {
            params.insert("symbol".into(), symbol.clone());
        }
        if let Some(ref side) = self.side {
            params.insert("side".into(), side.clone());
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
        if let Some(ref types) = self.types {
            params.insert("types".into(), types.clone());
        }
        params
    }
}

impl GetOrdersRequest {
    pub fn new(symbol: impl Into<String>, states: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            states: states.into(),
            types: None,
            direct: None,
            start_time: None,
            end_time: None,
            from: None,
            size: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        params.insert("states".into(), self.states.clone());
        if let Some(ref types) = self.types {
            params.insert("types".into(), types.clone());
        }
        if let Some(ref direct) = self.direct {
            params.insert("direct".into(), direct.clone());
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
        params
    }
}

impl OrdersHistoryRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            direct: None,
            start_time: None,
            end_time: None,
            size: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        if let Some(ref direct) = self.direct {
            params.insert("direct".into(), direct.clone());
        }
        if let Some(st) = self.start_time {
            params.insert("start-time".into(), st.to_string());
        }
        if let Some(et) = self.end_time {
            params.insert("end-time".into(), et.to_string());
        }
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        params
    }
}

impl BatchCancelOpenOrdersRequest {
    pub fn new(account_id: i64) -> Self {
        Self {
            account_id,
            symbol: None,
            side: None,
            types: None,
            size: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("account-id".into(), self.account_id.to_string());
        if let Some(ref symbol) = self.symbol {
            params.insert("symbol".into(), symbol.clone());
        }
        if let Some(ref side) = self.side {
            params.insert("side".into(), side.clone());
        }
        if let Some(ref types) = self.types {
            params.insert("types".into(), types.clone());
        }
        params
    }
}

impl Default for BatchCancelOrdersRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchCancelOrdersRequest {
    pub fn new() -> Self {
        Self {
            order_ids: Vec::new(),
            client_order_ids: Vec::new(),
        }
    }

    pub fn to_body(&self) -> serde_json::Value {
        let mut body = serde_json::Map::new();
        if !self.order_ids.is_empty() {
            body.insert(
                "order-ids".into(),
                serde_json::to_value(&self.order_ids).unwrap(),
            );
        }
        if !self.client_order_ids.is_empty() {
            body.insert(
                "client-order-ids".into(),
                serde_json::to_value(&self.client_order_ids).unwrap(),
            );
        }
        serde_json::Value::Object(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request() {
        let req = PlaceOrderRequest::new(12345, "btcusdt", "buy-limit", "0.1")
            .with_price("50000.0")
            .with_client_order_id("my-order");
        let body = req.to_body();
        assert_eq!(body["account-id"], 12345);
        assert_eq!(body["symbol"], "btcusdt");
        assert_eq!(body["type"], "buy-limit");
        assert_eq!(body["amount"], "0.1");
        assert_eq!(body["price"], "50000.0");
        assert_eq!(body["client-order-id"], "my-order");
    }

    #[test]
    fn test_place_order_without_optional() {
        let req = PlaceOrderRequest::new(1, "ethusdt", "buy-market", "100");
        let body = req.to_body();
        assert!(!body.as_object().unwrap().contains_key("price"));
    }

    #[test]
    fn test_open_orders_request() {
        let req = OpenOrdersRequest::new(12345);
        let params = req.to_params();
        assert_eq!(params.get("account-id").unwrap(), "12345");
    }

    #[test]
    fn test_get_orders_request() {
        let req = GetOrdersRequest::new("btcusdt", "filled,canceled");
        let params = req.to_params();
        assert_eq!(params.get("symbol").unwrap(), "btcusdt");
        assert_eq!(params.get("states").unwrap(), "filled,canceled");
    }

    #[test]
    fn test_batch_cancel_orders_request() {
        let req = BatchCancelOrdersRequest {
            order_ids: vec![1, 2, 3],
            client_order_ids: vec![],
        };
        let body = req.to_body();
        assert!(body.get("order-ids").is_some());
    }
}
