use crate::AppState;
use crate::dto::pagination::PaginatedResponse;
use crate::dto::permission::PermissionBrief;
use crate::dto::response::ApiResponse;
use crate::dto::role::{
    AssignPermissionsRequest, CreateRoleRequest, RoleDetailResponse, RoleListQuery, RoleResponse,
    UpdateRoleRequest,
};
use crate::error::{AppError, AppResult};
use crate::middleware::current_user::CurrentUser;
use crate::service::role_service::RoleService;
use axum::Json;
use axum::extract::{Path, Query, State};
use std::sync::Arc;

pub async fn list_roles(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RoleListQuery>,
) -> AppResult<ApiResponse<PaginatedResponse<RoleResponse>>> {
    let result = RoleService::list(&state.db, query.pagination, query.filters).await?;
    Ok(ApiResponse::success(result))
}

pub async fn get_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<RoleDetailResponse>> {
    let role = RoleService::find_by_id(&state.db, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(ApiResponse::success(role))
}

pub async fn create_role(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateRoleRequest>,
) -> AppResult<ApiResponse<RoleResponse>> {
    let role = RoleService::create(&state.db, req).await?;
    Ok(ApiResponse::success(role))
}

pub async fn update_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> AppResult<ApiResponse<RoleResponse>> {
    let role = RoleService::update(&state.db, &id, req).await?;
    Ok(ApiResponse::success(role))
}

pub async fn delete_role(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<()>> {
    RoleService::delete(&state.db, &id).await?;
    Ok(ApiResponse::success_empty())
}

pub async fn assign_permissions(
    current_user: CurrentUser,
    State(state): State<Arc<AppState>>,
    Path(role_id): Path<String>,
    Json(req): Json<AssignPermissionsRequest>,
) -> AppResult<ApiResponse<Vec<PermissionBrief>>> {
    let perms =
        RoleService::assign_permissions(&state.db, &role_id, &req.permission_ids, &current_user)
            .await?;
    Ok(ApiResponse::success(perms))
}

pub async fn remove_permission(
    State(state): State<Arc<AppState>>,
    Path((role_id, perm_id)): Path<(String, String)>,
) -> AppResult<ApiResponse<()>> {
    RoleService::remove_permission(&state.db, &role_id, &perm_id).await?;
    Ok(ApiResponse::success_empty())
}
