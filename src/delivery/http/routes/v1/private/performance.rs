use axum::Router;
use axum::routing::{get, post, patch};
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::profile::performance::controller::{
    create_performance_ctrl, update_performance_ctrl, delete_performance_ctrl,
    get_performance_content_ctrl, update_performance_content_ctrl, get_performances_ctrl,
    get_performance_images_ctrl
};

pub fn performance_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create_performance_ctrl)
                .get(get_performances_ctrl),
        )
        .route(
            "/{performance_id}",
            patch(update_performance_ctrl).delete(delete_performance_ctrl),
        )
        .route(
            "/{performance_id}/content",
            get(get_performance_content_ctrl).patch(update_performance_content_ctrl),
        )
        .route(
            "/{performance_id}/images",
            get(get_performance_images_ctrl),
        )
}
