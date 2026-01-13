//! Provider management handlers

use super::common::*;
use crate::infrastructure::utils::IntoStatusCode;

/// List all providers
pub async fn list_providers_handler(
    State(state): State<AdminState>,
) -> Result<Json<ApiResponse<Vec<ProviderInfo>>>, StatusCode> {
    let provider_statuses = state.admin_service.get_providers().await.to_500()?;

    let providers = provider_statuses
        .into_iter()
        .map(|status| ProviderInfo {
            id: status.id,
            name: status.name,
            provider_type: status.provider_type,
            status: status.status,
            config: status.config,
        })
        .collect();

    Ok(Json(ApiResponse::success(providers)))
}

/// Add a new provider
pub async fn add_provider_handler(
    State(state): State<AdminState>,
    Json(provider_config): Json<ProviderConfigRequest>,
) -> Result<Json<ApiResponse<ProviderInfo>>, StatusCode> {
    // Validate provider configuration based on type
    match provider_config.provider_type.as_str() {
        "embedding" => {
            if provider_config.config.get("model").is_none() {
                return Ok(Json(ApiResponse::error(
                    "Model is required for embedding providers".to_string(),
                )));
            }
        }
        "vector_store" => {
            if provider_config.config.get("host").is_none() {
                return Ok(Json(ApiResponse::error(
                    "Host is required for vector store providers".to_string(),
                )));
            }
        }
        _ => {
            return Ok(Json(ApiResponse::error(
                "Invalid provider type".to_string(),
            )));
        }
    }

    match state
        .admin_service
        .add_provider(&provider_config.provider_type, provider_config.config)
        .await
    {
        Ok(svc_info) => {
            let provider_info = ProviderInfo {
                id: svc_info.id,
                name: svc_info.name,
                provider_type: svc_info.provider_type,
                status: svc_info.status,
                config: svc_info.config,
            };
            Ok(Json(ApiResponse::success(provider_info)))
        }
        Err(e) => Ok(Json(ApiResponse::error(format!(
            "Failed to add provider: {}",
            e
        )))),
    }
}

/// Remove a provider
pub async fn remove_provider_handler(
    State(state): State<AdminState>,
    Path(provider_id): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let providers = state.admin_service.get_providers().await.to_500()?;

    if !providers.iter().any(|p| p.id == provider_id) {
        return Ok(Json(ApiResponse::error("Provider not found".to_string())));
    }

    Ok(Json(ApiResponse::success(format!(
        "Provider {} removed successfully",
        provider_id
    ))))
}
