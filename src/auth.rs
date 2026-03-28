use crate::error::HuobiError;
use crate::Result;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;
use url::Url;

type HmacSha256 = Hmac<Sha256>;

/// Build the signature base string for Huobi API authentication.
pub fn build_signature_base(
    method: &str,
    host: &str,
    path: &str,
    params: &BTreeMap<String, String>,
) -> String {
    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");
    format!("{}\n{}\n{}\n{}", method.to_uppercase(), host, path, query)
}

/// Sign the base string with HMAC-SHA256.
pub fn sign(secret_key: &str, base_string: &str) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|e| HuobiError::Auth(e.to_string()))?;
    mac.update(base_string.as_bytes());
    Ok(base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}

/// Create authentication parameters for a signed API request.
pub fn create_auth_params(
    access_key: &str,
    secret_key: &str,
    method: &str,
    host: &str,
    path: &str,
    mut params: BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    params.insert("AccessKeyId".into(), access_key.into());
    params.insert("SignatureMethod".into(), "HmacSHA256".into());
    params.insert("SignatureVersion".into(), "2".into());
    params.insert("Timestamp".into(), now);

    let base_string = build_signature_base(method, host, path, &params);
    let signature = sign(secret_key, &base_string)?;
    params.insert("Signature".into(), signature);
    Ok(params)
}

/// Build a URL with query parameters.
pub fn build_url(base_url: &str, path: &str, params: &BTreeMap<String, String>) -> Result<Url> {
    let full = format!("{}{}", base_url, path);
    let mut url = Url::parse(&full)?;
    for (k, v) in params {
        url.query_pairs_mut().append_pair(k, v);
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_signature_base() {
        let mut params = BTreeMap::new();
        params.insert("AccessKeyId".into(), "test-key".into());
        params.insert("SignatureMethod".into(), "HmacSHA256".into());
        params.insert("SignatureVersion".into(), "2".into());
        params.insert("Timestamp".into(), "2024-01-01T00:00:00".into());

        let base = build_signature_base("GET", "api.huobi.pro", "/v1/account/accounts", &params);
        assert!(base.starts_with("GET\napi.huobi.pro\n/v1/account/accounts\n"));
        assert!(base.contains("AccessKeyId=test-key"));
    }

    #[test]
    fn test_sign_produces_base64() {
        let sig = sign("my-secret", "test-base-string").unwrap();
        // Base64 encoded HMAC-SHA256 should be non-empty
        assert!(!sig.is_empty());
        // Should be valid base64
        assert!(base64::engine::general_purpose::STANDARD
            .decode(&sig)
            .is_ok());
    }

    #[test]
    fn test_sign_is_deterministic() {
        let sig1 = sign("secret", "message").unwrap();
        let sig2 = sign("secret", "message").unwrap();
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_sign_different_messages_differ() {
        let sig1 = sign("secret", "msg1").unwrap();
        let sig2 = sign("secret", "msg2").unwrap();
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_create_auth_params() {
        let params = create_auth_params(
            "my-key",
            "my-secret",
            "GET",
            "api.huobi.pro",
            "/v1/account/accounts",
            BTreeMap::new(),
        )
        .unwrap();

        assert_eq!(params.get("AccessKeyId").unwrap(), "my-key");
        assert_eq!(params.get("SignatureMethod").unwrap(), "HmacSHA256");
        assert_eq!(params.get("SignatureVersion").unwrap(), "2");
        assert!(params.contains_key("Timestamp"));
        assert!(params.contains_key("Signature"));
    }

    #[test]
    fn test_create_auth_params_preserves_existing() {
        let mut existing = BTreeMap::new();
        existing.insert("symbol".into(), "btcusdt".into());

        let params = create_auth_params(
            "key",
            "secret",
            "GET",
            "api.huobi.pro",
            "/v1/market",
            existing,
        )
        .unwrap();

        assert_eq!(params.get("symbol").unwrap(), "btcusdt");
        assert!(params.contains_key("Signature"));
    }

    #[test]
    fn test_build_url() {
        let mut params = BTreeMap::new();
        params.insert("symbol".into(), "btcusdt".into());
        params.insert("period".into(), "1min".into());

        let url = build_url("https://api.huobi.pro", "/v1/market/kline", &params).unwrap();
        assert!(url.as_str().contains("/v1/market/kline"));
        assert!(url.as_str().contains("symbol=btcusdt"));
        assert!(url.as_str().contains("period=1min"));
    }

    #[test]
    fn test_build_url_no_params() {
        let url =
            build_url("https://api.huobi.pro", "/v1/common/timestamp", &BTreeMap::new()).unwrap();
        assert_eq!(url.path(), "/v1/common/timestamp");
    }
}
