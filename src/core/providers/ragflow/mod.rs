//! RAGFlow Provider
//!
//! RAGFlow provides RAG (Retrieval-Augmented Generation) capabilities with OpenAI-compatible API.

mod config;
mod model_info;
mod provider;

pub use config::RagflowConfig;
pub use model_info::{RagflowModel, get_model_info};
pub use provider::RagflowProvider;
