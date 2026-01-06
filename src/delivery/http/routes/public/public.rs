use axum::{Router, routing::get};
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::controllers::profile::life_status::controller::get_current_life_status_ctrl;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/profiles/{profile_id}/life-status/current",
            get(get_current_life_status_ctrl),
        )
}
