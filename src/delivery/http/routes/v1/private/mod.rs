use axum::{Router, middleware};
use crate::delivery::http::server::state::AppState;
use crate::delivery::http::routes::v1::private::profile::profile_routes;
use crate::delivery::http::routes::v1::private::image::image_routes;
use crate::delivery::http::routes::v1::private::performance::performance_routes;
use crate::delivery::http::middleware::auth_middleware::auth_middleware;

pub mod profile;
pub mod image;
pub mod performance;

pub fn private_v1_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/profiles", profile_routes())
        .nest("/profiles/{profile_id}/images", image_routes())
        .nest("/profiles/{profile_id}/performances", performance_routes())
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}
