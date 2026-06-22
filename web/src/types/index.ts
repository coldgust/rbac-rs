// ========== Common Response Types ==========

export interface ApiResponse<T = unknown> {
  code: number
  msg: string
  data: T | null
}

export interface PaginatedData<T> {
  total: number
  page: number
  page_size: number
  items: T[]
}

export interface PaginationParams {
  page?: number
  page_size?: number
}

// ========== Auth Types ==========

export interface LoginRequest {
  username: string
  password: string
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
  display_name?: string
}

export interface TokenResponse {
  access_token: string
  refresh_token: string
  token_type: string
  expires_in: number
}

export interface RefreshRequest {
  refresh_token: string
}

export interface LogoutRequest {
  refresh_token: string
}

// ========== User Types ==========

export interface UserBrief {
  id: string
  username: string
  email: string
  display_name?: string | null
  status?: number
  last_login_at?: number | null
  created_at: number
  updated_at: number
}

export interface RoleBrief {
  id: string
  name: string
  display_name?: string | null
}

export interface UserDetail extends UserBrief {
  roles: RoleBrief[]
  permissions: string[]
}

export interface UserListQuery extends PaginationParams {
  username?: string
  email?: string
  status?: number
}

export interface CreateUserRequest {
  username: string
  email: string
  password: string
  display_name?: string
  status?: number
}

export interface UpdateUserRequest {
  display_name?: string
  email?: string
  status?: number
}

export interface AssignRolesRequest {
  role_ids: string[]
  expires_at?: number | null
}

export interface ChangePasswordRequest {
  old_password: string
  new_password: string
}

export const UserStatus = {
  Inactive: 0,
  Active: 1,
  Locked: 2,
} as const

// ========== Role Types ==========

export interface RoleResponse {
  id: string
  name: string
  display_name?: string | null
  description?: string | null
  is_system: boolean
  sort_order: number
  created_at: number
  updated_at: number
}

export interface PermissionBrief {
  id: string
  resource: string
  action: string
  name: string
}

export interface RoleDetail extends RoleResponse {
  permissions: PermissionBrief[]
}

export interface RoleListQuery extends PaginationParams {
  name?: string
}

export interface CreateRoleRequest {
  name: string
  display_name?: string
  description?: string
  sort_order?: number
}

export interface UpdateRoleRequest {
  display_name?: string
  description?: string
  sort_order?: number
}

export interface AssignPermissionsRequest {
  permission_ids: string[]
}

// ========== Permission Types ==========

export interface PermissionResponse {
  id: string
  resource: string
  action: string
  name: string
  description?: string | null
  resource_path?: string | null
  http_method?: string | null
  created_at: number
  updated_at: number
}

export interface PermissionListQuery extends PaginationParams {
  resource?: string
}

export interface CreatePermissionRequest {
  resource: string
  action: string
  name: string
  description?: string
  resource_path?: string
  http_method?: string
}

export interface UpdatePermissionRequest {
  name?: string
  description?: string
  resource_path?: string
  http_method?: string
}
