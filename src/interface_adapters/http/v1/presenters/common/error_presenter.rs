use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::application::errors::ApplicationError;
use crate::interface_adapters::http::v1::presenters::common::api_response::ApiResponse;

pub struct ErrorPresenter;

impl ErrorPresenter {
    pub fn present(error: ApplicationError) -> impl IntoResponse {
        let (status, code, message): (StatusCode, &str, String) = match error {
            ApplicationError::NotFound { resource, identifier } => {
                (StatusCode::NOT_FOUND, "NOT_FOUND", format!("{} not found: {}", resource, identifier))
            }
            ApplicationError::ValidationError { message } => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message)
            }
            ApplicationError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Unauthorized".to_string())
            }
            ApplicationError::Forbidden => {
                (StatusCode::FORBIDDEN, "FORBIDDEN", "Forbidden".to_string())
            }
            ApplicationError::Conflict { message } => {
                (StatusCode::CONFLICT, "CONFLICT", message)
            }
            ApplicationError::Internal { message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message)
            }
            ApplicationError::Unexpected { message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR", message)
            }
        };

        let response: ApiResponse<()> = ApiResponse::error(code, &message);
        (status, Json(response))
    }
}
