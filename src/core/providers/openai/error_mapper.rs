//! OpenAI Error Mapper Implementation
//!
//! Maps HTTP errors to OpenAI-specific error types.
//! Handles OpenAI's structured error response format.

use super::error::OpenAIError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

/// Error mapper for OpenAI provider
#[derive(Debug, Clone, Copy, Default)]
pub struct OpenAIErrorMapper;

impl ErrorMapper<OpenAIError> for OpenAIErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> OpenAIError {
        // Map HTTP status codes to appropriate error types
        // Preserves response_body for structured error information
        match status_code {
            401 => OpenAIError::authentication("openai", response_body),
            403 => OpenAIError::authentication("openai", response_body),
            429 => OpenAIError::rate_limit_simple("openai", response_body),
            404 => OpenAIError::model_not_found("openai", response_body),
            400 => OpenAIError::invalid_request("openai", response_body),
            402 => OpenAIError::quota_exceeded("openai", response_body),
            413 => OpenAIError::context_length_exceeded("openai", 0, 0), // Context from response parsing
            408 | 504 => OpenAIError::timeout("openai", response_body),
            500 => OpenAIError::api_error("openai", status_code, response_body),
            502 | 503 => OpenAIError::provider_unavailable("openai", response_body),
            _ => OpenAIError::api_error("openai", status_code, response_body),
        }
    }
}
