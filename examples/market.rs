use huobi_rs::{Config, ReferenceClient, MarketClient};
use huobi_rs::request::market::{CandlestickRequest, TickerRequest};

/// Example: Public market data access (no API key required)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Public clients don't need API keys
    let config = Config::public();

    // Get exchange timestamp
    let ref_client = ReferenceClient::new(config.clone());
    let timestamp = ref_client.get_timestamp().await?;
    println!("Exchange timestamp: {}", timestamp);

    // Get all symbols
    let symbols = ref_client.get_symbols().await?;
    println!("Number of symbols: {}", symbols.len());
    if let Some(s) = symbols.first() {
        println!("First symbol: {} ({}/{})", s.symbol, s.base_currency, s.quote_currency);
    }

    // Get currencies
    let currencies = ref_client.get_currencies().await?;
    println!("Supported currencies: {:?}", &currencies[..5.min(currencies.len())]);

    // Get candlestick data
    let market_client = MarketClient::new(config);
    let request = CandlestickRequest::new("btcusdt", "1min").with_size(5);
    let candles = market_client.get_candlestick(&request).await?;
    println!("\nLatest {} candlesticks for btcusdt:", candles.len());
    for candle in &candles {
        println!(
            "  open: {} close: {} high: {} low: {} vol: {}",
            candle.open, candle.close, candle.high, candle.low, candle.vol
        );
    }

    // Get ticker
    let ticker_req = TickerRequest::new("btcusdt");
    let ticker = market_client.get_ticker(&ticker_req).await?;
    println!("\nBTC/USDT ticker:");
    println!("  Last: {} Volume: {}", ticker.close, ticker.vol);

    Ok(())
}
