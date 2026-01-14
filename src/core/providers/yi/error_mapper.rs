//! Yi (01.AI) Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct YiErrorMapper;

impl ErrorMapper<ProviderError> for YiErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("yi", response_body),
            429 => ProviderError::rate_limit("yi", None),
            404 => ProviderError::model_not_found("yi", response_body),
            400 => ProviderError::invalid_request("yi", response_body),
            _ => ProviderError::api_error("yi", status_code, response_body),
        }
    }
}
