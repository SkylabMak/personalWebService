use std::fmt::{Display, Formatter};
use axum::response::IntoResponse;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ApplicationError {
    NotFound {
        resource: &'static str,
        identifier: String,
    },
    ValidationError {
        message: String,
    },
    Unauthorized,
    Forbidden,
    Conflict {
        message: String,
    },
    Internal {
        message: String,
    },
    Unexpected {
        message: String,
    },
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::NotFound { resource, identifier } => {
                write!(f, "{} not found: {}", resource, identifier)
            }
            ApplicationError::ValidationError { message } => {
                write!(f, "Validation error: {}", message)
            }
            ApplicationError::Unauthorized => {
                write!(f, "Unauthorized")
            }
            ApplicationError::Forbidden => {
                write!(f, "Forbidden")
            }
            ApplicationError::Conflict { message } => {
                write!(f, "Conflict: {}", message)
            }
            ApplicationError::Internal { message } => {
                write!(f, "Internal error: {}", message)
            }
            ApplicationError::Unexpected { message } => {
                write!(f, "Unexpected error: {}", message)
            }
        }
    }
}

impl std::error::Error for ApplicationError {}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        crate::interface_adapters::http::v1::presenters::common::error_presenter::ErrorPresenter::present(self).into_response()
    }
}

pub trait MapToApplicationError<T> {
    fn map_app_err(self, message: &str) -> Result<T, ApplicationError>;
}

impl<T, E: std::fmt::Debug> MapToApplicationError<T> for Result<T, E> {
    fn map_app_err(self, message: &str) -> Result<T, ApplicationError> {
        self.map_err(|e| {
            tracing::error!("{}: {:?}", message, e);
            ApplicationError::Internal {
                message: message.to_string(),
            }
        })
    }
}
