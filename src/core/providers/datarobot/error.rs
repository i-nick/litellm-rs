//! Datarobot Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct DataRobotErrorMapper;

impl ErrorMapper<ProviderError> for DataRobotErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("datarobot", body),
            401 | 403 => ProviderError::authentication("datarobot", "Invalid API key"),
            404 => ProviderError::model_not_found("datarobot", body),
            429 => ProviderError::rate_limit("datarobot", None),
            500..=599 => ProviderError::api_error("datarobot", status_code, body),
            _ => ProviderError::api_error("datarobot", status_code, body),
        }
    }
}
