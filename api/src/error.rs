use crate::dto::response::ApiResponse;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Resource not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Internal server error")]
    Internal,
    #[error("Account disabled")]
    AccountDisabled,
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Username already exists")]
    UsernameExists,
    #[error("Email already registered")]
    EmailExists,
}

impl AppError {
    pub fn to_error_parts(&self) -> (i32, String) {
        match self {
            Self::Unauthorized => (10002, "Unauthorized".to_string()),
            Self::Forbidden => (40002, "Forbidden".to_string()),
            Self::NotFound => (30003, "Resource not found".to_string()),
            Self::BadRequest(msg) => (40001, msg.clone()),
            Self::Database(_) => (50001, "Database error".to_string()),
            Self::Internal => (50001, "Internal server error".to_string()),
            Self::AccountDisabled => (10004, "Account disabled".to_string()),
            Self::InvalidCredentials => (10001, "Invalid username or password".to_string()),
            Self::UsernameExists => (20001, "Username already exists".to_string()),
            Self::EmailExists => (20002, "Email already registered".to_string()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ApiResponse::<()>::from(self).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
