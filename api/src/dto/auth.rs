use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::Utc;

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
    pub expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // user id
    pub roles: Vec<String>, // role names
    pub perms: Vec<String>, // permissions
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: &str, roles: Vec<String>, perms: Vec<String>, exp: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: user_id.to_owned(),
            roles,
            perms,
            exp,
            iat: now,
        }
    }
}