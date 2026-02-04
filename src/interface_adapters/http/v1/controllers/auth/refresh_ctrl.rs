use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::delivery::http::server::state::AppState;

#[derive(Deserialize)]
pub struct RefreshInput {
    pub refresh_token: String,
}

pub async fn refresh_ctrl(
    State(state): State<AppState>,
    Json(payload): Json<RefreshInput>,
) -> impl IntoResponse {
    let res = state.auth.auth.refresh(&payload.refresh_token).await;
    res.into_response()
}
