//! Empower Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct EmpowerErrorMapper;

impl ErrorMapper<ProviderError> for EmpowerErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("empower", body),
            401 | 403 => ProviderError::authentication("empower", "Invalid API key"),
            404 => ProviderError::model_not_found("empower", body),
            429 => ProviderError::rate_limit("empower", None),
            500..=599 => ProviderError::api_error("empower", status_code, body),
            _ => ProviderError::api_error("empower", status_code, body),
        }
    }
}
