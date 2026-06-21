use sea_orm::entity::prelude::*;
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub status: i16,
    pub last_login_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,

    #[sea_orm(has_many, via = "user_role")]
    pub roles: HasMany<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
    Locked = 2,
}

impl From<i16> for UserStatus {
    fn from(value: i16) -> Self {
        match value {
            0 => UserStatus::Inactive,
            1 => UserStatus::Active,
            _ => UserStatus::Locked,
        }
    }
}
