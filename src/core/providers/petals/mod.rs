//! Petals Distributed LLM Provider
//!
//! Petals allows running large language models collaboratively through distributed inference.
//! This implementation provides access to chat completions through Petals' OpenAI-compatible API.

mod config;
mod model_info;
mod provider;

pub use config::PetalsConfig;
pub use model_info::{PetalsModel, get_model_info};
pub use provider::PetalsProvider;
