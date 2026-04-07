use axum::Router;
use crate::delivery::http::server::state::AppState;
use crate::delivery::http::routes::v1::public::profile::profile_routes;
use crate::delivery::http::routes::v1::public::website::website_routes;
use crate::delivery::http::routes::v1::public::performance::performance_routes;

pub mod profile;
pub mod website;
pub mod performance;

pub fn public_v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/profiles", profile_routes().merge(performance_routes()))
        .merge(website_routes())
}
