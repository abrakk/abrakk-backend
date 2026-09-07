use std::sync::Arc;

use application::{auth_service::AuthService, kit_service::KitService};

/// Shared application state injected into every handler via Axum's `State`.
#[derive(Clone)]
pub struct AppState {
    pub kit_service: Arc<KitService>,
    pub auth_service: Arc<AuthService>,
}
