use huobi_rs::{AccountClient, Config};

/// Example: Private API access (requires API key)
///
/// Set environment variables:
///   HUOBI_ACCESS_KEY - Your Huobi API access key
///   HUOBI_SECRET_KEY - Your Huobi API secret key
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_key = std::env::var("HUOBI_ACCESS_KEY")
        .expect("Set HUOBI_ACCESS_KEY environment variable");
    let secret_key = std::env::var("HUOBI_SECRET_KEY")
        .expect("Set HUOBI_SECRET_KEY environment variable");

    let config = Config::new(&access_key, &secret_key);

    // Get account info
    let account_client = AccountClient::new(config.clone());
    let accounts = account_client.get_accounts().await?;
    println!("Accounts:");
    for acc in &accounts {
        println!("  id: {} type: {} state: {}", acc.id, acc.account_type, acc.state);
    }

    // Get account balance
    if let Some(account) = accounts.first() {
        let balances = account_client.get_balance(account.id).await?;
        println!("\nBalances for account {}:", account.id);
        for balance in &balances {
            if balance.balance != "0" {
                println!(
                    "  {} ({}): {} (available: {:?})",
                    balance.currency, balance.balance_type, balance.balance, balance.available
                );
            }
        }
    }

    // Example: Place an order (commented out to avoid accidental trades)
    /*
    let trade_client = TradeClient::new(config);
    let order = PlaceOrderRequest::new(
        accounts[0].id,
        "btcusdt",
        "buy-limit",
        "0.001",
    ).with_price("20000.0");
    let order_id = trade_client.place_order(&order).await?;
    println!("Placed order: {}", order_id);

    // Cancel the order
    trade_client.submit_cancel_order(order_id).await?;
    println!("Cancelled order: {}", order_id);
    */

    Ok(())
}
