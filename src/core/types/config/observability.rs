//! Observability configuration types

use super::defaults::*;
use serde::{Deserialize, Serialize};

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Metrics configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,
    /// Tracing configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<TracingConfig>,
    /// Logging configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingConfig>,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            metrics: Some(MetricsConfig {
                enabled: true,
                endpoint: default_metrics_endpoint(),
                interval_seconds: default_metrics_interval(),
            }),
            tracing: Some(TracingConfig {
                enabled: true,
                sampling_rate: default_sampling_rate(),
                jaeger: None,
            }),
            logging: Some(LoggingConfig {
                level: default_log_level(),
                format: default_log_format(),
                outputs: vec![LogOutput::Console],
            }),
        }
    }
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Endpoint path
    #[serde(default = "default_metrics_endpoint")]
    pub endpoint: String,
    /// Collection interval (seconds)
    #[serde(default = "default_metrics_interval")]
    pub interval_seconds: u64,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Sampling rate (0.0-1.0)
    #[serde(default = "default_sampling_rate")]
    pub sampling_rate: f64,
    /// Jaeger configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jaeger: Option<JaegerConfig>,
}

/// Jaeger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaegerConfig {
    /// Agent endpoint
    pub agent_endpoint: String,
    /// Service name
    pub service_name: String,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Output format
    #[serde(default = "default_log_format")]
    pub format: LogFormat,
    /// Output targets
    #[serde(default)]
    pub outputs: Vec<LogOutput>,
}

/// Log format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
    Structured,
}

/// Log output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LogOutput {
    #[serde(rename = "console")]
    Console,
    #[serde(rename = "file")]
    File { path: String },
    #[serde(rename = "syslog")]
    Syslog { facility: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ObservabilityConfig Tests ====================

    #[test]
    fn test_observability_config_default() {
        let config = ObservabilityConfig::default();
        assert!(config.metrics.is_some());
        assert!(config.tracing.is_some());
        assert!(config.logging.is_some());
    }

    #[test]
    fn test_observability_config_default_metrics() {
        let config = ObservabilityConfig::default();
        let metrics = config.metrics.unwrap();
        assert!(metrics.enabled);
        assert_eq!(metrics.endpoint, "/metrics");
        assert_eq!(metrics.interval_seconds, 15);
    }

    #[test]
    fn test_observability_config_default_tracing() {
        let config = ObservabilityConfig::default();
        let tracing = config.tracing.unwrap();
        assert!(tracing.enabled);
        assert!((tracing.sampling_rate - 0.1).abs() < f64::EPSILON);
        assert!(tracing.jaeger.is_none());
    }

    #[test]
    fn test_observability_config_default_logging() {
        let config = ObservabilityConfig::default();
        let logging = config.logging.unwrap();
        assert_eq!(logging.level, "info");
        assert!(matches!(logging.format, LogFormat::Json));
        assert_eq!(logging.outputs.len(), 1);
    }

    #[test]
    fn test_observability_config_serialization() {
        let config = ObservabilityConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("metrics"));
        assert!(json.contains("tracing"));
        assert!(json.contains("logging"));
    }

    #[test]
    fn test_observability_config_deserialization() {
        let json = r#"{
            "metrics": {"enabled": true, "endpoint": "/metrics", "interval_seconds": 30},
            "tracing": {"enabled": false, "sampling_rate": 0.5},
            "logging": {"level": "debug", "format": "text", "outputs": []}
        }"#;
        let config: ObservabilityConfig = serde_json::from_str(json).unwrap();
        assert!(config.metrics.is_some());
        assert!(!config.tracing.unwrap().enabled);
        assert_eq!(config.logging.unwrap().level, "debug");
    }

    #[test]
    fn test_observability_config_empty_deserialization() {
        let json = r#"{}"#;
        let config: ObservabilityConfig = serde_json::from_str(json).unwrap();
        assert!(config.metrics.is_none());
        assert!(config.tracing.is_none());
        assert!(config.logging.is_none());
    }

    // ==================== MetricsConfig Tests ====================

    #[test]
    fn test_metrics_config_creation() {
        let config = MetricsConfig {
            enabled: true,
            endpoint: "/custom-metrics".to_string(),
            interval_seconds: 60,
        };
        assert!(config.enabled);
        assert_eq!(config.endpoint, "/custom-metrics");
        assert_eq!(config.interval_seconds, 60);
    }

    #[test]
    fn test_metrics_config_serialization() {
        let config = MetricsConfig {
            enabled: false,
            endpoint: "/metrics".to_string(),
            interval_seconds: 30,
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("/metrics"));
        assert!(json.contains("30"));
    }

    #[test]
    fn test_metrics_config_deserialization_with_defaults() {
        let json = r#"{}"#;
        let config: MetricsConfig = serde_json::from_str(json).unwrap();
        assert!(config.enabled);
        assert_eq!(config.endpoint, "/metrics");
        assert_eq!(config.interval_seconds, 15);
    }

    #[test]
    fn test_metrics_config_partial_deserialization() {
        let json = r#"{"enabled": false}"#;
        let config: MetricsConfig = serde_json::from_str(json).unwrap();
        assert!(!config.enabled);
        assert_eq!(config.endpoint, "/metrics");
    }

    // ==================== TracingConfig Tests ====================

    #[test]
    fn test_tracing_config_creation() {
        let config = TracingConfig {
            enabled: true,
            sampling_rate: 0.5,
            jaeger: None,
        };
        assert!(config.enabled);
        assert!((config.sampling_rate - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tracing_config_with_jaeger() {
        let config = TracingConfig {
            enabled: true,
            sampling_rate: 1.0,
            jaeger: Some(JaegerConfig {
                agent_endpoint: "localhost:6831".to_string(),
                service_name: "my-service".to_string(),
            }),
        };
        assert!(config.jaeger.is_some());
        let jaeger = config.jaeger.unwrap();
        assert_eq!(jaeger.agent_endpoint, "localhost:6831");
        assert_eq!(jaeger.service_name, "my-service");
    }

    #[test]
    fn test_tracing_config_serialization() {
        let config = TracingConfig {
            enabled: true,
            sampling_rate: 0.25,
            jaeger: None,
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("0.25"));
        assert!(!json.contains("jaeger"));
    }

    #[test]
    fn test_tracing_config_deserialization_with_defaults() {
        let json = r#"{}"#;
        let config: TracingConfig = serde_json::from_str(json).unwrap();
        assert!(config.enabled);
        assert!((config.sampling_rate - 0.1).abs() < f64::EPSILON);
    }

    // ==================== JaegerConfig Tests ====================

    #[test]
    fn test_jaeger_config_creation() {
        let config = JaegerConfig {
            agent_endpoint: "jaeger:6831".to_string(),
            service_name: "gateway".to_string(),
        };
        assert_eq!(config.agent_endpoint, "jaeger:6831");
        assert_eq!(config.service_name, "gateway");
    }

    #[test]
    fn test_jaeger_config_serialization() {
        let config = JaegerConfig {
            agent_endpoint: "localhost:6831".to_string(),
            service_name: "test-service".to_string(),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("localhost:6831"));
        assert!(json.contains("test-service"));
    }

    #[test]
    fn test_jaeger_config_deserialization() {
        let json = r#"{"agent_endpoint": "127.0.0.1:6831", "service_name": "api"}"#;
        let config: JaegerConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.agent_endpoint, "127.0.0.1:6831");
        assert_eq!(config.service_name, "api");
    }

    // ==================== LoggingConfig Tests ====================

    #[test]
    fn test_logging_config_creation() {
        let config = LoggingConfig {
            level: "warn".to_string(),
            format: LogFormat::Text,
            outputs: vec![LogOutput::Console],
        };
        assert_eq!(config.level, "warn");
        assert!(matches!(config.format, LogFormat::Text));
    }

    #[test]
    fn test_logging_config_multiple_outputs() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            format: LogFormat::Json,
            outputs: vec![
                LogOutput::Console,
                LogOutput::File {
                    path: "/var/log/app.log".to_string(),
                },
            ],
        };
        assert_eq!(config.outputs.len(), 2);
    }

    #[test]
    fn test_logging_config_serialization() {
        let config = LoggingConfig {
            level: "error".to_string(),
            format: LogFormat::Structured,
            outputs: vec![],
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("structured"));
    }

    #[test]
    fn test_logging_config_deserialization_with_defaults() {
        let json = r#"{}"#;
        let config: LoggingConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.level, "info");
        assert!(matches!(config.format, LogFormat::Json));
        assert!(config.outputs.is_empty());
    }

    // ==================== LogFormat Tests ====================

    #[test]
    fn test_log_format_text() {
        let format = LogFormat::Text;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "\"text\"");
    }

    #[test]
    fn test_log_format_json() {
        let format = LogFormat::Json;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "\"json\"");
    }

    #[test]
    fn test_log_format_structured() {
        let format = LogFormat::Structured;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "\"structured\"");
    }

    #[test]
    fn test_log_format_all_variants_deserialize() {
        let formats = ["text", "json", "structured"];
        for fmt in formats {
            let json = format!("\"{}\"", fmt);
            let _: LogFormat = serde_json::from_str(&json).unwrap();
        }
    }

    // ==================== LogOutput Tests ====================

    #[test]
    fn test_log_output_console() {
        let output = LogOutput::Console;
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("console"));
    }

    #[test]
    fn test_log_output_file() {
        let output = LogOutput::File {
            path: "/var/log/app.log".to_string(),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("file"));
        assert!(json.contains("/var/log/app.log"));
    }

    #[test]
    fn test_log_output_syslog() {
        let output = LogOutput::Syslog {
            facility: "local0".to_string(),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("syslog"));
        assert!(json.contains("local0"));
    }

    #[test]
    fn test_log_output_console_deserialization() {
        let json = r#"{"type": "console"}"#;
        let output: LogOutput = serde_json::from_str(json).unwrap();
        assert!(matches!(output, LogOutput::Console));
    }

    #[test]
    fn test_log_output_file_deserialization() {
        let json = r#"{"type": "file", "path": "/tmp/test.log"}"#;
        let output: LogOutput = serde_json::from_str(json).unwrap();
        match output {
            LogOutput::File { path } => assert_eq!(path, "/tmp/test.log"),
            _ => panic!("Expected File"),
        }
    }

    #[test]
    fn test_log_output_syslog_deserialization() {
        let json = r#"{"type": "syslog", "facility": "local1"}"#;
        let output: LogOutput = serde_json::from_str(json).unwrap();
        match output {
            LogOutput::Syslog { facility } => assert_eq!(facility, "local1"),
            _ => panic!("Expected Syslog"),
        }
    }

    // ==================== Clone and Debug Tests ====================

    #[test]
    fn test_observability_config_clone() {
        let config = ObservabilityConfig::default();
        let cloned = config.clone();
        assert!(cloned.metrics.is_some());
    }

    #[test]
    fn test_metrics_config_clone() {
        let config = MetricsConfig {
            enabled: true,
            endpoint: "/metrics".to_string(),
            interval_seconds: 15,
        };
        let cloned = config.clone();
        assert_eq!(cloned.endpoint, "/metrics");
    }

    #[test]
    fn test_tracing_config_clone() {
        let config = TracingConfig {
            enabled: true,
            sampling_rate: 0.5,
            jaeger: None,
        };
        let cloned = config.clone();
        assert!((cloned.sampling_rate - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_log_format_debug() {
        let format = LogFormat::Json;
        let debug = format!("{:?}", format);
        assert!(debug.contains("Json"));
    }

    #[test]
    fn test_log_output_debug() {
        let output = LogOutput::Console;
        let debug = format!("{:?}", output);
        assert!(debug.contains("Console"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_sampling_rate_zero() {
        let config = TracingConfig {
            enabled: true,
            sampling_rate: 0.0,
            jaeger: None,
        };
        assert!(config.sampling_rate.abs() < f64::EPSILON);
    }

    #[test]
    fn test_sampling_rate_one() {
        let config = TracingConfig {
            enabled: true,
            sampling_rate: 1.0,
            jaeger: None,
        };
        assert!((config.sampling_rate - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_empty_endpoint() {
        let config = MetricsConfig {
            enabled: true,
            endpoint: "".to_string(),
            interval_seconds: 15,
        };
        assert!(config.endpoint.is_empty());
    }

    #[test]
    fn test_zero_interval() {
        let config = MetricsConfig {
            enabled: true,
            endpoint: "/metrics".to_string(),
            interval_seconds: 0,
        };
        assert_eq!(config.interval_seconds, 0);
    }

    #[test]
    fn test_empty_outputs() {
        let config = LoggingConfig {
            level: "info".to_string(),
            format: LogFormat::Json,
            outputs: vec![],
        };
        assert!(config.outputs.is_empty());
    }
}
