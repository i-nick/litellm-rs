//! Databricks Models
//!
//! Model registry and information for Databricks Foundation Models.

use crate::core::types::common::ModelInfo;
use once_cell::sync::Lazy;

/// Databricks model registry
pub struct DatabricksModelRegistry {
    models: Vec<ModelInfo>,
}

impl DatabricksModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        let models = vec![
            // DBRX Models
            ModelInfo {
                id: "databricks/dbrx-instruct".to_string(),
                name: "DBRX Instruct".to_string(),
                description: Some("Databricks DBRX Instruct model".to_string()),
                context_length: 32768,
                max_output_tokens: 4096,
                supported_features: vec!["chat".to_string(), "streaming".to_string()],
                pricing: None,
            },
            // Llama Models on Databricks
            ModelInfo {
                id: "databricks/llama-2-70b-chat".to_string(),
                name: "Llama 2 70B Chat".to_string(),
                description: Some("Meta's Llama 2 70B Chat model on Databricks".to_string()),
                context_length: 4096,
                max_output_tokens: 4096,
                supported_features: vec!["chat".to_string(), "streaming".to_string()],
                pricing: None,
            },
            ModelInfo {
                id: "databricks/llama-3-70b-instruct".to_string(),
                name: "Llama 3 70B Instruct".to_string(),
                description: Some("Meta's Llama 3 70B Instruct model on Databricks".to_string()),
                context_length: 8192,
                max_output_tokens: 4096,
                supported_features: vec!["chat".to_string(), "streaming".to_string()],
                pricing: None,
            },
            ModelInfo {
                id: "databricks/llama-3.1-70b-instruct".to_string(),
                name: "Llama 3.1 70B Instruct".to_string(),
                description: Some("Meta's Llama 3.1 70B Instruct model on Databricks".to_string()),
                context_length: 128000,
                max_output_tokens: 4096,
                supported_features: vec!["chat".to_string(), "streaming".to_string()],
                pricing: None,
            },
            ModelInfo {
                id: "databricks/llama-3.1-405b-instruct".to_string(),
                name: "Llama 3.1 405B Instruct".to_string(),
                description: Some("Meta's Llama 3.1 405B Instruct model on Databricks".to_string()),
                context_length: 128000,
                max_output_tokens: 4096,
                supported_features: vec!["chat".to_string(), "streaming".to_string()],
                pricing: None,
            },
            // Mixtral Models on Databricks
            ModelInfo {
                id: "databricks/mixtral-8x7b-instruct".to_string(),
                name: "Mixtral 8x7B Instruct".to_string(),
                description: Some("Mistral's Mixtral 8x7B Instruct model on Databricks".to_string()),
                context_length: 32768,
                max_output_tokens: 4096,
                supported_features: vec!["chat".to_string(), "streaming".to_string()],
                pricing: None,
            },
            // Claude Models on Databricks
            ModelInfo {
                id: "databricks/claude-3-opus".to_string(),
                name: "Claude 3 Opus".to_string(),
                description: Some("Anthropic's Claude 3 Opus on Databricks".to_string()),
                context_length: 200000,
                max_output_tokens: 4096,
                supported_features: vec![
                    "chat".to_string(),
                    "streaming".to_string(),
                    "tools".to_string(),
                    "vision".to_string(),
                ],
                pricing: None,
            },
            ModelInfo {
                id: "databricks/claude-3-sonnet".to_string(),
                name: "Claude 3 Sonnet".to_string(),
                description: Some("Anthropic's Claude 3 Sonnet on Databricks".to_string()),
                context_length: 200000,
                max_output_tokens: 4096,
                supported_features: vec![
                    "chat".to_string(),
                    "streaming".to_string(),
                    "tools".to_string(),
                    "vision".to_string(),
                ],
                pricing: None,
            },
            ModelInfo {
                id: "databricks/claude-3.5-sonnet".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                description: Some("Anthropic's Claude 3.5 Sonnet on Databricks".to_string()),
                context_length: 200000,
                max_output_tokens: 8192,
                supported_features: vec![
                    "chat".to_string(),
                    "streaming".to_string(),
                    "tools".to_string(),
                    "vision".to_string(),
                ],
                pricing: None,
            },
            // Embedding Models
            ModelInfo {
                id: "databricks/bge-large-en".to_string(),
                name: "BGE Large English".to_string(),
                description: Some("BAAI BGE Large English embedding model".to_string()),
                context_length: 512,
                max_output_tokens: 0,
                supported_features: vec!["embeddings".to_string()],
                pricing: None,
            },
            ModelInfo {
                id: "databricks/gte-large-en".to_string(),
                name: "GTE Large English".to_string(),
                description: Some("GTE Large English embedding model".to_string()),
                context_length: 512,
                max_output_tokens: 0,
                supported_features: vec!["embeddings".to_string()],
                pricing: None,
            },
        ];

        Self { models }
    }

    /// Get all supported models
    pub fn models(&self) -> &[ModelInfo] {
        &self.models
    }

    /// Check if a model is an embedding model
    pub fn is_embedding_model(&self, model: &str) -> bool {
        let model_lower = model.to_lowercase();
        model_lower.contains("bge")
            || model_lower.contains("gte")
            || model_lower.contains("embedding")
            || model_lower.contains("e5")
    }

    /// Check if a model is a Claude model (for special handling)
    pub fn is_claude_model(&self, model: &str) -> bool {
        model.to_lowercase().contains("claude")
    }

    /// Check if model supports tool calling
    pub fn supports_tools(&self, model: &str) -> bool {
        self.models
            .iter()
            .find(|m| m.id == model || m.id.ends_with(model))
            .map(|m| m.supported_features.contains(&"tools".to_string()))
            .unwrap_or(false)
    }

    /// Check if model supports vision
    pub fn supports_vision(&self, model: &str) -> bool {
        self.models
            .iter()
            .find(|m| m.id == model || m.id.ends_with(model))
            .map(|m| m.supported_features.contains(&"vision".to_string()))
            .unwrap_or(false)
    }

    /// Get supported OpenAI parameters for a model
    pub fn get_supported_params(&self, model: &str) -> &'static [&'static str] {
        if self.is_claude_model(model) {
            &[
                "stream",
                "stop",
                "temperature",
                "top_p",
                "max_tokens",
                "max_completion_tokens",
                "n",
                "response_format",
                "tools",
                "tool_choice",
                "reasoning_effort",
                "thinking",
            ]
        } else {
            &[
                "stream",
                "stop",
                "temperature",
                "top_p",
                "top_k",
                "max_tokens",
                "max_completion_tokens",
                "n",
            ]
        }
    }

    /// Check if model is supported
    pub fn supports_model(&self, model: &str) -> bool {
        let model_lower = model.to_lowercase();
        self.models
            .iter()
            .any(|m| m.id.to_lowercase() == model_lower || m.id.ends_with(&model_lower))
    }
}

impl Default for DatabricksModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global model registry instance
pub static DATABRICKS_REGISTRY: Lazy<DatabricksModelRegistry> =
    Lazy::new(DatabricksModelRegistry::new);

/// Get the global Databricks model registry
pub fn get_databricks_registry() -> &'static DatabricksModelRegistry {
    &DATABRICKS_REGISTRY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_registry_creation() {
        let registry = DatabricksModelRegistry::new();
        assert!(!registry.models().is_empty());
    }

    #[test]
    fn test_is_embedding_model() {
        let registry = DatabricksModelRegistry::new();
        assert!(registry.is_embedding_model("bge-large-en"));
        assert!(registry.is_embedding_model("databricks/bge-large-en"));
        assert!(registry.is_embedding_model("gte-large-en"));
        assert!(!registry.is_embedding_model("llama-2-70b-chat"));
    }

    #[test]
    fn test_is_claude_model() {
        let registry = DatabricksModelRegistry::new();
        assert!(registry.is_claude_model("claude-3-opus"));
        assert!(registry.is_claude_model("databricks/claude-3.5-sonnet"));
        assert!(!registry.is_claude_model("llama-3-70b-instruct"));
    }

    #[test]
    fn test_supports_tools() {
        let registry = DatabricksModelRegistry::new();
        assert!(registry.supports_tools("databricks/claude-3-opus"));
        assert!(!registry.supports_tools("databricks/llama-2-70b-chat"));
    }

    #[test]
    fn test_supports_vision() {
        let registry = DatabricksModelRegistry::new();
        assert!(registry.supports_vision("databricks/claude-3-opus"));
        assert!(!registry.supports_vision("databricks/dbrx-instruct"));
    }

    #[test]
    fn test_get_supported_params_claude() {
        let registry = DatabricksModelRegistry::new();
        let params = registry.get_supported_params("claude-3-opus");
        assert!(params.contains(&"tools"));
        assert!(params.contains(&"thinking"));
    }

    #[test]
    fn test_get_supported_params_non_claude() {
        let registry = DatabricksModelRegistry::new();
        let params = registry.get_supported_params("llama-3-70b-instruct");
        assert!(params.contains(&"top_k"));
        assert!(!params.contains(&"tools"));
    }

    #[test]
    fn test_supports_model() {
        let registry = DatabricksModelRegistry::new();
        assert!(registry.supports_model("databricks/dbrx-instruct"));
        assert!(registry.supports_model("dbrx-instruct")); // Without prefix
    }

    #[test]
    fn test_global_registry() {
        let registry = get_databricks_registry();
        assert!(!registry.models().is_empty());
    }
}
