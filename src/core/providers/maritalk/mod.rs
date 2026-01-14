//! Maritalk Provider Implementation
//!
//! Maritalk is a Brazilian AI provider specializing in Portuguese language models.

pub mod config;
pub mod error_mapper;
pub mod model_info;
pub mod provider;

pub use config::MaritalkConfig;
pub use error_mapper::MaritalkErrorMapper;
pub use provider::MaritalkProvider;

pub const PROVIDER_NAME: &str = "maritalk";
pub const DEFAULT_BASE_URL: &str = "https://chat.maritaca.ai/api";
