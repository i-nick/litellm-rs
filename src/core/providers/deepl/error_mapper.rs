//! DeepL Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct DeepLErrorMapper;

impl ErrorMapper<ProviderError> for DeepLErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 | 403 => ProviderError::authentication("deepl", response_body),
            429 => ProviderError::rate_limit("deepl", None),
            404 => ProviderError::invalid_request("deepl", "Endpoint not found"),
            400 => ProviderError::invalid_request("deepl", response_body),
            456 => ProviderError::quota_exceeded("deepl", "Quota exceeded"),
            _ => ProviderError::api_error("deepl", status_code, response_body),
        }
    }
}
