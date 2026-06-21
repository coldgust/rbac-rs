use crate::dto::pagination::PaginationParams;
use crate::dto::permission::PermissionBrief;
use model::role::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub is_system: bool,
    pub sort_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Model> for RoleResponse {
    fn from(role: Model) -> Self {
        RoleResponse {
            id: role.id,
            name: role.name,
            display_name: role.display_name,
            description: role.description,
            is_system: role.is_system,
            sort_order: role.sort_order,
            created_at: role.created_at.timestamp_millis(),
            updated_at: role.updated_at.timestamp_millis(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleListQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(flatten)]
    pub filters: RoleFilters,
}

#[derive(Debug, Deserialize)]
pub struct RoleFilters {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleDetailResponse {
    #[serde(flatten)]
    pub role: RoleResponse,
    pub permissions: Vec<PermissionBrief>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AssignPermissionsRequest {
    pub permission_ids: Vec<String>,
}
