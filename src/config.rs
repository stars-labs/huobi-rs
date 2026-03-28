/// Configuration for the Huobi client.
#[derive(Debug, Clone)]
pub struct Config {
    /// Huobi API access key (optional for public endpoints)
    pub access_key: Option<String>,
    /// Huobi API secret key (optional for public endpoints)
    pub secret_key: Option<String>,
    /// REST API base URL
    pub rest_url: String,
    /// WebSocket API base URL
    pub ws_url: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            access_key: None,
            secret_key: None,
            rest_url: "https://api.huobi.pro".to_string(),
            ws_url: "wss://api.huobi.pro".to_string(),
            timeout_secs: 30,
        }
    }
}

impl Config {
    /// Create a public-only config (no API keys needed).
    pub fn public() -> Self {
        Self::default()
    }

    /// Create a config with API credentials.
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            access_key: Some(access_key.into()),
            secret_key: Some(secret_key.into()),
            ..Self::default()
        }
    }

    /// Set custom REST base URL.
    pub fn with_rest_url(mut self, url: impl Into<String>) -> Self {
        self.rest_url = url.into();
        self
    }

    /// Set custom WebSocket base URL.
    pub fn with_ws_url(mut self, url: impl Into<String>) -> Self {
        self.ws_url = url.into();
        self
    }

    /// Set request timeout.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.access_key.is_none());
        assert!(config.secret_key.is_none());
        assert_eq!(config.rest_url, "https://api.huobi.pro");
        assert_eq!(config.ws_url, "wss://api.huobi.pro");
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_public_config() {
        let config = Config::public();
        assert!(config.access_key.is_none());
    }

    #[test]
    fn test_config_with_credentials() {
        let config = Config::new("my-key", "my-secret");
        assert_eq!(config.access_key.as_deref(), Some("my-key"));
        assert_eq!(config.secret_key.as_deref(), Some("my-secret"));
    }

    #[test]
    fn test_config_builder() {
        let config = Config::new("k", "s")
            .with_rest_url("https://custom.api")
            .with_ws_url("wss://custom.api")
            .with_timeout(60);
        assert_eq!(config.rest_url, "https://custom.api");
        assert_eq!(config.ws_url, "wss://custom.api");
        assert_eq!(config.timeout_secs, 60);
    }
}
