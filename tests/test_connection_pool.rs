//! Integration test for connection pool and basic functionality

use litellm_rs::core::providers::base::{BaseConfig, BaseHttpClient};

#[tokio::test]
async fn test_http_client_creation() {
    let config = BaseConfig::default();
    let client = BaseHttpClient::new(config);
    assert!(client.is_ok(), "Should create HTTP client successfully");
}

#[tokio::test]
async fn test_base_config_defaults() {
    let config = BaseConfig::default();
    assert_eq!(config.timeout, 60);
    assert_eq!(config.max_retries, 3);
    assert!(config.api_key.is_none());
    assert!(config.api_base.is_none());
}

#[tokio::test]
async fn test_config_with_values() {
    let config = BaseConfig {
        api_key: Some("test-key".to_string()),
        api_base: Some("https://api.example.com".to_string()),
        timeout: 30,
        max_retries: 5,
        ..Default::default()
    };

    assert_eq!(config.api_key, Some("test-key".to_string()));
    assert_eq!(config.api_base, Some("https://api.example.com".to_string()));
    assert_eq!(config.timeout, 30);
    assert_eq!(config.max_retries, 5);
}
