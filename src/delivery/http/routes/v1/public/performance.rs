use axum::Router;
use axum::routing::get;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::performance::controller::get_public_performances_ctrl;

pub fn performance_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/{profile_id}/publicPerformances",
            get(get_public_performances_ctrl),
        )
}
