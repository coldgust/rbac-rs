use crate::AppState;
use crate::dto::pagination::PaginatedResponse;
use crate::dto::permission::{
    CreatePermissionRequest, PermissionListQuery, PermissionResponse, UpdatePermissionRequest,
};
use crate::dto::response::ApiResponse;
use crate::error::{AppError, AppResult};
use crate::service::permission_service::PermissionService;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use std::sync::Arc;

pub async fn list_permissions(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PermissionListQuery>,
) -> AppResult<ApiResponse<PaginatedResponse<PermissionResponse>>> {
    let result = PermissionService::list(&state.db, query.pagination, query.filters).await?;
    Ok(ApiResponse::success(result))
}

pub async fn get_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<PermissionResponse>> {
    let perm = PermissionService::find_by_id(&state.db, &id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(ApiResponse::success(perm))
}

pub async fn create_permission(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePermissionRequest>,
) -> AppResult<ApiResponse<PermissionResponse>> {
    let perm = PermissionService::create(&state.db, req).await?;
    Ok(ApiResponse::success(perm))
}

pub async fn update_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePermissionRequest>,
) -> AppResult<ApiResponse<PermissionResponse>> {
    let perm = PermissionService::update(&state.db, &id, req).await?;
    Ok(ApiResponse::success(perm))
}

pub async fn delete_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AppResult<ApiResponse<()>> {
    PermissionService::delete(&state.db, &id).await?;
    Ok(ApiResponse::success_empty())
}
