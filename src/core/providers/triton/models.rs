//! Triton Model Information
//!
//! Contains model configurations for Triton-hosted models.
//! Since Triton is a self-hosted inference server, models are dynamically
//! discovered from the server rather than statically defined.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model information for Triton-hosted models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonModelInfo {
    /// Model name as deployed on Triton
    pub name: String,

    /// Model version (optional)
    pub version: Option<String>,

    /// Model state (READY, LOADING, etc.)
    pub state: Option<String>,

    /// Model platform (tensorrt_llm, pytorch, onnx, etc.)
    pub platform: Option<String>,

    /// Maximum batch size supported
    pub max_batch_size: Option<u32>,

    /// Input tensor information
    pub inputs: Vec<TensorInfo>,

    /// Output tensor information
    pub outputs: Vec<TensorInfo>,

    /// Model parameters
    pub parameters: HashMap<String, String>,
}

/// Tensor information for model inputs/outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorInfo {
    /// Tensor name
    pub name: String,

    /// Data type (FP32, FP16, INT32, etc.)
    pub datatype: String,

    /// Shape dimensions (-1 for dynamic)
    pub shape: Vec<i64>,
}

/// Triton model metadata response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadataResponse {
    /// Model name
    pub name: String,

    /// Model versions available
    #[serde(default)]
    pub versions: Vec<String>,

    /// Model platform
    pub platform: Option<String>,

    /// Input tensors
    #[serde(default)]
    pub inputs: Vec<TensorMetadata>,

    /// Output tensors
    #[serde(default)]
    pub outputs: Vec<TensorMetadata>,
}

/// Tensor metadata from Triton API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorMetadata {
    /// Tensor name
    pub name: String,

    /// Data type
    pub datatype: String,

    /// Shape
    pub shape: Vec<i64>,
}

/// Triton inference request format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonInferRequest {
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Input tensors
    pub inputs: Vec<TritonTensor>,

    /// Requested outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<TritonOutputRequest>>,

    /// Request parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Output request specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonOutputRequest {
    /// Output tensor name
    pub name: String,

    /// Parameters for this output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Triton tensor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonTensor {
    /// Tensor name
    pub name: String,

    /// Data type
    pub datatype: String,

    /// Shape
    pub shape: Vec<i64>,

    /// Tensor data
    pub data: serde_json::Value,

    /// Parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Triton inference response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TritonInferResponse {
    /// Response ID
    pub id: Option<String>,

    /// Model name
    pub model_name: String,

    /// Model version
    pub model_version: Option<String>,

    /// Output tensors
    pub outputs: Vec<TritonTensor>,

    /// Response parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// Get model information for a given model
/// For Triton, this returns a placeholder since models are dynamically discovered
pub fn get_model_info(model_name: &str) -> Option<TritonModelInfo> {
    // Return a basic model info structure
    // Actual information should be fetched from Triton server
    Some(TritonModelInfo {
        name: model_name.to_string(),
        version: None,
        state: None,
        platform: None,
        max_batch_size: None,
        inputs: Vec::new(),
        outputs: Vec::new(),
        parameters: HashMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triton_model_info() {
        let info = TritonModelInfo {
            name: "llama-7b".to_string(),
            version: Some("1".to_string()),
            state: Some("READY".to_string()),
            platform: Some("tensorrt_llm".to_string()),
            max_batch_size: Some(8),
            inputs: vec![TensorInfo {
                name: "input_ids".to_string(),
                datatype: "INT32".to_string(),
                shape: vec![-1, -1],
            }],
            outputs: vec![TensorInfo {
                name: "output_ids".to_string(),
                datatype: "INT32".to_string(),
                shape: vec![-1, -1],
            }],
            parameters: HashMap::new(),
        };

        assert_eq!(info.name, "llama-7b");
        assert_eq!(info.version, Some("1".to_string()));
        assert_eq!(info.state, Some("READY".to_string()));
        assert_eq!(info.platform, Some("tensorrt_llm".to_string()));
        assert_eq!(info.max_batch_size, Some(8));
    }

    #[test]
    fn test_tensor_info() {
        let tensor = TensorInfo {
            name: "input_ids".to_string(),
            datatype: "INT32".to_string(),
            shape: vec![1, 512],
        };

        assert_eq!(tensor.name, "input_ids");
        assert_eq!(tensor.datatype, "INT32");
        assert_eq!(tensor.shape, vec![1, 512]);
    }

    #[test]
    fn test_get_model_info() {
        let info = get_model_info("my-model");
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.name, "my-model");
    }

    #[test]
    fn test_triton_infer_request_serialization() {
        let request = TritonInferRequest {
            id: Some("req-123".to_string()),
            inputs: vec![TritonTensor {
                name: "text".to_string(),
                datatype: "BYTES".to_string(),
                shape: vec![1],
                data: serde_json::json!(["Hello, world!"]),
                parameters: None,
            }],
            outputs: None,
            parameters: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "req-123");
        assert_eq!(json["inputs"][0]["name"], "text");
        assert_eq!(json["inputs"][0]["datatype"], "BYTES");
    }

    #[test]
    fn test_triton_infer_response_deserialization() {
        let json = r#"{
            "id": "resp-123",
            "model_name": "llama-7b",
            "model_version": "1",
            "outputs": [{
                "name": "text_output",
                "datatype": "BYTES",
                "shape": [1],
                "data": ["Generated text here"]
            }]
        }"#;

        let response: TritonInferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, Some("resp-123".to_string()));
        assert_eq!(response.model_name, "llama-7b");
        assert_eq!(response.model_version, Some("1".to_string()));
        assert_eq!(response.outputs.len(), 1);
        assert_eq!(response.outputs[0].name, "text_output");
    }

    #[test]
    fn test_model_metadata_response() {
        let json = r#"{
            "name": "llama-7b",
            "versions": ["1", "2"],
            "platform": "tensorrt_llm",
            "inputs": [{
                "name": "input_ids",
                "datatype": "INT32",
                "shape": [-1, -1]
            }],
            "outputs": [{
                "name": "output_ids",
                "datatype": "INT32",
                "shape": [-1, -1]
            }]
        }"#;

        let metadata: ModelMetadataResponse = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.name, "llama-7b");
        assert_eq!(metadata.versions, vec!["1", "2"]);
        assert_eq!(metadata.platform, Some("tensorrt_llm".to_string()));
        assert_eq!(metadata.inputs.len(), 1);
        assert_eq!(metadata.outputs.len(), 1);
    }

    #[test]
    fn test_triton_model_info_serialization() {
        let info = TritonModelInfo {
            name: "test-model".to_string(),
            version: Some("1".to_string()),
            state: Some("READY".to_string()),
            platform: Some("pytorch".to_string()),
            max_batch_size: Some(16),
            inputs: vec![],
            outputs: vec![],
            parameters: HashMap::new(),
        };

        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["name"], "test-model");
        assert_eq!(json["version"], "1");
        assert_eq!(json["state"], "READY");
        assert_eq!(json["platform"], "pytorch");
        assert_eq!(json["max_batch_size"], 16);
    }
}
