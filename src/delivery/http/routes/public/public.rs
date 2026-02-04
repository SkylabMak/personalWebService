use axum::Router;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::life_status::controller::get_current_life_status_ctrl;
use crate::interface_adapters::http::v1::controllers::profile::announce::controller::get_announce_list_ctrl;
use crate::interface_adapters::http::v1::controllers::profile::image::controller::{
    get_images_ctrl, get_image_ctrl, get_image_usage_ctrl, get_unused_images_ctrl
};
use crate::interface_adapters::http::v1::controllers::website::feature_status::controller::get_website_feature_status_ctrl;
use crate::interface_adapters::http::v1::controllers::auth::login_ctrl::login_ctrl;
use axum::routing::{get, post};

pub fn public_v1_routes() -> Router<AppState> {
    Router::new()
        // .route("/login", post(login_ctrl))
        .route(
            "/profiles/{profile_id}/life-status/current",
            get(get_current_life_status_ctrl),
        )
        .route(
            "/profiles/{profile_id}/announces",
            get(get_announce_list_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images",
            get(get_images_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/unused",
            get(get_unused_images_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/{image_id}",
            get(get_image_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/{image_id}/usage",
            get(get_image_usage_ctrl),
        )
        .route(
            "/features/{appID}/feature-status",
            get(get_website_feature_status_ctrl),
        )
}

