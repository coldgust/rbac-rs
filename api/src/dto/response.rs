use crate::error::AppError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 0,
            msg: "".into(),
            data: Some(data),
        }
    }

    pub fn success_empty() -> Self {
        ApiResponse {
            code: 0,
            msg: "".into(),
            data: None,
        }
    }

    pub fn error(code: impl Into<i32>, msg: impl Into<String>) -> Self {
        ApiResponse {
            code: code.into(),
            msg: msg.into(),
            data: None,
        }
    }
}

impl<T: Serialize> From<AppError> for ApiResponse<T> {
    fn from(value: AppError) -> Self {
        let (code, msg) = value.to_error_parts();
        ApiResponse {
            code,
            msg,
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
