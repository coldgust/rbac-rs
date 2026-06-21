use crate::dto::pagination::PaginationParams;
use model::permission::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PermissionBrief {
    pub id: String,
    pub resource: String,
    pub action: String,
    pub name: String,
}

impl From<Model> for PermissionBrief {
    fn from(p: Model) -> Self {
        PermissionBrief {
            id: p.id,
            resource: p.resource,
            action: p.action,
            name: p.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PermissionFilters {
    pub resource: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: String,
    pub resource: String,
    pub action: String,
    pub name: String,
    pub description: Option<String>,
    pub resource_path: Option<String>,
    pub http_method: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Model> for PermissionResponse {
    fn from(p: Model) -> Self {
        PermissionResponse {
            id: p.id,
            resource: p.resource,
            action: p.action,
            name: p.name,
            description: p.description,
            resource_path: p.resource_path,
            http_method: p.http_method,
            created_at: p.created_at.timestamp_millis(),
            updated_at: p.updated_at.timestamp_millis(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePermissionRequest {
    pub resource: String,
    pub action: String,
    pub name: String,
    pub description: Option<String>,
    pub resource_path: Option<String>,
    pub http_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub resource_path: Option<String>,
    pub http_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PermissionListQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(flatten)]
    pub filters: PermissionFilters,
}
