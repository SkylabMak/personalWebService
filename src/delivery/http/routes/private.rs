use axum::{Router, middleware};
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::image::controller::{
    upload_image_ctrl, update_image_metadata_ctrl, delete_image_ctrl, force_delete_image_ctrl, delete_unused_images_ctrl,
    track_image_usage_ctrl, untrack_image_usage_ctrl
};
use crate::interface_adapters::http::v1::controllers::profile::performance::controller::{
    create_performance_ctrl, update_performance_ctrl, delete_performance_ctrl,
    get_performance_content_ctrl, update_performance_content_ctrl
};
use axum::routing::{get, post, patch, delete};

use crate::delivery::http::middleware::auth_middleware::auth_middleware;

pub fn private_v1_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/profiles/{profile_id}/images",
            post(upload_image_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/unused",
            delete(delete_unused_images_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/usage",
            post(track_image_usage_ctrl).delete(untrack_image_usage_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/{image_id}",
            patch(update_image_metadata_ctrl)
                .delete(delete_image_ctrl),
        )
        .route(
            "/profiles/{profile_id}/images/{image_id}/force",
            delete(force_delete_image_ctrl),
        )
        .route(
            "/profiles/{profile_id}/performances",
            post(create_performance_ctrl),
        )
        .route(
            "/profiles/{profile_id}/performances/{performance_id}",
            patch(update_performance_ctrl).delete(delete_performance_ctrl),
        )
        .route(
            "/profiles/{profile_id}/performances/{performance_id}/content",
            get(get_performance_content_ctrl).patch(update_performance_content_ctrl),
        )
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}
