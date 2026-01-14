//! SiliconFlow Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct SiliconFlowErrorMapper;

impl ErrorMapper<ProviderError> for SiliconFlowErrorMapper {
    fn map_http_error(&self, status_code: u16, response_body: &str) -> ProviderError {
        match status_code {
            401 => ProviderError::authentication("siliconflow", response_body),
            429 => ProviderError::rate_limit("siliconflow", None),
            404 => ProviderError::model_not_found("siliconflow", response_body),
            400 => ProviderError::invalid_request("siliconflow", response_body),
            _ => ProviderError::api_error("siliconflow", status_code, response_body),
        }
    }
}
