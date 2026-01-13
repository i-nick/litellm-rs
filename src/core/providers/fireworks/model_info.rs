//! Fireworks AI Model Information
//!
//! Static model information for Fireworks AI models including capabilities,
//! context lengths, and pricing.

/// Fireworks AI model information
#[derive(Debug, Clone)]
pub struct FireworksModel {
    pub model_id: &'static str,
    pub display_name: &'static str,
    pub max_context_length: u32,
    pub max_output_length: u32,
    pub supports_tools: bool,
    pub supports_multimodal: bool,
    pub supports_reasoning: bool,
    pub input_cost_per_million: f64,
    pub output_cost_per_million: f64,
}

/// Static model registry for Fireworks AI
const FIREWORKS_MODELS: &[FireworksModel] = &[
    // Llama 3.3 models
    FireworksModel {
        model_id: "accounts/fireworks/models/llama-v3p3-70b-instruct",
        display_name: "Llama 3.3 70B Instruct",
        max_context_length: 131072,
        max_output_length: 16384,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
    // Llama 3.2 Vision models
    FireworksModel {
        model_id: "accounts/fireworks/models/llama-v3p2-11b-vision-instruct",
        display_name: "Llama 3.2 11B Vision",
        max_context_length: 131072,
        max_output_length: 16384,
        supports_tools: true,
        supports_multimodal: true,
        supports_reasoning: false,
        input_cost_per_million: 0.2,
        output_cost_per_million: 0.2,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/llama-v3p2-90b-vision-instruct",
        display_name: "Llama 3.2 90B Vision",
        max_context_length: 131072,
        max_output_length: 16384,
        supports_tools: true,
        supports_multimodal: true,
        supports_reasoning: false,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
    // Llama 3.1 models
    FireworksModel {
        model_id: "accounts/fireworks/models/llama-v3p1-8b-instruct",
        display_name: "Llama 3.1 8B Instruct",
        max_context_length: 131072,
        max_output_length: 16384,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.2,
        output_cost_per_million: 0.2,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/llama-v3p1-70b-instruct",
        display_name: "Llama 3.1 70B Instruct",
        max_context_length: 131072,
        max_output_length: 16384,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/llama-v3p1-405b-instruct",
        display_name: "Llama 3.1 405B Instruct",
        max_context_length: 131072,
        max_output_length: 16384,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 3.0,
        output_cost_per_million: 3.0,
    },
    // Qwen models with reasoning support
    FireworksModel {
        model_id: "accounts/fireworks/models/qwen3-8b",
        display_name: "Qwen3 8B",
        max_context_length: 32768,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: true,
        input_cost_per_million: 0.2,
        output_cost_per_million: 0.2,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/qwen3-32b",
        display_name: "Qwen3 32B",
        max_context_length: 32768,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: true,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/qwen2p5-coder-32b-instruct",
        display_name: "Qwen2.5 Coder 32B",
        max_context_length: 32768,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
    // DeepSeek models with reasoning
    FireworksModel {
        model_id: "accounts/fireworks/models/deepseek-v3",
        display_name: "DeepSeek V3",
        max_context_length: 65536,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: true,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/deepseek-r1",
        display_name: "DeepSeek R1",
        max_context_length: 65536,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: true,
        input_cost_per_million: 2.0,
        output_cost_per_million: 8.0,
    },
    // Mixtral models
    FireworksModel {
        model_id: "accounts/fireworks/models/mixtral-8x7b-instruct",
        display_name: "Mixtral 8x7B Instruct",
        max_context_length: 32768,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.5,
        output_cost_per_million: 0.5,
    },
    FireworksModel {
        model_id: "accounts/fireworks/models/mixtral-8x22b-instruct",
        display_name: "Mixtral 8x22B Instruct",
        max_context_length: 65536,
        max_output_length: 8192,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 1.2,
        output_cost_per_million: 1.2,
    },
    // Gemma models
    FireworksModel {
        model_id: "accounts/fireworks/models/gemma2-9b-it",
        display_name: "Gemma 2 9B",
        max_context_length: 8192,
        max_output_length: 4096,
        supports_tools: false,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.2,
        output_cost_per_million: 0.2,
    },
    // Firefunction model (optimized for function calling)
    FireworksModel {
        model_id: "accounts/fireworks/models/firefunction-v2",
        display_name: "FireFunction V2",
        max_context_length: 8192,
        max_output_length: 4096,
        supports_tools: true,
        supports_multimodal: false,
        supports_reasoning: false,
        input_cost_per_million: 0.9,
        output_cost_per_million: 0.9,
    },
];

/// Get available model IDs
pub fn get_available_models() -> Vec<&'static str> {
    FIREWORKS_MODELS.iter().map(|m| m.model_id).collect()
}

/// Get model information by ID
pub fn get_model_info(model_id: &str) -> Option<&'static FireworksModel> {
    // Normalize model ID by removing common prefixes
    let normalized = normalize_model_id(model_id);

    FIREWORKS_MODELS.iter().find(|m| {
        m.model_id == normalized
            || m.model_id.ends_with(&format!("/{}", normalized))
            || normalize_model_id(m.model_id) == normalized
    })
}

/// Normalize model ID by removing prefixes
fn normalize_model_id(model_id: &str) -> String {
    let id = model_id
        .trim_start_matches("fireworks_ai/")
        .trim_start_matches("fireworks/")
        .trim_start_matches("accounts/fireworks/models/");
    id.to_string()
}

/// Check if model supports reasoning
pub fn is_reasoning_model(model_id: &str) -> bool {
    if let Some(model) = get_model_info(model_id) {
        return model.supports_reasoning;
    }

    // Check by model name patterns for models not in registry
    let normalized = normalize_model_id(model_id);
    let reasoning_patterns = [
        "qwen3-8b",
        "qwen3-32b",
        "qwen3-coder-480b",
        "deepseek-v3",
        "deepseek-r1",
        "glm-4p5",
        "glm-4p6",
        "gpt-oss",
    ];

    reasoning_patterns
        .iter()
        .any(|pattern| normalized.contains(pattern))
}

/// Check if model supports function calling
pub fn supports_function_calling(model_id: &str) -> bool {
    if let Some(model) = get_model_info(model_id) {
        return model.supports_tools;
    }
    // Default to true for most Fireworks models
    true
}

/// Check if model supports tool choice
pub fn supports_tool_choice(model_id: &str) -> bool {
    // Fireworks AI supports tool_choice for models that support function calling
    supports_function_calling(model_id)
}

/// Format model ID to Fireworks AI format
pub fn format_model_id(model_id: &str) -> String {
    if model_id.starts_with("accounts/") {
        model_id.to_string()
    } else if model_id.contains('#') {
        // Custom model with hash - don't modify
        model_id.to_string()
    } else {
        format!("accounts/fireworks/models/{}", normalize_model_id(model_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_models() {
        let models = get_available_models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.contains("llama")));
    }

    #[test]
    fn test_get_model_info() {
        let model = get_model_info("accounts/fireworks/models/llama-v3p1-70b-instruct").unwrap();
        assert_eq!(model.display_name, "Llama 3.1 70B Instruct");
        assert!(model.supports_tools);
    }

    #[test]
    fn test_get_model_info_with_prefix() {
        let model = get_model_info("fireworks_ai/llama-v3p1-70b-instruct");
        assert!(model.is_some());
    }

    #[test]
    fn test_get_model_info_short_name() {
        let model = get_model_info("llama-v3p1-70b-instruct");
        assert!(model.is_some());
    }

    #[test]
    fn test_is_reasoning_model() {
        assert!(is_reasoning_model("qwen3-8b"));
        assert!(is_reasoning_model("accounts/fireworks/models/deepseek-v3"));
        assert!(!is_reasoning_model("llama-v3p1-70b-instruct"));
    }

    #[test]
    fn test_supports_function_calling() {
        assert!(supports_function_calling("llama-v3p1-70b-instruct"));
        assert!(supports_function_calling("firefunction-v2"));
    }

    #[test]
    fn test_format_model_id() {
        assert_eq!(
            format_model_id("llama-v3p1-70b-instruct"),
            "accounts/fireworks/models/llama-v3p1-70b-instruct"
        );
        assert_eq!(
            format_model_id("accounts/fireworks/models/llama-v3p1-70b-instruct"),
            "accounts/fireworks/models/llama-v3p1-70b-instruct"
        );
        assert_eq!(format_model_id("my-custom-model#v1"), "my-custom-model#v1");
    }

    #[test]
    fn test_normalize_model_id() {
        assert_eq!(
            normalize_model_id("fireworks_ai/llama-v3p1-70b-instruct"),
            "llama-v3p1-70b-instruct"
        );
        assert_eq!(
            normalize_model_id("accounts/fireworks/models/llama-v3p1-70b-instruct"),
            "llama-v3p1-70b-instruct"
        );
    }

    #[test]
    fn test_vision_models() {
        let model =
            get_model_info("accounts/fireworks/models/llama-v3p2-11b-vision-instruct").unwrap();
        assert!(model.supports_multimodal);
        assert!(model.supports_tools);
    }

    #[test]
    fn test_model_pricing() {
        let model = get_model_info("accounts/fireworks/models/llama-v3p1-405b-instruct").unwrap();
        assert_eq!(model.input_cost_per_million, 3.0);
        assert_eq!(model.output_cost_per_million, 3.0);
    }
}
