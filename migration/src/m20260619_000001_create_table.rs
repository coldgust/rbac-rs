use model::{permission, role, role_permission, user, user_role};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_user(manager).await?;
        self.create_role(manager).await?;
        self.create_permission(manager).await?;
        self.create_user_role(manager).await?;
        self.create_role_permission(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(role::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(permission::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(user_role::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(role_permission::Entity).to_owned())
            .await?;
        Ok(())
    }
}

impl Migration {
    async fn create_user(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(user::Column::Email).string())
                    .col(ColumnDef::new(user::Column::DisplayName).string())
                    .col(
                        ColumnDef::new(user::Column::Status)
                            .small_integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(user::Column::LastLoginAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(user::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(user::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(user::Column::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn create_role(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(role::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(role::Column::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(role::Column::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(role::Column::DisplayName).string())
                    .col(ColumnDef::new(role::Column::Description).text())
                    .col(ColumnDef::new(role::Column::SortOrder).integer().default(0))
                    .col(
                        ColumnDef::new(role::Column::IsSystem)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(role::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(role::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(role::Column::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn create_permission(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(permission::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(permission::Column::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(permission::Column::Resource)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(permission::Column::Action)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(permission::Column::Name).string().not_null())
                    .col(ColumnDef::new(permission::Column::Description).text())
                    .col(ColumnDef::new(permission::Column::ResourcePath).string())
                    .col(ColumnDef::new(permission::Column::HttpMethod).string())
                    .col(
                        ColumnDef::new(permission::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(permission::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(permission::Column::DeletedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_permissions_resource_action_unique")
                    .table(permission::Entity)
                    .col(permission::Column::Resource)
                    .col(permission::Column::Action)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn create_user_role(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user_role::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_role::Column::UserId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_role::Column::RoleId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(ColumnDef::new(user_role::Column::AssignedBy).string_len(36))
                    .col(
                        ColumnDef::new(user_role::Column::AssignedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(user_role::Column::ExpiresAt).timestamp_with_time_zone())
                    .primary_key(
                        Index::create()
                            .col(user_role::Column::UserId)
                            .col(user_role::Column::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_user_id")
                            .from(user_role::Entity, user_role::Column::UserId)
                            .to(user::Entity, user::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_role_id")
                            .from(user_role::Entity, user_role::Column::RoleId)
                            .to(role::Entity, role::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_assigned_by")
                            .from(user_role::Entity, user_role::Column::AssignedBy)
                            .to(user::Entity, user::Column::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_roles_user_id")
                    .table(user_role::Entity)
                    .col(user_role::Column::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_roles_role_id")
                    .table(user_role::Entity)
                    .col(user_role::Column::RoleId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_roles_expires_at")
                    .table(user_role::Entity)
                    .col(user_role::Column::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn create_role_permission(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(role_permission::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(role_permission::Column::RoleId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role_permission::Column::PermissionId)
                            .string_len(36)
                            .not_null(),
                    )
                    .col(ColumnDef::new(role_permission::Column::GrantedBy).string_len(36))
                    .col(
                        ColumnDef::new(role_permission::Column::GrantedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(role_permission::Column::RoleId)
                            .col(role_permission::Column::PermissionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_role_id")
                            .from(role_permission::Entity, role_permission::Column::RoleId)
                            .to(role::Entity, role::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_perm_id")
                            .from(
                                role_permission::Entity,
                                role_permission::Column::PermissionId,
                            )
                            .to(permission::Entity, permission::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_granted_by")
                            .from(role_permission::Entity, role_permission::Column::GrantedBy)
                            .to(user::Entity, user::Column::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_role_permissions_role_id")
                    .table(role_permission::Entity)
                    .col(role_permission::Column::RoleId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_role_permissions_perm_id")
                    .table(role_permission::Entity)
                    .col(role_permission::Column::PermissionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
