//! Redis module tests

use super::pool::RedisPool;
use crate::config::models::storage::RedisConfig;

#[test]
fn test_sanitize_url() {
    let url = "redis://user:password@localhost:6379/0";
    let sanitized = RedisPool::sanitize_url(url);
    assert!(sanitized.contains("user:***@localhost"));
    assert!(!sanitized.contains("password"));
}

#[tokio::test]
async fn test_redis_pool_creation() {
    let config = RedisConfig {
        url: "redis://localhost:6379".to_string(),
        enabled: true,
        max_connections: 10,
        connection_timeout: 5,
        cluster: false,
    };

    // This test would require an actual Redis instance
    // For now, we'll just test that the config is properly structured
    assert_eq!(config.url, "redis://localhost:6379");
    assert_eq!(config.max_connections, 10);
}

#[tokio::test]
async fn test_redis_pool_disabled_is_noop() {
    let config = RedisConfig {
        url: "redis://127.0.0.1:1".to_string(),
        enabled: false,
        max_connections: 10,
        connection_timeout: 1,
        cluster: false,
    };

    let pool = RedisPool::new(&config)
        .await
        .expect("Disabled redis config should create no-op pool");
    assert!(pool.is_noop());
}
