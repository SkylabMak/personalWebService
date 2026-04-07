use axum::Router;
use axum::routing::get;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::life_status::controller::get_current_life_status_ctrl;
use crate::interface_adapters::http::v1::controllers::profile::controller::get_profile_ctrl;
use crate::interface_adapters::http::v1::controllers::profile::announce::controller::get_announce_list_ctrl;

pub fn profile_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/{profile_id}/public",
            get(get_profile_ctrl),
        )
        .route(
            "/{profile_id}/life-status/current",
            get(get_current_life_status_ctrl),
        )
        .route(
            "/{profile_id}/announces",
            get(get_announce_list_ctrl),
        )
}
