use axum::{
    extract::State,
    response::IntoResponse,
    Extension,
};
use crate::delivery::http::server::state::AppState;
use crate::domain::entities::auth::claims::Claims;

pub async fn me_ctrl(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let res = state.auth.auth.me(&claims.sub).await;
    res.into_response()
}
