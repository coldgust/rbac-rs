use crate::dto::pagination::{PaginatedResponse, PaginationParams};
use crate::dto::permission::PermissionBrief;
use crate::dto::role::{
    CreateRoleRequest, RoleDetailResponse, RoleFilters, RoleResponse, UpdateRoleRequest,
};
use crate::error::{AppError, AppResult};
use crate::middleware::current_user::CurrentUser;
use crate::service::permission_service::PermissionService;
use chrono::Utc;
use model::{Permission, Role, permission, role, role_permission};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use tokio::task::id;
use uuid::Uuid;

pub struct RoleService;

impl RoleService {
    pub async fn get_role(db: &DatabaseConnection, id: &str) -> AppResult<role::Model> {
        Ok(role::Entity::find_by_id(id)
            .filter(role::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?)
    }

    pub async fn list(
        db: &DatabaseConnection,
        pagination: PaginationParams,
        role_filters: RoleFilters,
    ) -> AppResult<PaginatedResponse<RoleResponse>> {
        let page = pagination.page;
        let page_size = pagination.limit();
        let offset = pagination.offset();

        let mut query = Role::find().filter(role::Column::DeletedAt.is_null());
        if let Some(name) = role_filters.name {
            query = query.filter(role::Column::Name.contains(&name));
        }

        let total = query.clone().count(db).await?;

        let roles = query
            .order_by_asc(role::Column::SortOrder)
            .order_by_asc(role::Column::Name)
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await?;

        Ok(PaginatedResponse::new_total(
            total,
            page,
            page_size,
            roles.into_iter().map(|r| r.into()).collect(),
        ))
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> AppResult<Option<RoleDetailResponse>> {
        let role = Self::get_role(db, id).await.ok();

        if let Some(role) = role {
            let permissions = role.find_related(Permission).all(db).await?;
            let perm_briefs = permissions.into_iter().map(|p| p.into()).collect();

            Ok(Some(RoleDetailResponse {
                role: role.into(),
                permissions: perm_briefs,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create(
        db: &DatabaseConnection,
        req: CreateRoleRequest,
    ) -> AppResult<RoleResponse> {
        let existing = Role::find()
            .filter(role::Column::Name.eq(&req.name))
            .filter(role::Column::DeletedAt.is_null())
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(AppError::BadRequest("Role name already exists".to_string()));
        }

        let new_role = role::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(req.name),
            display_name: Set(req.display_name),
            description: Set(req.description),
            sort_order: Set(req.sort_order.unwrap_or(0)),
            is_system: Set(false),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            deleted_at: Set(None),
        }
        .insert(db)
        .await?;

        Ok(new_role.into())
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        req: UpdateRoleRequest,
    ) -> AppResult<RoleResponse> {
        let role = Self::get_role(db, id).await?;

        if role.is_system {
            return Err(AppError::BadRequest(
                "Cannot modify system role".to_string(),
            ));
        }

        let mut active: role::ActiveModel = role.into();

        if let Some(display_name) = req.display_name {
            active.display_name = Set(Some(display_name));
        }
        if let Some(description) = req.description {
            active.description = Set(Some(description));
        }
        if let Some(sort_order) = req.sort_order {
            active.sort_order = Set(sort_order);
        }
        active.updated_at = Set(Utc::now());

        let updated = active.update(db).await?;

        Ok(updated.into())
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResult<()> {
        let role = Self::get_role(db, id).await?;

        if role.is_system {
            return Err(AppError::BadRequest(
                "Cannot delete system role".to_string(),
            ));
        }

        let mut active: role::ActiveModel = role.into();
        active.deleted_at = Set(Some(Utc::now()));
        active.update(db).await?;
        Ok(())
    }

    pub async fn assign_permissions(
        db: &DatabaseConnection,
        role_id: &str,
        permission_ids: &[String],
        current_user: &CurrentUser,
    ) -> AppResult<Vec<PermissionBrief>> {
        let role = Self::get_role(db, role_id).await?;

        if role.is_system {
            return Err(AppError::BadRequest(
                "Cannot modify system role permissions".to_string(),
            ));
        }

        for perm_id in permission_ids {
            let exists = PermissionService::get_permission(db, perm_id).await.is_ok();
            if !exists {
                return Err(AppError::BadRequest(format!(
                    "Permission {} not found",
                    perm_id
                )));
            }
        }

        role_permission::Entity::delete_many()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;

        for perm_id in permission_ids {
            role_permission::ActiveModel {
                role_id: Set(role_id.to_owned()),
                permission_id: Set(perm_id.clone()),
                granted_by: Set(Some(current_user.0.sub.to_string())),
                granted_at: Set(Utc::now()),
            }
            .insert(db)
            .await?;
        }

        Ok(Self::find_by_id(db, role_id)
            .await?
            .ok_or(AppError::NotFound)?
            .permissions)
    }

    pub async fn remove_permission(
        db: &DatabaseConnection,
        role_id: &str,
        permission_id: &str,
    ) -> AppResult<()> {
        let role = Self::get_role(db, role_id).await?;

        if role.is_system {
            return Err(AppError::BadRequest(
                "Cannot modify system role permissions".to_string(),
            ));
        }

        role_permission::Entity::delete_many()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .filter(role_permission::Column::PermissionId.eq(permission_id))
            .exec(db)
            .await?;

        Ok(())
    }
}
