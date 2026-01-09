//! Load balancer module for provider selection
//!
//! **DEPRECATED**: This module is part of the legacy load balancer system.
//! For new code, use `crate::core::router::UnifiedRouter` instead, which provides:
//! - Deployment-based routing with sophisticated health tracking
//! - Built-in retry and fallback support via `execute()` method
//! - Lock-free concurrent access with DashMap
//! - Better integration with the router configuration system
//!
//! ## Migration Guide
//!
//! Replace the legacy LoadBalancer:
//! ```rust,no_run
//! # use litellm_rs::core::router::load_balancer::LoadBalancer;
//! # use litellm_rs::core::router::strategy::types::RoutingStrategy;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let lb = LoadBalancer::new(RoutingStrategy::RoundRobin).await?;
//! // lb.add_provider("openai", provider).await?;
//! # Ok(())
//! # }
//! ```
//!
//! With the new UnifiedRouter:
//! ```rust
//! # use litellm_rs::{UnifiedRouter, RouterConfig};
//! let router = UnifiedRouter::new(RouterConfig::default());
//! // router.add_deployment(deployment);
//! // let result = router.execute("gpt-4", |deployment_id| async {
//! //     // Your operation here
//! // }).await?;
//! ```

mod core;
mod deployment_info;
mod fallback_config;
mod fallback_selection;
mod selection;
mod tag_routing;

pub use core::{LoadBalancer, LoadBalancerStats};
pub use deployment_info::DeploymentInfo;
pub use fallback_config::FallbackConfig;
