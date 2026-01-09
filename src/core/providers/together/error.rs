//! Together AI specific error types and error mapping
//!
//! Handles error conversion from Together AI API responses to unified provider errors.

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;
use crate::core::types::errors::ProviderErrorTrait;
use thiserror::Error;

/// Together AI specific error types
#[derive(Debug, Error)]
pub enum TogetherError {
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

    #[error("Rerank error: {0}")]
    RerankError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl ProviderErrorTrait for TogetherError {
    fn error_type(&self) -> &'static str {
        match self {
            TogetherError::ApiError(_) => "api_error",
            TogetherError::AuthenticationError(_) => "authentication_error",
            TogetherError::RateLimitError(_) => "rate_limit_error",
            TogetherError::InvalidRequestError(_) => "invalid_request_error",
            TogetherError::ModelNotFoundError(_) => "model_not_found_error",
            TogetherError::ServiceUnavailableError(_) => "service_unavailable_error",
            TogetherError::StreamingError(_) => "streaming_error",
            TogetherError::ConfigurationError(_) => "configuration_error",
            TogetherError::NetworkError(_) => "network_error",
            TogetherError::RerankError(_) => "rerank_error",
            TogetherError::UnknownError(_) => "unknown_error",
        }
    }

    fn is_retryable(&self) -> bool {
        matches!(
            self,
            TogetherError::RateLimitError(_)
                | TogetherError::ServiceUnavailableError(_)
                | TogetherError::NetworkError(_)
        )
    }

    fn retry_delay(&self) -> Option<u64> {
        match self {
            TogetherError::RateLimitError(_) => Some(60), // Default 60 seconds for rate limit
            TogetherError::ServiceUnavailableError(_) => Some(5), // 5 seconds for service unavailable
            TogetherError::NetworkError(_) => Some(2),    // 2 seconds for network errors
            _ => None,
        }
    }

    fn http_status(&self) -> u16 {
        match self {
            TogetherError::AuthenticationError(_) => 401,
            TogetherError::RateLimitError(_) => 429,
            TogetherError::InvalidRequestError(_) => 400,
            TogetherError::ModelNotFoundError(_) => 404,
            TogetherError::ServiceUnavailableError(_) => 503,
            TogetherError::ApiError(_) => 500,
            _ => 500,
        }
    }

    fn not_supported(feature: &str) -> Self {
        TogetherError::InvalidRequestError(format!("Feature not supported: {}", feature))
    }

    fn authentication_failed(reason: &str) -> Self {
        TogetherError::AuthenticationError(reason.to_string())
    }

    fn rate_limited(retry_after: Option<u64>) -> Self {
        match retry_after {
            Some(seconds) => TogetherError::RateLimitError(format!(
                "Rate limit exceeded, retry after {} seconds",
                seconds
            )),
            None => TogetherError::RateLimitError("Rate limit exceeded".to_string()),
        }
    }

    fn network_error(details: &str) -> Self {
        TogetherError::NetworkError(details.to_string())
    }

    fn parsing_error(details: &str) -> Self {
        TogetherError::ApiError(format!("Response parsing error: {}", details))
    }

    fn not_implemented(feature: &str) -> Self {
        TogetherError::InvalidRequestError(format!("Feature not implemented: {}", feature))
    }
}

impl From<TogetherError> for ProviderError {
    fn from(error: TogetherError) -> Self {
        match error {
            TogetherError::ApiError(msg) => ProviderError::api_error("together", 500, msg),
            TogetherError::AuthenticationError(msg) => {
                ProviderError::authentication("together", msg)
            }
            TogetherError::RateLimitError(_) => ProviderError::rate_limit("together", None),
            TogetherError::InvalidRequestError(msg) => {
                ProviderError::invalid_request("together", msg)
            }
            TogetherError::ModelNotFoundError(msg) => {
                ProviderError::model_not_found("together", msg)
            }
            TogetherError::ServiceUnavailableError(msg) => {
                ProviderError::api_error("together", 503, msg)
            }
            TogetherError::StreamingError(msg) => {
                ProviderError::api_error("together", 500, format!("Streaming error: {}", msg))
            }
            TogetherError::ConfigurationError(msg) => ProviderError::configuration("together", msg),
            TogetherError::NetworkError(msg) => ProviderError::network("together", msg),
            TogetherError::RerankError(msg) => {
                ProviderError::api_error("together", 500, format!("Rerank error: {}", msg))
            }
            TogetherError::UnknownError(msg) => ProviderError::api_error("together", 500, msg),
        }
    }
}

/// Error mapper for Together AI provider
pub struct TogetherErrorMapper;

impl ErrorMapper<TogetherError> for TogetherErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> TogetherError {
        let message = if response_body.is_empty() {
            format!("HTTP error {}", status_code)
        } else {
            response_body.to_string()
        };

        match status_code {
            400 => TogetherError::InvalidRequestError(message),
            401 => TogetherError::AuthenticationError("Invalid API key".to_string()),
            403 => TogetherError::AuthenticationError("Access forbidden".to_string()),
            404 => TogetherError::ModelNotFoundError("Model not found".to_string()),
            429 => TogetherError::RateLimitError("Rate limit exceeded".to_string()),
            500 => TogetherError::ApiError("Internal server error".to_string()),
            502 => TogetherError::ServiceUnavailableError("Bad gateway".to_string()),
            503 => TogetherError::ServiceUnavailableError("Service unavailable".to_string()),
            _ => TogetherError::ApiError(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_together_error_display() {
        let err = TogetherError::ApiError("test error".to_string());
        assert_eq!(err.to_string(), "API error: test error");

        let err = TogetherError::AuthenticationError("invalid key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: invalid key");

        let err = TogetherError::RateLimitError("limit exceeded".to_string());
        assert_eq!(err.to_string(), "Rate limit exceeded: limit exceeded");

        let err = TogetherError::RerankError("rerank failed".to_string());
        assert_eq!(err.to_string(), "Rerank error: rerank failed");
    }

    #[test]
    fn test_together_error_type() {
        assert_eq!(
            TogetherError::ApiError("".to_string()).error_type(),
            "api_error"
        );
        assert_eq!(
            TogetherError::AuthenticationError("".to_string()).error_type(),
            "authentication_error"
        );
        assert_eq!(
            TogetherError::RateLimitError("".to_string()).error_type(),
            "rate_limit_error"
        );
        assert_eq!(
            TogetherError::InvalidRequestError("".to_string()).error_type(),
            "invalid_request_error"
        );
        assert_eq!(
            TogetherError::ModelNotFoundError("".to_string()).error_type(),
            "model_not_found_error"
        );
        assert_eq!(
            TogetherError::ServiceUnavailableError("".to_string()).error_type(),
            "service_unavailable_error"
        );
        assert_eq!(
            TogetherError::StreamingError("".to_string()).error_type(),
            "streaming_error"
        );
        assert_eq!(
            TogetherError::ConfigurationError("".to_string()).error_type(),
            "configuration_error"
        );
        assert_eq!(
            TogetherError::NetworkError("".to_string()).error_type(),
            "network_error"
        );
        assert_eq!(
            TogetherError::RerankError("".to_string()).error_type(),
            "rerank_error"
        );
        assert_eq!(
            TogetherError::UnknownError("".to_string()).error_type(),
            "unknown_error"
        );
    }

    #[test]
    fn test_together_error_is_retryable() {
        assert!(TogetherError::RateLimitError("".to_string()).is_retryable());
        assert!(TogetherError::ServiceUnavailableError("".to_string()).is_retryable());
        assert!(TogetherError::NetworkError("".to_string()).is_retryable());

        assert!(!TogetherError::ApiError("".to_string()).is_retryable());
        assert!(!TogetherError::AuthenticationError("".to_string()).is_retryable());
        assert!(!TogetherError::InvalidRequestError("".to_string()).is_retryable());
        assert!(!TogetherError::ModelNotFoundError("".to_string()).is_retryable());
        assert!(!TogetherError::RerankError("".to_string()).is_retryable());
    }

    #[test]
    fn test_together_error_retry_delay() {
        assert_eq!(
            TogetherError::RateLimitError("".to_string()).retry_delay(),
            Some(60)
        );
        assert_eq!(
            TogetherError::ServiceUnavailableError("".to_string()).retry_delay(),
            Some(5)
        );
        assert_eq!(
            TogetherError::NetworkError("".to_string()).retry_delay(),
            Some(2)
        );
        assert_eq!(TogetherError::ApiError("".to_string()).retry_delay(), None);
        assert_eq!(
            TogetherError::RerankError("".to_string()).retry_delay(),
            None
        );
    }

    #[test]
    fn test_together_error_http_status() {
        assert_eq!(
            TogetherError::AuthenticationError("".to_string()).http_status(),
            401
        );
        assert_eq!(
            TogetherError::RateLimitError("".to_string()).http_status(),
            429
        );
        assert_eq!(
            TogetherError::InvalidRequestError("".to_string()).http_status(),
            400
        );
        assert_eq!(
            TogetherError::ModelNotFoundError("".to_string()).http_status(),
            404
        );
        assert_eq!(
            TogetherError::ServiceUnavailableError("".to_string()).http_status(),
            503
        );
        assert_eq!(TogetherError::ApiError("".to_string()).http_status(), 500);
    }

    #[test]
    fn test_together_error_factory_methods() {
        let err = TogetherError::not_supported("vision");
        assert!(matches!(err, TogetherError::InvalidRequestError(_)));

        let err = TogetherError::authentication_failed("bad key");
        assert!(matches!(err, TogetherError::AuthenticationError(_)));

        let err = TogetherError::rate_limited(Some(30));
        assert!(matches!(err, TogetherError::RateLimitError(_)));

        let err = TogetherError::rate_limited(None);
        assert!(matches!(err, TogetherError::RateLimitError(_)));

        let err = TogetherError::network_error("connection failed");
        assert!(matches!(err, TogetherError::NetworkError(_)));

        let err = TogetherError::parsing_error("invalid json");
        assert!(matches!(err, TogetherError::ApiError(_)));

        let err = TogetherError::not_implemented("feature");
        assert!(matches!(err, TogetherError::InvalidRequestError(_)));
    }

    #[test]
    fn test_together_error_to_provider_error() {
        let err: ProviderError =
            TogetherError::AuthenticationError("bad key".to_string()).into();
        assert!(matches!(err, ProviderError::Authentication { .. }));

        let err: ProviderError = TogetherError::RateLimitError("limit".to_string()).into();
        assert!(matches!(err, ProviderError::RateLimit { .. }));

        let err: ProviderError = TogetherError::ModelNotFoundError("gpt-5".to_string()).into();
        assert!(matches!(err, ProviderError::ModelNotFound { .. }));

        let err: ProviderError =
            TogetherError::ConfigurationError("bad config".to_string()).into();
        assert!(matches!(err, ProviderError::Configuration { .. }));

        let err: ProviderError = TogetherError::NetworkError("timeout".to_string()).into();
        assert!(matches!(err, ProviderError::Network { .. }));
    }

    #[test]
    fn test_together_error_mapper_http_errors() {
        let mapper = TogetherErrorMapper;

        let err = mapper.map_http_error(400, "bad request");
        assert!(matches!(err, TogetherError::InvalidRequestError(_)));

        let err = mapper.map_http_error(401, "");
        assert!(matches!(err, TogetherError::AuthenticationError(_)));

        let err = mapper.map_http_error(403, "");
        assert!(matches!(err, TogetherError::AuthenticationError(_)));

        let err = mapper.map_http_error(404, "");
        assert!(matches!(err, TogetherError::ModelNotFoundError(_)));

        let err = mapper.map_http_error(429, "");
        assert!(matches!(err, TogetherError::RateLimitError(_)));

        let err = mapper.map_http_error(500, "");
        assert!(matches!(err, TogetherError::ApiError(_)));

        let err = mapper.map_http_error(502, "");
        assert!(matches!(err, TogetherError::ServiceUnavailableError(_)));

        let err = mapper.map_http_error(503, "");
        assert!(matches!(err, TogetherError::ServiceUnavailableError(_)));

        let err = mapper.map_http_error(418, "teapot");
        assert!(matches!(err, TogetherError::ApiError(_)));
    }

    #[test]
    fn test_together_error_mapper_empty_body() {
        let mapper = TogetherErrorMapper;
        let err = mapper.map_http_error(400, "");
        if let TogetherError::InvalidRequestError(msg) = err {
            assert!(msg.contains("HTTP error 400"));
        } else {
            panic!("Expected InvalidRequestError");
        }
    }
}
