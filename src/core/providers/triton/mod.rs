//! NVIDIA Triton Inference Server Provider
//!
//! Triton Inference Server is NVIDIA's open-source inference serving software
//! that supports multiple deep learning frameworks. This implementation provides
//! access to models deployed on Triton via its HTTP/REST API.
//!
//! ## Features
//! - Connect to Triton Inference Server
//! - Support for HTTP endpoints (gRPC planned)
//! - Model versioning support
//! - Health checks via Triton's health API
//! - Chat completions for LLM models deployed on Triton
//!
//! ## Configuration
//! The default API base is `http://localhost:8000`.
//! Set `TRITON_SERVER_URL` environment variable to customize.
//!
//! ## Triton API Endpoints
//! - Health: GET /v2/health/ready
//! - Inference: POST /v2/models/{model}/infer
//! - Model metadata: GET /v2/models/{model}

// Core modules
mod config;
mod error;
mod models;
mod provider;

// Re-export main types for external use
pub use config::TritonConfig;
pub use error::TritonError;
pub use models::{TritonModelInfo, get_model_info};
pub use provider::TritonProvider;
