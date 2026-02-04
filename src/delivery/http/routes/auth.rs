use axum::{Router, routing::{post, get}};
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::controllers::auth::{
    login_ctrl::login_ctrl,
    refresh_ctrl::refresh_ctrl,
    logout_ctrl::logout_ctrl,
    me_ctrl::me_ctrl,
};
use axum::middleware;
use crate::delivery::http::middleware::auth_middleware::auth_middleware;

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login_ctrl))
        .route("/auth/refresh", post(refresh_ctrl))
        .route("/auth/logout", post(logout_ctrl))
        .route("/auth/me", get(me_ctrl).layer(middleware::from_fn_with_state(state, auth_middleware)))
}
