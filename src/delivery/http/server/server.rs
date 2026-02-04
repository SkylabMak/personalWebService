use axum::{Router, http::Method};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::{Any, CorsLayer};
use crate::delivery::http::routes::public::public::public_v1_routes;
use crate::delivery::http::routes::private::private_v1_routes;
use crate::delivery::http::routes::auth::auth_routes;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::presenters::common::fallback_handler::fallback_handler;
use crate::interface_adapters::http::v1::presenters::common::panic_handler::handle_panic;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let public_routes = public_v1_routes();
    let private_routes = private_v1_routes(state.clone());
    let auth_routes = auth_routes(state.clone());

    Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .merge(auth_routes)
        .fallback(fallback_handler)
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(cors)
        .with_state(state)
}
