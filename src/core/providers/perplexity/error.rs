//! Perplexity Error Handling
//!
//! Error mapping for Perplexity API responses

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

/// Perplexity error mapper
#[derive(Debug)]
pub struct PerplexityErrorMapper;

impl ErrorMapper<ProviderError> for PerplexityErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("perplexity", "Invalid API key"),
            403 => ProviderError::authentication("perplexity", "Permission denied"),
            404 => ProviderError::model_not_found("perplexity", "Model not found"),
            429 => {
                let retry_after = parse_retry_after(response_body);
                ProviderError::rate_limit("perplexity", retry_after)
            }
            400 => {
                // Parse error message from response body
                let message =
                    parse_error_message(response_body).unwrap_or_else(|| "Bad request".to_string());
                ProviderError::invalid_request("perplexity", message)
            }
            500..=599 => ProviderError::api_error("perplexity", status_code, response_body),
            _ => ProviderError::api_error("perplexity", status_code, response_body),
        }
    }
}

/// Parse retry-after header or response body for rate limit info
fn parse_retry_after(response_body: &str) -> Option<u64> {
    // Try to parse JSON error response
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(response_body) {
        // Check for retry_after in error response
        if let Some(retry_after) = json.get("retry_after").and_then(|v| v.as_u64()) {
            return Some(retry_after);
        }
    }

    // Default retry time for rate limits
    if response_body.to_lowercase().contains("rate limit") {
        Some(60)
    } else {
        None
    }
}

/// Parse error message from response body
fn parse_error_message(response_body: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(response_body) {
        // Check standard error.message format
        if let Some(message) = json
            .get("error")
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
        {
            return Some(message.to_string());
        }
        // Check for top-level message
        if let Some(message) = json.get("message").and_then(|m| m.as_str()) {
            return Some(message.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perplexity_error_mapper_401() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(401, "Unauthorized");
        assert!(matches!(err, ProviderError::Authentication { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_403() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(403, "Forbidden");
        assert!(matches!(err, ProviderError::Authentication { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_404() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(404, "Not found");
        assert!(matches!(err, ProviderError::ModelNotFound { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_429() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(429, "rate limit exceeded");
        assert!(matches!(err, ProviderError::RateLimit { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_429_with_retry_after() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(429, r#"{"retry_after": 30}"#);
        assert!(matches!(err, ProviderError::RateLimit { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_400() {
        let mapper = PerplexityErrorMapper;
        let err =
            mapper.map_http_error(400, r#"{"error": {"message": "Invalid model specified"}}"#);
        assert!(matches!(err, ProviderError::InvalidRequest { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_500() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(500, "Internal error");
        assert!(matches!(err, ProviderError::ApiError { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_503() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(503, "Service unavailable");
        assert!(matches!(err, ProviderError::ApiError { .. }));
    }

    #[test]
    fn test_perplexity_error_mapper_unknown() {
        let mapper = PerplexityErrorMapper;
        let err = mapper.map_http_error(418, "I'm a teapot");
        assert!(matches!(err, ProviderError::ApiError { .. }));
    }

    #[test]
    fn test_parse_retry_after_with_rate_limit() {
        let result = parse_retry_after("rate limit exceeded");
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_parse_retry_after_json() {
        let result = parse_retry_after(r#"{"retry_after": 120}"#);
        assert_eq!(result, Some(120));
    }

    #[test]
    fn test_parse_retry_after_without_rate_limit() {
        let result = parse_retry_after("other error");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_retry_after_empty() {
        let result = parse_retry_after("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_error_message_standard() {
        let result = parse_error_message(r#"{"error": {"message": "Test error"}}"#);
        assert_eq!(result, Some("Test error".to_string()));
    }

    #[test]
    fn test_parse_error_message_top_level() {
        let result = parse_error_message(r#"{"message": "Top level error"}"#);
        assert_eq!(result, Some("Top level error".to_string()));
    }

    #[test]
    fn test_parse_error_message_none() {
        let result = parse_error_message("invalid json");
        assert_eq!(result, None);
    }
}
