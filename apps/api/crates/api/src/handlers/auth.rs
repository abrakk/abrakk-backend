use axum::{extract::State, http::StatusCode, Json};

use application::dto::auth::{AuthResponse, LoginRequest, RegisterRequest};

use crate::{errors::ApiError, state::AppState};

/// POST /api/v1/auth/register
///
/// Creates a new user account and returns a JWT.
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ApiError> {
    let response = state.auth_service.register(request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// POST /api/v1/auth/login
///
/// Authenticates credentials and returns a JWT.
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let response = state.auth_service.login(request).await?;
    Ok(Json(response))
}
