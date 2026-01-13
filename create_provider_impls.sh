#!/bin/bash

create_provider_impl() {
    local PROVIDER=$1
    local PROVIDER_STRUCT=$2
    
    DIR="src/core/providers/${PROVIDER}"
    
    cat > "${DIR}/provider.rs" << 'EOF'
//! __PROVIDER_STRUCT__ Provider Implementation

use async_trait::async_trait;
use futures::Stream;
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::core::providers::base::{
    GlobalPoolManager, HeaderPair, HttpMethod, get_pricing_db, header, header_owned,
};
use crate::core::providers::unified_provider::ProviderError;
use crate::core::traits::{ProviderConfig, provider::llm_provider::trait_definition::LLMProvider};
use crate::core::types::{
    common::{HealthStatus, ModelInfo, ProviderCapability, RequestContext},
    requests::ChatRequest,
    responses::{ChatChunk, ChatResponse},
};

use super::{__PROVIDER_STRUCT__Client, __PROVIDER_STRUCT__Config, __PROVIDER_STRUCT__ErrorMapper};

#[derive(Debug, Clone)]
pub struct __PROVIDER_STRUCT__Provider {
    config: __PROVIDER_STRUCT__Config,
    pool_manager: Arc<GlobalPoolManager>,
    supported_models: Vec<ModelInfo>,
}

impl __PROVIDER_STRUCT__Provider {
    fn get_request_headers(&self) -> Vec<HeaderPair> {
        let mut headers = Vec::with_capacity(2);

        if let Some(api_key) = &self.config.base.api_key {
            headers.push(header("Authorization", format!("Bearer {}", api_key)));
        }

        for (key, value) in &self.config.base.headers {
            headers.push(header_owned(key.clone(), value.clone()));
        }

        headers
    }

    pub fn new(config: __PROVIDER_STRUCT__Config) -> Result<Self, ProviderError> {
        config
            .validate()
            .map_err(|e| ProviderError::configuration("__PROVIDER__", e))?;

        let pool_manager = Arc::new(
            GlobalPoolManager::new()
                .map_err(|e| ProviderError::configuration("__PROVIDER__", e.to_string()))?,
        );
        let supported_models = __PROVIDER_STRUCT__Client::supported_models();

        Ok(Self {
            config,
            pool_manager,
            supported_models,
        })
    }

    pub fn from_env() -> Result<Self, ProviderError> {
        let config = __PROVIDER_STRUCT__Config::from_env();
        Self::new(config)
    }

    pub async fn with_api_key(api_key: impl Into<String>) -> Result<Self, ProviderError> {
        let mut config = __PROVIDER_STRUCT__Config::new("__PROVIDER__");
        config.base.api_key = Some(api_key.into());
        Self::new(config)
    }
}

#[async_trait]
impl LLMProvider for __PROVIDER_STRUCT__Provider {
    type Config = __PROVIDER_STRUCT__Config;
    type Error = ProviderError;
    type ErrorMapper = __PROVIDER_STRUCT__ErrorMapper;

    fn name(&self) -> &'static str {
        "__PROVIDER__"
    }

    fn capabilities(&self) -> &'static [ProviderCapability] {
        &[
            ProviderCapability::ChatCompletion,
            ProviderCapability::ChatCompletionStream,
        ]
    }

    fn models(&self) -> &[ModelInfo] {
        &self.supported_models
    }

    fn get_supported_openai_params(&self, _model: &str) -> &'static [&'static str] {
        __PROVIDER_STRUCT__Client::supported_openai_params()
    }

    async fn map_openai_params(
        &self,
        params: HashMap<String, Value>,
        _model: &str,
    ) -> Result<HashMap<String, Value>, Self::Error> {
        Ok(params)
    }

    async fn transform_request(
        &self,
        request: ChatRequest,
        _context: RequestContext,
    ) -> Result<Value, Self::Error> {
        Ok(__PROVIDER_STRUCT__Client::transform_chat_request(request))
    }

    async fn transform_response(
        &self,
        raw_response: &[u8],
        _model: &str,
        _request_id: &str,
    ) -> Result<ChatResponse, Self::Error> {
        let response: ChatResponse = serde_json::from_slice(raw_response)
            .map_err(|e| ProviderError::response_parsing("__PROVIDER__", e.to_string()))?;
        Ok(response)
    }

    fn get_error_mapper(&self) -> Self::ErrorMapper {
        __PROVIDER_STRUCT__ErrorMapper
    }

    async fn chat_completion(
        &self,
        request: ChatRequest,
        context: RequestContext,
    ) -> Result<ChatResponse, Self::Error> {
        let url = format!("{}/chat/completions", self.config.get_api_base());
        let body = __PROVIDER_STRUCT__Client::transform_chat_request(request.clone());

        let headers = self.get_request_headers();
        let body_data = Some(body);

        let response = self
            .pool_manager
            .execute_request(&url, HttpMethod::POST, headers, body_data)
            .await?;

        let status = response.status();
        let response_bytes = response
            .bytes()
            .await
            .map_err(|e| ProviderError::network("__PROVIDER__", e.to_string()))?;

        if !status.is_success() {
            let error_text = String::from_utf8_lossy(&response_bytes);
            let mapper = self.get_error_mapper();
            return Err(
                crate::core::traits::error_mapper::trait_def::ErrorMapper::map_http_error(
                    &mapper,
                    status.as_u16(),
                    &error_text,
                ),
            );
        }

        self.transform_response(&response_bytes, &request.model, &context.request_id)
            .await
    }

    async fn chat_completion_stream(
        &self,
        request: ChatRequest,
        _context: RequestContext,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatChunk, Self::Error>> + Send>>, Self::Error>
    {
        let url = format!("{}/chat/completions", self.config.get_api_base());

        let mut body = __PROVIDER_STRUCT__Client::transform_chat_request(request.clone());
        body["stream"] = serde_json::Value::Bool(true);

        let api_key = self
            .config
            .base
            .get_effective_api_key("__PROVIDER__")
            .ok_or_else(|| ProviderError::authentication("__PROVIDER__", "API key is required"))?;

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::network("__PROVIDER__", e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ProviderError::api_error(
                "__PROVIDER__",
                status.as_u16(),
                error_text,
            ));
        }

        let stream = response.bytes_stream();
        Ok(Box::pin(super::streaming::create___PROVIDER_LOWER___stream(stream)))
    }

    async fn health_check(&self) -> HealthStatus {
        if self
            .config
            .base
            .get_effective_api_key("__PROVIDER__")
            .is_some()
        {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }

    async fn calculate_cost(
        &self,
        model: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> Result<f64, Self::Error> {
        let usage = crate::core::providers::base::pricing::Usage {
            prompt_tokens: input_tokens,
            completion_tokens: output_tokens,
            total_tokens: input_tokens + output_tokens,
            reasoning_tokens: None,
        };

        Ok(get_pricing_db().calculate(model, &usage))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> __PROVIDER_STRUCT__Config {
        let mut config = __PROVIDER_STRUCT__Config::new("__PROVIDER__");
        config.base.api_key = Some("test-key".to_string());
        config
    }

    #[test]
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = __PROVIDER_STRUCT__Provider::new(config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_provider_name() {
        let config = create_test_config();
        let provider = __PROVIDER_STRUCT__Provider::new(config).unwrap();
        assert_eq!(provider.name(), "__PROVIDER__");
    }
}
EOF
    
    sed -i '' "s/__PROVIDER_STRUCT__/${PROVIDER_STRUCT}/g" "${DIR}/provider.rs"
    sed -i '' "s/__PROVIDER__/${PROVIDER}/g" "${DIR}/provider.rs"
    sed -i '' "s/__PROVIDER_LOWER__/${PROVIDER}/g" "${DIR}/provider.rs"
    
    echo "Created provider.rs for ${PROVIDER}"
}

create_provider_impl "empower" "Empower"
create_provider_impl "exa_ai" "ExaAi"
create_provider_impl "firecrawl" "Firecrawl"

echo "All provider implementations created!"
