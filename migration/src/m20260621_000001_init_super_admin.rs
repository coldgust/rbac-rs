use crate::sea_orm::{ActiveModelTrait, Set};
use crate::{DbErr, DeriveMigrationName, async_trait};
use model::user::UserStatus;
use model::{role, user, user_role};
use sea_orm_migration::prelude::prelude::Utc;
use sea_orm_migration::prelude::*;
use std::env;
use std::string::ToString;
use util::password::PasswordUtil;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SUPER_ADMIN: &str = "super_admin";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let role = role::Entity::find_by_name(SUPER_ADMIN)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("super_admin not found".to_string()))?;

        let name = env::var("APP_ADMIN_NAME").unwrap_or("admin".into());
        let pass = env::var("APP_ADMIN_PASS").unwrap_or("admin".into());
        let hash = PasswordUtil::hash(&pass)
            .map_err(|_| DbErr::RecordNotFound("hash error".to_string()))?;
        let user = user::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            username: Set(name.to_owned()),
            email: Set("".into()),
            password_hash: Set(hash),
            display_name: Set(None),
            status: Set(UserStatus::Active as i16),
            last_login_at: Set(None),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            deleted_at: Set(None),
        }
        .insert(db)
        .await?;

        user_role::ActiveModel {
            user_id: Set(user.id.to_string()),
            role_id: Set(role.id),
            assigned_by: Set(None),
            assigned_at: Set(Utc::now()),
            expires_at: Set(None),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
