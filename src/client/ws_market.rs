use crate::config::Config;
use crate::models::market::Candlestick;
use crate::ws::WsClient;

/// WebSocket market client (public, no auth required).
pub struct WebSocketMarketClient {
    ws: WsClient,
}

impl WebSocketMarketClient {
    pub fn new(config: &Config) -> Self {
        let url = format!("{}/ws", config.ws_url);
        Self {
            ws: WsClient::new(url),
        }
    }

    /// Subscribe to candlestick updates.
    pub async fn sub_kline<F>(
        &self,
        symbol: &str,
        period: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(Candlestick) + Send + 'static,
    {
        let channel = format!("market.{}.kline.{}", symbol, period);
        self.ws
            .subscribe_with_callback::<Candlestick, F>(&channel, callback)
            .await
    }

    /// Subscribe to trade updates.
    pub async fn sub_trade<F>(
        &self,
        symbol: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(crate::models::market::Trade) + Send + 'static,
    {
        let channel = format!("market.{}.trade.detail", symbol);
        self.ws
            .subscribe_with_callback::<crate::models::market::Trade, F>(&channel, callback)
            .await
    }

    /// Subscribe to market depth.
    pub async fn sub_depth<F>(
        &self,
        symbol: &str,
        step: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(crate::models::market::Depth) + Send + 'static,
    {
        let channel = format!("market.{}.depth.{}", symbol, step);
        self.ws
            .subscribe_with_callback::<crate::models::market::Depth, F>(&channel, callback)
            .await
    }

    /// Subscribe to ticker.
    pub async fn sub_ticker<F>(
        &self,
        symbol: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(crate::models::market::Ticker) + Send + 'static,
    {
        let channel = format!("market.{}.detail", symbol);
        self.ws
            .subscribe_with_callback::<crate::models::market::Ticker, F>(&channel, callback)
            .await
    }

    /// Subscribe to Best Bid/Offer (BBO) updates.
    pub async fn sub_bbo<F>(
        &self,
        symbol: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(crate::models::websocket::BBO) + Send + 'static,
    {
        let channel = format!("market.{}.bbo", symbol);
        self.ws
            .subscribe_with_callback::<crate::models::websocket::BBO, F>(&channel, callback)
            .await
    }

    /// Subscribe to Market By Price (MBP) updates.
    pub async fn sub_mbp<F>(
        &self,
        symbol: &str,
        levels: i32,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(crate::models::websocket::MBP) + Send + 'static,
    {
        let channel = format!("market.{}.mbp.{}", symbol, levels);
        self.ws
            .subscribe_with_callback::<crate::models::websocket::MBP, F>(&channel, callback)
            .await
    }

    /// Subscribe to market ticker (global ticker).
    pub async fn sub_market_ticker<F>(
        &self,
        symbol: &str,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Fn(crate::models::market::Ticker) + Send + 'static,
    {
        let channel = format!("market.{}.ticker", symbol);
        self.ws
            .subscribe_with_callback::<crate::models::market::Ticker, F>(&channel, callback)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_format() {
        // Test channel name formatting
        let ch = format!("market.{}.kline.{}", "btcusdt", "1min");
        assert_eq!(ch, "market.btcusdt.kline.1min");

        let ch = format!("market.{}.trade.detail", "ethusdt");
        assert_eq!(ch, "market.ethusdt.trade.detail");

        let ch = format!("market.{}.bbo", "btcusdt");
        assert_eq!(ch, "market.btcusdt.bbo");

        let ch = format!("market.{}.mbp.{}", "btcusdt", 20);
        assert_eq!(ch, "market.btcusdt.mbp.20");

        let ch = format!("market.{}.ticker", "btcusdt");
        assert_eq!(ch, "market.btcusdt.ticker");
    }

    #[test]
    fn test_ws_market_client_creation() {
        let config = Config::default();
        let _client = WebSocketMarketClient::new(&config);
        // Client is created successfully
    }
}
