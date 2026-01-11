//! OpenAI-Like Provider Error Handling
//!
//! Uses the unified ProviderError with specific constructor methods for OpenAI-like contexts

pub use crate::core::providers::unified_provider::ProviderError as OpenAILikeError;

/// Provider name constant for error context
pub const PROVIDER_NAME: &str = "openai_like";

/// OpenAI-like specific error constructors
impl OpenAILikeError {
    /// Create OpenAI-like authentication error
    pub fn openai_like_authentication(message: impl Into<String>) -> Self {
        Self::authentication(PROVIDER_NAME, message)
    }

    /// Create OpenAI-like rate limit error
    pub fn openai_like_rate_limit(retry_after: Option<u64>) -> Self {
        Self::rate_limit(PROVIDER_NAME, retry_after)
    }

    /// Create OpenAI-like rate limit error with detailed context
    pub fn openai_like_rate_limit_with_limits(
        retry_after: Option<u64>,
        rpm_limit: Option<u32>,
        tpm_limit: Option<u32>,
        current_usage: Option<f64>,
    ) -> Self {
        Self::rate_limit_with_limits(
            PROVIDER_NAME,
            retry_after,
            rpm_limit,
            tpm_limit,
            current_usage,
        )
    }

    /// Create OpenAI-like model not found error
    pub fn openai_like_model_not_found(model: impl Into<String>) -> Self {
        Self::model_not_found(PROVIDER_NAME, model)
    }

    /// Create OpenAI-like invalid request error
    pub fn openai_like_invalid_request(message: impl Into<String>) -> Self {
        Self::invalid_request(PROVIDER_NAME, message)
    }

    /// Create OpenAI-like network error
    pub fn openai_like_network_error(message: impl Into<String>) -> Self {
        Self::network(PROVIDER_NAME, message)
    }

    /// Create OpenAI-like timeout error
    pub fn openai_like_timeout(message: impl Into<String>) -> Self {
        Self::Timeout {
            provider: PROVIDER_NAME,
            message: message.into(),
        }
    }

    /// Create OpenAI-like configuration error
    pub fn openai_like_configuration(message: impl Into<String>) -> Self {
        Self::configuration(PROVIDER_NAME, message)
    }

    /// Create OpenAI-like response parsing error
    pub fn openai_like_response_parsing(message: impl Into<String>) -> Self {
        Self::response_parsing(PROVIDER_NAME, message)
    }

    /// Create OpenAI-like streaming error
    pub fn openai_like_streaming_error(
        stream_type: impl Into<String>,
        position: Option<u64>,
        message: impl Into<String>,
    ) -> Self {
        Self::streaming_error(PROVIDER_NAME, stream_type, position, None, message)
    }

    /// Create OpenAI-like API error with status code
    pub fn openai_like_api_error(status: u16, message: impl Into<String>) -> Self {
        Self::ApiError {
            provider: PROVIDER_NAME,
            status,
            message: message.into(),
        }
    }

    /// Create OpenAI-like quota exceeded error
    pub fn openai_like_quota_exceeded(message: impl Into<String>) -> Self {
        Self::quota_exceeded(PROVIDER_NAME, message)
    }

    /// Create OpenAI-like service unavailable error
    pub fn openai_like_unavailable(message: impl Into<String>) -> Self {
        Self::provider_unavailable(PROVIDER_NAME, message)
    }

    /// Create generic OpenAI-like error
    pub fn openai_like_other(message: impl Into<String>) -> Self {
        Self::other(PROVIDER_NAME, message)
    }

    /// Check if this is an OpenAI-like error
    pub fn is_openai_like_error(&self) -> bool {
        self.provider() == PROVIDER_NAME
    }

    /// Get OpenAI-like error category for metrics
    pub fn openai_like_category(&self) -> &'static str {
        match self {
            Self::Authentication { .. } => "auth",
            Self::RateLimit { .. } => "rate_limit",
            Self::QuotaExceeded { .. } => "quota",
            Self::ModelNotFound { .. } => "model",
            Self::InvalidRequest { .. } => "invalid_request",
            Self::Network { .. } | Self::Timeout { .. } => "network",
            Self::ResponseParsing { .. } | Self::Serialization { .. } => "parsing",
            Self::Streaming { .. } => "streaming",
            Self::Configuration { .. } => "config",
            Self::ProviderUnavailable { .. } => "unavailable",
            _ => "other",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_error() {
        let err = OpenAILikeError::openai_like_authentication("Invalid API key");
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "auth");
    }

    #[test]
    fn test_rate_limit_error() {
        let err = OpenAILikeError::openai_like_rate_limit(Some(60));
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "rate_limit");
    }

    #[test]
    fn test_network_error() {
        let err = OpenAILikeError::openai_like_network_error("Connection failed");
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "network");
    }

    #[test]
    fn test_model_not_found_error() {
        let err = OpenAILikeError::openai_like_model_not_found("custom-model");
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "model");
    }

    #[test]
    fn test_configuration_error() {
        let err = OpenAILikeError::openai_like_configuration("Missing api_base");
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "config");
    }

    #[test]
    fn test_api_error() {
        let err = OpenAILikeError::openai_like_api_error(500, "Internal server error");
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "other");
    }

    #[test]
    fn test_streaming_error() {
        let err =
            OpenAILikeError::openai_like_streaming_error("chat", Some(100), "Stream interrupted");
        assert!(err.is_openai_like_error());
        assert_eq!(err.openai_like_category(), "streaming");
    }
}
