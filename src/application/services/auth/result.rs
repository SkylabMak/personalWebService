use serde::Serialize;
use axum::response::{IntoResponse, Response};
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct AuthUserResponse {
    pub id: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: AuthUserResponse,
}

impl IntoResponse for LoginResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RefreshResult {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl IntoResponse for RefreshResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LogoutResult {
    pub message: String,
}

impl IntoResponse for LogoutResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MeResult {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl IntoResponse for MeResult {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::OK,
            axum::Json(ApiResponse::success(self)),
        )
            .into_response()
    }
}
