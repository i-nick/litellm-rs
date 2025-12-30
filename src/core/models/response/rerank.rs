//! Rerank response types

use serde::{Deserialize, Serialize};

/// Rerank response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankResponse {
    /// Response ID
    pub id: String,
    /// Model used
    pub model: String,
    /// Reranked results
    pub results: Vec<RerankResult>,
    /// Usage statistics
    pub usage: Option<RerankUsage>,
}

/// Rerank result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankResult {
    /// Document index
    pub index: u32,
    /// Relevance score
    pub relevance_score: f64,
    /// Document text (if requested)
    pub document: Option<String>,
}

/// Rerank usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankUsage {
    /// Total tokens
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== RerankUsage Tests ====================

    #[test]
    fn test_rerank_usage_creation() {
        let usage = RerankUsage { total_tokens: 100 };
        assert_eq!(usage.total_tokens, 100);
    }

    #[test]
    fn test_rerank_usage_serialization() {
        let usage = RerankUsage { total_tokens: 500 };
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("\"total_tokens\":500"));
    }

    #[test]
    fn test_rerank_usage_deserialization() {
        let json = r#"{"total_tokens":250}"#;
        let usage: RerankUsage = serde_json::from_str(json).unwrap();
        assert_eq!(usage.total_tokens, 250);
    }

    #[test]
    fn test_rerank_usage_clone() {
        let usage = RerankUsage { total_tokens: 1000 };
        let cloned = usage.clone();
        assert_eq!(cloned.total_tokens, usage.total_tokens);
    }

    #[test]
    fn test_rerank_usage_debug() {
        let usage = RerankUsage { total_tokens: 42 };
        let debug_str = format!("{:?}", usage);
        assert!(debug_str.contains("RerankUsage"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_rerank_usage_zero_tokens() {
        let usage = RerankUsage { total_tokens: 0 };
        assert_eq!(usage.total_tokens, 0);
    }

    #[test]
    fn test_rerank_usage_large_tokens() {
        let usage = RerankUsage {
            total_tokens: u32::MAX,
        };
        assert_eq!(usage.total_tokens, u32::MAX);
    }

    // ==================== RerankResult Tests ====================

    #[test]
    fn test_rerank_result_creation() {
        let result = RerankResult {
            index: 0,
            relevance_score: 0.95,
            document: Some("Test document".to_string()),
        };
        assert_eq!(result.index, 0);
        assert!((result.relevance_score - 0.95).abs() < f64::EPSILON);
        assert_eq!(result.document, Some("Test document".to_string()));
    }

    #[test]
    fn test_rerank_result_without_document() {
        let result = RerankResult {
            index: 1,
            relevance_score: 0.85,
            document: None,
        };
        assert!(result.document.is_none());
    }

    #[test]
    fn test_rerank_result_serialization() {
        let result = RerankResult {
            index: 2,
            relevance_score: 0.75,
            document: Some("Doc text".to_string()),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"index\":2"));
        assert!(json.contains("\"relevance_score\":0.75"));
        assert!(json.contains("\"document\":\"Doc text\""));
    }

    #[test]
    fn test_rerank_result_deserialization() {
        let json = r#"{"index":3,"relevance_score":0.9,"document":"Sample"}"#;
        let result: RerankResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.index, 3);
        assert!((result.relevance_score - 0.9).abs() < f64::EPSILON);
        assert_eq!(result.document, Some("Sample".to_string()));
    }

    #[test]
    fn test_rerank_result_deserialization_null_document() {
        let json = r#"{"index":0,"relevance_score":0.5,"document":null}"#;
        let result: RerankResult = serde_json::from_str(json).unwrap();
        assert!(result.document.is_none());
    }

    #[test]
    fn test_rerank_result_clone() {
        let result = RerankResult {
            index: 5,
            relevance_score: 0.88,
            document: Some("Clone me".to_string()),
        };
        let cloned = result.clone();
        assert_eq!(cloned.index, result.index);
        assert_eq!(cloned.relevance_score, result.relevance_score);
        assert_eq!(cloned.document, result.document);
    }

    #[test]
    fn test_rerank_result_debug() {
        let result = RerankResult {
            index: 10,
            relevance_score: 0.99,
            document: None,
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("RerankResult"));
        assert!(debug_str.contains("10"));
        assert!(debug_str.contains("0.99"));
    }

    #[test]
    fn test_rerank_result_score_boundaries() {
        // Test minimum score
        let min_result = RerankResult {
            index: 0,
            relevance_score: 0.0,
            document: None,
        };
        assert_eq!(min_result.relevance_score, 0.0);

        // Test maximum score
        let max_result = RerankResult {
            index: 0,
            relevance_score: 1.0,
            document: None,
        };
        assert_eq!(max_result.relevance_score, 1.0);
    }

    // ==================== RerankResponse Tests ====================

    #[test]
    fn test_rerank_response_creation() {
        let response = RerankResponse {
            id: "rerank-123".to_string(),
            model: "rerank-english-v2.0".to_string(),
            results: vec![],
            usage: None,
        };
        assert_eq!(response.id, "rerank-123");
        assert_eq!(response.model, "rerank-english-v2.0");
        assert!(response.results.is_empty());
        assert!(response.usage.is_none());
    }

    #[test]
    fn test_rerank_response_with_results() {
        let response = RerankResponse {
            id: "rerank-456".to_string(),
            model: "rerank-multilingual-v2.0".to_string(),
            results: vec![
                RerankResult {
                    index: 2,
                    relevance_score: 0.95,
                    document: None,
                },
                RerankResult {
                    index: 0,
                    relevance_score: 0.88,
                    document: None,
                },
                RerankResult {
                    index: 1,
                    relevance_score: 0.72,
                    document: None,
                },
            ],
            usage: Some(RerankUsage { total_tokens: 150 }),
        };

        assert_eq!(response.results.len(), 3);
        // Results should be in order of relevance (highest first)
        assert!(response.results[0].relevance_score > response.results[1].relevance_score);
        assert!(response.results[1].relevance_score > response.results[2].relevance_score);
        assert!(response.usage.is_some());
    }

    #[test]
    fn test_rerank_response_serialization() {
        let response = RerankResponse {
            id: "test-id".to_string(),
            model: "test-model".to_string(),
            results: vec![RerankResult {
                index: 0,
                relevance_score: 0.9,
                document: Some("doc".to_string()),
            }],
            usage: Some(RerankUsage { total_tokens: 50 }),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"id\":\"test-id\""));
        assert!(json.contains("\"model\":\"test-model\""));
        assert!(json.contains("\"results\""));
        assert!(json.contains("\"usage\""));
    }

    #[test]
    fn test_rerank_response_deserialization() {
        let json = r#"{
            "id": "rerank-789",
            "model": "rerank-model",
            "results": [
                {"index": 0, "relevance_score": 0.85, "document": null}
            ],
            "usage": {"total_tokens": 100}
        }"#;

        let response: RerankResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "rerank-789");
        assert_eq!(response.model, "rerank-model");
        assert_eq!(response.results.len(), 1);
        assert!(response.usage.is_some());
        assert_eq!(response.usage.unwrap().total_tokens, 100);
    }

    #[test]
    fn test_rerank_response_without_usage() {
        let json = r#"{
            "id": "no-usage",
            "model": "model",
            "results": [],
            "usage": null
        }"#;

        let response: RerankResponse = serde_json::from_str(json).unwrap();
        assert!(response.usage.is_none());
    }

    #[test]
    fn test_rerank_response_clone() {
        let response = RerankResponse {
            id: "clone-test".to_string(),
            model: "clone-model".to_string(),
            results: vec![RerankResult {
                index: 0,
                relevance_score: 0.5,
                document: None,
            }],
            usage: Some(RerankUsage { total_tokens: 25 }),
        };

        let cloned = response.clone();
        assert_eq!(cloned.id, response.id);
        assert_eq!(cloned.model, response.model);
        assert_eq!(cloned.results.len(), response.results.len());
    }

    #[test]
    fn test_rerank_response_debug() {
        let response = RerankResponse {
            id: "debug-id".to_string(),
            model: "debug-model".to_string(),
            results: vec![],
            usage: None,
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("RerankResponse"));
        assert!(debug_str.contains("debug-id"));
        assert!(debug_str.contains("debug-model"));
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_rerank_workflow() {
        // Simulate a rerank response with multiple documents
        let response = RerankResponse {
            id: "workflow-test".to_string(),
            model: "rerank-english-v2.0".to_string(),
            results: vec![
                RerankResult {
                    index: 3,
                    relevance_score: 0.98,
                    document: Some("Most relevant document".to_string()),
                },
                RerankResult {
                    index: 1,
                    relevance_score: 0.85,
                    document: Some("Second most relevant".to_string()),
                },
                RerankResult {
                    index: 0,
                    relevance_score: 0.72,
                    document: Some("Third in ranking".to_string()),
                },
                RerankResult {
                    index: 2,
                    relevance_score: 0.45,
                    document: Some("Least relevant".to_string()),
                },
            ],
            usage: Some(RerankUsage { total_tokens: 200 }),
        };

        // The original document at index 3 is most relevant
        assert_eq!(response.results[0].index, 3);
        assert!(response.results[0].relevance_score > 0.9);

        // Can get top-k results
        let top_2: Vec<_> = response.results.iter().take(2).collect();
        assert_eq!(top_2.len(), 2);
        assert!(top_2[0].relevance_score >= top_2[1].relevance_score);
    }

    #[test]
    fn test_rerank_json_roundtrip() {
        let original = RerankResponse {
            id: "roundtrip".to_string(),
            model: "test-model".to_string(),
            results: vec![
                RerankResult {
                    index: 0,
                    relevance_score: 0.9,
                    document: Some("Doc 0".to_string()),
                },
                RerankResult {
                    index: 1,
                    relevance_score: 0.8,
                    document: None,
                },
            ],
            usage: Some(RerankUsage { total_tokens: 75 }),
        };

        let json = serde_json::to_string(&original).unwrap();
        let parsed: RerankResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.model, original.model);
        assert_eq!(parsed.results.len(), original.results.len());
        assert_eq!(
            parsed.usage.as_ref().unwrap().total_tokens,
            original.usage.as_ref().unwrap().total_tokens
        );
    }
}
