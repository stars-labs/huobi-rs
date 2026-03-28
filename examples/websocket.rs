use huobi_rs::{Config, WebSocketMarketClient, MarketClient};
use huobi_rs::request::market::CandlestickRequest;

/// Example: WebSocket subscriptions for real-time data
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::public();

    // First, get current data via REST
    let market = MarketClient::new(config.clone());
    let candles = market
        .get_candlestick(&CandlestickRequest::new("btcusdt", "1min").with_size(3))
        .await?;
    println!("Current candlesticks via REST:");
    for c in &candles {
        println!("  open: {} close: {}", c.open, c.close);
    }

    // Subscribe to real-time updates via WebSocket
    println!("\nSubscribing to WebSocket updates...");
    println!("(In production, this would connect to Huobi's WebSocket)");
    
    let _ws_client = WebSocketMarketClient::new(&config);

    // Subscribe to candlestick updates
    // Note: This will attempt to connect to the actual WebSocket endpoint
    // For testing, you can use the mock pattern shown in tests
    println!("WebSocket client created with URL: {}", config.ws_url);

    // Example subscription (commented for demo - requires running WS connection):
    /*
    ws_client.sub_kline("btcusdt", "1min", |candle| {
        println!("New candle: open={} close={}", candle.open, candle.close);
    }).await?;

    ws_client.sub_trade("btcusdt", |trade| {
        println!("Trade: price={} amount={} direction={}", 
            trade.price, trade.amount, trade.direction);
    }).await?;

    // Keep running to receive updates
    tokio::signal::ctrl_c().await?;
    */

    Ok(())
}
