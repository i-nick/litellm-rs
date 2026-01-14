//! Yi (01.AI) Provider Implementation

pub mod config;
pub mod error_mapper;
pub mod model_info;
pub mod provider;

pub use config::YiConfig;
pub use error_mapper::YiErrorMapper;
pub use provider::YiProvider;

pub const PROVIDER_NAME: &str = "yi";
pub const DEFAULT_BASE_URL: &str = "https://api.lingyiwanwu.com/v1";
