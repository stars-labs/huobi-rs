/// Request for candlestick data.
#[derive(Debug, Clone)]
pub struct CandlestickRequest {
    pub symbol: String,
    pub period: String,
    pub size: Option<i32>,
}

/// Request for ticker data.
#[derive(Debug, Clone)]
pub struct TickerRequest {
    pub symbol: String,
}

/// Request for market depth.
#[derive(Debug, Clone)]
pub struct DepthRequest {
    pub symbol: String,
    pub depth: Option<i32>,
    pub step: Option<String>,
}

/// Request for latest trades.
#[derive(Debug, Clone)]
pub struct TradeRequest {
    pub symbol: String,
}

/// Request for historical trades.
#[derive(Debug, Clone)]
pub struct HistoryTradeRequest {
    pub symbol: String,
    pub size: Option<i32>,
}

impl CandlestickRequest {
    pub fn new(symbol: impl Into<String>, period: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            period: period.into(),
            size: None,
        }
    }

    pub fn with_size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        params.insert("period".into(), self.period.clone());
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        params
    }
}

impl TickerRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        params
    }
}

impl DepthRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            depth: None,
            step: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        if let Some(depth) = self.depth {
            params.insert("depth".into(), depth.to_string());
        }
        params.insert(
            "type".into(),
            self.step.clone().unwrap_or_else(|| "step0".into()),
        );
        params
    }
}

impl TradeRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        params
    }
}

impl HistoryTradeRequest {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            size: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        params.insert("symbol".into(), self.symbol.clone());
        if let Some(size) = self.size {
            params.insert("size".into(), size.to_string());
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candlestick_request() {
        let req = CandlestickRequest::new("btcusdt", "1min").with_size(50);
        let params = req.to_params();
        assert_eq!(params.get("symbol").unwrap(), "btcusdt");
        assert_eq!(params.get("period").unwrap(), "1min");
        assert_eq!(params.get("size").unwrap(), "50");
    }

    #[test]
    fn test_candlestick_request_default_size() {
        let req = CandlestickRequest::new("btcusdt", "1hour");
        let params = req.to_params();
        assert!(!params.contains_key("size"));
    }

    #[test]
    fn test_ticker_request() {
        let req = TickerRequest::new("ethusdt");
        let params = req.to_params();
        assert_eq!(params.get("symbol").unwrap(), "ethusdt");
    }

    #[test]
    fn test_depth_request() {
        let req = DepthRequest::new("btcusdt");
        let params = req.to_params();
        assert_eq!(params.get("symbol").unwrap(), "btcusdt");
        assert_eq!(params.get("type").unwrap(), "step0");
    }

    #[test]
    fn test_trade_request() {
        let req = TradeRequest::new("btcusdt");
        let params = req.to_params();
        assert_eq!(params.get("symbol").unwrap(), "btcusdt");
    }

    #[test]
    fn test_history_trade_request() {
        let req = HistoryTradeRequest::new("btcusdt");
        let params = req.to_params();
        assert_eq!(params.get("symbol").unwrap(), "btcusdt");
        assert!(!params.contains_key("size"));
    }
}
