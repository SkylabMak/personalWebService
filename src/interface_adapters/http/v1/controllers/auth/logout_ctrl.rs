use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::delivery::http::server::state::AppState;

#[derive(Deserialize)]
pub struct LogoutInput {
    pub refresh_token: String,
}

pub async fn logout_ctrl(
    State(state): State<AppState>,
    Json(payload): Json<LogoutInput>,
) -> impl IntoResponse {
    let res = state.auth.auth.logout(&payload.refresh_token).await;
    res.into_response()
}
