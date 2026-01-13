//! Featherless Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct FeatherlessErrorMapper;

impl ErrorMapper<ProviderError> for FeatherlessErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("featherless", body),
            401 | 403 => ProviderError::authentication("featherless", "Invalid API key"),
            404 => ProviderError::model_not_found("featherless", body),
            429 => ProviderError::rate_limit("featherless", None),
            500..=599 => ProviderError::api_error("featherless", status_code, body),
            _ => ProviderError::api_error("featherless", status_code, body),
        }
    }
}
