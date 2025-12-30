//! Completion response types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::super::openai::Usage;

/// Completion response (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Response ID
    pub id: String,
    /// Object type
    pub object: String,
    /// Creation timestamp
    pub created: u64,
    /// Model used
    pub model: String,
    /// Choices
    pub choices: Vec<CompletionChoice>,
    /// Usage statistics
    pub usage: Option<Usage>,
}

/// Completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionChoice {
    /// Choice index
    pub index: u32,
    /// Generated text
    pub text: String,
    /// Logprobs
    pub logprobs: Option<CompletionLogprobs>,
    /// Finish reason
    pub finish_reason: Option<String>,
}

/// Completion logprobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionLogprobs {
    /// Tokens
    pub tokens: Vec<String>,
    /// Token logprobs
    pub token_logprobs: Vec<f64>,
    /// Top logprobs
    pub top_logprobs: Vec<HashMap<String, f64>>,
    /// Text offset
    pub text_offset: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CompletionLogprobs Tests ====================

    #[test]
    fn test_completion_logprobs_basic() {
        let logprobs = CompletionLogprobs {
            tokens: vec!["Hello".to_string(), " world".to_string()],
            token_logprobs: vec![-0.5, -0.3],
            top_logprobs: vec![HashMap::new(), HashMap::new()],
            text_offset: vec![0, 5],
        };

        assert_eq!(logprobs.tokens.len(), 2);
        assert_eq!(logprobs.token_logprobs.len(), 2);
        assert_eq!(logprobs.text_offset, vec![0, 5]);
    }

    #[test]
    fn test_completion_logprobs_empty() {
        let logprobs = CompletionLogprobs {
            tokens: vec![],
            token_logprobs: vec![],
            top_logprobs: vec![],
            text_offset: vec![],
        };

        assert!(logprobs.tokens.is_empty());
        assert!(logprobs.token_logprobs.is_empty());
    }

    #[test]
    fn test_completion_logprobs_with_top_logprobs() {
        let mut top = HashMap::new();
        top.insert("Hello".to_string(), -0.5);
        top.insert("Hi".to_string(), -1.2);

        let logprobs = CompletionLogprobs {
            tokens: vec!["Hello".to_string()],
            token_logprobs: vec![-0.5],
            top_logprobs: vec![top],
            text_offset: vec![0],
        };

        assert_eq!(logprobs.top_logprobs.len(), 1);
        assert_eq!(logprobs.top_logprobs[0].get("Hello"), Some(&-0.5));
    }

    #[test]
    fn test_completion_logprobs_clone() {
        let logprobs = CompletionLogprobs {
            tokens: vec!["test".to_string()],
            token_logprobs: vec![-0.1],
            top_logprobs: vec![],
            text_offset: vec![0],
        };

        let cloned = logprobs.clone();
        assert_eq!(logprobs.tokens, cloned.tokens);
        assert_eq!(logprobs.token_logprobs, cloned.token_logprobs);
    }

    #[test]
    fn test_completion_logprobs_serialization() {
        let logprobs = CompletionLogprobs {
            tokens: vec!["Hello".to_string()],
            token_logprobs: vec![-0.5],
            top_logprobs: vec![HashMap::new()],
            text_offset: vec![0],
        };

        let json = serde_json::to_value(&logprobs).unwrap();
        assert_eq!(json["tokens"][0], "Hello");
        assert_eq!(json["token_logprobs"][0], -0.5);
    }

    // ==================== CompletionChoice Tests ====================

    #[test]
    fn test_completion_choice_basic() {
        let choice = CompletionChoice {
            index: 0,
            text: "Hello, world!".to_string(),
            logprobs: None,
            finish_reason: Some("stop".to_string()),
        };

        assert_eq!(choice.index, 0);
        assert_eq!(choice.text, "Hello, world!");
        assert!(choice.logprobs.is_none());
        assert_eq!(choice.finish_reason, Some("stop".to_string()));
    }

    #[test]
    fn test_completion_choice_with_logprobs() {
        let logprobs = CompletionLogprobs {
            tokens: vec!["test".to_string()],
            token_logprobs: vec![-0.1],
            top_logprobs: vec![],
            text_offset: vec![0],
        };

        let choice = CompletionChoice {
            index: 0,
            text: "test".to_string(),
            logprobs: Some(logprobs),
            finish_reason: Some("stop".to_string()),
        };

        assert!(choice.logprobs.is_some());
    }

    #[test]
    fn test_completion_choice_no_finish_reason() {
        let choice = CompletionChoice {
            index: 0,
            text: "partial response...".to_string(),
            logprobs: None,
            finish_reason: None,
        };

        assert!(choice.finish_reason.is_none());
    }

    #[test]
    fn test_completion_choice_length_finish() {
        let choice = CompletionChoice {
            index: 0,
            text: "Long text...".to_string(),
            logprobs: None,
            finish_reason: Some("length".to_string()),
        };

        assert_eq!(choice.finish_reason, Some("length".to_string()));
    }

    #[test]
    fn test_completion_choice_clone() {
        let choice = CompletionChoice {
            index: 1,
            text: "clone test".to_string(),
            logprobs: None,
            finish_reason: Some("stop".to_string()),
        };

        let cloned = choice.clone();
        assert_eq!(choice.index, cloned.index);
        assert_eq!(choice.text, cloned.text);
        assert_eq!(choice.finish_reason, cloned.finish_reason);
    }

    #[test]
    fn test_completion_choice_serialization() {
        let choice = CompletionChoice {
            index: 0,
            text: "Hello".to_string(),
            logprobs: None,
            finish_reason: Some("stop".to_string()),
        };

        let json = serde_json::to_value(&choice).unwrap();
        assert_eq!(json["index"], 0);
        assert_eq!(json["text"], "Hello");
        assert_eq!(json["finish_reason"], "stop");
    }

    #[test]
    fn test_completion_choice_deserialization() {
        let json = r#"{
            "index": 0,
            "text": "Generated text",
            "logprobs": null,
            "finish_reason": "stop"
        }"#;

        let choice: CompletionChoice = serde_json::from_str(json).unwrap();
        assert_eq!(choice.index, 0);
        assert_eq!(choice.text, "Generated text");
        assert_eq!(choice.finish_reason, Some("stop".to_string()));
    }

    // ==================== CompletionResponse Tests ====================

    #[test]
    fn test_completion_response_basic() {
        let response = CompletionResponse {
            id: "cmpl-123".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "text-davinci-003".to_string(),
            choices: vec![],
            usage: None,
        };

        assert_eq!(response.id, "cmpl-123");
        assert_eq!(response.object, "text_completion");
        assert_eq!(response.created, 1234567890);
        assert_eq!(response.model, "text-davinci-003");
        assert!(response.choices.is_empty());
    }

    #[test]
    fn test_completion_response_with_choices() {
        let response = CompletionResponse {
            id: "cmpl-456".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "gpt-3.5-turbo-instruct".to_string(),
            choices: vec![
                CompletionChoice {
                    index: 0,
                    text: "Response 1".to_string(),
                    logprobs: None,
                    finish_reason: Some("stop".to_string()),
                },
                CompletionChoice {
                    index: 1,
                    text: "Response 2".to_string(),
                    logprobs: None,
                    finish_reason: Some("stop".to_string()),
                },
            ],
            usage: None,
        };

        assert_eq!(response.choices.len(), 2);
        assert_eq!(response.choices[0].text, "Response 1");
        assert_eq!(response.choices[1].text, "Response 2");
    }

    #[test]
    fn test_completion_response_with_usage() {
        let response = CompletionResponse {
            id: "cmpl-789".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "gpt-3.5-turbo-instruct".to_string(),
            choices: vec![],
            usage: Some(Usage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
                prompt_tokens_details: None,
                completion_tokens_details: None,
            }),
        };

        let usage = response.usage.unwrap();
        assert_eq!(usage.prompt_tokens, 10);
        assert_eq!(usage.completion_tokens, 20);
        assert_eq!(usage.total_tokens, 30);
    }

    #[test]
    fn test_completion_response_clone() {
        let response = CompletionResponse {
            id: "cmpl-clone".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "test-model".to_string(),
            choices: vec![],
            usage: None,
        };

        let cloned = response.clone();
        assert_eq!(response.id, cloned.id);
        assert_eq!(response.model, cloned.model);
    }

    #[test]
    fn test_completion_response_debug() {
        let response = CompletionResponse {
            id: "cmpl-debug".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "debug-model".to_string(),
            choices: vec![],
            usage: None,
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("CompletionResponse"));
        assert!(debug_str.contains("cmpl-debug"));
    }

    #[test]
    fn test_completion_response_serialization() {
        let response = CompletionResponse {
            id: "cmpl-ser".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "test-model".to_string(),
            choices: vec![CompletionChoice {
                index: 0,
                text: "Test".to_string(),
                logprobs: None,
                finish_reason: Some("stop".to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: 5,
                completion_tokens: 10,
                total_tokens: 15,
                prompt_tokens_details: None,
                completion_tokens_details: None,
            }),
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["id"], "cmpl-ser");
        assert_eq!(json["object"], "text_completion");
        assert_eq!(json["created"], 1234567890);
        assert_eq!(json["choices"][0]["text"], "Test");
        assert_eq!(json["usage"]["total_tokens"], 15);
    }

    #[test]
    fn test_completion_response_deserialization() {
        let json = r#"{
            "id": "cmpl-deser",
            "object": "text_completion",
            "created": 1234567890,
            "model": "test-model",
            "choices": [
                {
                    "index": 0,
                    "text": "Generated text",
                    "logprobs": null,
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        }"#;

        let response: CompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "cmpl-deser");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].text, "Generated text");
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_completion_choice_empty_text() {
        let choice = CompletionChoice {
            index: 0,
            text: "".to_string(),
            logprobs: None,
            finish_reason: Some("stop".to_string()),
        };

        assert!(choice.text.is_empty());
    }

    #[test]
    fn test_completion_response_large_created_timestamp() {
        let response = CompletionResponse {
            id: "cmpl-large".to_string(),
            object: "text_completion".to_string(),
            created: u64::MAX,
            model: "test".to_string(),
            choices: vec![],
            usage: None,
        };

        assert_eq!(response.created, u64::MAX);
    }

    #[test]
    fn test_completion_response_roundtrip() {
        let response = CompletionResponse {
            id: "cmpl-roundtrip".to_string(),
            object: "text_completion".to_string(),
            created: 1234567890,
            model: "gpt-3.5-turbo-instruct".to_string(),
            choices: vec![CompletionChoice {
                index: 0,
                text: "Hello, world!".to_string(),
                logprobs: None,
                finish_reason: Some("stop".to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: 5,
                completion_tokens: 3,
                total_tokens: 8,
                prompt_tokens_details: None,
                completion_tokens_details: None,
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: CompletionResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.id, deserialized.id);
        assert_eq!(response.model, deserialized.model);
        assert_eq!(response.choices.len(), deserialized.choices.len());
        assert_eq!(response.choices[0].text, deserialized.choices[0].text);
    }

    #[test]
    fn test_completion_choice_multiple_indices() {
        let choices: Vec<CompletionChoice> = (0..5)
            .map(|i| CompletionChoice {
                index: i,
                text: format!("Choice {}", i),
                logprobs: None,
                finish_reason: Some("stop".to_string()),
            })
            .collect();

        assert_eq!(choices.len(), 5);
        for (i, choice) in choices.iter().enumerate() {
            assert_eq!(choice.index, i as u32);
        }
    }
}
