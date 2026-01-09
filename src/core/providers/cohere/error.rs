//! Cohere Provider Error Handling
//!
//! Uses the unified ProviderError with Cohere-specific constructor methods

pub use crate::core::providers::unified_provider::ProviderError as CohereError;

/// Cohere-specific error constructors
impl CohereError {
    /// Create Cohere authentication error
    pub fn cohere_authentication(message: impl Into<String>) -> Self {
        Self::authentication("cohere", message)
    }

    /// Create Cohere rate limit error
    pub fn cohere_rate_limit(retry_after: Option<u64>) -> Self {
        Self::rate_limit("cohere", retry_after)
    }

    /// Create Cohere model not found error
    pub fn cohere_model_not_found(model: impl Into<String>) -> Self {
        Self::model_not_found("cohere", model)
    }

    /// Create Cohere invalid request error
    pub fn cohere_invalid_request(message: impl Into<String>) -> Self {
        Self::invalid_request("cohere", message)
    }

    /// Create Cohere network error
    pub fn cohere_network_error(message: impl Into<String>) -> Self {
        Self::network("cohere", message)
    }

    /// Create Cohere timeout error
    pub fn cohere_timeout(message: impl Into<String>) -> Self {
        Self::Timeout {
            provider: "cohere",
            message: message.into(),
        }
    }

    /// Create Cohere response parsing error
    pub fn cohere_response_parsing(message: impl Into<String>) -> Self {
        Self::response_parsing("cohere", message)
    }

    /// Create Cohere configuration error
    pub fn cohere_configuration(message: impl Into<String>) -> Self {
        Self::configuration("cohere", message)
    }

    /// Create Cohere API error with status code
    pub fn cohere_api_error(status: u16, message: impl Into<String>) -> Self {
        Self::ApiError {
            provider: "cohere",
            status,
            message: message.into(),
        }
    }

    /// Check if this is a Cohere-specific error
    pub fn is_cohere_error(&self) -> bool {
        self.provider() == "cohere"
    }

    /// Get Cohere error category for metrics
    pub fn cohere_category(&self) -> &'static str {
        match self {
            Self::Authentication { .. } => "auth",
            Self::RateLimit { .. } => "rate_limit",
            Self::ModelNotFound { .. } => "model",
            Self::Network { .. } | Self::Timeout { .. } => "network",
            Self::ResponseParsing { .. } | Self::Serialization { .. } => "parsing",
            Self::InvalidRequest { .. } => "invalid_request",
            Self::Configuration { .. } => "configuration",
            _ => "other",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohere_authentication_error() {
        let err = CohereError::cohere_authentication("Invalid API key");
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "auth");
    }

    #[test]
    fn test_cohere_rate_limit_error() {
        let err = CohereError::cohere_rate_limit(Some(60));
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "rate_limit");
    }

    #[test]
    fn test_cohere_model_not_found_error() {
        let err = CohereError::cohere_model_not_found("unknown-model");
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "model");
    }

    #[test]
    fn test_cohere_network_error() {
        let err = CohereError::cohere_network_error("Connection failed");
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "network");
    }

    #[test]
    fn test_cohere_timeout_error() {
        let err = CohereError::cohere_timeout("Request timed out");
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "network");
    }

    #[test]
    fn test_cohere_api_error() {
        let err = CohereError::cohere_api_error(500, "Internal server error");
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "other");
    }

    #[test]
    fn test_cohere_invalid_request() {
        let err = CohereError::cohere_invalid_request("Bad request format");
        assert!(err.is_cohere_error());
        assert_eq!(err.cohere_category(), "invalid_request");
    }
}
