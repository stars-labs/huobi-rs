/// ETP (Exchange Traded Product) reference data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EtpReference {
    pub currency: String,
    #[serde(rename = "etpName")]
    pub etp_name: String,
    #[serde(default, rename = "nav")]
    pub nav: Option<String>,
    #[serde(default, rename = "outstandingShares")]
    pub outstanding_shares: Option<String>,
    #[serde(default, rename = "leverage")]
    pub leverage: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, rename = "creationFeeRate")]
    pub creation_fee_rate: Option<String>,
    #[serde(default, rename = "redemptionFeeRate")]
    pub redemption_fee_rate: Option<String>,
    #[serde(default, rename = "minCreation")]
    pub min_creation: Option<String>,
    #[serde(default, rename = "minRedemption")]
    pub min_redemption: Option<String>,
    #[serde(default, rename = "publishRate")]
    pub publish_rate: Option<String>,
}

/// ETP transaction (creation or redemption).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EtpTransaction {
    pub id: i64,
    #[serde(rename = "etpName")]
    pub etp_name: String,
    #[serde(rename = "transactAmt")]
    pub transact_amt: String,
    #[serde(rename = "transactType")]
    pub transact_type: String,
    #[serde(default, rename = "nav")]
    pub nav: Option<String>,
    #[serde(default, rename = "units")]
    pub units: Option<String>,
    #[serde(default, rename = "feeAmt")]
    pub fee_amt: Option<String>,
    #[serde(default, rename = "feeCoin")]
    pub fee_coin: Option<String>,
    #[serde(default, rename = "transactState")]
    pub transact_state: Option<String>,
    #[serde(default, rename = "transactTime")]
    pub transact_time: Option<i64>,
}

/// ETP rebalance record.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EtpRebalance {
    #[serde(default, rename = "etpName")]
    pub etp_name: Option<String>,
    #[serde(default, rename = "navDate")]
    pub nav_date: Option<i64>,
    #[serde(default, rename = "leverage")]
    pub leverage: Option<String>,
    #[serde(default, rename = "newRebalanceShares")]
    pub new_rebalance_shares: Option<String>,
    #[serde(default, rename = "positionBal")]
    pub position_bal: Option<String>,
    #[serde(default, rename = "netAsset")]
    pub net_asset: Option<String>,
}

/// ETP limit (holding limit) response.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EtpLimit {
    pub currency: String,
    #[serde(default, rename = "maxRedeem")]
    pub max_redeem: Option<String>,
    #[serde(default, rename = "maxBuy")]
    pub max_buy: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etp_reference_deserialize() {
        let json = r#"{
            "currency": "hb10",
            "etpName": "HB10",
            "nav": "1.234",
            "outstandingShares": "1000000",
            "leverage": "3"
        }"#;
        let etp: EtpReference = serde_json::from_str(json).unwrap();
        assert_eq!(etp.currency, "hb10");
        assert_eq!(etp.etp_name, "HB10");
    }

    #[test]
    fn test_etp_transaction_deserialize() {
        let json = r#"{
            "id": 12345,
            "etpName": "HB10",
            "transactAmt": "100",
            "transactType": "creation"
        }"#;
        let tx: EtpTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(tx.id, 12345);
        assert_eq!(tx.transact_type, "creation");
    }
}
