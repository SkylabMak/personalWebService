use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::interface_adapters::http::presenters::common::api_response::ApiResponse;

pub async fn fallback_handler() -> impl IntoResponse {
    let response: ApiResponse<()> = ApiResponse::error(
        "NOT_FOUND",
        "The requested resource was not found on this server.",
    );

    (StatusCode::NOT_FOUND, Json(response))
}
