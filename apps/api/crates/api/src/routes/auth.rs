use axum::{routing::post, Router};

use crate::{handlers::auth, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
}
