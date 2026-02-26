//! Gateway configuration integration
//!
//! This module contains the from_gateway_config method for creating
//! a Router from gateway configuration.

use super::config::RouterConfig;
use super::deployment::{Deployment, DeploymentConfig};
use super::error::RouterError;
use super::unified::Router;
use crate::config::models::router::{GatewayRouterConfig, RoutingStrategyConfig};
use crate::config::models::provider::ProviderConfig;
use crate::core::providers::{Provider, create_provider};

/// Build runtime router config from gateway YAML router config.
pub fn runtime_router_config_from_gateway(config: &GatewayRouterConfig) -> Result<RouterConfig, String> {
    let routing_strategy = match &config.strategy {
        RoutingStrategyConfig::RoundRobin => super::config::RoutingStrategy::RoundRobin,
        RoutingStrategyConfig::LeastLatency => super::config::RoutingStrategy::LatencyBased,
        RoutingStrategyConfig::LeastCost => super::config::RoutingStrategy::CostBased,
        RoutingStrategyConfig::Random => super::config::RoutingStrategy::SimpleShuffle,
        RoutingStrategyConfig::Weighted { .. } => {
            return Err(
                "router.strategy.type=weighted is not yet supported by runtime unified router".to_string(),
            );
        }
        RoutingStrategyConfig::Priority { .. } => {
            return Err(
                "router.strategy.type=priority is not yet supported by runtime unified router".to_string(),
            );
        }
        RoutingStrategyConfig::ABTest { .. } => {
            return Err(
                "router.strategy.type=ab_test is not yet supported by runtime unified router".to_string(),
            );
        }
        RoutingStrategyConfig::Custom { .. } => {
            return Err(
                "router.strategy.type=custom is not yet supported by runtime unified router".to_string(),
            );
        }
    };

    Ok(RouterConfig {
        routing_strategy,
        // Gateway circuit-breaker thresholds are the closest semantic mapping here.
        allowed_fails: config.circuit_breaker.failure_threshold,
        cooldown_time_secs: config.circuit_breaker.recovery_timeout,
        // Keep other defaults until the gateway/router schemas are fully unified.
        enable_pre_call_checks: config.load_balancer.health_check_enabled,
        ..RouterConfig::default()
    })
}

impl Router {
    /// Create a Router from gateway configuration
    ///
    /// This method initializes a Router with deployments created from provider configurations.
    /// Each provider in the config becomes a deployment in the router.
    pub async fn from_gateway_config(
        providers: &[ProviderConfig],
        router_config: Option<RouterConfig>,
    ) -> Result<Self, RouterError> {
        let config = router_config.unwrap_or_default();
        let router = Self::new(config);

        for provider_config in providers {
            if !provider_config.enabled {
                continue;
            }

            // Create provider instance via the single canonical factory.
            let provider = create_provider(provider_config.clone()).await.map_err(|e| {
                RouterError::DeploymentNotFound(format!(
                    "Failed to create provider {}: {}",
                    provider_config.name, e
                ))
            })?;

            // Determine which models this deployment serves
            let models: Vec<String> = if !provider_config.models.is_empty() {
                provider_config.models.clone()
            } else {
                provider
                    .list_models()
                    .iter()
                    .map(|m| m.id.clone())
                    .collect()
            };

            // Create deployments
            if models.is_empty() {
                // Create a single deployment with provider name
                let deployment = create_deployment_from_config(
                    &provider_config.name,
                    provider.clone(),
                    &provider_config.name,
                    provider_config,
                );
                router.add_deployment(deployment);
            } else {
                // Create one deployment per model
                for model in models {
                    let deployment_id = format!("{}-{}", provider_config.name, model);
                    let deployment = create_deployment_from_config(
                        &deployment_id,
                        provider.clone(),
                        &model,
                        provider_config,
                    );
                    router.add_deployment(deployment);
                }
            }
        }

        Ok(router)
    }
}

/// Helper function to create deployment from provider config
fn create_deployment_from_config(
    deployment_id: &str,
    provider: Provider,
    model: &str,
    config: &ProviderConfig,
) -> Deployment {
    let deployment_config = DeploymentConfig {
        tpm_limit: if config.tpm > 0 {
            Some(config.tpm as u64)
        } else {
            None
        },
        rpm_limit: if config.rpm > 0 {
            Some(config.rpm as u64)
        } else {
            None
        },
        max_parallel_requests: if config.max_concurrent_requests > 0 {
            Some(config.max_concurrent_requests)
        } else {
            None
        },
        weight: (config.weight.max(1.0)).round() as u32,
        timeout_secs: config.timeout,
        priority: 0,
    };

    Deployment::new(
        deployment_id.to_string(),
        provider,
        model.to_string(),
        model.to_string(),
    )
    .with_config(deployment_config)
    .with_tags(config.tags.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::models::router::{
        CircuitBreakerConfig, GatewayRouterConfig, LoadBalancerConfig, RoutingStrategyConfig,
    };

    #[test]
    fn test_runtime_router_config_from_gateway_round_robin() {
        let gateway = GatewayRouterConfig::default();
        let runtime = runtime_router_config_from_gateway(&gateway).unwrap();
        assert_eq!(runtime.routing_strategy, super::super::config::RoutingStrategy::RoundRobin);
    }

    #[test]
    fn test_runtime_router_config_from_gateway_strategy_mapping() {
        let gateway = GatewayRouterConfig {
            strategy: RoutingStrategyConfig::LeastLatency,
            circuit_breaker: CircuitBreakerConfig::default(),
            load_balancer: LoadBalancerConfig::default(),
        };
        let runtime = runtime_router_config_from_gateway(&gateway).unwrap();
        assert_eq!(
            runtime.routing_strategy,
            super::super::config::RoutingStrategy::LatencyBased
        );
    }

    #[test]
    fn test_runtime_router_config_from_gateway_circuit_breaker_mapping() {
        let gateway = GatewayRouterConfig {
            strategy: RoutingStrategyConfig::RoundRobin,
            circuit_breaker: CircuitBreakerConfig {
                failure_threshold: 8,
                recovery_timeout: 45,
                min_requests: 10,
                success_threshold: 3,
            },
            load_balancer: LoadBalancerConfig::default(),
        };
        let runtime = runtime_router_config_from_gateway(&gateway).unwrap();
        assert_eq!(runtime.allowed_fails, 8);
        assert_eq!(runtime.cooldown_time_secs, 45);
    }

    #[test]
    fn test_runtime_router_config_from_gateway_rejects_weighted() {
        let gateway = GatewayRouterConfig {
            strategy: RoutingStrategyConfig::Weighted {
                weights: std::collections::HashMap::new(),
            },
            circuit_breaker: CircuitBreakerConfig::default(),
            load_balancer: LoadBalancerConfig::default(),
        };
        let result = runtime_router_config_from_gateway(&gateway);
        assert!(result.is_err());
    }
}
