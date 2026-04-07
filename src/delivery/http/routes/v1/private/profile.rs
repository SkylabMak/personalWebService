use axum::Router;
use axum::routing::get;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::controller::get_profile_ctrl;

pub fn profile_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/{profile_id}",
            get(get_profile_ctrl),
        )
}
