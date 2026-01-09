//! OpenAI Error Mapper Implementation
//!
//! Maps HTTP errors to OpenAI-specific error types.

use super::error::OpenAIError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

/// Error mapper for OpenAI provider
#[derive(Debug, Clone, Copy, Default)]
pub struct OpenAIErrorMapper;

impl ErrorMapper<OpenAIError> for OpenAIErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> OpenAIError {
        // Simple error mapping - in real implementation would parse OpenAI error format
        match status_code {
            401 => OpenAIError::Authentication {
                provider: "openai",
                message: "Invalid API key".to_string(),
            },
            429 => OpenAIError::rate_limit_simple("openai", "Rate limit exceeded"),
            400 => OpenAIError::InvalidRequest {
                provider: "openai",
                message: response_body.to_string(),
            },
            _ => OpenAIError::Other {
                provider: "openai",
                message: format!("HTTP {}: {}", status_code, response_body),
            },
        }
    }
}
