#!/bin/bash

for PROVIDER in datarobot docker_model_runner empower exa_ai featherless firecrawl; do
    UPPER=$(echo $PROVIDER | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1' | sed 's/ //g')
    
    cat > "src/core/providers/${PROVIDER}/error.rs" << EOF
//! ${UPPER} Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct ${UPPER}ErrorMapper;

impl ErrorMapper<ProviderError> for ${UPPER}ErrorMapper {
    fn map_http_error(&self, status_code: u16, body: &str) -> ProviderError {
        match status_code {
            400 => ProviderError::invalid_request("${PROVIDER}", body),
            401 | 403 => ProviderError::authentication("${PROVIDER}", "Invalid API key"),
            404 => ProviderError::model_not_found("${PROVIDER}", body),
            429 => ProviderError::rate_limit("${PROVIDER}", None),
            500..=599 => ProviderError::api_error("${PROVIDER}", status_code, body),
            _ => ProviderError::api_error("${PROVIDER}", status_code, body),
        }
    }
}
EOF
    
    echo "Fixed error mapper for ${PROVIDER}"
done

echo "All error mappers fixed!"
