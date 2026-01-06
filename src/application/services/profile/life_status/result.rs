use axum::response::IntoResponse;
use serde::Serialize;
use crate::interface_adapters::http::presenters::common::presenter_output::PresenterOutput;
use crate::interface_adapters::http::presenters::common::api_response::ApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct LifeStatusResult {
    pub name: String,
    pub description: Option<String>,
    pub color_token: String,
}

impl PresenterOutput for LifeStatusResult {
    fn into_response(self) -> impl IntoResponse {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
    }
}
