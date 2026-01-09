//! Ollama-specific error types and error mapping
//!
//! Handles error conversion from Ollama API responses to unified provider errors.

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;
use crate::core::types::errors::ProviderErrorTrait;
use thiserror::Error;

/// Ollama-specific error types
#[derive(Debug, Error)]
pub enum OllamaError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

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

    #[error("Connection refused: {0}")]
    ConnectionRefusedError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Context length exceeded: max {max}, got {actual}")]
    ContextLengthExceeded { max: usize, actual: usize },

    #[error("Model loading error: {0}")]
    ModelLoadingError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl ProviderErrorTrait for OllamaError {
    fn error_type(&self) -> &'static str {
        match self {
            OllamaError::ApiError(_) => "api_error",
            OllamaError::AuthenticationError(_) => "authentication_error",
            OllamaError::InvalidRequestError(_) => "invalid_request_error",
            OllamaError::ModelNotFoundError(_) => "model_not_found_error",
            OllamaError::ServiceUnavailableError(_) => "service_unavailable_error",
            OllamaError::StreamingError(_) => "streaming_error",
            OllamaError::ConfigurationError(_) => "configuration_error",
            OllamaError::NetworkError(_) => "network_error",
            OllamaError::ConnectionRefusedError(_) => "connection_refused_error",
            OllamaError::TimeoutError(_) => "timeout_error",
            OllamaError::ContextLengthExceeded { .. } => "context_length_exceeded",
            OllamaError::ModelLoadingError(_) => "model_loading_error",
            OllamaError::UnknownError(_) => "unknown_error",
        }
    }

    fn is_retryable(&self) -> bool {
        matches!(
            self,
            OllamaError::ServiceUnavailableError(_)
                | OllamaError::NetworkError(_)
                | OllamaError::ConnectionRefusedError(_)
                | OllamaError::TimeoutError(_)
                | OllamaError::ModelLoadingError(_)
        )
    }

    fn retry_delay(&self) -> Option<u64> {
        match self {
            OllamaError::ServiceUnavailableError(_) => Some(5),
            OllamaError::NetworkError(_) => Some(2),
            OllamaError::ConnectionRefusedError(_) => Some(5),
            OllamaError::TimeoutError(_) => Some(10),
            OllamaError::ModelLoadingError(_) => Some(30), // Model loading can take time
            _ => None,
        }
    }

    fn http_status(&self) -> u16 {
        match self {
            OllamaError::AuthenticationError(_) => 401,
            OllamaError::InvalidRequestError(_) => 400,
            OllamaError::ModelNotFoundError(_) => 404,
            OllamaError::ServiceUnavailableError(_) => 503,
            OllamaError::ContextLengthExceeded { .. } => 400,
            OllamaError::ApiError(_) => 500,
            OllamaError::ConnectionRefusedError(_) => 503,
            OllamaError::TimeoutError(_) => 504,
            _ => 500,
        }
    }

    fn not_supported(feature: &str) -> Self {
        OllamaError::InvalidRequestError(format!("Feature not supported: {}", feature))
    }

    fn authentication_failed(reason: &str) -> Self {
        OllamaError::AuthenticationError(reason.to_string())
    }

    fn rate_limited(retry_after: Option<u64>) -> Self {
        match retry_after {
            Some(seconds) => OllamaError::ServiceUnavailableError(format!(
                "Rate limited, retry after {} seconds",
                seconds
            )),
            None => OllamaError::ServiceUnavailableError("Rate limited".to_string()),
        }
    }

    fn network_error(details: &str) -> Self {
        OllamaError::NetworkError(details.to_string())
    }

    fn parsing_error(details: &str) -> Self {
        OllamaError::ApiError(format!("Response parsing error: {}", details))
    }

    fn not_implemented(feature: &str) -> Self {
        OllamaError::InvalidRequestError(format!("Feature not implemented: {}", feature))
    }
}

impl From<OllamaError> for ProviderError {
    fn from(error: OllamaError) -> Self {
        match error {
            OllamaError::ApiError(msg) => ProviderError::api_error("ollama", 500, msg),
            OllamaError::AuthenticationError(msg) => ProviderError::authentication("ollama", msg),
            OllamaError::InvalidRequestError(msg) => ProviderError::invalid_request("ollama", msg),
            OllamaError::ModelNotFoundError(msg) => ProviderError::model_not_found("ollama", msg),
            OllamaError::ServiceUnavailableError(msg) => {
                ProviderError::api_error("ollama", 503, msg)
            }
            OllamaError::StreamingError(msg) => {
                ProviderError::streaming_error("ollama", "chat", None, None, msg)
            }
            OllamaError::ConfigurationError(msg) => ProviderError::configuration("ollama", msg),
            OllamaError::NetworkError(msg) => ProviderError::network("ollama", msg),
            OllamaError::ConnectionRefusedError(msg) => ProviderError::network(
                "ollama",
                format!("Connection refused: {}. Is Ollama running?", msg),
            ),
            OllamaError::TimeoutError(msg) => ProviderError::Timeout {
                provider: "ollama",
                message: msg,
            },
            OllamaError::ContextLengthExceeded { max, actual } => {
                ProviderError::ContextLengthExceeded {
                    provider: "ollama",
                    max,
                    actual,
                }
            }
            OllamaError::ModelLoadingError(msg) => ProviderError::api_error(
                "ollama",
                503,
                format!("Model loading failed: {}", msg),
            ),
            OllamaError::UnknownError(msg) => ProviderError::api_error("ollama", 500, msg),
        }
    }
}

/// Error mapper for Ollama provider
pub struct OllamaErrorMapper;

impl ErrorMapper<OllamaError> for OllamaErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> OllamaError {
        let message = if response_body.is_empty() {
            format!("HTTP error {}", status_code)
        } else {
            // Try to parse as JSON error
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(response_body) {
                json.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or(response_body)
                    .to_string()
            } else {
                response_body.to_string()
            }
        };

        // Check for specific Ollama error patterns
        let message_lower = message.to_lowercase();

        if message_lower.contains("model") && message_lower.contains("not found") {
            return OllamaError::ModelNotFoundError(message);
        }

        if message_lower.contains("context length") || message_lower.contains("too long") {
            return OllamaError::ContextLengthExceeded {
                max: 0,    // Unknown
                actual: 0, // Unknown
            };
        }

        if message_lower.contains("loading") || message_lower.contains("pulling") {
            return OllamaError::ModelLoadingError(message);
        }

        match status_code {
            400 => OllamaError::InvalidRequestError(message),
            401 => OllamaError::AuthenticationError("Invalid API key".to_string()),
            403 => OllamaError::AuthenticationError("Access forbidden".to_string()),
            404 => OllamaError::ModelNotFoundError(message),
            408 | 504 => OllamaError::TimeoutError(message),
            500 => OllamaError::ApiError(message),
            502 | 503 => OllamaError::ServiceUnavailableError(message),
            _ => OllamaError::ApiError(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_error_display() {
        let err = OllamaError::ApiError("test error".to_string());
        assert_eq!(err.to_string(), "API error: test error");

        let err = OllamaError::AuthenticationError("invalid key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: invalid key");

        let err = OllamaError::ConnectionRefusedError("localhost:11434".to_string());
        assert_eq!(
            err.to_string(),
            "Connection refused: localhost:11434"
        );
    }

    #[test]
    fn test_ollama_error_type() {
        assert_eq!(
            OllamaError::ApiError("".to_string()).error_type(),
            "api_error"
        );
        assert_eq!(
            OllamaError::AuthenticationError("".to_string()).error_type(),
            "authentication_error"
        );
        assert_eq!(
            OllamaError::ModelNotFoundError("".to_string()).error_type(),
            "model_not_found_error"
        );
        assert_eq!(
            OllamaError::ConnectionRefusedError("".to_string()).error_type(),
            "connection_refused_error"
        );
        assert_eq!(
            OllamaError::TimeoutError("".to_string()).error_type(),
            "timeout_error"
        );
        assert_eq!(
            OllamaError::ContextLengthExceeded { max: 0, actual: 0 }.error_type(),
            "context_length_exceeded"
        );
        assert_eq!(
            OllamaError::ModelLoadingError("".to_string()).error_type(),
            "model_loading_error"
        );
    }

    #[test]
    fn test_ollama_error_is_retryable() {
        assert!(OllamaError::ServiceUnavailableError("".to_string()).is_retryable());
        assert!(OllamaError::NetworkError("".to_string()).is_retryable());
        assert!(OllamaError::ConnectionRefusedError("".to_string()).is_retryable());
        assert!(OllamaError::TimeoutError("".to_string()).is_retryable());
        assert!(OllamaError::ModelLoadingError("".to_string()).is_retryable());

        assert!(!OllamaError::ApiError("".to_string()).is_retryable());
        assert!(!OllamaError::AuthenticationError("".to_string()).is_retryable());
        assert!(!OllamaError::InvalidRequestError("".to_string()).is_retryable());
        assert!(!OllamaError::ModelNotFoundError("".to_string()).is_retryable());
    }

    #[test]
    fn test_ollama_error_retry_delay() {
        assert_eq!(
            OllamaError::ServiceUnavailableError("".to_string()).retry_delay(),
            Some(5)
        );
        assert_eq!(
            OllamaError::NetworkError("".to_string()).retry_delay(),
            Some(2)
        );
        assert_eq!(
            OllamaError::ConnectionRefusedError("".to_string()).retry_delay(),
            Some(5)
        );
        assert_eq!(
            OllamaError::TimeoutError("".to_string()).retry_delay(),
            Some(10)
        );
        assert_eq!(
            OllamaError::ModelLoadingError("".to_string()).retry_delay(),
            Some(30)
        );
        assert_eq!(OllamaError::ApiError("".to_string()).retry_delay(), None);
    }

    #[test]
    fn test_ollama_error_http_status() {
        assert_eq!(
            OllamaError::AuthenticationError("".to_string()).http_status(),
            401
        );
        assert_eq!(
            OllamaError::InvalidRequestError("".to_string()).http_status(),
            400
        );
        assert_eq!(
            OllamaError::ModelNotFoundError("".to_string()).http_status(),
            404
        );
        assert_eq!(
            OllamaError::ServiceUnavailableError("".to_string()).http_status(),
            503
        );
        assert_eq!(
            OllamaError::ConnectionRefusedError("".to_string()).http_status(),
            503
        );
        assert_eq!(
            OllamaError::TimeoutError("".to_string()).http_status(),
            504
        );
        assert_eq!(OllamaError::ApiError("".to_string()).http_status(), 500);
    }

    #[test]
    fn test_ollama_error_factory_methods() {
        let err = OllamaError::not_supported("vision");
        assert!(matches!(err, OllamaError::InvalidRequestError(_)));

        let err = OllamaError::authentication_failed("bad key");
        assert!(matches!(err, OllamaError::AuthenticationError(_)));

        let err = OllamaError::rate_limited(Some(30));
        assert!(matches!(err, OllamaError::ServiceUnavailableError(_)));

        let err = OllamaError::network_error("connection failed");
        assert!(matches!(err, OllamaError::NetworkError(_)));

        let err = OllamaError::parsing_error("invalid json");
        assert!(matches!(err, OllamaError::ApiError(_)));

        let err = OllamaError::not_implemented("feature");
        assert!(matches!(err, OllamaError::InvalidRequestError(_)));
    }

    #[test]
    fn test_ollama_error_to_provider_error() {
        let err: ProviderError = OllamaError::AuthenticationError("bad key".to_string()).into();
        assert!(matches!(err, ProviderError::Authentication { .. }));

        let err: ProviderError = OllamaError::ModelNotFoundError("llama2".to_string()).into();
        assert!(matches!(err, ProviderError::ModelNotFound { .. }));

        let err: ProviderError = OllamaError::ConfigurationError("bad config".to_string()).into();
        assert!(matches!(err, ProviderError::Configuration { .. }));

        let err: ProviderError = OllamaError::NetworkError("timeout".to_string()).into();
        assert!(matches!(err, ProviderError::Network { .. }));

        let err: ProviderError = OllamaError::TimeoutError("30s".to_string()).into();
        assert!(matches!(err, ProviderError::Timeout { .. }));

        let err: ProviderError =
            OllamaError::ContextLengthExceeded { max: 4096, actual: 5000 }.into();
        assert!(matches!(err, ProviderError::ContextLengthExceeded { .. }));
    }

    #[test]
    fn test_ollama_error_mapper_http_errors() {
        let mapper = OllamaErrorMapper;

        let err = mapper.map_http_error(400, "bad request");
        assert!(matches!(err, OllamaError::InvalidRequestError(_)));

        let err = mapper.map_http_error(401, "");
        assert!(matches!(err, OllamaError::AuthenticationError(_)));

        let err = mapper.map_http_error(404, "model not found");
        assert!(matches!(err, OllamaError::ModelNotFoundError(_)));

        let err = mapper.map_http_error(500, "");
        assert!(matches!(err, OllamaError::ApiError(_)));

        let err = mapper.map_http_error(503, "");
        assert!(matches!(err, OllamaError::ServiceUnavailableError(_)));

        let err = mapper.map_http_error(504, "gateway timeout");
        assert!(matches!(err, OllamaError::TimeoutError(_)));
    }

    #[test]
    fn test_ollama_error_mapper_pattern_matching() {
        let mapper = OllamaErrorMapper;

        // Model not found pattern
        let err = mapper.map_http_error(400, "model 'llama3' not found");
        assert!(matches!(err, OllamaError::ModelNotFoundError(_)));

        // Context length pattern
        let err = mapper.map_http_error(400, "context length exceeded");
        assert!(matches!(err, OllamaError::ContextLengthExceeded { .. }));

        // Model loading pattern
        let err = mapper.map_http_error(503, "model is loading");
        assert!(matches!(err, OllamaError::ModelLoadingError(_)));
    }

    #[test]
    fn test_ollama_error_mapper_json_error() {
        let mapper = OllamaErrorMapper;

        let json_body = r#"{"error": "model not found"}"#;
        let err = mapper.map_http_error(404, json_body);
        assert!(matches!(err, OllamaError::ModelNotFoundError(_)));
    }

    #[test]
    fn test_ollama_error_mapper_empty_body() {
        let mapper = OllamaErrorMapper;
        let err = mapper.map_http_error(400, "");
        if let OllamaError::InvalidRequestError(msg) = err {
            assert!(msg.contains("HTTP error 400"));
        } else {
            panic!("Expected InvalidRequestError");
        }
    }
}
