//! AIML API Provider Implementation

pub mod config;
pub mod error_mapper;
pub mod model_info;
pub mod provider;

pub use config::AimlConfig;
pub use error_mapper::AimlErrorMapper;
pub use provider::AimlProvider;

pub const PROVIDER_NAME: &str = "aiml";
pub const DEFAULT_BASE_URL: &str = "https://api.aimlapi.com/v1";
