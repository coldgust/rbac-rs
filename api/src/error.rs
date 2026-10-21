use crate::dto::response::ApiResponse;
use axum::Json;
use axum::http::StatusCode;
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
    #[error("Access token expired")]
    TokenExpired,
    #[error("Invalid refresh token")]
    InvalidRefreshToken,
    #[error("Refresh token reused")]
    RefreshTokenReused,
    #[error("Refresh already in progress")]
    RefreshInProgress,
    #[error("Session expired")]
    SessionExpired,
    #[error("Redis pool error: {0}")]
    Redis(#[from] deadpool_redis::PoolError),
    #[error("Redis command error: {0}")]
    RedisCmd(#[from] redis::RedisError),
    #[error("Redis pool create error: {0}")]
    RedisPoolCreatError(#[from] deadpool_redis::CreatePoolError),
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials
            | Self::Unauthorized
            | Self::TokenExpired
            | Self::InvalidRefreshToken
            | Self::RefreshTokenReused
            | Self::SessionExpired => StatusCode::UNAUTHORIZED,
            Self::AccountDisabled | Self::Forbidden => StatusCode::FORBIDDEN,
            Self::RefreshInProgress | Self::UsernameExists | Self::EmailExists => {
                StatusCode::CONFLICT
            }
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Redis(_) | Self::RedisCmd(_) | Self::RedisPoolCreatError(_) => {
                StatusCode::SERVICE_UNAVAILABLE
            }
            Self::Database(_) | Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

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
            Self::TokenExpired => (10003, "Access token expired".to_string()),
            Self::InvalidRefreshToken => (10005, "Invalid refresh token".to_string()),
            Self::RefreshTokenReused => (10006, "Refresh token reused".to_string()),
            Self::RefreshInProgress => (10007, "Refresh already in progress".to_string()),
            Self::SessionExpired => (10008, "Session expired".to_string()),
            Self::Redis(_) | Self::RedisCmd(_) | Self::RedisPoolCreatError(_) => {
                (50002, "Session storage unavailable".to_string())
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let (code, msg) = self.to_error_parts();
        (status, Json(ApiResponse::<()>::error(code, msg))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
