//! Firecrawl Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct FirecrawlErrorMapper;

impl ErrorMapper<ProviderError> for FirecrawlErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("firecrawl", body),
            401 | 403 => ProviderError::authentication("firecrawl", "Invalid API key"),
            404 => ProviderError::model_not_found("firecrawl", body),
            429 => ProviderError::rate_limit("firecrawl", None),
            500..=599 => ProviderError::api_error("firecrawl", status_code, body),
            _ => ProviderError::api_error("firecrawl", status_code, body),
        }
    }
}
