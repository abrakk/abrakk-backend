use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::state::AppState;

/// Extractor that validates the Bearer token and provides the authenticated user's ID.
///
/// Use as a handler parameter to require authentication:
/// ```rust
/// async fn my_handler(AuthenticatedUser(user_id): AuthenticatedUser) { ... }
/// ```
pub struct AuthenticatedUser(pub Uuid);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // Expect "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        // Validate token via auth service
        let user_id = state
            .auth_service
            .validate_token(token)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthenticatedUser(user_id))
    }
}

pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };

        (
            status,
            Json(json!({ "error": message, "status": status.as_u16() })),
        )
            .into_response()
    }
}
