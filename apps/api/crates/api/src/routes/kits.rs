use axum::{routing::get, Router};

use crate::{handlers::kits, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/kits", get(kits::list_kits).post(kits::create_kit))
        .route(
            "/api/v1/kits/:id",
            get(kits::get_kit)
                .patch(kits::update_kit)
                .delete(kits::delete_kit),
        )
        .route("/api/v1/kits/:id/download", get(kits::record_download))
}
