//! OVHcloud AI Endpoints Provider
//!
//! OVHcloud provides AI endpoints with OpenAI-compatible API.
//! Supports chat completions, embeddings, and audio transcription.

mod config;
mod model_info;
mod provider;

pub use config::OvhcloudConfig;
pub use model_info::{OvhcloudModel, get_model_info};
pub use provider::OvhcloudProvider;
