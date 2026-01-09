//! vLLM-specific error types and error mapping
//!
//! Handles error conversion from vLLM API responses to unified provider errors.

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;
use crate::core::types::errors::ProviderErrorTrait;
use thiserror::Error;

/// vLLM-specific error types
#[derive(Debug, Error)]
pub enum VLLMError {
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

    #[error("Batch processing error: {0}")]
    BatchError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl ProviderErrorTrait for VLLMError {
    fn error_type(&self) -> &'static str {
        match self {
            VLLMError::ApiError(_) => "api_error",
            VLLMError::AuthenticationError(_) => "authentication_error",
            VLLMError::RateLimitError(_) => "rate_limit_error",
            VLLMError::InvalidRequestError(_) => "invalid_request_error",
            VLLMError::ModelNotFoundError(_) => "model_not_found_error",
            VLLMError::ServiceUnavailableError(_) => "service_unavailable_error",
            VLLMError::StreamingError(_) => "streaming_error",
            VLLMError::ConfigurationError(_) => "configuration_error",
            VLLMError::NetworkError(_) => "network_error",
            VLLMError::BatchError(_) => "batch_error",
            VLLMError::UnknownError(_) => "unknown_error",
        }
    }

    fn is_retryable(&self) -> bool {
        matches!(
            self,
            VLLMError::RateLimitError(_)
                | VLLMError::ServiceUnavailableError(_)
                | VLLMError::NetworkError(_)
        )
    }

    fn retry_delay(&self) -> Option<u64> {
        match self {
            VLLMError::RateLimitError(_) => Some(30), // 30 seconds for rate limit
            VLLMError::ServiceUnavailableError(_) => Some(5), // 5 seconds for service unavailable
            VLLMError::NetworkError(_) => Some(2),    // 2 seconds for network errors
            _ => None,
        }
    }

    fn http_status(&self) -> u16 {
        match self {
            VLLMError::AuthenticationError(_) => 401,
            VLLMError::RateLimitError(_) => 429,
            VLLMError::InvalidRequestError(_) => 400,
            VLLMError::ModelNotFoundError(_) => 404,
            VLLMError::ServiceUnavailableError(_) => 503,
            VLLMError::ApiError(_) => 500,
            VLLMError::BatchError(_) => 400,
            _ => 500,
        }
    }

    fn not_supported(feature: &str) -> Self {
        VLLMError::InvalidRequestError(format!("Feature not supported: {}", feature))
    }

    fn authentication_failed(reason: &str) -> Self {
        VLLMError::AuthenticationError(reason.to_string())
    }

    fn rate_limited(retry_after: Option<u64>) -> Self {
        match retry_after {
            Some(seconds) => VLLMError::RateLimitError(format!(
                "Rate limit exceeded, retry after {} seconds",
                seconds
            )),
            None => VLLMError::RateLimitError("Rate limit exceeded".to_string()),
        }
    }

    fn network_error(details: &str) -> Self {
        VLLMError::NetworkError(details.to_string())
    }

    fn parsing_error(details: &str) -> Self {
        VLLMError::ApiError(format!("Response parsing error: {}", details))
    }

    fn not_implemented(feature: &str) -> Self {
        VLLMError::InvalidRequestError(format!("Feature not implemented: {}", feature))
    }
}

impl From<VLLMError> for ProviderError {
    fn from(error: VLLMError) -> Self {
        match error {
            VLLMError::ApiError(msg) => ProviderError::api_error("vllm", 500, msg),
            VLLMError::AuthenticationError(msg) => ProviderError::authentication("vllm", msg),
            VLLMError::RateLimitError(_) => ProviderError::rate_limit("vllm", None),
            VLLMError::InvalidRequestError(msg) => ProviderError::invalid_request("vllm", msg),
            VLLMError::ModelNotFoundError(msg) => ProviderError::model_not_found("vllm", msg),
            VLLMError::ServiceUnavailableError(msg) => ProviderError::api_error("vllm", 503, msg),
            VLLMError::StreamingError(msg) => {
                ProviderError::api_error("vllm", 500, format!("Streaming error: {}", msg))
            }
            VLLMError::ConfigurationError(msg) => ProviderError::configuration("vllm", msg),
            VLLMError::NetworkError(msg) => ProviderError::network("vllm", msg),
            VLLMError::BatchError(msg) => {
                ProviderError::api_error("vllm", 400, format!("Batch error: {}", msg))
            }
            VLLMError::UnknownError(msg) => ProviderError::api_error("vllm", 500, msg),
        }
    }
}

/// Error mapper for vLLM provider
pub struct VLLMErrorMapper;

impl ErrorMapper<VLLMError> for VLLMErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> VLLMError {
        let message = if response_body.is_empty() {
            format!("HTTP error {}", status_code)
        } else {
            response_body.to_string()
        };

        match status_code {
            400 => VLLMError::InvalidRequestError(message),
            401 => VLLMError::AuthenticationError("Invalid API key".to_string()),
            403 => VLLMError::AuthenticationError("Access forbidden".to_string()),
            404 => VLLMError::ModelNotFoundError("Model not found".to_string()),
            429 => VLLMError::RateLimitError("Rate limit exceeded".to_string()),
            500 => VLLMError::ApiError("Internal server error".to_string()),
            502 => VLLMError::ServiceUnavailableError("Bad gateway".to_string()),
            503 => VLLMError::ServiceUnavailableError("Service unavailable".to_string()),
            _ => VLLMError::ApiError(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vllm_error_display() {
        let err = VLLMError::ApiError("test error".to_string());
        assert_eq!(err.to_string(), "API error: test error");

        let err = VLLMError::AuthenticationError("invalid key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: invalid key");

        let err = VLLMError::RateLimitError("limit exceeded".to_string());
        assert_eq!(err.to_string(), "Rate limit exceeded: limit exceeded");

        let err = VLLMError::BatchError("batch failed".to_string());
        assert_eq!(err.to_string(), "Batch processing error: batch failed");
    }

    #[test]
    fn test_vllm_error_type() {
        assert_eq!(
            VLLMError::ApiError("".to_string()).error_type(),
            "api_error"
        );
        assert_eq!(
            VLLMError::AuthenticationError("".to_string()).error_type(),
            "authentication_error"
        );
        assert_eq!(
            VLLMError::RateLimitError("".to_string()).error_type(),
            "rate_limit_error"
        );
        assert_eq!(
            VLLMError::InvalidRequestError("".to_string()).error_type(),
            "invalid_request_error"
        );
        assert_eq!(
            VLLMError::ModelNotFoundError("".to_string()).error_type(),
            "model_not_found_error"
        );
        assert_eq!(
            VLLMError::ServiceUnavailableError("".to_string()).error_type(),
            "service_unavailable_error"
        );
        assert_eq!(
            VLLMError::StreamingError("".to_string()).error_type(),
            "streaming_error"
        );
        assert_eq!(
            VLLMError::ConfigurationError("".to_string()).error_type(),
            "configuration_error"
        );
        assert_eq!(
            VLLMError::NetworkError("".to_string()).error_type(),
            "network_error"
        );
        assert_eq!(
            VLLMError::BatchError("".to_string()).error_type(),
            "batch_error"
        );
        assert_eq!(
            VLLMError::UnknownError("".to_string()).error_type(),
            "unknown_error"
        );
    }

    #[test]
    fn test_vllm_error_is_retryable() {
        assert!(VLLMError::RateLimitError("".to_string()).is_retryable());
        assert!(VLLMError::ServiceUnavailableError("".to_string()).is_retryable());
        assert!(VLLMError::NetworkError("".to_string()).is_retryable());

        assert!(!VLLMError::ApiError("".to_string()).is_retryable());
        assert!(!VLLMError::AuthenticationError("".to_string()).is_retryable());
        assert!(!VLLMError::InvalidRequestError("".to_string()).is_retryable());
        assert!(!VLLMError::ModelNotFoundError("".to_string()).is_retryable());
        assert!(!VLLMError::BatchError("".to_string()).is_retryable());
    }

    #[test]
    fn test_vllm_error_retry_delay() {
        assert_eq!(
            VLLMError::RateLimitError("".to_string()).retry_delay(),
            Some(30)
        );
        assert_eq!(
            VLLMError::ServiceUnavailableError("".to_string()).retry_delay(),
            Some(5)
        );
        assert_eq!(
            VLLMError::NetworkError("".to_string()).retry_delay(),
            Some(2)
        );
        assert_eq!(VLLMError::ApiError("".to_string()).retry_delay(), None);
    }

    #[test]
    fn test_vllm_error_http_status() {
        assert_eq!(
            VLLMError::AuthenticationError("".to_string()).http_status(),
            401
        );
        assert_eq!(VLLMError::RateLimitError("".to_string()).http_status(), 429);
        assert_eq!(
            VLLMError::InvalidRequestError("".to_string()).http_status(),
            400
        );
        assert_eq!(
            VLLMError::ModelNotFoundError("".to_string()).http_status(),
            404
        );
        assert_eq!(
            VLLMError::ServiceUnavailableError("".to_string()).http_status(),
            503
        );
        assert_eq!(VLLMError::ApiError("".to_string()).http_status(), 500);
        assert_eq!(VLLMError::BatchError("".to_string()).http_status(), 400);
    }

    #[test]
    fn test_vllm_error_factory_methods() {
        let err = VLLMError::not_supported("vision");
        assert!(matches!(err, VLLMError::InvalidRequestError(_)));

        let err = VLLMError::authentication_failed("bad key");
        assert!(matches!(err, VLLMError::AuthenticationError(_)));

        let err = VLLMError::rate_limited(Some(30));
        assert!(matches!(err, VLLMError::RateLimitError(_)));

        let err = VLLMError::rate_limited(None);
        assert!(matches!(err, VLLMError::RateLimitError(_)));

        let err = VLLMError::network_error("connection failed");
        assert!(matches!(err, VLLMError::NetworkError(_)));

        let err = VLLMError::parsing_error("invalid json");
        assert!(matches!(err, VLLMError::ApiError(_)));

        let err = VLLMError::not_implemented("feature");
        assert!(matches!(err, VLLMError::InvalidRequestError(_)));
    }

    #[test]
    fn test_vllm_error_to_provider_error() {
        let err: ProviderError = VLLMError::AuthenticationError("bad key".to_string()).into();
        assert!(matches!(err, ProviderError::Authentication { .. }));

        let err: ProviderError = VLLMError::RateLimitError("limit".to_string()).into();
        assert!(matches!(err, ProviderError::RateLimit { .. }));

        let err: ProviderError = VLLMError::ModelNotFoundError("model".to_string()).into();
        assert!(matches!(err, ProviderError::ModelNotFound { .. }));

        let err: ProviderError = VLLMError::ConfigurationError("bad config".to_string()).into();
        assert!(matches!(err, ProviderError::Configuration { .. }));

        let err: ProviderError = VLLMError::NetworkError("timeout".to_string()).into();
        assert!(matches!(err, ProviderError::Network { .. }));
    }

    #[test]
    fn test_vllm_error_mapper_http_errors() {
        let mapper = VLLMErrorMapper;

        let err = mapper.map_http_error(400, "bad request");
        assert!(matches!(err, VLLMError::InvalidRequestError(_)));

        let err = mapper.map_http_error(401, "");
        assert!(matches!(err, VLLMError::AuthenticationError(_)));

        let err = mapper.map_http_error(403, "");
        assert!(matches!(err, VLLMError::AuthenticationError(_)));

        let err = mapper.map_http_error(404, "");
        assert!(matches!(err, VLLMError::ModelNotFoundError(_)));

        let err = mapper.map_http_error(429, "");
        assert!(matches!(err, VLLMError::RateLimitError(_)));

        let err = mapper.map_http_error(500, "");
        assert!(matches!(err, VLLMError::ApiError(_)));

        let err = mapper.map_http_error(502, "");
        assert!(matches!(err, VLLMError::ServiceUnavailableError(_)));

        let err = mapper.map_http_error(503, "");
        assert!(matches!(err, VLLMError::ServiceUnavailableError(_)));

        let err = mapper.map_http_error(418, "teapot");
        assert!(matches!(err, VLLMError::ApiError(_)));
    }

    #[test]
    fn test_vllm_error_mapper_empty_body() {
        let mapper = VLLMErrorMapper;
        let err = mapper.map_http_error(400, "");
        if let VLLMError::InvalidRequestError(msg) = err {
            assert!(msg.contains("HTTP error 400"));
        } else {
            panic!("Expected InvalidRequestError");
        }
    }
}
