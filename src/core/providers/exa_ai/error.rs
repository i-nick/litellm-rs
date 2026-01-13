//! ExaAi Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct ExaAiErrorMapper;

impl ErrorMapper<ProviderError> for ExaAiErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("exa_ai", body),
            401 | 403 => ProviderError::authentication("exa_ai", "Invalid API key"),
            404 => ProviderError::model_not_found("exa_ai", body),
            429 => ProviderError::rate_limit("exa_ai", None),
            500..=599 => ProviderError::api_error("exa_ai", status_code, body),
            _ => ProviderError::api_error("exa_ai", status_code, body),
        }
    }
}
