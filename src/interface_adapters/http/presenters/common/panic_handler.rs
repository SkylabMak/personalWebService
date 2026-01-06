use std::any::Any;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use crate::interface_adapters::http::presenters::common::api_response::ApiResponse;

pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let details = if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic".to_string()
    };

    let response: ApiResponse<()> = ApiResponse::error(
        "INTERNAL_SERVER_ERROR",
        &format!("A critical error occurred: {}", details),
    );

    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
}
