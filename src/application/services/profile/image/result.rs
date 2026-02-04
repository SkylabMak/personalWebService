use serde::Serialize;
use axum::response::{IntoResponse, Response};
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct ImageResult {
    pub id: String,
    pub storage_url: String,
    pub filename: String,
    pub original_filename: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: i32,
    pub mime_type: String,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub created_at: String,
    pub usage_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImageListResult {
    pub images: Vec<ImageResult>,
    pub total: usize,
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImageUsageResult {
    pub image_id: String,
    pub total_usage: i32,
    pub performances: Vec<PerformanceUsageInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceUsageInfo {
    pub performance_id: String,
    pub title: String,
    pub usage_count: i32,
    pub first_used_at: String,
    pub last_used_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnusedImagesResult {
    pub unused_images: Vec<ImageResult>,
    pub total_size_bytes: i64,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteUnusedImagesResult {
    pub deleted_count: i64,
    pub freed_bytes: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageResult {
    pub message: String,
    pub id: Option<String>,
}

impl IntoResponse for UnusedImagesResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

impl IntoResponse for DeleteUnusedImagesResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

impl IntoResponse for MessageResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

impl IntoResponse for ImageResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

impl IntoResponse for ImageListResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

impl IntoResponse for ImageUsageResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}
