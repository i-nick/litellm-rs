//! Provider selection helpers for AI routes

use crate::core::providers::{Provider, ProviderRegistry};
use crate::core::router::{RouterError, UnifiedRouter};
use crate::core::types::model::ProviderCapability;
use crate::utils::error::gateway_error::GatewayError;
use std::borrow::Cow;

pub struct ProviderSelection<'a> {
    pub provider: Cow<'a, Provider>,
    pub model: String,
}

pub fn select_provider_for_model<'a>(
    pool: &'a ProviderRegistry,
    unified_router: Option<&UnifiedRouter>,
    model: &str,
    capability: ProviderCapability,
) -> Result<ProviderSelection<'a>, GatewayError> {
    if model.trim().is_empty() {
        return Err(GatewayError::validation("Model is required"));
    }

    if let Some((prefix, actual)) = model.split_once('/') {
        if pool.contains(prefix) {
            let provider = pool
                .get(prefix)
                .ok_or_else(|| GatewayError::internal("Provider not available"))?;
            if !provider_supports_capability(provider, &capability) {
                return Err(GatewayError::validation(format!(
                    "Provider '{}' does not support {:?}",
                    prefix, capability
                )));
            }
            return Ok(ProviderSelection {
                provider: Cow::Borrowed(provider),
                model: actual.to_string(),
            });
        }
    }

    if let Some(router) = unified_router {
        return select_provider_from_unified_router(router, model, capability);
    }

    let mut candidates: Vec<&Provider> = pool
        .find_supporting_model(model)
        .into_iter()
        .filter(|p| provider_supports_capability(p, &capability))
        .collect();

    if candidates.len() == 1 {
        return Ok(ProviderSelection {
            provider: Cow::Borrowed(candidates.remove(0)),
            model: model.to_string(),
        });
    }

    let mut capable: Vec<&Provider> = pool
        .values()
        .filter(|p| provider_supports_capability(p, &capability))
        .collect();

    if capable.len() == 1 {
        return Ok(ProviderSelection {
            provider: Cow::Borrowed(capable.remove(0)),
            model: model.to_string(),
        });
    }

    if capable.is_empty() {
        return Err(GatewayError::internal(format!(
            "No providers available for {:?}",
            capability
        )));
    }

    Err(GatewayError::validation(
        "Multiple providers available; use provider/model prefix to disambiguate",
    ))
}

pub fn select_provider_for_optional_model<'a>(
    pool: &'a ProviderRegistry,
    unified_router: Option<&UnifiedRouter>,
    model: Option<&str>,
    capability: ProviderCapability,
) -> Result<(Cow<'a, Provider>, Option<String>), GatewayError> {
    if let Some(model) = model {
        let selection = select_provider_for_model(pool, unified_router, model, capability)?;
        return Ok((selection.provider, Some(selection.model)));
    }

    let mut capable: Vec<&Provider> = pool
        .values()
        .filter(|p| provider_supports_capability(p, &capability))
        .collect();

    if capable.len() == 1 {
        return Ok((Cow::Borrowed(capable.remove(0)), None));
    }

    if capable.is_empty() {
        return Err(GatewayError::internal(format!(
            "No providers available for {:?}",
            capability
        )));
    }

    Err(GatewayError::validation(
        "Multiple providers available; specify model with provider prefix",
    ))
}

fn provider_supports_capability(provider: &Provider, capability: &ProviderCapability) -> bool {
    provider.capabilities().iter().any(|cap| cap == capability)
}

fn select_provider_from_unified_router<'a>(
    router: &UnifiedRouter,
    model: &str,
    capability: ProviderCapability,
) -> Result<ProviderSelection<'a>, GatewayError> {
    let deployment_id = router.select_deployment(model).map_err(map_router_error)?;

    let deployment = router
        .get_deployment(&deployment_id)
        .ok_or_else(|| GatewayError::internal("Selected deployment not found"))?;

    let provider = deployment.provider.clone();
    let selected_model = deployment.model.clone();
    drop(deployment);

    // select_deployment increments active_requests; release immediately because
    // route handlers invoke providers directly (without Router::execute* wrapper).
    router.release_deployment(&deployment_id);

    if !provider_supports_capability(&provider, &capability) {
        return Err(GatewayError::validation(format!(
            "Selected deployment does not support {:?}",
            capability
        )));
    }

    Ok(ProviderSelection {
        provider: Cow::Owned(provider),
        model: selected_model,
    })
}

fn map_router_error(error: RouterError) -> GatewayError {
    match error {
        RouterError::ModelNotFound(model) => {
            GatewayError::validation(format!("Model not found in router: {}", model))
        }
        RouterError::NoAvailableDeployment(model)
        | RouterError::AllDeploymentsInCooldown(model)
        | RouterError::RateLimitExceeded(model) => {
            GatewayError::service_unavailable(format!("No available deployment for {}", model))
        }
        RouterError::DeploymentNotFound(deployment_id) => {
            GatewayError::internal(format!("Router deployment not found: {}", deployment_id))
        }
    }
}
