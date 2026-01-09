//! Databricks Provider
//!
//! Provides integration with Databricks Foundation Model APIs.
//! Supports chat completions and embeddings via Databricks serving endpoints.

pub mod config;
pub mod error;
pub mod models;
pub mod provider;
pub mod streaming;

pub use config::DatabricksConfig;
pub use error::DatabricksErrorMapper;
pub use models::{DatabricksModelRegistry, get_databricks_registry};
pub use provider::DatabricksProvider;
