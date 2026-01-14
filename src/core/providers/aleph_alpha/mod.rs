//! Aleph Alpha Provider Implementation

pub mod config;
pub mod error_mapper;
pub mod model_info;
pub mod provider;

pub use config::AlephAlphaConfig;
pub use error_mapper::AlephAlphaErrorMapper;
pub use provider::AlephAlphaProvider;

pub const PROVIDER_NAME: &str = "aleph_alpha";
pub const DEFAULT_BASE_URL: &str = "https://api.aleph-alpha.com/v1";
