use axum::response::IntoResponse;
use serde::Serialize;
use crate::interface_adapters::http::v1::presenters::common::presenter_output::PresenterOutput;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;
use crate::domain::entities::profile::announce::announce::Announce;

#[derive(Debug, Clone, Serialize)]
pub struct AnnounceResult {
    pub items: Vec<Announce>,
}

impl PresenterOutput for AnnounceResult {
    fn into_response(self) -> impl IntoResponse {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
    }
}
