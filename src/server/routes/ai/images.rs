//! Image generation endpoint

use crate::core::models::openai::{ImageGenerationRequest, ImageGenerationResponse};
use crate::core::providers::ProviderRegistry;
use crate::core::types::context::RequestContext;
use crate::core::types::image::ImageGenerationRequest as CoreImageRequest;
use crate::core::types::model::ProviderCapability;
use crate::server::state::AppState;
use crate::utils::error::gateway_error::GatewayError;
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use tracing::info;

use super::context::handle_ai_request;
use super::provider_selection::select_provider_for_optional_model;

/// Image generation endpoint
///
/// OpenAI-compatible image generation API.
pub async fn image_generations(
    state: web::Data<AppState>,
    req: HttpRequest,
    request: web::Json<ImageGenerationRequest>,
) -> ActixResult<HttpResponse> {
    info!("Image generation request for model: {:?}", request.model);

    handle_ai_request(
        &req,
        request.into_inner(),
        "Image generation",
        |request, context| handle_image_generation_with_state(state.get_ref(), request, context),
    )
    .await
}

/// Handle image generation with app state (supports unified router when model is provided)
pub async fn handle_image_generation_with_state(
    state: &AppState,
    request: ImageGenerationRequest,
    context: RequestContext,
) -> Result<ImageGenerationResponse, GatewayError> {
    handle_image_generation_internal(&state.router, state.unified_router.as_deref(), request, context)
        .await
}

/// Handle image generation via provider pool
#[allow(dead_code)] // Compatibility path for direct pool-based invocations/tests.
pub async fn handle_image_generation_via_pool(
    pool: &ProviderRegistry,
    request: ImageGenerationRequest,
    context: RequestContext,
) -> Result<ImageGenerationResponse, GatewayError> {
    handle_image_generation_internal(pool, None, request, context).await
}

async fn handle_image_generation_internal(
    pool: &ProviderRegistry,
    unified_router: Option<&crate::core::router::UnifiedRouter>,
    request: ImageGenerationRequest,
    context: RequestContext,
) -> Result<ImageGenerationResponse, GatewayError> {
    let selection = select_provider_for_optional_model(
        pool,
        unified_router,
        request.model.as_deref(),
        ProviderCapability::ImageGeneration,
    )?;

    let core_request = CoreImageRequest {
        prompt: request.prompt,
        model: selection.1,
        n: request.n,
        size: request.size,
        response_format: request.response_format,
        user: request.user,
        quality: None,
        style: None,
    };

    let core_response = selection
        .0
        .create_images(core_request, context)
        .await
        .map_err(|e| GatewayError::internal(format!("Image generation error: {}", e)))?;

    // Convert core response to OpenAI format
    let response = ImageGenerationResponse {
        created: core_response.created,
        data: core_response
            .data
            .into_iter()
            .map(|d| crate::core::models::openai::ImageObject {
                url: d.url,
                b64_json: d.b64_json,
            })
            .collect(),
    };

    Ok(response)
}
