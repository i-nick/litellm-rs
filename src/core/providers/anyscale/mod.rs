//! Anyscale Provider Implementation

pub mod config;
pub mod error_mapper;
pub mod model_info;
pub mod provider;

pub use config::AnyscaleConfig;
pub use error_mapper::AnyscaleErrorMapper;
pub use provider::AnyscaleProvider;

pub const PROVIDER_NAME: &str = "anyscale";
pub const DEFAULT_BASE_URL: &str = "https://api.endpoints.anyscale.com/v1";
