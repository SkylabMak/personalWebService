use axum::{Router, routing::get};
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::life_status::controller::get_current_life_status_ctrl;
use crate::interface_adapters::http::v1::controllers::website::feature_status::controller::get_website_feature_status_ctrl;

pub fn public_v1_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/profiles/{profile_id}/life-status/current",
            get(get_current_life_status_ctrl),
        )
        .route(
            "/websites/{website_id}/feature-status",
            get(get_website_feature_status_ctrl),
        )
}

