use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_system: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,

    #[sea_orm(has_many, via = "user_role")]
    pub users: HasMany<super::user::Entity>,

    #[sea_orm(has_many, via = "role_permission")]
    pub permissions: HasMany<super::permission::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
