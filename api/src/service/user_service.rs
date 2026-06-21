use crate::dto::auth::Claims;
use crate::dto::pagination::{OrderDir, PaginatedResponse, PaginationParams};
use crate::dto::user::{
    AssignRolesRequest, ChangePasswordRequest, RoleBrief, UpdateUserRequest, UserDetail,
    UserRolesAndPermissions,
};
use crate::dto::user::{UserBrief, UserFilters};
use crate::error::{AppError, AppResult};
use crate::middleware::current_user::CurrentUser;
use crate::service::role_service::RoleService;
use chrono::{TimeZone, Utc};
use model::{Permission, Role, User, role_permission, user, user_role};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use std::collections::HashSet;
use util::password::PasswordUtil;

pub struct UserService;

impl UserService {
    pub async fn get_user_roles_and_permissions(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<UserRolesAndPermissions> {
        let roles: Vec<RoleBrief> = Self::get_user_roles(db, user_id).await?;

        let perms: Vec<String> = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(roles.iter().map(|r| &r.id)))
            .find_also_related(Permission)
            .all(db)
            .await?
            .into_iter()
            .filter_map(|(_, p)| p)
            .map(|p| format!("{}:{}", p.resource, p.action))
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        Ok(UserRolesAndPermissions {
            roles,
            permissions: perms,
        })
    }

    pub async fn get_user_roles(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<Vec<RoleBrief>> {
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(
                Condition::any()
                    .add(user_role::Column::ExpiresAt.is_null())
                    .add(user_role::Column::ExpiresAt.gt(Utc::now())),
            )
            .find_also_related(Role)
            .all(db)
            .await?;

        Ok(user_roles
            .into_iter()
            .filter_map(|(_, r)| r)
            .map(|r| RoleBrief {
                id: r.id,
                name: r.name,
                display_name: r.display_name,
            })
            .collect())
    }

    pub async fn list(
        db: &DatabaseConnection,
        pagination: PaginationParams,
        filters: UserFilters,
    ) -> AppResult<PaginatedResponse<UserBrief>> {
        let page = pagination.page;
        let page_size = pagination.limit();
        let offset = pagination.offset();

        let mut query = User::find().filter(user::Column::DeletedAt.is_null());
        if let Some(username) = filters.username {
            query = query.filter(user::Column::Username.contains(username));
        };
        if let Some(email) = filters.email {
            query = query.filter(user::Column::Email.contains(email));
        };
        if let Some(staus) = filters.status {
            query = query.filter(user::Column::Status.eq(staus));
        };

        let total = query.clone().count(db).await?;

        if let Some(order) = pagination.order_by {
            query = match pagination.order_dir {
                OrderDir::Asc => query.order_by_asc(Expr::col(order)),
                OrderDir::Desc => query.order_by_desc(Expr::col(order)),
            };
        } else {
            query = query.order_by_desc(user::Column::UpdatedAt);
        }
        let users: Vec<UserBrief> = query
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(PaginatedResponse::new_total(total, page, page_size, users))
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> AppResult<Option<UserDetail>> {
        let user = user::Entity::find_by_id(id)
            .filter(user::Column::DeletedAt.is_null())
            .one(db)
            .await?;
        if let Some(u) = user {
            Ok(Some(UserDetail {
                user_brief: u.into(),
                user_roles_and_permissions: Self::get_user_roles_and_permissions(db, id).await?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        req: UpdateUserRequest,
    ) -> AppResult<UserBrief> {
        let user = Self::get_user(db, id).await?;

        let mut model: user::ActiveModel = user.into();
        if let Some(name) = req.display_name {
            model.display_name = Set(Some(name));
        }
        if let Some(email) = req.email {
            let existing = User::find()
                .filter(user::Column::Email.eq(&email))
                .filter(user::Column::Id.ne(id))
                .filter(user::Column::DeletedAt.is_null())
                .one(db)
                .await?;
            if existing.is_some() {
                return Err(AppError::EmailExists);
            }
            model.email = Set(email);
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(Utc::now());

        let updated = model.update(db).await?;
        Ok(updated.into())
    }

    pub async fn get_user(db: &DatabaseConnection, id: &str) -> AppResult<user::Model> {
        Ok(User::find_by_id(id)
            .filter(user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?)
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> AppResult<()> {
        let user = Self::get_user(db, id).await?;

        let mut model: user::ActiveModel = user.into();
        model.deleted_at = Set(Some(Utc::now()));
        model.update(db).await?;
        Ok(())
    }

    pub async fn assign_roles(
        db: &DatabaseConnection,
        user_id: &str,
        req: AssignRolesRequest,
        current_user: &CurrentUser,
    ) -> AppResult<Vec<RoleBrief>> {
        Self::get_user(db, user_id).await?;

        user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        for role_id in &req.role_ids {
            let role = RoleService::get_role(db, role_id).await;
            if role.is_err() {
                return Err(AppError::BadRequest(format!("Role {} not found", role_id)));
            }
            user_role::ActiveModel {
                user_id: Set(user_id.to_owned()),
                role_id: Set(role_id.to_owned()),
                assigned_by: Set(Some(current_user.0.sub.to_string())),
                assigned_at: Set(Utc::now()),
                expires_at: Set(req
                    .expires_at
                    .and_then(|e| Utc.timestamp_millis_opt(e).single())),
            }
            .insert(db)
            .await?;
        }

        Ok(Self::get_user_roles(db, user_id).await?)
    }

    pub async fn remove_role(
        db: &DatabaseConnection,
        user_id: &str,
        role_id: &str,
    ) -> AppResult<()> {
        user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(user_role::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn change_password(
        db: &DatabaseConnection,
        req: ChangePasswordRequest,
        user_id: &str,
    ) -> AppResult<()> {
        let user = Self::get_user(db, user_id).await?;
        if PasswordUtil::verify(&req.old_password, &user.password_hash).is_err() {
            return Err(AppError::InvalidCredentials);
        }
        let new_pass = PasswordUtil::hash(&req.new_password).map_err(|_| AppError::Internal)?;
        let mut model: user::ActiveModel = user.into();
        model.password_hash = Set(new_pass);
        model.update(db).await?;
        Ok(())
    }
}
