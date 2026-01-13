//! Error mapper implementation

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct ErrorMapperImpl;

impl ErrorMapper<ProviderError> for ErrorMapperImpl {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("bytez", response_body),
            429 => ProviderError::rate_limit("bytez", None),
            404 => ProviderError::model_not_found("bytez", response_body),
            400 => ProviderError::invalid_request("bytez", response_body),
            _ => ProviderError::api_error("bytez", status_code, response_body),
        }
    }
}
