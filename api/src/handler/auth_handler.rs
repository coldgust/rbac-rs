use crate::dto::auth::{LoginRequest, RegisterRequest, TokenResponse};
use crate::dto::response::ApiResponse;
use crate::error::AppResult;
use crate::service::auth_service::AuthService;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use std::sync::Arc;
use crate::dto::user::UserBrief;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<ApiResponse<UserBrief>> {
    let user = AuthService::register(&state.db, req).await?;
    Ok(ApiResponse::success(Into::<UserBrief>::into(user)))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<ApiResponse<TokenResponse>> {
    Ok(ApiResponse::success(
        AuthService::login(&state.db, &state.config, req).await?,
    ))
}
