//! API error response types

use serde::{Deserialize, Serialize};

/// Error response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ApiError,
}

/// API error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error message
    pub message: String,

    /// Error type
    #[serde(rename = "type")]
    pub error_type: String,

    /// Parameter that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,

    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ErrorResponse Tests ====================

    #[test]
    fn test_error_response_creation() {
        let response = ErrorResponse {
            error: ApiError {
                message: "Something went wrong".to_string(),
                error_type: "server_error".to_string(),
                param: None,
                code: None,
            },
        };
        assert_eq!(response.error.message, "Something went wrong");
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse {
            error: ApiError {
                message: "Invalid request".to_string(),
                error_type: "invalid_request_error".to_string(),
                param: Some("model".to_string()),
                code: Some("invalid_model".to_string()),
            },
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("Invalid request"));
        assert!(json.contains("invalid_request_error"));
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{
            "error": {
                "message": "API key is invalid",
                "type": "authentication_error",
                "code": "invalid_api_key"
            }
        }"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.error.message, "API key is invalid");
        assert_eq!(response.error.error_type, "authentication_error");
        assert_eq!(response.error.code, Some("invalid_api_key".to_string()));
    }

    // ==================== ApiError Tests ====================

    #[test]
    fn test_api_error_creation() {
        let error = ApiError {
            message: "Rate limit exceeded".to_string(),
            error_type: "rate_limit_error".to_string(),
            param: None,
            code: Some("rate_limit_exceeded".to_string()),
        };
        assert_eq!(error.message, "Rate limit exceeded");
        assert_eq!(error.error_type, "rate_limit_error");
    }

    #[test]
    fn test_api_error_with_param() {
        let error = ApiError {
            message: "Invalid value for parameter".to_string(),
            error_type: "invalid_request_error".to_string(),
            param: Some("temperature".to_string()),
            code: None,
        };
        assert_eq!(error.param, Some("temperature".to_string()));
    }

    #[test]
    fn test_api_error_serialization_minimal() {
        let error = ApiError {
            message: "Error".to_string(),
            error_type: "error".to_string(),
            param: None,
            code: None,
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("message"));
        assert!(json.contains("type"));
        assert!(!json.contains("param"));
        assert!(!json.contains("code"));
    }

    #[test]
    fn test_api_error_serialization_full() {
        let error = ApiError {
            message: "Model not found".to_string(),
            error_type: "invalid_request_error".to_string(),
            param: Some("model".to_string()),
            code: Some("model_not_found".to_string()),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("param"));
        assert!(json.contains("code"));
    }

    #[test]
    fn test_api_error_deserialization() {
        let json = r#"{
            "message": "Context length exceeded",
            "type": "context_length_exceeded",
            "param": "messages",
            "code": "context_length_exceeded"
        }"#;
        let error: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(error.message, "Context length exceeded");
        assert_eq!(error.param, Some("messages".to_string()));
    }

    #[test]
    fn test_api_error_deserialization_minimal() {
        let json = r#"{"message": "Error", "type": "error"}"#;
        let error: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(error.message, "Error");
        assert!(error.param.is_none());
        assert!(error.code.is_none());
    }

    // ==================== Clone and Debug Tests ====================

    #[test]
    fn test_error_response_clone() {
        let response = ErrorResponse {
            error: ApiError {
                message: "Test".to_string(),
                error_type: "test".to_string(),
                param: None,
                code: None,
            },
        };
        let cloned = response.clone();
        assert_eq!(cloned.error.message, "Test");
    }

    #[test]
    fn test_api_error_clone() {
        let error = ApiError {
            message: "Clone test".to_string(),
            error_type: "test".to_string(),
            param: Some("param".to_string()),
            code: Some("code".to_string()),
        };
        let cloned = error.clone();
        assert_eq!(cloned.message, "Clone test");
        assert_eq!(cloned.param, Some("param".to_string()));
    }

    #[test]
    fn test_error_response_debug() {
        let response = ErrorResponse {
            error: ApiError {
                message: "Debug test".to_string(),
                error_type: "test".to_string(),
                param: None,
                code: None,
            },
        };
        let debug = format!("{:?}", response);
        assert!(debug.contains("ErrorResponse"));
        assert!(debug.contains("Debug test"));
    }

    #[test]
    fn test_api_error_debug() {
        let error = ApiError {
            message: "Debug".to_string(),
            error_type: "test".to_string(),
            param: None,
            code: None,
        };
        let debug = format!("{:?}", error);
        assert!(debug.contains("ApiError"));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_api_error_empty_message() {
        let error = ApiError {
            message: "".to_string(),
            error_type: "error".to_string(),
            param: None,
            code: None,
        };
        assert!(error.message.is_empty());
    }

    #[test]
    fn test_api_error_roundtrip() {
        let original = ApiError {
            message: "Roundtrip test".to_string(),
            error_type: "test_error".to_string(),
            param: Some("test_param".to_string()),
            code: Some("test_code".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ApiError = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.message, original.message);
        assert_eq!(deserialized.error_type, original.error_type);
        assert_eq!(deserialized.param, original.param);
        assert_eq!(deserialized.code, original.code);
    }
}
