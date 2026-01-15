//! AIML API Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

use super::PROVIDER_NAME;

pub struct AimlErrorMapper;

impl ErrorMapper<ProviderError> for AimlErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 | 403 => ProviderError::authentication(PROVIDER_NAME, response_body),
            429 => ProviderError::rate_limit(PROVIDER_NAME, None),
            404 => ProviderError::model_not_found(PROVIDER_NAME, response_body),
            400 => ProviderError::invalid_request(PROVIDER_NAME, response_body),
            402 => ProviderError::quota_exceeded(PROVIDER_NAME, response_body),
            413 => ProviderError::context_length_exceeded(PROVIDER_NAME, 0, 0),
            408 | 504 => ProviderError::timeout(PROVIDER_NAME, response_body),
            500 => ProviderError::api_error(PROVIDER_NAME, status_code, response_body),
            502 | 503 => ProviderError::provider_unavailable(PROVIDER_NAME, response_body),
            _ => ProviderError::api_error(PROVIDER_NAME, status_code, response_body),
        }
    }
}
