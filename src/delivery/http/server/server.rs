use axum::Router;
use tower_http::catch_panic::CatchPanicLayer;
use crate::delivery::http::routes::public::public::public_routes;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::presenters::common::fallback_handler::fallback_handler;
use crate::interface_adapters::http::presenters::common::panic_handler::handle_panic;

pub fn create_router(state: AppState) -> Router {
    public_routes()
        .fallback(fallback_handler)
        .layer(CatchPanicLayer::custom(handle_panic))
        .with_state(state)
}
