use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuobiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: code={code}, message={message}")]
    Api { code: String, message: String },

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let err = HuobiError::Api {
            code: "bad-request".into(),
            message: "invalid symbol".into(),
        };
        assert_eq!(
            err.to_string(),
            "API error: code=bad-request, message=invalid symbol"
        );
    }

    #[test]
    fn test_config_error_display() {
        let err = HuobiError::Config("missing api key".into());
        assert_eq!(err.to_string(), "Config error: missing api key");
    }

    #[test]
    fn test_auth_error_display() {
        let err = HuobiError::Auth("invalid signature".into());
        assert_eq!(err.to_string(), "Authentication error: invalid signature");
    }

    #[test]
    fn test_websocket_error_display() {
        let err = HuobiError::WebSocket("connection closed".into());
        assert_eq!(err.to_string(), "WebSocket error: connection closed");
    }
}
