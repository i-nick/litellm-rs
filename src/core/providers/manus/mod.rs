//! Manus AI Provider
//!
//! Manus AI provides AI-powered automation and LLM services.
//! This implementation provides access to chat completions through Manus's OpenAI-compatible API.

mod config;
mod model_info;
mod provider;

pub use config::ManusConfig;
pub use model_info::{ManusModel, get_model_info};
pub use provider::ManusProvider;
