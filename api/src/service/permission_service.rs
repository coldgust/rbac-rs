use crate::dto::pagination::{PaginatedResponse, PaginationParams};
use crate::dto::permission::{
    CreatePermissionRequest, PermissionFilters, PermissionResponse, UpdatePermissionRequest,
};
use crate::error::{AppError, AppResult};
use chrono::Utc;
use model::{Permission, permission, role};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

pub struct PermissionService;

impl PermissionService {
    pub async fn get_permission(db: &DatabaseConnection, id: &str) -> AppResult<permission::Model> {
        Ok(permission::Entity::find_by_id(id)
            .filter(permission::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?)
    }

    pub async fn list(
        db: &DatabaseConnection,
        pagination: PaginationParams,
        permission_filters: PermissionFilters,
    ) -> AppResult<PaginatedResponse<PermissionResponse>> {
        let page = pagination.page;
        let page_size = pagination.limit();
        let offset = pagination.offset();

        let mut query = Permission::find().filter(permission::Column::DeletedAt.is_null());
        if let Some(resource) = permission_filters.resource {
            query = query.filter(permission::Column::Resource.eq(resource));
        }

        let total = query.clone().count(db).await?;
        let perms = query
            .order_by_asc(permission::Column::Resource)
            .order_by_asc(permission::Column::Action)
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await?;

        let items = perms.into_iter().map(|p| p.into()).collect();

        Ok(PaginatedResponse::new_total(total, page, page_size, items))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> AppResult<Option<PermissionResponse>> {
        Ok(Self::get_permission(db, id).await.map(|p| p.into()).ok())
    }

    pub async fn create(
        db: &DatabaseConnection,
        req: CreatePermissionRequest,
    ) -> AppResult<PermissionResponse> {
        let existing = Permission::find()
            .filter(permission::Column::Resource.eq(&req.resource))
            .filter(permission::Column::Action.eq(&req.action))
            .filter(permission::Column::DeletedAt.is_null())
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(AppError::BadRequest(
                "Permission already exists (resource+action)".to_string(),
            ));
        }

        let new_perm = permission::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            resource: Set(req.resource),
            action: Set(req.action),
            name: Set(req.name),
            description: Set(req.description),
            resource_path: Set(req.resource_path),
            http_method: Set(req.http_method),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            deleted_at: Set(None),
        }
        .insert(db)
        .await?;

        Ok(new_perm.into())
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        req: UpdatePermissionRequest,
    ) -> AppResult<PermissionResponse> {
        let perm = Self::get_permission(db, id).await?;

        let mut active: permission::ActiveModel = perm.into();
        if let Some(name) = req.name {
            active.name = Set(name);
        }
        if let Some(description) = req.description {
            active.description = Set(Some(description));
        }
        if let Some(resource_path) = req.resource_path {
            active.resource_path = Set(Some(resource_path));
        }
        if let Some(http_method) = req.http_method {
            active.http_method = Set(Some(http_method));
        }
        active.updated_at = Set(Utc::now());

        let updated = active.update(db).await?;

        Ok(updated.into())
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResult<()> {
        let perm = Self::get_permission(db, id).await?;

        let mut active: permission::ActiveModel = perm.into();
        active.deleted_at = Set(Some(Utc::now()));
        active.update(db).await?;
        Ok(())
    }
}
