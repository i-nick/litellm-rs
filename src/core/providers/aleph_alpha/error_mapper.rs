//! Aleph Alpha Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct AlephAlphaErrorMapper;

impl ErrorMapper<ProviderError> for AlephAlphaErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("aleph_alpha", response_body),
            429 => ProviderError::rate_limit("aleph_alpha", None),
            404 => ProviderError::model_not_found("aleph_alpha", response_body),
            400 => ProviderError::invalid_request("aleph_alpha", response_body),
            _ => ProviderError::api_error("aleph_alpha", status_code, response_body),
        }
    }
}
