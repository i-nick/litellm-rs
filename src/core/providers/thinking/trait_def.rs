//! Thinking Provider Trait Definition
//!
//! This module contains the core trait definition and the NoThinkingSupport default implementation.

use serde_json::Value;

use crate::core::providers::unified_provider::ProviderError;
use crate::core::types::thinking::{
    ThinkingCapabilities, ThinkingConfig, ThinkingContent, ThinkingEffort, ThinkingUsage,
};

/// Trait for providers that support thinking/reasoning capabilities
///
/// This trait enables providers to:
/// 1. Advertise thinking support for specific models
/// 2. Transform thinking configuration to provider-specific format
/// 3. Extract thinking content from responses
/// 4. Track thinking token usage and costs
pub trait ThinkingProvider {
    /// Check if a specific model supports thinking
    ///
    /// # Arguments
    /// * `model` - The model identifier to check
    ///
    /// # Returns
    /// `true` if the model supports thinking, `false` otherwise
    fn supports_thinking(&self, model: &str) -> bool;

    /// Get thinking capabilities for a specific model
    ///
    /// Returns detailed information about what thinking features are supported.
    fn thinking_capabilities(&self, model: &str) -> ThinkingCapabilities;

    /// Transform thinking configuration to provider-specific format
    ///
    /// # Arguments
    /// * `config` - The unified thinking configuration
    /// * `model` - The model being used
    ///
    /// # Returns
    /// A JSON value with provider-specific thinking parameters
    fn transform_thinking_config(
        &self,
        config: &ThinkingConfig,
        model: &str,
    ) -> Result<Value, ProviderError>;

    /// Extract thinking content from a provider response
    ///
    /// # Arguments
    /// * `response` - The raw JSON response from the provider
    ///
    /// # Returns
    /// The extracted thinking content, if present
    fn extract_thinking(&self, response: &Value) -> Option<ThinkingContent>;

    /// Extract thinking usage statistics from a provider response
    ///
    /// # Arguments
    /// * `response` - The raw JSON response from the provider
    ///
    /// # Returns
    /// Thinking usage statistics, if available
    fn extract_thinking_usage(&self, response: &Value) -> Option<ThinkingUsage>;

    /// Get the default thinking effort for this provider
    ///
    /// Returns the default effort level when none is specified.
    fn default_thinking_effort(&self) -> ThinkingEffort {
        ThinkingEffort::Medium
    }

    /// Get maximum thinking tokens allowed for a model
    ///
    /// Returns `None` if there's no limit or it's unknown.
    fn max_thinking_tokens(&self, model: &str) -> Option<u32> {
        self.thinking_capabilities(model).max_thinking_tokens
    }

    /// Check if the provider supports streaming thinking content
    fn supports_streaming_thinking(&self, model: &str) -> bool {
        self.thinking_capabilities(model)
            .supports_streaming_thinking
    }
}

/// Default implementation helper for providers without thinking support
pub struct NoThinkingSupport;

impl ThinkingProvider for NoThinkingSupport {
    fn supports_thinking(&self, _model: &str) -> bool {
        false
    }

    fn thinking_capabilities(&self, _model: &str) -> ThinkingCapabilities {
        ThinkingCapabilities::unsupported()
    }

    fn transform_thinking_config(
        &self,
        _config: &ThinkingConfig,
        _model: &str,
    ) -> Result<Value, ProviderError> {
        Ok(Value::Object(serde_json::Map::new()))
    }

    fn extract_thinking(&self, _response: &Value) -> Option<ThinkingContent> {
        None
    }

    fn extract_thinking_usage(&self, _response: &Value) -> Option<ThinkingUsage> {
        None
    }
}
