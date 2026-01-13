//! Error mapper implementation

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct ErrorMapperImpl;

impl ErrorMapper<ProviderError> for ErrorMapperImpl {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("compactifai", response_body),
            429 => ProviderError::rate_limit("compactifai", None),
            404 => ProviderError::model_not_found("compactifai", response_body),
            400 => ProviderError::invalid_request("compactifai", response_body),
            _ => ProviderError::api_error("compactifai", status_code, response_body),
        }
    }
}
