//! OVHcloud AI Endpoints Model Information

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OvhcloudModel {
    Llama2_13B_Chat,
    Mistral_7B_Instruct,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub model_id: &'static str,
    pub display_name: &'static str,
    pub max_context_length: u32,
    pub max_output_length: u32,
    pub supports_tools: bool,
    pub supports_multimodal: bool,
    pub input_cost_per_million: f64,
    pub output_cost_per_million: f64,
}

static MODEL_CONFIGS: LazyLock<HashMap<&'static str, ModelInfo>> = LazyLock::new(|| {
    let mut configs = HashMap::new();

    configs.insert(
        "llama-2-13b-chat",
        ModelInfo {
            model_id: "llama-2-13b-chat",
            display_name: "Llama 2 13B Chat",
            max_context_length: 4096,
            max_output_length: 2048,
            supports_tools: false,
            supports_multimodal: false,
            input_cost_per_million: 0.0,
            output_cost_per_million: 0.0,
        },
    );

    configs.insert(
        "mistral-7b-instruct",
        ModelInfo {
            model_id: "mistral-7b-instruct",
            display_name: "Mistral 7B Instruct",
            max_context_length: 8192,
            max_output_length: 4096,
            supports_tools: true,
            supports_multimodal: false,
            input_cost_per_million: 0.0,
            output_cost_per_million: 0.0,
        },
    );

    configs
});

pub fn get_model_info(model_id: &str) -> Option<&'static ModelInfo> {
    MODEL_CONFIGS.get(model_id)
}

pub fn get_available_models() -> Vec<&'static str> {
    MODEL_CONFIGS.keys().copied().collect()
}
