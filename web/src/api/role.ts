import request from '@/utils/request'
import type {
  ApiResponse,
  PaginatedData,
  RoleResponse,
  RoleDetail,
  RoleListQuery,
  CreateRoleRequest,
  UpdateRoleRequest,
  AssignPermissionsRequest,
  PermissionBrief,
} from '@/types'

export function listRoles(params: RoleListQuery) {
  return request.get<ApiResponse<PaginatedData<RoleResponse>>>('/roles', { params })
}

export function getRole(id: string) {
  return request.get<ApiResponse<RoleDetail>>(`/roles/${id}`)
}

export function createRole(data: CreateRoleRequest) {
  return request.post<ApiResponse<RoleResponse>>('/roles', data)
}

export function updateRole(id: string, data: UpdateRoleRequest) {
  return request.put<ApiResponse<RoleResponse>>(`/roles/${id}`, data)
}

export function deleteRole(id: string) {
  return request.delete(`/roles/${id}`)
}

export function assignPermissions(id: string, data: AssignPermissionsRequest) {
  return request.post<ApiResponse<PermissionBrief[]>>(`/roles/${id}/permissions`, data)
}

export function removePermission(roleId: string, permId: string) {
  return request.delete(`/roles/${roleId}/permissions/${permId}`)
}
