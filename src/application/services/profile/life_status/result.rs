use axum::response::IntoResponse;
use serde::Serialize;
use crate::interface_adapters::http::v1::presenters::common::presenter_output::PresenterOutput;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;

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
            .into_response()
    }
}

impl IntoResponse for LifeStatusResult {
    fn into_response(self) -> axum::response::Response {
        PresenterOutput::into_response(self).into_response()
    }
}
