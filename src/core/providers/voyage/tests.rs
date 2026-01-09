//! Tests for Voyage AI Provider

use super::*;

#[cfg(test)]
mod provider_tests {
    use super::*;
    use crate::core::types::requests::EmbeddingInput;

    async fn create_test_provider() -> VoyageProvider {
        let config = VoyageConfig {
            api_key: Some("test-key".to_string()),
            ..Default::default()
        };
        VoyageProvider::new(config).await.unwrap()
    }

    #[tokio::test]
    async fn test_provider_creation() {
        let config = VoyageConfig {
            api_key: Some("test-key".to_string()),
            ..Default::default()
        };
        let provider = VoyageProvider::new(config).await;
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_provider_with_api_key() {
        let provider = VoyageProvider::with_api_key("test-key").await;
        assert!(provider.is_ok());
    }

    #[tokio::test]
    async fn test_provider_name() {
        let provider = create_test_provider().await;
        assert_eq!(provider.name(), "voyage");
    }

    #[tokio::test]
    async fn test_provider_capabilities() {
        let provider = create_test_provider().await;
        let capabilities = provider.capabilities();
        assert!(capabilities.contains(&ProviderCapability::Embeddings));
        assert!(!capabilities.contains(&ProviderCapability::ChatCompletion));
    }

    #[tokio::test]
    async fn test_provider_models() {
        let provider = create_test_provider().await;
        let models = provider.models();
        assert!(!models.is_empty());

        // Check that we have voyage-3 model
        assert!(models.iter().any(|m| m.id == "voyage-3"));
    }

    #[tokio::test]
    async fn test_supports_embeddings() {
        let provider = create_test_provider().await;
        assert!(provider.supports_embeddings());
    }

    #[tokio::test]
    async fn test_does_not_support_chat() {
        let provider = create_test_provider().await;
        assert!(!provider.supports_streaming());
        assert!(!provider.supports_tools());
    }

    #[tokio::test]
    async fn test_get_supported_openai_params() {
        let provider = create_test_provider().await;
        let params = provider.get_supported_openai_params("voyage-3");
        assert!(params.contains(&"encoding_format"));
        assert!(params.contains(&"dimensions"));
    }

    #[tokio::test]
    async fn test_map_openai_params() {
        let provider = create_test_provider().await;
        let mut params = std::collections::HashMap::new();
        params.insert("dimensions".to_string(), serde_json::json!(512));

        let mapped = provider
            .map_openai_params(params, "voyage-3")
            .await
            .unwrap();

        // dimensions should be mapped to output_dimension for voyage-3
        assert!(mapped.contains_key("output_dimension"));
        assert!(!mapped.contains_key("dimensions"));
    }

    #[tokio::test]
    async fn test_map_openai_params_non_v3() {
        let provider = create_test_provider().await;
        let mut params = std::collections::HashMap::new();
        params.insert("dimensions".to_string(), serde_json::json!(512));

        let mapped = provider
            .map_openai_params(params, "voyage-2")
            .await
            .unwrap();

        // dimensions should NOT be mapped for voyage-2 (doesn't support custom dimensions)
        assert!(!mapped.contains_key("output_dimension"));
    }

    #[tokio::test]
    async fn test_calculate_cost() {
        let provider = create_test_provider().await;
        let cost = provider
            .calculate_cost("voyage-3", 1000000, 0)
            .await
            .unwrap();

        // Voyage 3 costs $0.06 per million tokens
        assert!((cost - 0.06).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_calculate_cost_lite() {
        let provider = create_test_provider().await;
        let cost = provider
            .calculate_cost("voyage-3-lite", 1000000, 0)
            .await
            .unwrap();

        // Voyage 3 Lite costs $0.02 per million tokens
        assert!((cost - 0.02).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_calculate_cost_unknown_model() {
        let provider = create_test_provider().await;
        let result = provider.calculate_cost("unknown-model", 1000, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_chat_completion_returns_error() {
        use crate::core::types::requests::{ChatMessage, MessageContent, MessageRole};

        let provider = create_test_provider().await;

        let request = ChatRequest {
            model: "voyage-3".to_string(),
            messages: vec![ChatMessage {
                role: MessageRole::User,
                content: Some(MessageContent::Text("Hello".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            }],
            temperature: None,
            max_tokens: None,
            stream: false,
            tools: None,
            tool_choice: None,
            response_format: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop: None,
            n: None,
            logprobs: None,
            top_logprobs: None,
            user: None,
            metadata: None,
            max_completion_tokens: None,
            seed: None,
            logit_bias: None,
            reasoning_effort: None,
            thinking: None,
            parallel_tool_calls: None,
        };

        let context = RequestContext::default();
        let result = provider.chat_completion(request, context).await;

        assert!(result.is_err());
        if let Err(VoyageError::InvalidRequestError(msg)) = result {
            assert!(msg.contains("embedding-only"));
        } else {
            panic!("Expected InvalidRequestError");
        }
    }

    #[tokio::test]
    async fn test_transform_embedding_request() {
        let provider = create_test_provider().await;

        let request = EmbeddingRequest {
            model: "voyage-3".to_string(),
            input: EmbeddingInput::Text("Hello world".to_string()),
            encoding_format: Some("float".to_string()),
            dimensions: Some(512),
            user: None,
            task_type: Some("document".to_string()),
        };

        let result = provider.transform_embedding_request(&request);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert_eq!(json["model"], "voyage-3");
        assert_eq!(json["encoding_format"], "float");
        assert_eq!(json["output_dimension"], 512);
        assert_eq!(json["input_type"], "document");
    }

    #[tokio::test]
    async fn test_transform_embedding_request_array() {
        let provider = create_test_provider().await;

        let request = EmbeddingRequest {
            model: "voyage-3".to_string(),
            input: EmbeddingInput::Array(vec!["Hello".to_string(), "World".to_string()]),
            encoding_format: None,
            dimensions: None,
            user: None,
            task_type: None,
        };

        let result = provider.transform_embedding_request(&request);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json["input"].as_array().is_some());
        assert_eq!(json["input"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_transform_embedding_response() {
        let provider = create_test_provider().await;

        let response = serde_json::json!({
            "object": "list",
            "model": "voyage-3",
            "data": [{
                "object": "embedding",
                "index": 0,
                "embedding": [0.1, 0.2, 0.3, 0.4, 0.5]
            }],
            "usage": {
                "total_tokens": 5
            }
        });

        let result = provider.transform_embedding_response(response);
        assert!(result.is_ok());

        let embedding_response = result.unwrap();
        assert_eq!(embedding_response.object, "list");
        assert_eq!(embedding_response.model, "voyage-3");
        assert_eq!(embedding_response.data.len(), 1);
        assert_eq!(embedding_response.data[0].embedding.len(), 5);

        let usage = embedding_response.usage.unwrap();
        assert_eq!(usage.prompt_tokens, 5);
        assert_eq!(usage.total_tokens, 5);
    }

    #[tokio::test]
    async fn test_transform_embedding_response_multiple() {
        let provider = create_test_provider().await;

        let response = serde_json::json!({
            "object": "list",
            "model": "voyage-3",
            "data": [
                {
                    "object": "embedding",
                    "index": 0,
                    "embedding": [0.1, 0.2, 0.3]
                },
                {
                    "object": "embedding",
                    "index": 1,
                    "embedding": [0.4, 0.5, 0.6]
                }
            ],
            "usage": {
                "total_tokens": 10
            }
        });

        let result = provider.transform_embedding_response(response);
        assert!(result.is_ok());

        let embedding_response = result.unwrap();
        assert_eq!(embedding_response.data.len(), 2);
        assert_eq!(embedding_response.data[0].index, 0);
        assert_eq!(embedding_response.data[1].index, 1);
    }

    #[tokio::test]
    async fn test_error_mapper() {
        let mapper = VoyageErrorMapper;

        let err = mapper.map_http_error(401, "");
        assert!(matches!(err, VoyageError::AuthenticationError(_)));

        let err = mapper.map_http_error(429, "");
        assert!(matches!(err, VoyageError::RateLimitError(_)));

        let err = mapper.map_http_error(400, "token limit exceeded");
        assert!(matches!(err, VoyageError::TokenLimitExceeded(_)));
    }
}

#[cfg(test)]
mod model_info_tests {
    use super::model_info::*;

    #[test]
    fn test_get_available_models() {
        let models = get_available_models();
        assert!(!models.is_empty());
        assert!(models.contains(&"voyage-3"));
    }

    #[test]
    fn test_get_model_info() {
        let model = get_model_info("voyage-3").unwrap();
        assert_eq!(model.display_name, "Voyage 3");
        assert_eq!(model.embedding_dimensions, 1024);
    }

    #[test]
    fn test_get_default_model() {
        assert_eq!(get_default_model(), "voyage-3");
    }

    #[test]
    fn test_get_model_dimensions() {
        assert_eq!(get_model_dimensions("voyage-3"), Some(1024));
        assert_eq!(get_model_dimensions("voyage-3-lite"), Some(512));
        assert_eq!(get_model_dimensions("unknown"), None);
    }

    #[test]
    fn test_supports_custom_dimensions() {
        assert!(supports_custom_dimensions("voyage-3"));
        assert!(supports_custom_dimensions("voyage-3-lite"));
        assert!(!supports_custom_dimensions("voyage-2"));
    }
}

#[cfg(test)]
mod config_tests {
    use super::config::*;
    use crate::core::traits::ProviderConfig;

    #[test]
    fn test_config_default() {
        let config = VoyageConfig::default();
        assert!(config.api_key.is_none());
        assert!(config.api_base.is_none());
        assert_eq!(config.timeout, 60);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_config_validation_with_key() {
        let config = VoyageConfig {
            api_key: Some("test-key".to_string()),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_zero_timeout() {
        let config = VoyageConfig {
            api_key: Some("test-key".to_string()),
            timeout: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_get_api_base_default() {
        let config = VoyageConfig::default();
        assert_eq!(config.get_api_base(), "https://api.voyageai.com/v1");
    }

    #[test]
    fn test_get_embeddings_url() {
        let config = VoyageConfig::default();
        assert_eq!(
            config.get_embeddings_url(),
            "https://api.voyageai.com/v1/embeddings"
        );
    }
}

#[cfg(test)]
mod error_tests {
    use super::error::*;
    use crate::core::types::errors::ProviderErrorTrait;

    #[test]
    fn test_error_display() {
        let err = VoyageError::ApiError("test".to_string());
        assert_eq!(err.to_string(), "API error: test");
    }

    #[test]
    fn test_error_is_retryable() {
        assert!(VoyageError::RateLimitError("".to_string()).is_retryable());
        assert!(VoyageError::ServiceUnavailableError("".to_string()).is_retryable());
        assert!(VoyageError::NetworkError("".to_string()).is_retryable());
        assert!(!VoyageError::AuthenticationError("".to_string()).is_retryable());
    }

    #[test]
    fn test_error_http_status() {
        assert_eq!(
            VoyageError::AuthenticationError("".to_string()).http_status(),
            401
        );
        assert_eq!(
            VoyageError::RateLimitError("".to_string()).http_status(),
            429
        );
        assert_eq!(
            VoyageError::TokenLimitExceeded("".to_string()).http_status(),
            400
        );
    }
}
