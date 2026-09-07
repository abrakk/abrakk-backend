use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use application::{auth_service::AuthService, kit_service::KitService};
use infrastructure::repositories::{
    kit_repository::PostgresKitRepository, user_repository::PostgresUserRepository,
};

use crate::{config::Config, routes, state::AppState};

/// Build the full Axum application with all routes and middleware.
pub async fn build_app(pool: PgPool, config: Config) -> Router {
    // Repositories
    let kit_repo = Arc::new(PostgresKitRepository::new(pool.clone()));
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));

    // Services
    let kit_service = Arc::new(KitService::new(kit_repo));
    let auth_service = Arc::new(AuthService::new(
        user_repo,
        config.jwt_secret.clone(),
        config.jwt_expiration_hours,
    ));

    let state = AppState {
        kit_service,
        auth_service,
    };

    // CORS — in production, restrict origins
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routes::health::router())
        .merge(routes::kits::router())
        .merge(routes::auth::router())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
