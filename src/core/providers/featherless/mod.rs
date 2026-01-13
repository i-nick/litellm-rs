//! Featherless AI Provider
//!
//! Featherless AI provides serverless GPU inference for open-source models.

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod provider;
pub mod streaming;

pub use client::FeatherlessClient;
pub use config::FeatherlessConfig;
pub use error::FeatherlessErrorMapper;
pub use models::{FeatherlessModelRegistry, get_featherless_registry};
pub use provider::FeatherlessProvider;
