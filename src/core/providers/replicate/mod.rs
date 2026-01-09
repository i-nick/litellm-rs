//! Replicate Provider
//!
//! Replicate is a platform that lets you run machine learning models with a cloud API.
//! This provider supports both chat completions (via Llama models) and image generation
//! (via Stable Diffusion, SDXL, Flux, and other models).

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod prediction;
pub mod provider;
pub mod streaming;

pub use client::ReplicateClient;
pub use config::ReplicateConfig;
pub use error::ReplicateErrorMapper;
pub use models::{ReplicateModelRegistry, get_replicate_registry};
pub use prediction::{PredictionResponse, PredictionStatus};
pub use provider::ReplicateProvider;
