//! AIML API Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct AimlErrorMapper;

impl ErrorMapper<ProviderError> for AimlErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("aiml", response_body),
            429 => ProviderError::rate_limit("aiml", None),
            404 => ProviderError::model_not_found("aiml", response_body),
            400 => ProviderError::invalid_request("aiml", response_body),
            _ => ProviderError::api_error("aiml", status_code, response_body),
        }
    }
}
