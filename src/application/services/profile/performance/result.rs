use serde::Serialize;
use axum::response::IntoResponse;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceResult {
    pub id: String,
    pub title: String,
    pub content_url: Option<String>,
    pub images_tracked: usize,
    pub created_at: String,
}

impl IntoResponse for PerformanceResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceUpdateResult {
    pub id: String,
    pub title: String,
    pub images_added: usize,
    pub images_removed: usize,
    pub updated_at: String,
}

impl IntoResponse for PerformanceUpdateResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceDeleteResult {
    pub message: String,
    pub deleted_id: String,
}

impl IntoResponse for PerformanceDeleteResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceContentResult {
    pub content_markdown: String,
}

impl IntoResponse for PerformanceContentResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceContentUpdateResult {
    pub performance_id: String,
    pub content_url: String,
    pub images_added: usize,
    pub images_removed: usize,
}

impl IntoResponse for PerformanceContentUpdateResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

use crate::domain::entities::profile::performance::performance::Performance;

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceListResult {
    pub performances: Vec<Performance>,
}

impl IntoResponse for PerformanceListResult {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}
