use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 64,
        message = "Username must be between 3 and 64 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    /// Access Token 剩余秒数。
    pub expires_in: i64,
    /// Refresh Token 剩余秒数。
    pub refresh_expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub perms: Vec<String>,
    /// 固定为 "access"，用于阻止 Refresh/旧令牌被当作 Access Token 使用。
    pub token_use: String,
    pub jti: String,
    pub iss: String,
    pub aud: String,
    /// 秒级时间戳。
    pub iat: i64,
    /// 秒级时间戳。
    pub exp: i64,
}
