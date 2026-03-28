/// Request for reference currencies.
#[derive(Debug, Clone)]
pub struct ReferenceCurrenciesRequest {
    pub currency: Option<String>,
    pub authorized_user: Option<String>,
}

impl Default for ReferenceCurrenciesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferenceCurrenciesRequest {
    pub fn new() -> Self {
        Self {
            currency: None,
            authorized_user: None,
        }
    }

    pub fn to_params(&self) -> std::collections::BTreeMap<String, String> {
        let mut params = std::collections::BTreeMap::new();
        if let Some(ref currency) = self.currency {
            params.insert("currency".into(), currency.clone());
        }
        if let Some(ref au) = self.authorized_user {
            params.insert("authorizedUser".into(), au.clone());
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_currencies_request_empty() {
        let req = ReferenceCurrenciesRequest::new();
        let params = req.to_params();
        assert!(params.is_empty());
    }
}
