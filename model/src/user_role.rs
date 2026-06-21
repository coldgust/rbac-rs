use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: String,
    pub assigned_by: Option<String>,
    pub assigned_at: DateTimeUtc,
    pub expires_at: Option<DateTimeUtc>,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: Option<super::user::Entity>,

    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: Option<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
