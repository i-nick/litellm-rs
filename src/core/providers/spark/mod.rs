//! iFlytek Spark Provider
//!
//! iFlytek Spark (讯飞星火) LLM platform integration
//! Note: Spark uses WebSocket-based API with HMAC authentication

pub mod config;
pub mod model_info;
pub mod provider;

// Re-export core components
pub use config::{SparkConfig, SparkConfigBuilder};
pub use model_info::{ModelFeature, ModelSpec, SparkModelRegistry, get_spark_registry};
pub use provider::{SparkProvider, SparkProviderBuilder};

// Type aliases
pub type Error = crate::core::providers::unified_provider::ProviderError;
pub type Config = config::SparkConfig;
pub type Provider = provider::SparkProvider;

/// Provider name constant
pub const PROVIDER_NAME: &str = "spark";

/// API base URL for Spark
pub const DEFAULT_API_BASE: &str = "https://spark-api.xf-yun.com";

/// WebSocket endpoint versions
pub const WS_V3_5_URL: &str = "wss://spark-api.xf-yun.com/v3.5/chat";
pub const WS_V3_URL: &str = "wss://spark-api.xf-yun.com/v3.1/chat";
pub const WS_V2_URL: &str = "wss://spark-api.xf-yun.com/v2.1/chat";
pub const WS_V1_5_URL: &str = "wss://spark-api.xf-yun.com/v1.1/chat";

/// Create a new Spark provider
pub fn new_provider(
    app_id: impl Into<String>,
    api_key: impl Into<String>,
    api_secret: impl Into<String>,
) -> Result<SparkProvider, Error> {
    let config = SparkConfig::new(app_id, api_key, api_secret);
    SparkProvider::new(config)
}

/// Create provider from environment
pub fn new_provider_from_env() -> Result<SparkProvider, Error> {
    let config = SparkConfig::from_env()?;
    SparkProvider::new(config)
}

/// Create provider builder
pub fn builder() -> SparkProviderBuilder {
    SparkProviderBuilder::new()
}

/// Default model
pub fn default_model() -> &'static str {
    "spark-desk-v3.5"
}

/// List supported models
pub fn supported_models() -> Vec<String> {
    get_spark_registry()
        .list_models()
        .into_iter()
        .map(|spec| spec.model_info.id.clone())
        .collect()
}

/// Check if model is supported
pub fn is_model_supported(model_id: &str) -> bool {
    get_spark_registry().get_model_spec(model_id).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_name() {
        assert_eq!(PROVIDER_NAME, "spark");
    }

    #[test]
    fn test_default_model() {
        assert_eq!(default_model(), "spark-desk-v3.5");
    }

    #[test]
    fn test_supported_models() {
        let models = supported_models();
        assert!(!models.is_empty());
        assert!(models.contains(&"spark-desk-v3.5".to_string()));
        assert!(models.contains(&"spark-desk-v3".to_string()));
        assert!(models.contains(&"spark-desk-v2".to_string()));
        assert!(models.contains(&"spark-desk-v1.5".to_string()));
    }

    #[test]
    fn test_model_support() {
        assert!(is_model_supported("spark-desk-v3.5"));
        assert!(is_model_supported("spark-desk-v3"));
        assert!(!is_model_supported("gpt-4"));
    }
}
