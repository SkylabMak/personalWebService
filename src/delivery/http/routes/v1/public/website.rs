use axum::Router;
use axum::routing::get;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::website::feature_status::controller::get_website_feature_status_ctrl;
use crate::interface_adapters::http::v1::controllers::website::config::controller::get_app_config_ctrl;

pub fn website_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/features/{appID}/feature-status",
            get(get_website_feature_status_ctrl),
        )
        .route(
            "/app/{appID}/config",
            get(get_app_config_ctrl),
        )
}
