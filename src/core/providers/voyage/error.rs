//! Voyage AI-specific error types and error mapping
//!
//! Handles error conversion from Voyage AI API responses to unified provider errors.

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;
use crate::core::types::errors::ProviderErrorTrait;
use thiserror::Error;

/// Voyage AI-specific error types
#[derive(Debug, Error)]
pub enum VoyageError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Invalid request: {0}")]
    InvalidRequestError(String),

    #[error("Model not found: {0}")]
    ModelNotFoundError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailableError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Token limit exceeded: {0}")]
    TokenLimitExceeded(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl ProviderErrorTrait for VoyageError {
    fn error_type(&self) -> &'static str {
        match self {
            VoyageError::ApiError(_) => "api_error",
            VoyageError::AuthenticationError(_) => "authentication_error",
            VoyageError::RateLimitError(_) => "rate_limit_error",
            VoyageError::InvalidRequestError(_) => "invalid_request_error",
            VoyageError::ModelNotFoundError(_) => "model_not_found_error",
            VoyageError::ServiceUnavailableError(_) => "service_unavailable_error",
            VoyageError::ConfigurationError(_) => "configuration_error",
            VoyageError::NetworkError(_) => "network_error",
            VoyageError::TokenLimitExceeded(_) => "token_limit_exceeded",
            VoyageError::UnknownError(_) => "unknown_error",
        }
    }

    fn is_retryable(&self) -> bool {
        matches!(
            self,
            VoyageError::RateLimitError(_)
                | VoyageError::ServiceUnavailableError(_)
                | VoyageError::NetworkError(_)
        )
    }

    fn retry_delay(&self) -> Option<u64> {
        match self {
            VoyageError::RateLimitError(_) => Some(60),
            VoyageError::ServiceUnavailableError(_) => Some(5),
            VoyageError::NetworkError(_) => Some(2),
            _ => None,
        }
    }

    fn http_status(&self) -> u16 {
        match self {
            VoyageError::AuthenticationError(_) => 401,
            VoyageError::RateLimitError(_) => 429,
            VoyageError::InvalidRequestError(_) => 400,
            VoyageError::ModelNotFoundError(_) => 404,
            VoyageError::ServiceUnavailableError(_) => 503,
            VoyageError::TokenLimitExceeded(_) => 400,
            VoyageError::ApiError(_) => 500,
            _ => 500,
        }
    }

    fn not_supported(feature: &str) -> Self {
        VoyageError::InvalidRequestError(format!("Feature not supported: {}", feature))
    }

    fn authentication_failed(reason: &str) -> Self {
        VoyageError::AuthenticationError(reason.to_string())
    }

    fn rate_limited(retry_after: Option<u64>) -> Self {
        match retry_after {
            Some(seconds) => VoyageError::RateLimitError(format!(
                "Rate limit exceeded, retry after {} seconds",
                seconds
            )),
            None => VoyageError::RateLimitError("Rate limit exceeded".to_string()),
        }
    }

    fn network_error(details: &str) -> Self {
        VoyageError::NetworkError(details.to_string())
    }

    fn parsing_error(details: &str) -> Self {
        VoyageError::ApiError(format!("Response parsing error: {}", details))
    }

    fn not_implemented(feature: &str) -> Self {
        VoyageError::InvalidRequestError(format!("Feature not implemented: {}", feature))
    }
}

impl From<VoyageError> for ProviderError {
    fn from(error: VoyageError) -> Self {
        match error {
            VoyageError::ApiError(msg) => ProviderError::api_error("voyage", 500, msg),
            VoyageError::AuthenticationError(msg) => ProviderError::authentication("voyage", msg),
            VoyageError::RateLimitError(_) => ProviderError::rate_limit("voyage", None),
            VoyageError::InvalidRequestError(msg) => ProviderError::invalid_request("voyage", msg),
            VoyageError::ModelNotFoundError(msg) => ProviderError::model_not_found("voyage", msg),
            VoyageError::ServiceUnavailableError(msg) => {
                ProviderError::api_error("voyage", 503, msg)
            }
            VoyageError::ConfigurationError(msg) => ProviderError::configuration("voyage", msg),
            VoyageError::NetworkError(msg) => ProviderError::network("voyage", msg),
            VoyageError::TokenLimitExceeded(msg) => {
                ProviderError::invalid_request("voyage", format!("Token limit exceeded: {}", msg))
            }
            VoyageError::UnknownError(msg) => ProviderError::api_error("voyage", 500, msg),
        }
    }
}

/// Error mapper for Voyage AI provider
pub struct VoyageErrorMapper;

impl ErrorMapper<VoyageError> for VoyageErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> VoyageError {
        let message = if response_body.is_empty() {
            format!("HTTP error {}", status_code)
        } else {
            response_body.to_string()
        };

        match status_code {
            400 => {
                // Check for token limit exceeded
                if message.contains("token") && message.contains("limit") {
                    VoyageError::TokenLimitExceeded(message)
                } else {
                    VoyageError::InvalidRequestError(message)
                }
            }
            401 => VoyageError::AuthenticationError("Invalid API key".to_string()),
            403 => VoyageError::AuthenticationError("Access forbidden".to_string()),
            404 => VoyageError::ModelNotFoundError("Model not found".to_string()),
            429 => VoyageError::RateLimitError("Rate limit exceeded".to_string()),
            500 => VoyageError::ApiError("Internal server error".to_string()),
            502 => VoyageError::ServiceUnavailableError("Bad gateway".to_string()),
            503 => VoyageError::ServiceUnavailableError("Service unavailable".to_string()),
            _ => VoyageError::ApiError(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voyage_error_display() {
        let err = VoyageError::ApiError("test error".to_string());
        assert_eq!(err.to_string(), "API error: test error");

        let err = VoyageError::AuthenticationError("invalid key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: invalid key");

        let err = VoyageError::RateLimitError("limit exceeded".to_string());
        assert_eq!(err.to_string(), "Rate limit exceeded: limit exceeded");

        let err = VoyageError::TokenLimitExceeded("too many tokens".to_string());
        assert_eq!(err.to_string(), "Token limit exceeded: too many tokens");
    }

    #[test]
    fn test_voyage_error_type() {
        assert_eq!(
            VoyageError::ApiError("".to_string()).error_type(),
            "api_error"
        );
        assert_eq!(
            VoyageError::AuthenticationError("".to_string()).error_type(),
            "authentication_error"
        );
        assert_eq!(
            VoyageError::RateLimitError("".to_string()).error_type(),
            "rate_limit_error"
        );
        assert_eq!(
            VoyageError::TokenLimitExceeded("".to_string()).error_type(),
            "token_limit_exceeded"
        );
    }

    #[test]
    fn test_voyage_error_is_retryable() {
        assert!(VoyageError::RateLimitError("".to_string()).is_retryable());
        assert!(VoyageError::ServiceUnavailableError("".to_string()).is_retryable());
        assert!(VoyageError::NetworkError("".to_string()).is_retryable());

        assert!(!VoyageError::ApiError("".to_string()).is_retryable());
        assert!(!VoyageError::AuthenticationError("".to_string()).is_retryable());
        assert!(!VoyageError::TokenLimitExceeded("".to_string()).is_retryable());
    }

    #[test]
    fn test_voyage_error_retry_delay() {
        assert_eq!(
            VoyageError::RateLimitError("".to_string()).retry_delay(),
            Some(60)
        );
        assert_eq!(
            VoyageError::ServiceUnavailableError("".to_string()).retry_delay(),
            Some(5)
        );
        assert_eq!(
            VoyageError::NetworkError("".to_string()).retry_delay(),
            Some(2)
        );
        assert_eq!(VoyageError::ApiError("".to_string()).retry_delay(), None);
    }

    #[test]
    fn test_voyage_error_http_status() {
        assert_eq!(
            VoyageError::AuthenticationError("".to_string()).http_status(),
            401
        );
        assert_eq!(
            VoyageError::RateLimitError("".to_string()).http_status(),
            429
        );
        assert_eq!(
            VoyageError::InvalidRequestError("".to_string()).http_status(),
            400
        );
        assert_eq!(
            VoyageError::ModelNotFoundError("".to_string()).http_status(),
            404
        );
        assert_eq!(
            VoyageError::TokenLimitExceeded("".to_string()).http_status(),
            400
        );
    }

    #[test]
    fn test_voyage_error_factory_methods() {
        let err = VoyageError::not_supported("chat");
        assert!(matches!(err, VoyageError::InvalidRequestError(_)));

        let err = VoyageError::authentication_failed("bad key");
        assert!(matches!(err, VoyageError::AuthenticationError(_)));

        let err = VoyageError::rate_limited(Some(30));
        assert!(matches!(err, VoyageError::RateLimitError(_)));

        let err = VoyageError::network_error("connection failed");
        assert!(matches!(err, VoyageError::NetworkError(_)));

        let err = VoyageError::parsing_error("invalid json");
        assert!(matches!(err, VoyageError::ApiError(_)));
    }

    #[test]
    fn test_voyage_error_to_provider_error() {
        let err: ProviderError = VoyageError::AuthenticationError("bad key".to_string()).into();
        assert!(matches!(err, ProviderError::Authentication { .. }));

        let err: ProviderError = VoyageError::RateLimitError("limit".to_string()).into();
        assert!(matches!(err, ProviderError::RateLimit { .. }));

        let err: ProviderError = VoyageError::ModelNotFoundError("model".to_string()).into();
        assert!(matches!(err, ProviderError::ModelNotFound { .. }));

        let err: ProviderError = VoyageError::ConfigurationError("bad config".to_string()).into();
        assert!(matches!(err, ProviderError::Configuration { .. }));

        let err: ProviderError = VoyageError::NetworkError("timeout".to_string()).into();
        assert!(matches!(err, ProviderError::Network { .. }));
    }

    #[test]
    fn test_voyage_error_mapper_http_errors() {
        let mapper = VoyageErrorMapper;

        let err = mapper.map_http_error(400, "bad request");
        assert!(matches!(err, VoyageError::InvalidRequestError(_)));

        let err = mapper.map_http_error(400, "token limit exceeded");
        assert!(matches!(err, VoyageError::TokenLimitExceeded(_)));

        let err = mapper.map_http_error(401, "");
        assert!(matches!(err, VoyageError::AuthenticationError(_)));

        let err = mapper.map_http_error(429, "");
        assert!(matches!(err, VoyageError::RateLimitError(_)));

        let err = mapper.map_http_error(503, "");
        assert!(matches!(err, VoyageError::ServiceUnavailableError(_)));
    }
}
