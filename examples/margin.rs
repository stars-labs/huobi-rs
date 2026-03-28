use huobi_rs::{Config, CrossMarginClient, IsolatedMarginClient};

/// Example: Margin trading operations
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_key = std::env::var("HUOBI_ACCESS_KEY")
        .expect("Set HUOBI_ACCESS_KEY environment variable");
    let secret_key = std::env::var("HUOBI_SECRET_KEY")
        .expect("Set HUOBI_SECRET_KEY environment variable");

    let config = Config::new(&access_key, &secret_key);

    // Cross Margin
    let cross_client = CrossMarginClient::new(config.clone());

    // Get cross margin balance
    let balance = cross_client.get_balance(0).await?;
    println!("Cross Margin Account:");
    println!("  ID: {} State: {}", balance.id, balance.state);
    for b in &balance.list {
        println!("  {}: {}", b.currency, b.balance);
    }

    // Get loan info
    let loan_info = cross_client.get_loan_info().await?;
    println!("\nCross Margin Loan Info:");
    for info in &loan_info {
        println!(
            "  {}: rate={} min={} max={}",
            info.currency, info.interest_rate, info.min_loan_amt, info.max_loan_amt
        );
    }

    // Example: Apply for loan (commented out)
    /*
    let request = CrossMarginTransferOrApplyRequest::new("usdt", "100");
    let loan_id = cross_client.margin_orders(&request).await?;
    println!("Loan ID: {}", loan_id);
    */

    // Isolated Margin
    let iso_client = IsolatedMarginClient::new(config);

    // Get isolated margin balance for btcusdt
    let iso_balances = iso_client.get_balance("btcusdt").await?;
    println!("\nIsolated Margin for btcusdt:");
    for b in &iso_balances {
        println!("  Symbol: {} Risk Rate: {}", b.symbol, b.risk_rate);
        for bal in &b.list {
            println!("    {}: {}", bal.currency, bal.balance);
        }
    }

    Ok(())
}
