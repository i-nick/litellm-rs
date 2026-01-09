//! Stability AI Provider
//!
//! Provides integration with Stability AI's image generation APIs.
//! Supports Stable Diffusion 3, Stable Image Ultra, and Stable Image Core.

pub mod config;
pub mod error;
pub mod models;
pub mod provider;

pub use config::StabilityConfig;
pub use error::StabilityErrorMapper;
pub use models::{StabilityModelRegistry, get_stability_registry};
pub use provider::StabilityProvider;
