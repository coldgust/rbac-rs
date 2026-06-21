pub mod user;
pub mod role;
pub mod user_role;
pub mod permission;
pub mod role_permission;

pub use permission::Entity as Permission;
pub use role::Entity as Role;
pub use role_permission::Entity as RolePermission;
pub use user::Entity as User;
pub use user_role::Entity as UserRole;
