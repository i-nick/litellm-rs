#!/bin/bash

# Create all provider files for empower, exa_ai, firecrawl

create_provider_files() {
    local PROVIDER=$1
    local PROVIDER_STRUCT=$2
    local API_BASE=$3
    
    DIR="src/core/providers/${PROVIDER}"
    
    # error.rs
    cat > "${DIR}/error.rs" << EOF
//! ${PROVIDER_STRUCT} Error Mapper

use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::error_mapper::trait_def::ErrorMapper;

pub struct ${PROVIDER_STRUCT}ErrorMapper;

impl ErrorMapper for ${PROVIDER_STRUCT}ErrorMapper {
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

    # models.rs
    cat > "${DIR}/models.rs" << 'EOF'
//! __PROVIDER_STRUCT__ Model Information

use crate::core::types::common::ModelInfo;
use std::collections::HashMap;

pub struct __PROVIDER_STRUCT__ModelRegistry;

impl __PROVIDER_STRUCT__ModelRegistry {
    pub fn get_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "__PROVIDER__-model".to_string(),
                name: "__PROVIDER_STRUCT__ Model".to_string(),
                provider: "__PROVIDER__".to_string(),
                max_context_length: 8192,
                max_output_length: Some(4096),
                supports_streaming: true,
                supports_tools: false,
                supports_multimodal: false,
                input_cost_per_1k_tokens: None,
                output_cost_per_1k_tokens: None,
                currency: "USD".to_string(),
                capabilities: vec![
                    crate::core::types::common::ProviderCapability::ChatCompletion,
                    crate::core::types::common::ProviderCapability::ChatCompletionStream,
                ],
                created_at: None,
                updated_at: None,
                metadata: HashMap::new(),
            },
        ]
    }
}

pub fn get___PROVIDER_LOWER___registry() -> __PROVIDER_STRUCT__ModelRegistry {
    __PROVIDER_STRUCT__ModelRegistry
}
EOF
    sed -i '' "s/__PROVIDER_STRUCT__/${PROVIDER_STRUCT}/g" "${DIR}/models.rs"
    sed -i '' "s/__PROVIDER__/${PROVIDER}/g" "${DIR}/models.rs"
    sed -i '' "s/__PROVIDER_LOWER__/${PROVIDER}/g" "${DIR}/models.rs"

    # client.rs  
    cat > "${DIR}/client.rs" << 'EOF'
//! __PROVIDER_STRUCT__ Client

use crate::core::types::common::ModelInfo;
use crate::core::types::requests::ChatRequest;
use serde_json::Value;

pub struct __PROVIDER_STRUCT__Client;

impl __PROVIDER_STRUCT__Client {
    pub fn supported_models() -> Vec<ModelInfo> {
        super::models::__PROVIDER_STRUCT__ModelRegistry::get_models()
    }

    pub fn supported_openai_params() -> &'static [&'static str] {
        &["temperature", "max_tokens", "top_p", "stream", "stop"]
    }

    pub fn transform_chat_request(request: ChatRequest) -> Value {
        serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
            "top_p": request.top_p,
            "stream": request.stream,
            "stop": request.stop,
        })
    }
}
EOF
    sed -i '' "s/__PROVIDER_STRUCT__/${PROVIDER_STRUCT}/g" "${DIR}/client.rs"

    # streaming.rs
    cat > "${DIR}/streaming.rs" << 'EOF'
//! __PROVIDER_STRUCT__ Streaming Support

use futures::Stream;
use crate::core::providers::unified_provider::ProviderError;
use crate::core::types::responses::ChatChunk;

pub fn create___PROVIDER_LOWER___stream(
    stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
) -> impl Stream<Item = Result<ChatChunk, ProviderError>> + Send {
    crate::core::providers::base::streaming::create_sse_stream(stream, "__PROVIDER__")
}
EOF
    sed -i '' "s/__PROVIDER_STRUCT__/${PROVIDER_STRUCT}/g" "${DIR}/streaming.rs"
    sed -i '' "s/__PROVIDER__/${PROVIDER}/g" "${DIR}/streaming.rs"
    sed -i '' "s/__PROVIDER_LOWER__/${PROVIDER}/g" "${DIR}/streaming.rs"

    # Update config.rs with correct API base
    sed -i '' "s|https://api.example.com|${API_BASE}|g" "${DIR}/config.rs"

    echo "Created all files for ${PROVIDER}"
}

# Create providers
create_provider_files "empower" "Empower" "https://api.empower.dev/v1"
create_provider_files "exa_ai" "ExaAi" "https://api.exa.ai/v1"  
create_provider_files "firecrawl" "Firecrawl" "https://api.firecrawl.dev/v1"

echo "All provider files created!"
