//! DeepL Translation Provider Implementation

pub mod config;
pub mod error_mapper;
pub mod model_info;
pub mod provider;

pub use config::DeepLConfig;
pub use error_mapper::DeepLErrorMapper;
pub use provider::DeepLProvider;

pub const PROVIDER_NAME: &str = "deepl";
pub const DEFAULT_BASE_URL: &str = "https://api-free.deepl.com/v2";
pub const PRO_BASE_URL: &str = "https://api.deepl.com/v2";
