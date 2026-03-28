/// Candlestick (K-line) data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Candlestick {
    pub id: i64,
    pub amount: String,
    pub count: i64,
    pub open: String,
    pub close: String,
    pub low: String,
    pub high: String,
    pub vol: String,
    #[serde(default)]
    pub version: Option<i64>,
}

/// Price/size pair for order book levels.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DepthPair {
    pub price: String,
    pub size: String,
}

/// Market depth (order book) data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Depth {
    pub asks: Vec<DepthPair>,
    pub bids: Vec<DepthPair>,
    pub ts: i64,
    pub version: Option<i64>,
}

/// A single trade.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Trade {
    pub id: i64,
    #[serde(default)]
    pub trade_id: Option<i64>,
    pub amount: String,
    pub price: String,
    pub ts: i64,
    pub direction: String,
}

/// Ticker for a trading pair.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ticker {
    pub id: i64,
    pub amount: String,
    pub count: i64,
    pub open: String,
    pub close: String,
    pub low: String,
    pub high: String,
    pub vol: String,
    #[serde(default)]
    pub symbol: Option<String>,
    pub bid: Option<DepthPair>,
    pub ask: Option<DepthPair>,
}

/// Market status.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MarketStatus {
    #[serde(rename = "marketStatus")]
    pub market_status: i32,
    #[serde(default, rename = "haltStartTime")]
    pub halt_start_time: Option<i64>,
    #[serde(default, rename = "haltEndTime")]
    pub halt_end_time: Option<i64>,
    #[serde(default, rename = "haltReason")]
    pub halt_reason: Option<i32>,
    #[serde(default, rename = "affectedSymbols")]
    pub affected_symbols: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candlestick_deserialize() {
        let json = r#"{"id": 1609459200, "amount": "123.45", "count": 100, "open": "50000.0", "close": "50100.0", "low": "49900.0", "high": "50200.0", "vol": "6172500.0"}"#;
        let candle: Candlestick = serde_json::from_str(json).unwrap();
        assert_eq!(candle.id, 1609459200);
        assert_eq!(candle.open, "50000.0");
        assert_eq!(candle.close, "50100.0");
        assert_eq!(candle.high, "50200.0");
        assert_eq!(candle.low, "49900.0");
    }

    #[test]
    fn test_depth_deserialize() {
        let json = r#"{
            "asks": [{"price": "50100.0", "size": "0.5"}],
            "bids": [{"price": "50000.0", "size": "1.0"}],
            "ts": 1609459200000
        }"#;
        let depth: Depth = serde_json::from_str(json).unwrap();
        assert_eq!(depth.asks.len(), 1);
        assert_eq!(depth.bids.len(), 1);
        assert_eq!(depth.asks[0].price, "50100.0");
        assert_eq!(depth.bids[0].size, "1.0");
    }

    #[test]
    fn test_trade_deserialize() {
        let json = r#"{"id": 12345, "amount": "0.1", "price": "50000.0", "ts": 1609459200000, "direction": "buy"}"#;
        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 12345);
        assert_eq!(trade.direction, "buy");
        assert_eq!(trade.price, "50000.0");
    }

    #[test]
    fn test_trade_with_trade_id() {
        let json = r#"{"id": 1, "trade_id": 999, "amount": "1.0", "price": "100.0", "ts": 100, "direction": "sell"}"#;
        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, Some(999));
    }

    #[test]
    fn test_ticker_deserialize() {
        let json = r#"{
            "id": 1609459200,
            "amount": "1000.0",
            "count": 500,
            "open": "50000.0",
            "close": "50100.0",
            "low": "49900.0",
            "high": "50200.0",
            "vol": "50000000.0",
            "symbol": "btcusdt"
        }"#;
        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol.as_deref(), Some("btcusdt"));
        assert_eq!(ticker.count, 500);
    }

    #[test]
    fn test_ticker_without_symbol() {
        let json = r#"{"id": 1, "amount": "10.0", "count": 5, "open": "100.0", "close": "101.0", "low": "99.0", "high": "102.0", "vol": "1000.0"}"#;
        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert!(ticker.symbol.is_none());
    }

    #[test]
    fn test_market_status_deserialize() {
        let json = r#"{"marketStatus": 0}"#;
        let status: MarketStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.market_status, 0);
    }

    #[test]
    fn test_depth_pair_deserialize() {
        let json = r#"{"price": "100.5", "size": "2.5"}"#;
        let pair: DepthPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.price, "100.5");
        assert_eq!(pair.size, "2.5");
    }

    #[test]
    fn test_candlestick_array_deserialize() {
        let json = r#"[{"id": 1, "amount": "10", "count": 5, "open": "100", "close": "101", "low": "99", "high": "102", "vol": "1000"}]"#;
        let candles: Vec<Candlestick> = serde_json::from_str(json).unwrap();
        assert_eq!(candles.len(), 1);
    }
}
