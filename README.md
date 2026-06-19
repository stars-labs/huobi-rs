# huobi-rs

An async Rust SDK for the [Huobi/HTX](https://www.htx.com/) Spot API v3, covering
REST and WebSocket endpoints for market data, trading, accounts, wallets, margin,
algo orders, and sub-user management.

## Features

- **Async** — built on `tokio` and `reqwest`.
- **Signed requests** — HMAC-SHA256 request signing handled automatically for
  private endpoints.
- **Domain clients** — one focused client per API area.
- **WebSocket** — real-time market, order, trade, and asset streams.
- **Typed models** — responses deserialized into structs via `serde`.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
huobi-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

### Public market data (no API key)

```rust,no_run
use huobi_rs::{Config, MarketClient};
use huobi_rs::request::market::TickerRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketClient::new(Config::public());
    let ticker = client.get_ticker(&TickerRequest::new("btcusdt")).await?;
    println!("BTC/USDT last price: {}", ticker.close);
    Ok(())
}
```

### Private endpoints (API key required)

```rust,no_run
use huobi_rs::{Config, AccountClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        std::env::var("HUOBI_ACCESS_KEY")?,
        std::env::var("HUOBI_SECRET_KEY")?,
    );

    let accounts = AccountClient::new(config).get_accounts().await?;
    for acc in &accounts {
        println!("id={} type={} state={}", acc.id, acc.account_type, acc.state);
    }
    Ok(())
}
```

## Clients

| Client | Purpose |
| --- | --- |
| `ReferenceClient` | Symbols, currencies, server timestamp |
| `MarketClient` | Candlesticks, tickers, order book, trades |
| `AccountClient` | Accounts, balances, ledger, transfers, valuation |
| `TradeClient` | Place, cancel, and query orders |
| `WalletClient` | Deposits, withdrawals, deposit addresses |
| `CrossMarginClient` / `IsolatedMarginClient` | Margin borrow/repay and accounts |
| `AlgoClient` | Conditional / algo orders |
| `SubUserClient` | Sub-user management and transfers |
| `WebSocketMarketClient` | Real-time market data streams |
| `WebSocketOrdersClient` / `WebSocketTradeClient` / `WebSocketAssetClient` | Authenticated real-time order, trade, and asset streams |

## Configuration

```rust
use huobi_rs::Config;

// Public-only
let config = Config::public();

// With credentials and custom options
let config = Config::new("access-key", "secret-key")
    .with_rest_url("https://api.huobi.pro")
    .with_ws_url("wss://api.huobi.pro")
    .with_timeout(30);
```

## Examples

Runnable examples live in [`examples/`](examples):

```bash
cargo run --example market                 # public market data
HUOBI_ACCESS_KEY=... HUOBI_SECRET_KEY=... cargo run --example trading
cargo run --example websocket
cargo run --example margin
```

## Development

```bash
cargo build
cargo test
cargo clippy
```

## License

Licensed under the [MIT License](LICENSE).
