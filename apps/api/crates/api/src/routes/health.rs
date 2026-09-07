use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

/// Health check endpoint.
///
/// Returns 200 OK with service status. Used by Docker/load balancers.
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "edukit-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
