//! Embedding response types

use serde::{Deserialize, Serialize};

use super::usage::Usage;

/// Embedding response (simple format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedResponse {
    /// Object type
    pub object: String,

    /// Embedding data list
    pub data: Vec<EmbeddingData>,

    /// Model used
    pub model: String,

    /// Usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<EmbeddingUsage>,
}

/// Embedding data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingData {
    /// Object type
    pub object: String,

    /// Index
    pub index: u32,

    /// Embedding vector
    pub embedding: Vec<f32>,
}

/// Embedding usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    /// Prompt token count
    pub prompt_tokens: u32,

    /// Total token count
    pub total_tokens: u32,
}

/// Embedding response (full format with backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Object type
    pub object: String,

    /// Embedding data list
    pub data: Vec<EmbeddingData>,

    /// Model used
    pub model: String,

    /// Usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,

    /// Embedding data list (backward compatibility field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<EmbeddingData>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== EmbedResponse Tests ====================

    #[test]
    fn test_embed_response_creation() {
        let response = EmbedResponse {
            object: "list".to_string(),
            data: vec![],
            model: "text-embedding-ada-002".to_string(),
            usage: None,
        };
        assert_eq!(response.object, "list");
        assert_eq!(response.model, "text-embedding-ada-002");
    }

    #[test]
    fn test_embed_response_with_data() {
        let response = EmbedResponse {
            object: "list".to_string(),
            data: vec![EmbeddingData {
                object: "embedding".to_string(),
                index: 0,
                embedding: vec![0.1, 0.2, 0.3],
            }],
            model: "text-embedding-3-small".to_string(),
            usage: Some(EmbeddingUsage {
                prompt_tokens: 5,
                total_tokens: 5,
            }),
        };
        assert_eq!(response.data.len(), 1);
        assert!(response.usage.is_some());
    }

    #[test]
    fn test_embed_response_serialization() {
        let response = EmbedResponse {
            object: "list".to_string(),
            data: vec![EmbeddingData {
                object: "embedding".to_string(),
                index: 0,
                embedding: vec![0.5],
            }],
            model: "model".to_string(),
            usage: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("list"));
        assert!(json.contains("embedding"));
        assert!(!json.contains("usage"));
    }

    #[test]
    fn test_embed_response_deserialization() {
        let json = r#"{
            "object": "list",
            "data": [{"object": "embedding", "index": 0, "embedding": [0.1, 0.2]}],
            "model": "text-embedding-ada-002",
            "usage": {"prompt_tokens": 10, "total_tokens": 10}
        }"#;
        let response: EmbedResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert!(response.usage.is_some());
        assert_eq!(response.usage.unwrap().prompt_tokens, 10);
    }

    // ==================== EmbeddingData Tests ====================

    #[test]
    fn test_embedding_data_creation() {
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        };
        assert_eq!(data.object, "embedding");
        assert_eq!(data.index, 0);
        assert_eq!(data.embedding.len(), 5);
    }

    #[test]
    fn test_embedding_data_serialization() {
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 1,
            embedding: vec![0.5, -0.5],
        };
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("embedding"));
        assert!(json.contains("1"));
        assert!(json.contains("0.5"));
    }

    #[test]
    fn test_embedding_data_deserialization() {
        let json = r#"{"object": "embedding", "index": 2, "embedding": [0.1, 0.2, 0.3]}"#;
        let data: EmbeddingData = serde_json::from_str(json).unwrap();
        assert_eq!(data.index, 2);
        assert_eq!(data.embedding.len(), 3);
    }

    #[test]
    fn test_embedding_data_empty_vector() {
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: vec![],
        };
        assert!(data.embedding.is_empty());
    }

    #[test]
    fn test_embedding_data_large_vector() {
        let large_embedding: Vec<f32> = (0..1536).map(|i| i as f32 * 0.001).collect();
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: large_embedding.clone(),
        };
        assert_eq!(data.embedding.len(), 1536);
    }

    // ==================== EmbeddingUsage Tests ====================

    #[test]
    fn test_embedding_usage_creation() {
        let usage = EmbeddingUsage {
            prompt_tokens: 100,
            total_tokens: 100,
        };
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.total_tokens, 100);
    }

    #[test]
    fn test_embedding_usage_serialization() {
        let usage = EmbeddingUsage {
            prompt_tokens: 50,
            total_tokens: 50,
        };
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("50"));
        assert!(json.contains("prompt_tokens"));
        assert!(json.contains("total_tokens"));
    }

    #[test]
    fn test_embedding_usage_deserialization() {
        let json = r#"{"prompt_tokens": 25, "total_tokens": 25}"#;
        let usage: EmbeddingUsage = serde_json::from_str(json).unwrap();
        assert_eq!(usage.prompt_tokens, 25);
        assert_eq!(usage.total_tokens, 25);
    }

    // ==================== EmbeddingResponse Tests ====================

    #[test]
    fn test_embedding_response_creation() {
        let response = EmbeddingResponse {
            object: "list".to_string(),
            data: vec![],
            model: "text-embedding-3-large".to_string(),
            usage: None,
            embeddings: None,
        };
        assert_eq!(response.object, "list");
        assert!(response.embeddings.is_none());
    }

    #[test]
    fn test_embedding_response_with_backward_compat() {
        let data = vec![EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: vec![0.1],
        }];
        let response = EmbeddingResponse {
            object: "list".to_string(),
            data: data.clone(),
            model: "model".to_string(),
            usage: None,
            embeddings: Some(data),
        };
        assert!(response.embeddings.is_some());
        assert_eq!(response.embeddings.unwrap().len(), 1);
    }

    #[test]
    fn test_embedding_response_serialization() {
        let response = EmbeddingResponse {
            object: "list".to_string(),
            data: vec![],
            model: "model".to_string(),
            usage: None,
            embeddings: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(!json.contains("usage"));
        assert!(!json.contains("embeddings"));
    }

    // ==================== Clone and Debug Tests ====================

    #[test]
    fn test_embed_response_clone() {
        let response = EmbedResponse {
            object: "list".to_string(),
            data: vec![EmbeddingData {
                object: "embedding".to_string(),
                index: 0,
                embedding: vec![0.1],
            }],
            model: "model".to_string(),
            usage: None,
        };
        let cloned = response.clone();
        assert_eq!(cloned.data.len(), 1);
    }

    #[test]
    fn test_embedding_data_clone() {
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: vec![0.1, 0.2],
        };
        let cloned = data.clone();
        assert_eq!(cloned.embedding.len(), 2);
    }

    #[test]
    fn test_embedding_usage_clone() {
        let usage = EmbeddingUsage {
            prompt_tokens: 10,
            total_tokens: 10,
        };
        let cloned = usage.clone();
        assert_eq!(cloned.prompt_tokens, 10);
    }

    #[test]
    fn test_embedding_data_debug() {
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: vec![0.1],
        };
        let debug = format!("{:?}", data);
        assert!(debug.contains("EmbeddingData"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_embedding_usage_zero_tokens() {
        let usage = EmbeddingUsage {
            prompt_tokens: 0,
            total_tokens: 0,
        };
        assert_eq!(usage.prompt_tokens, 0);
        assert_eq!(usage.total_tokens, 0);
    }

    #[test]
    fn test_embedding_data_negative_values() {
        let data = EmbeddingData {
            object: "embedding".to_string(),
            index: 0,
            embedding: vec![-0.5, -0.25, 0.0, 0.25, 0.5],
        };
        assert!(data.embedding.iter().any(|&v| v < 0.0));
    }

    #[test]
    fn test_embed_response_multiple_embeddings() {
        let response = EmbedResponse {
            object: "list".to_string(),
            data: vec![
                EmbeddingData {
                    object: "embedding".to_string(),
                    index: 0,
                    embedding: vec![0.1],
                },
                EmbeddingData {
                    object: "embedding".to_string(),
                    index: 1,
                    embedding: vec![0.2],
                },
                EmbeddingData {
                    object: "embedding".to_string(),
                    index: 2,
                    embedding: vec![0.3],
                },
            ],
            model: "model".to_string(),
            usage: Some(EmbeddingUsage {
                prompt_tokens: 15,
                total_tokens: 15,
            }),
        };
        assert_eq!(response.data.len(), 3);
        assert_eq!(response.data[2].index, 2);
    }
}
