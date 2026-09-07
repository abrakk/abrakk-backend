use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use domain::errors::DomainError;

/// Unified API error type that converts to an HTTP response.
pub struct ApiError(DomainError);

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        ApiError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            DomainError::KitNotFound(_)
            | DomainError::ActivityNotFound(_)
            | DomainError::UserNotFound(_) => (StatusCode::NOT_FOUND, self.0.to_string()),

            DomainError::UserEmailConflict(_) | DomainError::UserUsernameConflict(_) => {
                (StatusCode::CONFLICT, self.0.to_string())
            }

            DomainError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".into())
            }

            DomainError::AccessDenied => (StatusCode::FORBIDDEN, "Access denied".into()),

            DomainError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
