//! Heroku Provider
//!
//! Heroku AI Inference API integration (part of Salesforce ecosystem).
//! Heroku uses an OpenAI-compatible API format for chat completions, embeddings, and image generation.

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod provider;
pub mod streaming;

pub use client::HerokuClient;
pub use config::HerokuConfig;
pub use error::HerokuErrorMapper;
pub use models::{get_heroku_registry, HerokuModelRegistry};
pub use provider::HerokuProvider;
