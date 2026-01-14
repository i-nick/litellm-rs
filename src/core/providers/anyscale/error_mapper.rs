//! Anyscale Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct AnyscaleErrorMapper;

impl ErrorMapper<ProviderError> for AnyscaleErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("anyscale", response_body),
            429 => ProviderError::rate_limit("anyscale", None),
            404 => ProviderError::model_not_found("anyscale", response_body),
            400 => ProviderError::invalid_request("anyscale", response_body),
            _ => ProviderError::api_error("anyscale", status_code, response_body),
        }
    }
}
