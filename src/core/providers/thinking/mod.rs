//! Thinking/Reasoning Provider Trait
//!
//! This module defines the trait for providers that support thinking/reasoning capabilities.
//! It provides a unified interface for:
//! - OpenAI o1/o3/o4 reasoning
//! - Anthropic Claude extended thinking
//! - DeepSeek R1/Reasoner
//! - Gemini 2.0 Flash Thinking / 3.0 Deep Think
//! - OpenRouter passthrough

mod providers;
mod trait_def;

#[cfg(test)]
mod tests;

// Re-export the main trait and types
pub use trait_def::{NoThinkingSupport, ThinkingProvider};

// Re-export provider-specific modules
pub use providers::{
    anthropic_thinking, deepseek_thinking, gemini_thinking, openai_thinking, openrouter_thinking,
};
