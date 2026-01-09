//! Replicate Prediction Types
//!
//! Types for handling Replicate prediction lifecycle

use serde::{Deserialize, Serialize};

/// Prediction status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PredictionStatus {
    /// Prediction is queued and waiting to be processed
    Starting,
    /// Prediction is currently being processed
    Processing,
    /// Prediction completed successfully
    Succeeded,
    /// Prediction failed
    Failed,
    /// Prediction was canceled
    Canceled,
}

impl PredictionStatus {
    /// Check if the prediction has completed (either succeeded or failed)
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            PredictionStatus::Succeeded | PredictionStatus::Failed | PredictionStatus::Canceled
        )
    }

    /// Check if the prediction is still in progress
    pub fn is_in_progress(&self) -> bool {
        matches!(
            self,
            PredictionStatus::Starting | PredictionStatus::Processing
        )
    }

    /// Check if the prediction succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, PredictionStatus::Succeeded)
    }
}

/// URLs returned in prediction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionUrls {
    /// URL to cancel the prediction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<String>,

    /// URL to get the prediction status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<String>,

    /// URL for streaming output (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

/// Prediction metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionMetrics {
    /// Time to generate prediction in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predict_time: Option<f64>,

    /// Total time including queue time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time: Option<f64>,
}

/// Replicate prediction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResponse {
    /// Unique prediction ID
    pub id: String,

    /// Model version used for the prediction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Prediction status
    pub status: PredictionStatus,

    /// Prediction input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,

    /// Prediction output (format depends on the model)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,

    /// Error message if prediction failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Logs from the prediction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,

    /// Prediction metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<PredictionMetrics>,

    /// URLs for prediction operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<PredictionUrls>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    /// Completion timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    /// Model used for the prediction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Data URL for the prediction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_removed: Option<bool>,
}

impl PredictionResponse {
    /// Get the prediction URL for polling
    pub fn get_prediction_url(&self) -> Option<&str> {
        self.urls.as_ref()?.get.as_deref()
    }

    /// Get the stream URL if available
    pub fn get_stream_url(&self) -> Option<&str> {
        self.urls.as_ref()?.stream.as_deref()
    }

    /// Get the cancel URL
    pub fn get_cancel_url(&self) -> Option<&str> {
        self.urls.as_ref()?.cancel.as_deref()
    }

    /// Check if the prediction is still in progress
    pub fn is_in_progress(&self) -> bool {
        self.status.is_in_progress()
    }

    /// Check if the prediction has completed
    pub fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Check if the prediction succeeded
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Get the output as a string (for text models)
    pub fn get_text_output(&self) -> Option<String> {
        let output = self.output.as_ref()?;

        // Output can be a string or an array of strings
        if let Some(s) = output.as_str() {
            return Some(s.to_string());
        }

        if let Some(arr) = output.as_array() {
            let texts: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
            if !texts.is_empty() {
                return Some(texts.join(""));
            }
        }

        None
    }

    /// Get the output as URLs (for image models)
    pub fn get_image_urls(&self) -> Option<Vec<String>> {
        let output = self.output.as_ref()?;

        // Output is typically an array of URLs for image models
        if let Some(arr) = output.as_array() {
            let urls: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            if !urls.is_empty() {
                return Some(urls);
            }
        }

        // Single URL output
        if let Some(s) = output.as_str() {
            return Some(vec![s.to_string()]);
        }

        None
    }
}

/// Request to create a new prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePredictionRequest {
    /// Input parameters for the model
    pub input: serde_json::Value,

    /// Model version to use (optional, for versioned predictions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Whether to stream output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Webhook URL to receive prediction updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,

    /// Events to trigger webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_events_filter: Option<Vec<String>>,
}

impl CreatePredictionRequest {
    /// Create a new prediction request with input
    pub fn new(input: serde_json::Value) -> Self {
        Self {
            input,
            version: None,
            stream: None,
            webhook: None,
            webhook_events_filter: None,
        }
    }

    /// Set the model version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Enable streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction_status_is_terminal() {
        assert!(PredictionStatus::Succeeded.is_terminal());
        assert!(PredictionStatus::Failed.is_terminal());
        assert!(PredictionStatus::Canceled.is_terminal());
        assert!(!PredictionStatus::Starting.is_terminal());
        assert!(!PredictionStatus::Processing.is_terminal());
    }

    #[test]
    fn test_prediction_status_is_in_progress() {
        assert!(PredictionStatus::Starting.is_in_progress());
        assert!(PredictionStatus::Processing.is_in_progress());
        assert!(!PredictionStatus::Succeeded.is_in_progress());
        assert!(!PredictionStatus::Failed.is_in_progress());
        assert!(!PredictionStatus::Canceled.is_in_progress());
    }

    #[test]
    fn test_prediction_status_is_success() {
        assert!(PredictionStatus::Succeeded.is_success());
        assert!(!PredictionStatus::Failed.is_success());
        assert!(!PredictionStatus::Processing.is_success());
    }

    #[test]
    fn test_prediction_response_get_text_output_string() {
        let response = PredictionResponse {
            id: "test".to_string(),
            version: None,
            status: PredictionStatus::Succeeded,
            input: None,
            output: Some(serde_json::json!("Hello, world!")),
            error: None,
            logs: None,
            metrics: None,
            urls: None,
            created_at: None,
            started_at: None,
            completed_at: None,
            model: None,
            data_removed: None,
        };
        assert_eq!(
            response.get_text_output(),
            Some("Hello, world!".to_string())
        );
    }

    #[test]
    fn test_prediction_response_get_text_output_array() {
        let response = PredictionResponse {
            id: "test".to_string(),
            version: None,
            status: PredictionStatus::Succeeded,
            input: None,
            output: Some(serde_json::json!(["Hello", ", ", "world", "!"])),
            error: None,
            logs: None,
            metrics: None,
            urls: None,
            created_at: None,
            started_at: None,
            completed_at: None,
            model: None,
            data_removed: None,
        };
        assert_eq!(
            response.get_text_output(),
            Some("Hello, world!".to_string())
        );
    }

    #[test]
    fn test_prediction_response_get_image_urls() {
        let response = PredictionResponse {
            id: "test".to_string(),
            version: None,
            status: PredictionStatus::Succeeded,
            input: None,
            output: Some(serde_json::json!([
                "https://example.com/image1.png",
                "https://example.com/image2.png"
            ])),
            error: None,
            logs: None,
            metrics: None,
            urls: None,
            created_at: None,
            started_at: None,
            completed_at: None,
            model: None,
            data_removed: None,
        };
        let urls = response.get_image_urls().unwrap();
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], "https://example.com/image1.png");
    }

    #[test]
    fn test_prediction_response_get_prediction_url() {
        let response = PredictionResponse {
            id: "test".to_string(),
            version: None,
            status: PredictionStatus::Processing,
            input: None,
            output: None,
            error: None,
            logs: None,
            metrics: None,
            urls: Some(PredictionUrls {
                cancel: Some("https://api.replicate.com/v1/predictions/test/cancel".to_string()),
                get: Some("https://api.replicate.com/v1/predictions/test".to_string()),
                stream: None,
            }),
            created_at: None,
            started_at: None,
            completed_at: None,
            model: None,
            data_removed: None,
        };
        assert_eq!(
            response.get_prediction_url(),
            Some("https://api.replicate.com/v1/predictions/test")
        );
    }

    #[test]
    fn test_create_prediction_request() {
        let request = CreatePredictionRequest::new(serde_json::json!({
            "prompt": "Hello"
        }))
        .with_version("abc123")
        .with_stream(true);

        assert_eq!(request.version, Some("abc123".to_string()));
        assert_eq!(request.stream, Some(true));
    }

    #[test]
    fn test_prediction_status_serialization() {
        assert_eq!(
            serde_json::to_string(&PredictionStatus::Succeeded).unwrap(),
            "\"succeeded\""
        );
        assert_eq!(
            serde_json::to_string(&PredictionStatus::Processing).unwrap(),
            "\"processing\""
        );
    }

    #[test]
    fn test_prediction_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<PredictionStatus>("\"succeeded\"").unwrap(),
            PredictionStatus::Succeeded
        );
        assert_eq!(
            serde_json::from_str::<PredictionStatus>("\"failed\"").unwrap(),
            PredictionStatus::Failed
        );
    }

    #[test]
    fn test_prediction_response_is_in_progress() {
        let response = PredictionResponse {
            id: "test".to_string(),
            version: None,
            status: PredictionStatus::Processing,
            input: None,
            output: None,
            error: None,
            logs: None,
            metrics: None,
            urls: None,
            created_at: None,
            started_at: None,
            completed_at: None,
            model: None,
            data_removed: None,
        };
        assert!(response.is_in_progress());
        assert!(!response.is_terminal());
    }

    #[test]
    fn test_prediction_urls_serialization() {
        let urls = PredictionUrls {
            cancel: Some("https://example.com/cancel".to_string()),
            get: Some("https://example.com/get".to_string()),
            stream: None,
        };
        let json = serde_json::to_value(&urls).unwrap();
        assert!(json.get("cancel").is_some());
        assert!(json.get("stream").is_none());
    }

    #[test]
    fn test_prediction_metrics() {
        let metrics = PredictionMetrics {
            predict_time: Some(1.5),
            total_time: Some(2.0),
        };
        assert_eq!(metrics.predict_time, Some(1.5));
        assert_eq!(metrics.total_time, Some(2.0));
    }
}
