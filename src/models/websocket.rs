/// MBP (Market By Price) order book snapshot.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MBP {
    pub asks: Vec<super::market::DepthPair>,
    pub bids: Vec<super::market::DepthPair>,
    #[serde(rename = "seqNum")]
    pub seq_num: i64,
    #[serde(rename = "prevSeqNum")]
    pub prev_seq_num: i64,
}

/// BBO (Best Bid Offer).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BBO {
    pub symbol: String,
    #[serde(rename = "quoteTime")]
    pub quote_time: i64,
    pub bid: String,
    #[serde(rename = "bidSize")]
    pub bid_size: String,
    pub ask: String,
    #[serde(rename = "askSize")]
    pub ask_size: String,
    #[serde(rename = "seqId")]
    pub seq_id: i64,
}

/// Account change update (WebSocket).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AccountsUpdate {
    pub currency: String,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    pub balance: String,
    pub available: String,
    #[serde(rename = "changeType")]
    pub change_type: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(rename = "seqNum")]
    pub seq_num: i64,
}

/// Order update (WebSocket).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OrdersUpdate {
    pub event: String,
    pub symbol: String,
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(default, rename = "orderPrice")]
    pub order_price: Option<String>,
    #[serde(default, rename = "orderSize")]
    pub order_size: Option<String>,
    #[serde(default)]
    pub order_type: Option<String>,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "orderCreateTime")]
    pub order_create_time: i64,
    #[serde(default, rename = "tradePrice")]
    pub trade_price: Option<String>,
    #[serde(default, rename = "tradeVolume")]
    pub trade_volume: Option<String>,
    #[serde(default, rename = "tradeId")]
    pub trade_id: Option<i64>,
    #[serde(default, rename = "tradeTime")]
    pub trade_time: Option<i64>,
    #[serde(default)]
    pub aggressor: Option<bool>,
    #[serde(default, rename = "remainAmt")]
    pub remain_amt: Option<String>,
    #[serde(default, rename = "lastActTime")]
    pub last_act_time: Option<i64>,
    #[serde(default, rename = "orderSide")]
    pub order_side: Option<String>,
    #[serde(default, rename = "errCode")]
    pub err_code: Option<i32>,
    #[serde(default, rename = "errMessage")]
    pub err_message: Option<String>,
    #[serde(default, rename = "orderValue")]
    pub order_value: Option<String>,
}

/// Trade clearing update (WebSocket).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TradeClearing {
    pub event: String,
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "tradePrice")]
    pub trade_price: String,
    #[serde(rename = "tradeVolume")]
    pub trade_volume: String,
    #[serde(rename = "orderSide")]
    pub order_side: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(default)]
    pub aggressor: Option<bool>,
    #[serde(rename = "tradeId")]
    pub trade_id: i64,
    #[serde(rename = "tradeTime")]
    pub trade_time: i64,
    #[serde(default, rename = "transactFee")]
    pub transact_fee: Option<String>,
    #[serde(default, rename = "feeCurrency")]
    pub fee_currency: Option<String>,
    #[serde(default, rename = "accountId")]
    pub account_id: Option<i64>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default, rename = "orderPrice")]
    pub order_price: Option<String>,
    #[serde(default, rename = "orderSize")]
    pub order_size: Option<String>,
    #[serde(default, rename = "orderValue")]
    pub order_value: Option<String>,
    #[serde(default, rename = "clientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(default, rename = "stopPrice")]
    pub stop_price: Option<String>,
    #[serde(default, rename = "orderCreateTime")]
    pub order_create_time: Option<i64>,
    #[serde(default, rename = "orderStatus")]
    pub order_status: Option<String>,
    #[serde(default, rename = "remainAmt")]
    pub remain_amt: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbp_deserialize() {
        let json = r#"{
            "asks": [{"price": "50100", "size": "0.5"}],
            "bids": [{"price": "50000", "size": "1.0"}],
            "seqNum": 100,
            "prevSeqNum": 99
        }"#;
        let mbp: MBP = serde_json::from_str(json).unwrap();
        assert_eq!(mbp.seq_num, 100);
        assert_eq!(mbp.asks.len(), 1);
    }

    #[test]
    fn test_bbo_deserialize() {
        let json = r#"{
            "symbol": "btcusdt",
            "quoteTime": 1609459200000,
            "bid": "50000",
            "bidSize": "1.0",
            "ask": "50100",
            "askSize": "0.5",
            "seqId": 100
        }"#;
        let bbo: BBO = serde_json::from_str(json).unwrap();
        assert_eq!(bbo.symbol, "btcusdt");
        assert_eq!(bbo.bid, "50000");
    }

    #[test]
    fn test_accounts_update_deserialize() {
        let json = r#"{
            "currency": "usdt",
            "accountId": 12345,
            "balance": "10000",
            "available": "9000",
            "changeType": "transfer",
            "accountType": "spot",
            "changeTime": 1609459200000,
            "seqNum": 1
        }"#;
        let update: AccountsUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.currency, "usdt");
        assert_eq!(update.change_type, "transfer");
    }

    #[test]
    fn test_orders_update_deserialize() {
        let json = r#"{
            "event": "order.create",
            "symbol": "btcusdt",
            "accountId": 12345,
            "orderId": 999,
            "orderStatus": "submitted",
            "orderCreateTime": 1609459200000,
            "orderSide": "buy"
        }"#;
        let update: OrdersUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.event, "order.create");
        assert_eq!(update.order_id, 999);
        assert_eq!(update.order_side.as_deref(), Some("buy"));
    }

    #[test]
    fn test_trade_clearing_deserialize() {
        let json = r#"{
            "event": "trade.clearing",
            "symbol": "btcusdt",
            "orderId": 999,
            "tradePrice": "50000",
            "tradeVolume": "0.1",
            "orderSide": "buy",
            "orderType": "limit",
            "tradeId": 500,
            "tradeTime": 1609459200000
        }"#;
        let clearing: TradeClearing = serde_json::from_str(json).unwrap();
        assert_eq!(clearing.symbol, "btcusdt");
        assert_eq!(clearing.trade_price, "50000");
    }
}
