use axum::response::IntoResponse;
use serde::Serialize;
use crate::interface_adapters::http::v1::presenters::common::presenter_output::PresenterOutput;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct FeatureStatusResult {
    pub feature_code: String,
    pub feature_name: String,
    pub status_name: String,
    pub is_closed: bool,
    pub updated_at: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebsiteFeatureStatusesResult {
    pub features: Vec<FeatureStatusResult>,
}

impl PresenterOutput for WebsiteFeatureStatusesResult {
    fn into_response(self) -> impl IntoResponse {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
    }
}
