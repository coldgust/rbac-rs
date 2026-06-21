use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub resource: String,
    pub action: String,
    pub name: String,
    pub description: Option<String>,
    pub resource_path: Option<String>,
    pub http_method: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,

    #[sea_orm(has_many, via = "role_permission")]
    pub roles: HasMany<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}