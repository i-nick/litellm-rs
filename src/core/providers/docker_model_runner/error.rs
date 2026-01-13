//! DockerModelRunner Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct DockerModelRunnerErrorMapper;

impl ErrorMapper<ProviderError> for DockerModelRunnerErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("docker_model_runner", body),
            401 | 403 => ProviderError::authentication("docker_model_runner", "Invalid API key"),
            404 => ProviderError::model_not_found("docker_model_runner", body),
            429 => ProviderError::rate_limit("docker_model_runner", None),
            500..=599 => ProviderError::api_error("docker_model_runner", status_code, body),
            _ => ProviderError::api_error("docker_model_runner", status_code, body),
        }
    }
}
