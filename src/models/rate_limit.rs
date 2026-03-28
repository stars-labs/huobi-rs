/// Rate limit information from HTTP response headers.
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    /// Remaining requests in the current window.
    pub requests_remain: Option<i64>,
    /// Time window expiry in milliseconds.
    pub requests_expire: Option<i64>,
}

impl RateLimitInfo {
    /// Parse rate limit headers from an HTTP response.
    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> Self {
        let requests_remain = headers
            .get("X-HB-RateLimit-Requests-Remain")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        let requests_expire = headers
            .get("X-HB-RateLimit-Requests-Expire")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        Self {
            requests_remain,
            requests_expire,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue};

    #[test]
    fn test_rate_limit_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-HB-RateLimit-Requests-Remain",
            HeaderValue::from_static("95"),
        );
        headers.insert(
            "X-HB-RateLimit-Requests-Expire",
            HeaderValue::from_static("30000"),
        );

        let info = RateLimitInfo::from_headers(&headers);
        assert_eq!(info.requests_remain, Some(95));
        assert_eq!(info.requests_expire, Some(30000));
    }

    #[test]
    fn test_rate_limit_empty_headers() {
        let headers = HeaderMap::new();
        let info = RateLimitInfo::from_headers(&headers);
        assert!(info.requests_remain.is_none());
        assert!(info.requests_expire.is_none());
    }

    #[test]
    fn test_rate_limit_partial_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-HB-RateLimit-Requests-Remain",
            HeaderValue::from_static("100"),
        );

        let info = RateLimitInfo::from_headers(&headers);
        assert_eq!(info.requests_remain, Some(100));
        assert!(info.requests_expire.is_none());
    }
}
