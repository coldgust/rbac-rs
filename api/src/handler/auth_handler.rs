use crate::AppState;
use crate::dto::auth::{
    LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest, TokenResponse,
};
use crate::dto::response::ApiResponse;
use crate::dto::user::UserBrief;
use crate::error::AppResult;
use crate::service::auth_service::AuthService;
use crate::service::token_service::ClientContext;
use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use std::sync::Arc;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<ApiResponse<UserBrief>> {
    let user = AuthService::register(&state.db, req).await?;
    Ok(ApiResponse::success(Into::<UserBrief>::into(user)))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> AppResult<ApiResponse<TokenResponse>> {
    let ctx = ClientContext::from_headers(&headers);
    Ok(ApiResponse::success(
        AuthService::login(&state.db, &state.redis, &state.config, req, &ctx).await?,
    ))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<RefreshRequest>,
) -> AppResult<ApiResponse<TokenResponse>> {
    let ctx = ClientContext::from_headers(&headers);
    Ok(ApiResponse::success(
        AuthService::refresh(&state.db, &state.redis, &state.config, req, &ctx).await?,
    ))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LogoutRequest>,
) -> AppResult<StatusCode> {
    AuthService::logout(&state.redis, req).await?;
    Ok(StatusCode::NO_CONTENT)
}
