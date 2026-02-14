use axum::response::IntoResponse;
use serde::Serialize;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;
use crate::domain::entities::profile::profile::Profile;

#[derive(Debug, Clone, Serialize)]
pub struct ProfileResult {
    pub profile: Profile,
}

impl IntoResponse for ProfileResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}
