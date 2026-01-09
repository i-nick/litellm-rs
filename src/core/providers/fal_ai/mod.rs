//! Fal AI Provider
//!
//! Provider for Fal AI image generation API
//! Supports models like Flux, Stable Diffusion, DALL-E compatible, etc.

pub mod config;
pub mod error;
pub mod models;
pub mod provider;

pub use config::FalAIConfig;
pub use error::FalAIErrorMapper;
pub use models::{FalAIModel, FalAIModelRegistry, ImageSize};
pub use provider::FalAIProvider;
