use crate::{DbErr, DeriveMigrationName, async_trait};
use sea_orm_migration::prelude::*;
use std::string::ToString;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

const PERMISSIONS_DATA: [(&str, &str, &str); 12] = [
    ("user", "create", "Create User"),
    ("user", "read", "Read User"),
    ("user", "update", "Update User"),
    ("user", "delete", "Delete User"),
    ("user", "assign_role", "Assign Role"),
    ("role", "create", "Create Role"),
    ("role", "read", "Read Role"),
    ("role", "update", "Update Role"),
    ("role", "delete", "Delete Role"),
    ("role", "assign_perm", "Assign Permission"),
    ("perm", "create", "Create Permission"),
    ("perm", "read", "Read Permission"),
];

const SUPER_ADMIN: &str = "super_admin";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        async fn exists(
            manager: &SchemaManager<'_>,
            table: &str,
            column: &str,
            value: &str,
        ) -> Result<bool, DbErr> {
            let mut q = Query::select();
            q.column(column.to_string())
                .from(Alias::new(table))
                .and_where(Expr::col(column.to_string()).eq(value));
            let result = manager.get_connection().query_one(&q).await?;
            Ok(result.is_some())
        }

        let super_admin_id = Uuid::new_v4().to_string();
        if !exists(manager, "roles", "name", SUPER_ADMIN).await? {
            let insert = Query::insert()
                .into_table(Alias::new("roles"))
                .columns([
                    Alias::new("id"),
                    Alias::new("name"),
                    Alias::new("display_name"),
                    Alias::new("is_system"),
                    Alias::new("created_at"),
                    Alias::new("updated_at"),
                ])
                .values_panic(vec![
                    super_admin_id.clone().into(),
                    SUPER_ADMIN.into(),
                    "Super Administrator".into(),
                    true.into(),
                    Expr::current_timestamp().into(),
                    Expr::current_timestamp().into(),
                ])
                .to_owned();
            manager.execute(insert).await?;
        }

        let mut permission_ids = Vec::new();
        for (resource, action, name) in PERMISSIONS_DATA {
            let row = manager
                .get_connection()
                .query_one(
                    Query::select()
                        .column("id")
                        .from(Alias::new("permissions"))
                        .and_where(Expr::col("resource").eq(resource))
                        .and_where(Expr::col("action").eq(action)),
                )
                .await?;

            if let Some(row) = row {
                let id: String = row.try_get("", "id")?;
                permission_ids.push(id);
            } else {
                let id = Uuid::new_v4().to_string();
                let insert = Query::insert()
                    .into_table(Alias::new("permissions"))
                    .columns(vec![
                        Alias::new("id"),
                        Alias::new("resource"),
                        Alias::new("action"),
                        Alias::new("name"),
                        Alias::new("created_at"),
                        Alias::new("updated_at"),
                    ])
                    .values_panic(vec![
                        id.clone().into(),
                        resource.into(),
                        action.into(),
                        name.into(),
                        Expr::current_timestamp().into(),
                        Expr::current_timestamp().into(),
                    ])
                    .to_owned();
                manager.execute(insert).await?;
                permission_ids.push(id);
            }
        }

        let super_admin_row = manager
            .get_connection()
            .query_one(
                Query::select()
                    .column("id")
                    .from(Alias::new("roles"))
                    .and_where(Expr::col("name").eq(SUPER_ADMIN)),
            )
            .await?
            .ok_or(DbErr::RecordNotFound("super_admin not found".to_string()))?;
        let super_admin_id_db: String = super_admin_row.try_get("", "id")?;

        for perm_id in &permission_ids {
            let exists_rel = manager
                .get_connection()
                .query_one(
                    Query::select()
                        .column("role_id")
                        .from(Alias::new("role_permissions"))
                        .and_where(Expr::col("role_id").eq(&super_admin_id_db))
                        .and_where(Expr::col("permission_id").eq(perm_id)),
                )
                .await?
                .is_some();

            if !exists_rel {
                let insert = Query::insert()
                    .into_table(Alias::new("role_permissions"))
                    .columns(vec![Alias::new("role_id"), Alias::new("permission_id")])
                    .values_panic(vec![
                        super_admin_id_db.clone().into(),
                        perm_id.clone().into(),
                    ])
                    .to_owned();
                manager.execute(insert).await?;
            }
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if let Some(row) = manager
            .get_connection()
            .query_one(
                Query::select()
                    .column("id")
                    .from("roles")
                    .and_where(Expr::col("name").eq(SUPER_ADMIN)),
            )
            .await?
        {
            let role_id: String = row.try_get("", "id")?;
            manager
                .execute(
                    Query::delete()
                        .from_table("role_permissions")
                        .and_where(Expr::col("role_id").eq(&role_id))
                        .to_owned(),
                )
                .await?;

            manager
                .execute(
                    Query::delete()
                        .from_table("user_roles")
                        .and_where(Expr::col("role_id").eq(&role_id))
                        .to_owned(),
                )
                .await?;

            manager
                .execute(
                    Query::delete()
                        .from_table("roles")
                        .and_where(Expr::col("id").eq(&role_id))
                        .to_owned(),
                )
                .await?;
        };

        for (resource, action, _) in PERMISSIONS_DATA {
            manager
                .execute(
                    Query::delete()
                        .from_table("permissions")
                        .and_where(Expr::col("resource").eq(resource))
                        .and_where(Expr::col("action").eq(action))
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}
