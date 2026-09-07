use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use application::dto::kit::{CreateKitRequest, KitResponse, ListKitsQuery, UpdateKitRequest};

use crate::{errors::ApiError, middleware::auth::AuthenticatedUser, state::AppState};

/// GET /api/v1/kits
///
/// Returns a paginated list of published kits.
pub async fn list_kits(
    State(state): State<AppState>,
    Query(query): Query<ListKitsQuery>,
) -> Result<Json<Vec<KitResponse>>, ApiError> {
    let kits = state.kit_service.list_kits(query).await?;
    Ok(Json(kits))
}

/// GET /api/v1/kits/:id
///
/// Returns a single kit by ID.
pub async fn get_kit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<KitResponse>, ApiError> {
    let kit = state.kit_service.get_kit(id).await?;
    Ok(Json(kit))
}

/// POST /api/v1/kits
///
/// Creates a new kit. Requires authentication.
pub async fn create_kit(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<CreateKitRequest>,
) -> Result<(StatusCode, Json<KitResponse>), ApiError> {
    let kit = state.kit_service.create_kit(request, user_id).await?;
    Ok((StatusCode::CREATED, Json(kit)))
}

/// PATCH /api/v1/kits/:id
///
/// Updates a kit. Only the kit's author may update it.
pub async fn update_kit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(request): Json<UpdateKitRequest>,
) -> Result<Json<KitResponse>, ApiError> {
    let kit = state.kit_service.update_kit(id, request, user_id).await?;
    Ok(Json(kit))
}

/// DELETE /api/v1/kits/:id
///
/// Deletes a kit. Only the kit's author may delete it.
pub async fn delete_kit(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<StatusCode, ApiError> {
    state.kit_service.delete_kit(id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/kits/:id/download
///
/// Increments the download counter and returns the kit.
pub async fn record_download(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<KitResponse>, ApiError> {
    state.kit_service.record_download(id).await?;
    let kit = state.kit_service.get_kit(id).await?;
    Ok(Json(kit))
}
