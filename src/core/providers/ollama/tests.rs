//! Ollama Provider Tests
//!
//! Comprehensive unit tests for the Ollama provider implementation.

use super::*;
use crate::core::providers::ollama::config::OllamaConfig;
use crate::core::providers::ollama::error::{OllamaError, OllamaErrorMapper};
use crate::core::providers::ollama::model_info::{
    get_model_info, OllamaModelEntry, OllamaModelInfo, OllamaShowResponse, OllamaTagsResponse,
};
use crate::core::providers::ollama::streaming::{OllamaStreamChunk, OllamaToolCall};
use crate::core::traits::error_mapper::trait_def::ErrorMapper;
use crate::core::types::errors::ProviderErrorTrait;

// ==================== Config Tests ====================

#[test]
fn test_config_default_values() {
    let config = OllamaConfig::default();
    assert!(config.api_key.is_none());
    assert!(config.api_base.is_none());
    assert_eq!(config.timeout, 120);
    assert_eq!(config.max_retries, 3);
    assert!(!config.debug);
    assert!(config.mirostat.is_none());
    assert!(config.num_ctx.is_none());
}

#[test]
fn test_config_get_api_base_default() {
    let config = OllamaConfig::default();
    assert_eq!(config.get_api_base(), "http://localhost:11434");
}

#[test]
fn test_config_get_api_base_custom() {
    let config = OllamaConfig {
        api_base: Some("http://192.168.1.100:11434".to_string()),
        ..Default::default()
    };
    assert_eq!(config.get_api_base(), "http://192.168.1.100:11434");
}

#[test]
fn test_config_endpoints() {
    let config = OllamaConfig {
        api_base: Some("http://test:11434".to_string()),
        ..Default::default()
    };
    assert_eq!(config.get_chat_endpoint(), "http://test:11434/api/chat");
    assert_eq!(
        config.get_generate_endpoint(),
        "http://test:11434/api/generate"
    );
    assert_eq!(
        config.get_embeddings_endpoint(),
        "http://test:11434/api/embed"
    );
    assert_eq!(config.get_tags_endpoint(), "http://test:11434/api/tags");
    assert_eq!(config.get_show_endpoint(), "http://test:11434/api/show");
}

#[test]
fn test_config_validation_ok() {
    let config = OllamaConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_zero_timeout() {
    let config = OllamaConfig {
        timeout: 0,
        ..Default::default()
    };
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Timeout"));
}

#[test]
fn test_config_validation_invalid_mirostat() {
    let config = OllamaConfig {
        mirostat: Some(5),
        ..Default::default()
    };
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Mirostat"));
}

#[test]
fn test_config_build_options() {
    let config = OllamaConfig {
        mirostat: Some(1),
        mirostat_eta: Some(0.1),
        mirostat_tau: Some(5.0),
        num_ctx: Some(4096),
        num_gpu: Some(-1),
        num_thread: Some(8),
        repeat_penalty: Some(1.1),
        ..Default::default()
    };

    let options = config.build_options();
    assert_eq!(options["mirostat"], 1);
    assert_eq!(options["mirostat_eta"], 0.1);
    assert_eq!(options["mirostat_tau"], 5.0);
    assert_eq!(options["num_ctx"], 4096);
    assert_eq!(options["num_gpu"], -1);
    assert_eq!(options["num_thread"], 8);
    assert_eq!(options["repeat_penalty"], 1.1);
}

#[test]
fn test_config_serialization() {
    let config = OllamaConfig {
        api_base: Some("http://custom:11434".to_string()),
        timeout: 60,
        num_ctx: Some(8192),
        ..Default::default()
    };

    let json = serde_json::to_value(&config).unwrap();
    assert_eq!(json["api_base"], "http://custom:11434");
    assert_eq!(json["timeout"], 60);
    assert_eq!(json["num_ctx"], 8192);
}

#[test]
fn test_config_deserialization() {
    let json = r#"{
        "api_base": "http://192.168.1.100:11434",
        "timeout": 60,
        "num_ctx": 4096,
        "mirostat": 1
    }"#;

    let config: OllamaConfig = serde_json::from_str(json).unwrap();
    assert_eq!(
        config.api_base,
        Some("http://192.168.1.100:11434".to_string())
    );
    assert_eq!(config.timeout, 60);
    assert_eq!(config.num_ctx, Some(4096));
    assert_eq!(config.mirostat, Some(1));
}

// ==================== Error Tests ====================

#[test]
fn test_error_display() {
    let err = OllamaError::ApiError("test error".to_string());
    assert_eq!(err.to_string(), "API error: test error");

    let err = OllamaError::ConnectionRefusedError("localhost:11434".to_string());
    assert_eq!(err.to_string(), "Connection refused: localhost:11434");

    let err = OllamaError::ModelNotFoundError("llama3".to_string());
    assert_eq!(err.to_string(), "Model not found: llama3");

    let err = OllamaError::ContextLengthExceeded {
        max: 4096,
        actual: 5000,
    };
    assert_eq!(
        err.to_string(),
        "Context length exceeded: max 4096, got 5000"
    );
}

#[test]
fn test_error_type() {
    assert_eq!(
        OllamaError::ApiError("".to_string()).error_type(),
        "api_error"
    );
    assert_eq!(
        OllamaError::ConnectionRefusedError("".to_string()).error_type(),
        "connection_refused_error"
    );
    assert_eq!(
        OllamaError::TimeoutError("".to_string()).error_type(),
        "timeout_error"
    );
    assert_eq!(
        OllamaError::ModelLoadingError("".to_string()).error_type(),
        "model_loading_error"
    );
    assert_eq!(
        OllamaError::ContextLengthExceeded { max: 0, actual: 0 }.error_type(),
        "context_length_exceeded"
    );
}

#[test]
fn test_error_is_retryable() {
    assert!(OllamaError::ServiceUnavailableError("".to_string()).is_retryable());
    assert!(OllamaError::NetworkError("".to_string()).is_retryable());
    assert!(OllamaError::ConnectionRefusedError("".to_string()).is_retryable());
    assert!(OllamaError::TimeoutError("".to_string()).is_retryable());
    assert!(OllamaError::ModelLoadingError("".to_string()).is_retryable());

    assert!(!OllamaError::ApiError("".to_string()).is_retryable());
    assert!(!OllamaError::AuthenticationError("".to_string()).is_retryable());
    assert!(!OllamaError::InvalidRequestError("".to_string()).is_retryable());
    assert!(!OllamaError::ModelNotFoundError("".to_string()).is_retryable());
}

#[test]
fn test_error_retry_delay() {
    assert_eq!(
        OllamaError::ServiceUnavailableError("".to_string()).retry_delay(),
        Some(5)
    );
    assert_eq!(
        OllamaError::NetworkError("".to_string()).retry_delay(),
        Some(2)
    );
    assert_eq!(
        OllamaError::ConnectionRefusedError("".to_string()).retry_delay(),
        Some(5)
    );
    assert_eq!(
        OllamaError::TimeoutError("".to_string()).retry_delay(),
        Some(10)
    );
    assert_eq!(
        OllamaError::ModelLoadingError("".to_string()).retry_delay(),
        Some(30)
    );
    assert_eq!(OllamaError::ApiError("".to_string()).retry_delay(), None);
}

#[test]
fn test_error_http_status() {
    assert_eq!(
        OllamaError::AuthenticationError("".to_string()).http_status(),
        401
    );
    assert_eq!(
        OllamaError::InvalidRequestError("".to_string()).http_status(),
        400
    );
    assert_eq!(
        OllamaError::ModelNotFoundError("".to_string()).http_status(),
        404
    );
    assert_eq!(
        OllamaError::ServiceUnavailableError("".to_string()).http_status(),
        503
    );
    assert_eq!(
        OllamaError::ConnectionRefusedError("".to_string()).http_status(),
        503
    );
    assert_eq!(
        OllamaError::TimeoutError("".to_string()).http_status(),
        504
    );
}

#[test]
fn test_error_mapper_http_errors() {
    let mapper = OllamaErrorMapper;

    let err = mapper.map_http_error(400, "bad request");
    assert!(matches!(err, OllamaError::InvalidRequestError(_)));

    let err = mapper.map_http_error(401, "");
    assert!(matches!(err, OllamaError::AuthenticationError(_)));

    let err = mapper.map_http_error(404, "model not found");
    assert!(matches!(err, OllamaError::ModelNotFoundError(_)));

    let err = mapper.map_http_error(503, "");
    assert!(matches!(err, OllamaError::ServiceUnavailableError(_)));

    let err = mapper.map_http_error(504, "timeout");
    assert!(matches!(err, OllamaError::TimeoutError(_)));
}

#[test]
fn test_error_mapper_pattern_matching() {
    let mapper = OllamaErrorMapper;

    // Model not found pattern
    let err = mapper.map_http_error(400, "model 'llama3' not found");
    assert!(matches!(err, OllamaError::ModelNotFoundError(_)));

    // Context length pattern
    let err = mapper.map_http_error(400, "context length exceeded");
    assert!(matches!(err, OllamaError::ContextLengthExceeded { .. }));

    // Model loading pattern
    let err = mapper.map_http_error(503, "model is loading");
    assert!(matches!(err, OllamaError::ModelLoadingError(_)));
}

#[test]
fn test_error_mapper_json_body() {
    let mapper = OllamaErrorMapper;

    let json_body = r#"{"error": "model not found"}"#;
    let err = mapper.map_http_error(404, json_body);
    assert!(matches!(err, OllamaError::ModelNotFoundError(_)));
}

// ==================== Model Info Tests ====================

#[test]
fn test_model_info_new() {
    let info = OllamaModelInfo::new("llama3:8b");
    assert_eq!(info.name, "llama3:8b");
    assert_eq!(info.display_name, "llama3:8b");
    assert!(!info.supports_tools);
    assert!(!info.supports_vision);
}

#[test]
fn test_model_info_infer_llama() {
    let info = get_model_info("llama3:8b");
    assert_eq!(info.family, Some("llama".to_string()));
    assert!(info.supports_tools);
    assert!(!info.supports_vision);
    assert_eq!(info.parameter_size, Some("8B".to_string()));
}

#[test]
fn test_model_info_infer_vision() {
    let info = get_model_info("llava:13b");
    assert!(info.supports_vision);

    let info = get_model_info("llama3-vision:11b");
    assert!(info.supports_vision);

    let info = get_model_info("moondream:1.8b");
    assert!(info.supports_vision);

    let info = get_model_info("bakllava:7b");
    assert!(info.supports_vision);
}

#[test]
fn test_model_info_infer_mistral() {
    let info = get_model_info("mistral:7b");
    assert_eq!(info.family, Some("mistral".to_string()));
    assert!(info.supports_tools);
    assert_eq!(info.context_length, Some(32768));
}

#[test]
fn test_model_info_infer_mixtral() {
    let info = get_model_info("mixtral:8x7b");
    assert_eq!(info.family, Some("mixtral".to_string()));
    assert!(info.supports_tools);
}

#[test]
fn test_model_info_infer_qwen() {
    let info = get_model_info("qwen2:7b");
    assert_eq!(info.family, Some("qwen".to_string()));
    assert!(info.supports_tools);
}

#[test]
fn test_model_info_infer_gemma() {
    let info = get_model_info("gemma:7b");
    assert_eq!(info.family, Some("gemma".to_string()));
    assert_eq!(info.context_length, Some(8192));
}

#[test]
fn test_model_info_infer_deepseek() {
    let info = get_model_info("deepseek-coder:6.7b");
    assert_eq!(info.family, Some("deepseek".to_string()));
    assert!(info.supports_tools);
}

#[test]
fn test_model_info_infer_phi() {
    let info = get_model_info("phi:3b");
    assert_eq!(info.family, Some("phi".to_string()));
    assert_eq!(info.context_length, Some(4096));
}

#[test]
fn test_show_response_supports_tools() {
    let response = OllamaShowResponse {
        modelfile: None,
        parameters: None,
        template: Some("{{ .Tools }}".to_string()),
        details: None,
        model_info: None,
    };
    assert!(response.supports_tools());

    let response = OllamaShowResponse {
        modelfile: None,
        parameters: None,
        template: Some("{{ .System }}".to_string()),
        details: None,
        model_info: None,
    };
    assert!(!response.supports_tools());
}

#[test]
fn test_show_response_get_context_length() {
    let response = OllamaShowResponse {
        modelfile: None,
        parameters: None,
        template: None,
        details: None,
        model_info: Some(serde_json::json!({
            "context_length": 8192
        })),
    };
    assert_eq!(response.get_context_length(), Some(8192));

    let response = OllamaShowResponse {
        modelfile: None,
        parameters: None,
        template: None,
        details: None,
        model_info: Some(serde_json::json!({
            "num_ctx": 4096
        })),
    };
    assert_eq!(response.get_context_length(), Some(4096));
}

#[test]
fn test_tags_response_deserialization() {
    let json = r#"{
        "models": [
            {
                "name": "llama3:8b",
                "modified_at": "2024-01-01T00:00:00Z",
                "size": 4000000000,
                "details": {
                    "family": "llama",
                    "parameter_size": "8B"
                }
            },
            {
                "name": "mistral:7b",
                "modified_at": "2024-01-02T00:00:00Z",
                "size": 3500000000
            }
        ]
    }"#;

    let response: OllamaTagsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.models.len(), 2);
    assert_eq!(response.models[0].name, "llama3:8b");
    assert_eq!(response.models[1].name, "mistral:7b");
}

#[test]
fn test_model_entry_to_model_info() {
    let entry = OllamaModelEntry {
        name: "llama3:8b".to_string(),
        model: Some("llama3:8b".to_string()),
        modified_at: Some("2024-01-01T00:00:00Z".to_string()),
        size: Some(4_000_000_000),
        digest: None,
        details: Some(super::model_info::OllamaModelDetails {
            parent_model: None,
            format: Some("gguf".to_string()),
            family: Some("llama".to_string()),
            families: None,
            parameter_size: Some("8B".to_string()),
            quantization_level: Some("Q4_0".to_string()),
        }),
    };

    let info: OllamaModelInfo = entry.into();
    assert_eq!(info.name, "llama3:8b");
    assert_eq!(info.family, Some("llama".to_string()));
    assert_eq!(info.parameter_size, Some("8B".to_string()));
    assert_eq!(info.quantization, Some("Q4_0".to_string()));
    assert!(info.supports_tools);
}

// ==================== Streaming Tests ====================

#[test]
fn test_stream_chunk_deserialization_basic() {
    let json = r#"{
        "model": "llama3:8b",
        "created_at": "2024-01-01T00:00:00Z",
        "message": {
            "role": "assistant",
            "content": "Hello"
        },
        "done": false
    }"#;

    let chunk: OllamaStreamChunk = serde_json::from_str(json).unwrap();
    assert_eq!(chunk.model, "llama3:8b");
    assert!(!chunk.done);
    assert!(chunk.message.is_some());
    assert_eq!(chunk.message.unwrap().content, Some("Hello".to_string()));
}

#[test]
fn test_stream_chunk_deserialization_done() {
    let json = r#"{
        "model": "llama3:8b",
        "message": {
            "role": "assistant",
            "content": ""
        },
        "done": true,
        "done_reason": "stop",
        "prompt_eval_count": 10,
        "eval_count": 50,
        "total_duration": 1000000000
    }"#;

    let chunk: OllamaStreamChunk = serde_json::from_str(json).unwrap();
    assert!(chunk.done);
    assert_eq!(chunk.done_reason, Some("stop".to_string()));
    assert_eq!(chunk.prompt_eval_count, Some(10));
    assert_eq!(chunk.eval_count, Some(50));
}

#[test]
fn test_stream_chunk_deserialization_tool_calls() {
    let json = r#"{
        "model": "llama3:8b",
        "message": {
            "role": "assistant",
            "content": "",
            "tool_calls": [
                {
                    "function": {
                        "name": "get_weather",
                        "arguments": {"location": "NYC"}
                    }
                }
            ]
        },
        "done": true,
        "done_reason": "tool_calls"
    }"#;

    let chunk: OllamaStreamChunk = serde_json::from_str(json).unwrap();
    let tool_calls = chunk.message.as_ref().unwrap().tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].function.name, "get_weather");
}

#[test]
fn test_stream_chunk_deserialization_thinking() {
    let json = r#"{
        "model": "deepseek-r1",
        "message": {
            "role": "assistant",
            "content": "",
            "thinking": "Let me think about this..."
        },
        "done": false
    }"#;

    let chunk: OllamaStreamChunk = serde_json::from_str(json).unwrap();
    let message = chunk.message.unwrap();
    assert_eq!(
        message.thinking,
        Some("Let me think about this...".to_string())
    );
}

#[test]
fn test_stream_chunk_deserialization_error() {
    let json = r#"{
        "model": "llama3:8b",
        "error": "model not found",
        "done": true
    }"#;

    let chunk: OllamaStreamChunk = serde_json::from_str(json).unwrap();
    assert_eq!(chunk.error, Some("model not found".to_string()));
}

#[test]
fn test_tool_call_serialization() {
    let tool_call = OllamaToolCall {
        id: Some("call_123".to_string()),
        function: super::streaming::OllamaToolFunction {
            name: "get_weather".to_string(),
            arguments: serde_json::json!({"location": "NYC"}),
        },
    };

    let json = serde_json::to_string(&tool_call).unwrap();
    assert!(json.contains("get_weather"));
    assert!(json.contains("NYC"));
}

// ==================== Provider Tests ====================

#[tokio::test]
async fn test_provider_creation() {
    let provider = OllamaProvider::new(OllamaConfig::default()).await;
    assert!(provider.is_ok());

    let provider = provider.unwrap();
    assert_eq!(provider.name(), "ollama");
}

#[tokio::test]
async fn test_provider_with_base_url() {
    let provider = OllamaProvider::with_base_url("http://192.168.1.100:11434").await;
    assert!(provider.is_ok());
}

#[tokio::test]
async fn test_provider_capabilities() {
    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();
    let capabilities = provider.capabilities();

    assert!(capabilities.contains(&ProviderCapability::ChatCompletion));
    assert!(capabilities.contains(&ProviderCapability::ChatCompletionStream));
    assert!(capabilities.contains(&ProviderCapability::Embeddings));
    assert!(capabilities.contains(&ProviderCapability::ToolCalling));
}

#[tokio::test]
async fn test_provider_supported_params() {
    use crate::core::traits::provider::llm_provider::trait_definition::LLMProvider;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();
    let params = provider.get_supported_openai_params("llama3:8b");

    assert!(params.contains(&"temperature"));
    assert!(params.contains(&"top_p"));
    assert!(params.contains(&"max_tokens"));
    assert!(params.contains(&"stream"));
    assert!(params.contains(&"stop"));
    assert!(params.contains(&"tools"));
    assert!(params.contains(&"num_ctx"));
    assert!(params.contains(&"mirostat"));
}

#[tokio::test]
async fn test_provider_map_openai_params() {
    use crate::core::traits::provider::llm_provider::trait_definition::LLMProvider;
    use std::collections::HashMap;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let mut params = HashMap::new();
    params.insert("max_tokens".to_string(), serde_json::json!(100));
    params.insert("temperature".to_string(), serde_json::json!(0.7));

    let mapped = provider
        .map_openai_params(params, "llama3:8b")
        .await
        .unwrap();

    // max_tokens should be mapped to num_predict
    assert!(mapped.contains_key("num_predict"));
    assert!(!mapped.contains_key("max_tokens"));
    assert!(mapped.contains_key("temperature"));
}

#[tokio::test]
async fn test_provider_calculate_cost() {
    use crate::core::traits::provider::llm_provider::trait_definition::LLMProvider;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    // Ollama is free, so cost should always be 0
    let cost = provider
        .calculate_cost("llama3:8b", 1000, 500)
        .await
        .unwrap();
    assert_eq!(cost, 0.0);
}

// ==================== Request Building Tests ====================

#[tokio::test]
async fn test_provider_build_chat_request_basic() {
    use crate::core::types::requests::ChatMessage;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let request = ChatRequest {
        model: "llama3:8b".to_string(),
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: Some(MessageContent::Text("Hello".to_string())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        temperature: Some(0.7),
        top_p: Some(0.9),
        max_tokens: Some(100),
        stream: false,
        ..Default::default()
    };

    let body = provider.build_chat_request(&request, false).unwrap();

    assert_eq!(body["model"], "llama3:8b");
    assert_eq!(body["stream"], false);
    assert!(body["messages"].is_array());
    assert_eq!(body["messages"][0]["role"], "user");
    assert_eq!(body["messages"][0]["content"], "Hello");
    assert_eq!(body["options"]["temperature"], 0.7);
    assert_eq!(body["options"]["top_p"], 0.9);
    assert_eq!(body["options"]["num_predict"], 100);
}

#[tokio::test]
async fn test_provider_build_chat_request_with_system() {
    use crate::core::types::requests::ChatMessage;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let request = ChatRequest {
        model: "llama3:8b".to_string(),
        messages: vec![
            ChatMessage {
                role: MessageRole::System,
                content: Some(MessageContent::Text("You are a helpful assistant.".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: Some(MessageContent::Text("Hello".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
        stream: false,
        ..Default::default()
    };

    let body = provider.build_chat_request(&request, false).unwrap();

    assert_eq!(body["messages"].as_array().unwrap().len(), 2);
    assert_eq!(body["messages"][0]["role"], "system");
    assert_eq!(body["messages"][1]["role"], "user");
}

#[tokio::test]
async fn test_provider_build_chat_request_with_tools() {
    use crate::core::types::requests::{ChatMessage, Tool, ToolFunction};

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let request = ChatRequest {
        model: "llama3:8b".to_string(),
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: Some(MessageContent::Text("What's the weather?".to_string())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        tools: Some(vec![Tool {
            r#type: "function".to_string(),
            function: ToolFunction {
                name: "get_weather".to_string(),
                description: Some("Get the weather for a location".to_string()),
                parameters: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {"type": "string"}
                    }
                })),
                strict: None,
            },
        }]),
        stream: false,
        ..Default::default()
    };

    let body = provider.build_chat_request(&request, false).unwrap();

    assert!(body["tools"].is_array());
    assert_eq!(body["tools"][0]["function"]["name"], "get_weather");
}

#[tokio::test]
async fn test_provider_build_chat_request_with_response_format() {
    use crate::core::types::requests::ChatMessage;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let request = ChatRequest {
        model: "llama3:8b".to_string(),
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: Some(MessageContent::Text("Return JSON".to_string())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        response_format: Some(serde_json::json!({"type": "json_object"})),
        stream: false,
        ..Default::default()
    };

    let body = provider.build_chat_request(&request, false).unwrap();

    assert_eq!(body["format"], "json");
}

#[tokio::test]
async fn test_provider_build_chat_request_strips_ollama_prefix() {
    use crate::core::types::requests::ChatMessage;

    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let request = ChatRequest {
        model: "ollama/llama3:8b".to_string(),
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: Some(MessageContent::Text("Hello".to_string())),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        stream: false,
        ..Default::default()
    };

    let body = provider.build_chat_request(&request, false).unwrap();

    // Should strip the "ollama/" prefix
    assert_eq!(body["model"], "llama3:8b");
}

// ==================== Response Parsing Tests ====================

#[tokio::test]
async fn test_provider_parse_chat_response_basic() {
    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let response = serde_json::json!({
        "model": "llama3:8b",
        "message": {
            "role": "assistant",
            "content": "Hello! How can I help you?"
        },
        "done": true,
        "done_reason": "stop",
        "prompt_eval_count": 10,
        "eval_count": 15
    });

    let chat_response = provider.parse_chat_response(response, "llama3:8b").unwrap();

    assert!(chat_response.id.starts_with("ollama-"));
    assert_eq!(chat_response.object, "chat.completion");
    assert_eq!(chat_response.model, "ollama/llama3:8b");
    assert_eq!(chat_response.choices.len(), 1);
    assert_eq!(chat_response.choices[0].finish_reason, Some("stop".to_string()));

    let message = &chat_response.choices[0].message;
    assert_eq!(message.role, MessageRole::Assistant);
    if let Some(MessageContent::Text(content)) = &message.content {
        assert_eq!(content, "Hello! How can I help you?");
    }

    let usage = chat_response.usage.unwrap();
    assert_eq!(usage.prompt_tokens, 10);
    assert_eq!(usage.completion_tokens, 15);
    assert_eq!(usage.total_tokens, 25);
}

#[tokio::test]
async fn test_provider_parse_chat_response_with_tool_calls() {
    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let response = serde_json::json!({
        "model": "llama3:8b",
        "message": {
            "role": "assistant",
            "content": "",
            "tool_calls": [
                {
                    "function": {
                        "name": "get_weather",
                        "arguments": {"location": "NYC"}
                    }
                }
            ]
        },
        "done": true,
        "done_reason": "tool_calls"
    });

    let chat_response = provider.parse_chat_response(response, "llama3:8b").unwrap();

    let message = &chat_response.choices[0].message;
    assert!(message.tool_calls.is_some());

    let tool_calls = message.tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].function.name, "get_weather");
}

#[tokio::test]
async fn test_provider_parse_chat_response_with_thinking() {
    let provider = OllamaProvider::new(OllamaConfig::default()).await.unwrap();

    let response = serde_json::json!({
        "model": "deepseek-r1",
        "message": {
            "role": "assistant",
            "content": "The answer is 42.",
            "thinking": "Let me think about this step by step..."
        },
        "done": true,
        "done_reason": "stop"
    });

    let chat_response = provider
        .parse_chat_response(response, "deepseek-r1")
        .unwrap();

    let message = &chat_response.choices[0].message;
    assert_eq!(
        message.thinking,
        Some("Let me think about this step by step...".to_string())
    );
}
