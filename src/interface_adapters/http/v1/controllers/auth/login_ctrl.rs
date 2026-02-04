use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::delivery::http::server::state::AppState;

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub async fn login_ctrl(
    State(state): State<AppState>,
    Json(payload): Json<LoginInput>,
) -> impl IntoResponse {
    let res = state.auth.auth.login(&payload.username, &payload.password).await;
    res.into_response()
}
