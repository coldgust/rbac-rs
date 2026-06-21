use crate::AppState;
use crate::dto::pagination::PaginatedResponse;
use crate::dto::response::ApiResponse;
use crate::dto::user::{
    AssignRolesRequest, ChangePasswordRequest, RoleBrief, UpdateUserRequest, UserBrief, UserDetail,
    UserListQuery,
};
use crate::error::{AppError, AppResult};
use crate::middleware::current_user::CurrentUser;
use crate::service::user_service::UserService;
use axum::Json;
use axum::extract::{Path, Query, State};
use std::sync::Arc;

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(query): Query<UserListQuery>,
) -> AppResult<ApiResponse<PaginatedResponse<UserBrief>>> {
    let res = UserService::list(&state.db, query.pagination, query.filters).await?;
    Ok(ApiResponse::success(res))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<Option<UserDetail>>> {
    let res = UserService::find_by_id(&state.db, &id).await?;
    Ok(ApiResponse::success(res))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<ApiResponse<UserBrief>> {
    let res = UserService::update(&state.db, &id, req).await?;
    Ok(ApiResponse::success(res))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<()>> {
    UserService::delete(&state.db, &id).await?;
    Ok(ApiResponse::success(()))
}

pub async fn assign_roles(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<AssignRolesRequest>,
) -> AppResult<ApiResponse<Vec<RoleBrief>>> {
    let res = UserService::assign_roles(&state.db, &id, req, &current_user).await?;
    Ok(ApiResponse::success(res))
}

pub async fn remove_role(
    State(state): State<Arc<AppState>>,
    Path((user_id, role_id)): Path<(String, String)>,
) -> AppResult<ApiResponse<()>> {
    UserService::remove_role(&state.db, &user_id, &role_id).await?;
    Ok(ApiResponse::success_empty())
}
pub async fn me(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
) -> AppResult<ApiResponse<UserDetail>> {
    let user = UserService::find_by_id(&state.db, &current_user.0.sub)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(ApiResponse::success(user))
}

pub async fn change_password(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<ApiResponse<()>> {
    Ok(ApiResponse::success(
        UserService::change_password(&state.db, req, &current_user.0.sub).await?,
    ))
}
