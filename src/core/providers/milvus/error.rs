//! Error types for Milvus provider.
//!
//! Uses the unified ProviderError type for consistent error handling
//! across all providers.

pub use crate::core::providers::unified_provider::ProviderError;

/// Milvus error type (alias to unified ProviderError)
pub type MilvusError = ProviderError;
