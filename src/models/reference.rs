/// Trading symbol.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Symbol {
    #[serde(default, rename = "base-currency")]
    pub base_currency: String,
    #[serde(default, rename = "quote-currency")]
    pub quote_currency: String,
    #[serde(default, rename = "price-precision")]
    pub price_precision: Option<i32>,
    #[serde(default, rename = "amount-precision")]
    pub amount_precision: Option<i32>,
    #[serde(default, rename = "symbol-partition")]
    pub symbol_partition: Option<String>,
    pub symbol: String,
    pub state: String,
    #[serde(default, rename = "value-precision")]
    pub value_precision: Option<i32>,
    #[serde(default, rename = "min-order-amt")]
    pub min_order_amt: Option<f64>,
    #[serde(default, rename = "max-order-amt")]
    pub max_order_amt: Option<f64>,
    #[serde(default, rename = "min-order-value")]
    pub min_order_value: Option<f64>,
    #[serde(default, rename = "leverage-ratio")]
    pub leverage_ratio: Option<i32>,
    #[serde(default, rename = "limit-order-min-order-amt")]
    pub limit_order_min_order_amt: Option<f64>,
    #[serde(default, rename = "limit-order-max-order-amt")]
    pub limit_order_max_order_amt: Option<f64>,
    #[serde(default, rename = "sell-market-min-order-amt")]
    pub sell_market_min_order_amt: Option<f64>,
    #[serde(default, rename = "sell-market-max-order-amt")]
    pub sell_market_max_order_amt: Option<f64>,
    #[serde(default, rename = "buy-market-max-order-amt")]
    pub buy_market_max_order_amt: Option<f64>,
    #[serde(default, rename = "max-order-value")]
    pub max_order_value: Option<f64>,
}

/// Chain info for a currency.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Chain {
    pub chain: String,
    #[serde(default, rename = "numOfConfirmations")]
    pub num_of_confirmations: Option<i32>,
    #[serde(default, rename = "numOfFastConfirmations")]
    pub num_of_fast_confirmations: Option<i32>,
    #[serde(default, rename = "depositStatus")]
    pub deposit_status: Option<String>,
    #[serde(default, rename = "minDepositAmt")]
    pub min_deposit_amt: Option<String>,
    #[serde(default, rename = "withdrawStatus")]
    pub withdraw_status: Option<String>,
    #[serde(default, rename = "minWithdrawAmt")]
    pub min_withdraw_amt: Option<String>,
    #[serde(default, rename = "withdrawPrecision")]
    pub withdraw_precision: Option<i32>,
    #[serde(default, rename = "maxWithdrawAmt")]
    pub max_withdraw_amt: Option<String>,
    #[serde(default, rename = "withdrawQuotaPerDay")]
    pub withdraw_quota_per_day: Option<String>,
    #[serde(default, rename = "withdrawQuotaPerYear")]
    pub withdraw_quota_per_year: Option<String>,
    #[serde(default, rename = "withdrawQuotaTotal")]
    pub withdraw_quota_total: Option<String>,
    #[serde(default, rename = "withdrawFeeType")]
    pub withdraw_fee_type: Option<String>,
    #[serde(default, rename = "transactFeeWithdraw")]
    pub transact_fee_withdraw: Option<String>,
    #[serde(default, rename = "baseChain")]
    pub base_chain: Option<String>,
    #[serde(default, rename = "baseChainProtocol")]
    pub base_chain_protocol: Option<String>,
}

/// Currency and chain information.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReferenceCurrencies {
    pub currency: String,
    #[serde(default, rename = "instStatus")]
    pub inst_status: Option<String>,
    #[serde(default)]
    pub chains: Vec<Chain>,
}

/// Market symbols (abbreviated fields).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MarketSymbols {
    pub symbol: String,
    #[serde(default)]
    pub bc: Option<String>,
    #[serde(default)]
    pub qc: Option<String>,
    pub state: String,
    #[serde(default)]
    pub sp: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
    #[serde(default)]
    pub lr: Option<f64>,
    #[serde(default)]
    pub pp: Option<i32>,
    #[serde(default)]
    pub ap: Option<i32>,
    #[serde(default)]
    pub vp: Option<i32>,
    #[serde(default)]
    pub minoa: Option<f64>,
    #[serde(default)]
    pub maxoa: Option<f64>,
    #[serde(default)]
    pub minov: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_deserialize() {
        let json = r#"{
            "base-currency": "btc",
            "quote-currency": "usdt",
            "price-precision": 2,
            "amount-precision": 6,
            "symbol-partition": "main",
            "symbol": "btcusdt",
            "state": "online"
        }"#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.symbol, "btcusdt");
        assert_eq!(symbol.base_currency, "btc");
        assert_eq!(symbol.quote_currency, "usdt");
        assert_eq!(symbol.price_precision, Some(2));
        assert_eq!(symbol.state, "online");
    }

    #[test]
    fn test_symbol_with_order_limits() {
        let json = r#"{
            "base-currency": "eth",
            "quote-currency": "usdt",
            "symbol": "ethusdt",
            "state": "online",
            "min-order-amt": 0.001,
            "max-order-amt": 10000
        }"#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.min_order_amt, Some(0.001));
    }

    #[test]
    fn test_chain_deserialize() {
        let json = r#"{
            "chain": "trc20usdt",
            "numOfConfirmations": 20,
            "depositStatus": "allowed",
            "withdrawStatus": "allowed",
            "minWithdrawAmt": "1.0"
        }"#;
        let chain: Chain = serde_json::from_str(json).unwrap();
        assert_eq!(chain.chain, "trc20usdt");
        assert_eq!(chain.num_of_confirmations, Some(20));
    }

    #[test]
    fn test_reference_currencies_deserialize() {
        let json = r#"{
            "currency": "usdt",
            "instStatus": "normal",
            "chains": [
                {"chain": "trc20usdt", "depositStatus": "allowed"},
                {"chain": "erc20usdt", "depositStatus": "allowed"}
            ]
        }"#;
        let ref_curr: ReferenceCurrencies = serde_json::from_str(json).unwrap();
        assert_eq!(ref_curr.currency, "usdt");
        assert_eq!(ref_curr.chains.len(), 2);
    }

    #[test]
    fn test_symbol_array() {
        let json = r#"[{"symbol": "btcusdt", "state": "online"}, {"symbol": "ethusdt", "state": "online"}]"#;
        let symbols: Vec<Symbol> = serde_json::from_str(json).unwrap();
        assert_eq!(symbols.len(), 2);
    }

    #[test]
    fn test_reference_currencies_array() {
        let json = r#"[{"currency": "btc"}, {"currency": "usdt"}]"#;
        let currencies: Vec<ReferenceCurrencies> = serde_json::from_str(json).unwrap();
        assert_eq!(currencies.len(), 2);
    }

    #[test]
    fn test_market_symbols_deserialize() {
        let json = r#"{"symbol": "btcusdt", "state": "online", "pp": 2, "ap": 6}"#;
        let ms: MarketSymbols = serde_json::from_str(json).unwrap();
        assert_eq!(ms.symbol, "btcusdt");
        assert_eq!(ms.pp, Some(2));
    }
}
