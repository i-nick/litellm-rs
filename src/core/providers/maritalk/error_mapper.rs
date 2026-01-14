//! Maritalk Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct MaritalkErrorMapper;

impl ErrorMapper<ProviderError> for MaritalkErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("maritalk", response_body),
            429 => ProviderError::rate_limit("maritalk", None),
            404 => ProviderError::model_not_found("maritalk", response_body),
            400 => ProviderError::invalid_request("maritalk", response_body),
            _ => ProviderError::api_error("maritalk", status_code, response_body),
        }
    }
}
