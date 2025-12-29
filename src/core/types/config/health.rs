//! Health check configuration types

use super::defaults::*;
use serde::{Deserialize, Serialize};

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Check interval (seconds)
    #[serde(default = "default_health_check_interval")]
    pub interval_seconds: u64,
    /// Timeout duration (seconds)
    #[serde(default = "default_health_check_timeout")]
    pub timeout_seconds: u64,
    /// Healthy threshold (consecutive successes)
    #[serde(default = "default_health_threshold")]
    pub healthy_threshold: u32,
    /// Unhealthy threshold (consecutive failures)
    #[serde(default = "default_unhealthy_threshold")]
    pub unhealthy_threshold: u32,
    /// Health check endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_seconds: default_health_check_interval(),
            timeout_seconds: default_health_check_timeout(),
            healthy_threshold: default_health_threshold(),
            unhealthy_threshold: default_unhealthy_threshold(),
            endpoint: None,
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Default Tests ====================

    #[test]
    fn test_health_check_config_default() {
        let config = HealthCheckConfig::default();
        assert_eq!(config.interval_seconds, 30);
        assert_eq!(config.timeout_seconds, 5);
        assert_eq!(config.healthy_threshold, 2);
        assert_eq!(config.unhealthy_threshold, 3);
        assert!(config.endpoint.is_none());
        assert!(config.enabled);
    }

    #[test]
    fn test_health_check_config_default_enabled() {
        let config = HealthCheckConfig::default();
        assert!(config.enabled);
    }

    #[test]
    fn test_health_check_config_default_no_endpoint() {
        let config = HealthCheckConfig::default();
        assert!(config.endpoint.is_none());
    }

    // ==================== Creation Tests ====================

    #[test]
    fn test_health_check_config_creation() {
        let config = HealthCheckConfig {
            interval_seconds: 60,
            timeout_seconds: 10,
            healthy_threshold: 3,
            unhealthy_threshold: 5,
            endpoint: Some("/health".to_string()),
            enabled: true,
        };
        assert_eq!(config.interval_seconds, 60);
        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.healthy_threshold, 3);
        assert_eq!(config.unhealthy_threshold, 5);
        assert_eq!(config.endpoint, Some("/health".to_string()));
    }

    #[test]
    fn test_health_check_config_disabled() {
        let config = HealthCheckConfig {
            interval_seconds: 30,
            timeout_seconds: 5,
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            endpoint: None,
            enabled: false,
        };
        assert!(!config.enabled);
    }

    // ==================== Serialization Tests ====================

    #[test]
    fn test_health_check_config_serialization() {
        let config = HealthCheckConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("interval_seconds"));
        assert!(json.contains("timeout_seconds"));
        assert!(json.contains("healthy_threshold"));
        assert!(json.contains("unhealthy_threshold"));
        assert!(json.contains("enabled"));
        assert!(!json.contains("endpoint"));
    }

    #[test]
    fn test_health_check_config_serialization_with_endpoint() {
        let config = HealthCheckConfig {
            interval_seconds: 30,
            timeout_seconds: 5,
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            endpoint: Some("/healthz".to_string()),
            enabled: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("/healthz"));
    }

    #[test]
    fn test_health_check_config_serialization_values() {
        let config = HealthCheckConfig {
            interval_seconds: 45,
            timeout_seconds: 15,
            healthy_threshold: 4,
            unhealthy_threshold: 6,
            endpoint: None,
            enabled: false,
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("45"));
        assert!(json.contains("15"));
        assert!(json.contains("false"));
    }

    // ==================== Deserialization Tests ====================

    #[test]
    fn test_health_check_config_deserialization() {
        let json = r#"{
            "interval_seconds": 60,
            "timeout_seconds": 10,
            "healthy_threshold": 3,
            "unhealthy_threshold": 5,
            "endpoint": "/health",
            "enabled": true
        }"#;
        let config: HealthCheckConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.interval_seconds, 60);
        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.healthy_threshold, 3);
        assert_eq!(config.unhealthy_threshold, 5);
        assert_eq!(config.endpoint, Some("/health".to_string()));
        assert!(config.enabled);
    }

    #[test]
    fn test_health_check_config_deserialization_with_defaults() {
        let json = r#"{}"#;
        let config: HealthCheckConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.interval_seconds, 30);
        assert_eq!(config.timeout_seconds, 5);
        assert_eq!(config.healthy_threshold, 2);
        assert_eq!(config.unhealthy_threshold, 3);
        assert!(config.enabled);
    }

    #[test]
    fn test_health_check_config_partial_deserialization() {
        let json = r#"{"interval_seconds": 120}"#;
        let config: HealthCheckConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.interval_seconds, 120);
        assert_eq!(config.timeout_seconds, 5);
    }

    #[test]
    fn test_health_check_config_deserialization_disabled() {
        let json = r#"{"enabled": false}"#;
        let config: HealthCheckConfig = serde_json::from_str(json).unwrap();
        assert!(!config.enabled);
    }

    // ==================== Clone and Debug Tests ====================

    #[test]
    fn test_health_check_config_clone() {
        let config = HealthCheckConfig {
            interval_seconds: 45,
            timeout_seconds: 8,
            healthy_threshold: 3,
            unhealthy_threshold: 4,
            endpoint: Some("/ready".to_string()),
            enabled: true,
        };
        let cloned = config.clone();
        assert_eq!(cloned.interval_seconds, 45);
        assert_eq!(cloned.timeout_seconds, 8);
        assert_eq!(cloned.endpoint, Some("/ready".to_string()));
    }

    #[test]
    fn test_health_check_config_debug() {
        let config = HealthCheckConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("HealthCheckConfig"));
        assert!(debug.contains("interval_seconds"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_health_check_config_zero_values() {
        let config = HealthCheckConfig {
            interval_seconds: 0,
            timeout_seconds: 0,
            healthy_threshold: 0,
            unhealthy_threshold: 0,
            endpoint: None,
            enabled: true,
        };
        assert_eq!(config.interval_seconds, 0);
        assert_eq!(config.timeout_seconds, 0);
        assert_eq!(config.healthy_threshold, 0);
        assert_eq!(config.unhealthy_threshold, 0);
    }

    #[test]
    fn test_health_check_config_large_values() {
        let config = HealthCheckConfig {
            interval_seconds: 86400,
            timeout_seconds: 3600,
            healthy_threshold: 100,
            unhealthy_threshold: 100,
            endpoint: None,
            enabled: true,
        };
        assert_eq!(config.interval_seconds, 86400);
        assert_eq!(config.timeout_seconds, 3600);
    }

    #[test]
    fn test_health_check_config_empty_endpoint() {
        let config = HealthCheckConfig {
            interval_seconds: 30,
            timeout_seconds: 5,
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            endpoint: Some("".to_string()),
            enabled: true,
        };
        assert_eq!(config.endpoint, Some("".to_string()));
    }

    #[test]
    fn test_health_check_config_roundtrip() {
        let config = HealthCheckConfig {
            interval_seconds: 45,
            timeout_seconds: 10,
            healthy_threshold: 3,
            unhealthy_threshold: 5,
            endpoint: Some("/api/health".to_string()),
            enabled: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: HealthCheckConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.interval_seconds, config.interval_seconds);
        assert_eq!(deserialized.timeout_seconds, config.timeout_seconds);
        assert_eq!(deserialized.endpoint, config.endpoint);
    }
}
