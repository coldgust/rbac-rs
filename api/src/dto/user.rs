use crate::dto::pagination::PaginationParams;
use model::user::Model;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Serialize)]
pub struct RoleBrief {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserRolesAndPermissions {
    pub roles: Vec<RoleBrief>,
    pub permissions: Vec<String>, // "resource:action"
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct UserFilters {
    pub username: Option<String>,
    pub email: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub status: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(flatten)]
    pub filters: UserFilters,
}

#[derive(Debug, Serialize)]
pub struct UserBrief {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub status: i16,
    pub last_login_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct UserDetail {
    #[serde(flatten)]
    pub user_brief: UserBrief,
    #[serde(flatten)]
    pub user_roles_and_permissions: UserRolesAndPermissions,
}

impl From<Model> for UserBrief {
    fn from(value: Model) -> Self {
        UserBrief {
            id: value.id,
            username: value.username,
            email: value.email,
            display_name: value.display_name,
            status: value.status,
            last_login_at: value.last_login_at.map(|t| t.timestamp_millis()),
            created_at: value.created_at.timestamp_millis(),
            updated_at: value.updated_at.timestamp_millis(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub status: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct AssignRolesRequest {
    pub role_ids: Vec<String>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}