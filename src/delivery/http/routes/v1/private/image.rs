use axum::Router;
use axum::routing::{get, post, patch, delete};
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::image::controller::{
    upload_image_ctrl, update_image_metadata_ctrl, delete_image_ctrl,
    force_delete_image_ctrl, delete_unused_images_ctrl, track_image_usage_ctrl,
    untrack_image_usage_ctrl, get_images_ctrl, get_unused_images_ctrl,
    get_image_usage_ctrl, get_image_ctrl
};

pub fn image_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(upload_image_ctrl)
                .get(get_images_ctrl)
        )
        .route(
            "/unused",
            delete(delete_unused_images_ctrl)
                .get(get_unused_images_ctrl),
        )
        .route(
            "/usage",
            post(track_image_usage_ctrl)
                .delete(untrack_image_usage_ctrl),
        )
        .route(
            "/{image_id}",
            patch(update_image_metadata_ctrl)
                .delete(delete_image_ctrl)
                .get(get_image_ctrl),
        )
        .route(
            "/{image_id}/performances",
            get(get_image_usage_ctrl),
        )
        .route(
            "/{image_id}/force",
            delete(force_delete_image_ctrl),
        )
}
