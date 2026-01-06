use std::fmt::{Display, Formatter};

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
            ApplicationError::Unexpected { message } => {
                write!(f, "Unexpected error: {}", message)
            }
        }
    }
}

impl std::error::Error for ApplicationError {}
