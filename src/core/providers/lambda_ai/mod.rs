//! Lambda Labs AI Provider
//!
//! Lambda Labs provides GPU-accelerated inference using their cloud infrastructure.
//! This implementation provides access to various open-source models through Lambda's
//! OpenAI-compatible API with GPU-accelerated inference.

// Core modules
mod config;
mod error;
mod model_info;
mod provider;

// Feature modules
pub mod streaming;

// Tests
#[cfg(test)]
mod tests;

// Re-export main types for external use
pub use config::LambdaAIConfig;
pub use error::LambdaAIError;
pub use model_info::{LambdaModel, get_model_info};
pub use provider::LambdaAIProvider;
