//! Fireworks AI-specific error types and error mapping
//!
//! Handles error conversion from Fireworks AI API responses to unified provider errors.

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;
use crate::core::types::errors::ProviderErrorTrait;
use thiserror::Error;

/// Fireworks AI-specific error types
#[derive(Debug, Error)]
pub enum FireworksError {
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

    #[error("Streaming error: {0}")]
    StreamingError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Context length exceeded: {0}")]
    ContextLengthExceeded(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl ProviderErrorTrait for FireworksError {
    fn error_type(&self) -> &'static str {
        match self {
            FireworksError::ApiError(_) => "api_error",
            FireworksError::AuthenticationError(_) => "authentication_error",
            FireworksError::RateLimitError(_) => "rate_limit_error",
            FireworksError::InvalidRequestError(_) => "invalid_request_error",
            FireworksError::ModelNotFoundError(_) => "model_not_found_error",
            FireworksError::ServiceUnavailableError(_) => "service_unavailable_error",
            FireworksError::StreamingError(_) => "streaming_error",
            FireworksError::ConfigurationError(_) => "configuration_error",
            FireworksError::NetworkError(_) => "network_error",
            FireworksError::ContextLengthExceeded(_) => "context_length_exceeded",
            FireworksError::UnknownError(_) => "unknown_error",
        }
    }

    fn is_retryable(&self) -> bool {
        matches!(
            self,
            FireworksError::RateLimitError(_)
                | FireworksError::ServiceUnavailableError(_)
                | FireworksError::NetworkError(_)
        )
    }

    fn retry_delay(&self) -> Option<u64> {
        match self {
            FireworksError::RateLimitError(_) => Some(60),
            FireworksError::ServiceUnavailableError(_) => Some(5),
            FireworksError::NetworkError(_) => Some(2),
            _ => None,
        }
    }

    fn http_status(&self) -> u16 {
        match self {
            FireworksError::AuthenticationError(_) => 401,
            FireworksError::RateLimitError(_) => 429,
            FireworksError::InvalidRequestError(_) => 400,
            FireworksError::ModelNotFoundError(_) => 404,
            FireworksError::ServiceUnavailableError(_) => 503,
            FireworksError::ContextLengthExceeded(_) => 400,
            FireworksError::ApiError(_) => 500,
            _ => 500,
        }
    }

    fn not_supported(feature: &str) -> Self {
        FireworksError::InvalidRequestError(format!("Feature not supported: {}", feature))
    }

    fn authentication_failed(reason: &str) -> Self {
        FireworksError::AuthenticationError(reason.to_string())
    }

    fn rate_limited(retry_after: Option<u64>) -> Self {
        match retry_after {
            Some(seconds) => FireworksError::RateLimitError(format!(
                "Rate limit exceeded, retry after {} seconds",
                seconds
            )),
            None => FireworksError::RateLimitError("Rate limit exceeded".to_string()),
        }
    }

    fn network_error(details: &str) -> Self {
        FireworksError::NetworkError(details.to_string())
    }

    fn parsing_error(details: &str) -> Self {
        FireworksError::ApiError(format!("Response parsing error: {}", details))
    }

    fn not_implemented(feature: &str) -> Self {
        FireworksError::InvalidRequestError(format!("Feature not implemented: {}", feature))
    }
}

impl From<FireworksError> for ProviderError {
    fn from(error: FireworksError) -> Self {
        match error {
            FireworksError::ApiError(msg) => ProviderError::api_error("fireworks", 500, msg),
            FireworksError::AuthenticationError(msg) => {
                ProviderError::authentication("fireworks", msg)
            }
            FireworksError::RateLimitError(_) => ProviderError::rate_limit("fireworks", None),
            FireworksError::InvalidRequestError(msg) => {
                ProviderError::invalid_request("fireworks", msg)
            }
            FireworksError::ModelNotFoundError(msg) => {
                ProviderError::model_not_found("fireworks", msg)
            }
            FireworksError::ServiceUnavailableError(msg) => {
                ProviderError::api_error("fireworks", 503, msg)
            }
            FireworksError::StreamingError(msg) => {
                ProviderError::api_error("fireworks", 500, format!("Streaming error: {}", msg))
            }
            FireworksError::ConfigurationError(msg) => {
                ProviderError::configuration("fireworks", msg)
            }
            FireworksError::NetworkError(msg) => ProviderError::network("fireworks", msg),
            FireworksError::ContextLengthExceeded(msg) => ProviderError::invalid_request(
                "fireworks",
                format!("Context length exceeded: {}", msg),
            ),
            FireworksError::UnknownError(msg) => ProviderError::api_error("fireworks", 500, msg),
        }
    }
}

/// Error mapper for Fireworks AI provider
pub struct FireworksErrorMapper;

impl ErrorMapper<FireworksError> for FireworksErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> FireworksError {
        let message = if response_body.is_empty() {
            format!("HTTP error {}", status_code)
        } else {
            response_body.to_string()
        };

        match status_code {
            400 => {
                // Check for context length exceeded
                if message.contains("context_length_exceeded")
                    || message.contains("maximum context length")
                {
                    FireworksError::ContextLengthExceeded(message)
                } else {
                    FireworksError::InvalidRequestError(message)
                }
            }
            401 => FireworksError::AuthenticationError("Invalid API key".to_string()),
            403 => FireworksError::AuthenticationError("Access forbidden".to_string()),
            404 => FireworksError::ModelNotFoundError("Model not found".to_string()),
            429 => FireworksError::RateLimitError("Rate limit exceeded".to_string()),
            500 => FireworksError::ApiError("Internal server error".to_string()),
            502 => FireworksError::ServiceUnavailableError("Bad gateway".to_string()),
            503 => FireworksError::ServiceUnavailableError("Service unavailable".to_string()),
            _ => FireworksError::ApiError(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fireworks_error_display() {
        let err = FireworksError::ApiError("test error".to_string());
        assert_eq!(err.to_string(), "API error: test error");

        let err = FireworksError::AuthenticationError("invalid key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: invalid key");

        let err = FireworksError::RateLimitError("limit exceeded".to_string());
        assert_eq!(err.to_string(), "Rate limit exceeded: limit exceeded");

        let err = FireworksError::ContextLengthExceeded("too long".to_string());
        assert_eq!(err.to_string(), "Context length exceeded: too long");
    }

    #[test]
    fn test_fireworks_error_type() {
        assert_eq!(
            FireworksError::ApiError("".to_string()).error_type(),
            "api_error"
        );
        assert_eq!(
            FireworksError::AuthenticationError("".to_string()).error_type(),
            "authentication_error"
        );
        assert_eq!(
            FireworksError::RateLimitError("".to_string()).error_type(),
            "rate_limit_error"
        );
        assert_eq!(
            FireworksError::InvalidRequestError("".to_string()).error_type(),
            "invalid_request_error"
        );
        assert_eq!(
            FireworksError::ModelNotFoundError("".to_string()).error_type(),
            "model_not_found_error"
        );
        assert_eq!(
            FireworksError::ContextLengthExceeded("".to_string()).error_type(),
            "context_length_exceeded"
        );
    }

    #[test]
    fn test_fireworks_error_is_retryable() {
        assert!(FireworksError::RateLimitError("".to_string()).is_retryable());
        assert!(FireworksError::ServiceUnavailableError("".to_string()).is_retryable());
        assert!(FireworksError::NetworkError("".to_string()).is_retryable());

        assert!(!FireworksError::ApiError("".to_string()).is_retryable());
        assert!(!FireworksError::AuthenticationError("".to_string()).is_retryable());
        assert!(!FireworksError::InvalidRequestError("".to_string()).is_retryable());
    }

    #[test]
    fn test_fireworks_error_retry_delay() {
        assert_eq!(
            FireworksError::RateLimitError("".to_string()).retry_delay(),
            Some(60)
        );
        assert_eq!(
            FireworksError::ServiceUnavailableError("".to_string()).retry_delay(),
            Some(5)
        );
        assert_eq!(
            FireworksError::NetworkError("".to_string()).retry_delay(),
            Some(2)
        );
        assert_eq!(FireworksError::ApiError("".to_string()).retry_delay(), None);
    }

    #[test]
    fn test_fireworks_error_http_status() {
        assert_eq!(
            FireworksError::AuthenticationError("".to_string()).http_status(),
            401
        );
        assert_eq!(
            FireworksError::RateLimitError("".to_string()).http_status(),
            429
        );
        assert_eq!(
            FireworksError::InvalidRequestError("".to_string()).http_status(),
            400
        );
        assert_eq!(
            FireworksError::ModelNotFoundError("".to_string()).http_status(),
            404
        );
        assert_eq!(
            FireworksError::ContextLengthExceeded("".to_string()).http_status(),
            400
        );
    }

    #[test]
    fn test_fireworks_error_factory_methods() {
        let err = FireworksError::not_supported("vision");
        assert!(matches!(err, FireworksError::InvalidRequestError(_)));

        let err = FireworksError::authentication_failed("bad key");
        assert!(matches!(err, FireworksError::AuthenticationError(_)));

        let err = FireworksError::rate_limited(Some(30));
        assert!(matches!(err, FireworksError::RateLimitError(_)));

        let err = FireworksError::network_error("connection failed");
        assert!(matches!(err, FireworksError::NetworkError(_)));

        let err = FireworksError::parsing_error("invalid json");
        assert!(matches!(err, FireworksError::ApiError(_)));
    }

    #[test]
    fn test_fireworks_error_to_provider_error() {
        let err: ProviderError = FireworksError::AuthenticationError("bad key".to_string()).into();
        assert!(matches!(err, ProviderError::Authentication { .. }));

        let err: ProviderError = FireworksError::RateLimitError("limit".to_string()).into();
        assert!(matches!(err, ProviderError::RateLimit { .. }));

        let err: ProviderError = FireworksError::ModelNotFoundError("model".to_string()).into();
        assert!(matches!(err, ProviderError::ModelNotFound { .. }));

        let err: ProviderError =
            FireworksError::ConfigurationError("bad config".to_string()).into();
        assert!(matches!(err, ProviderError::Configuration { .. }));

        let err: ProviderError = FireworksError::NetworkError("timeout".to_string()).into();
        assert!(matches!(err, ProviderError::Network { .. }));
    }

    #[test]
    fn test_fireworks_error_mapper_http_errors() {
        let mapper = FireworksErrorMapper;

        let err = mapper.map_http_error(400, "bad request");
        assert!(matches!(err, FireworksError::InvalidRequestError(_)));

        let err = mapper.map_http_error(400, "context_length_exceeded");
        assert!(matches!(err, FireworksError::ContextLengthExceeded(_)));

        let err = mapper.map_http_error(401, "");
        assert!(matches!(err, FireworksError::AuthenticationError(_)));

        let err = mapper.map_http_error(429, "");
        assert!(matches!(err, FireworksError::RateLimitError(_)));

        let err = mapper.map_http_error(503, "");
        assert!(matches!(err, FireworksError::ServiceUnavailableError(_)));
    }
}
