import request from '@/utils/request'
import type {
  ApiResponse,
  PaginatedData,
  PermissionResponse,
  PermissionListQuery,
  CreatePermissionRequest,
  UpdatePermissionRequest,
} from '@/types'

export function listPermissions(params: PermissionListQuery) {
  return request.get<ApiResponse<PaginatedData<PermissionResponse>>>('/permissions', { params })
}

export function getPermission(id: string) {
  return request.get<ApiResponse<PermissionResponse>>(`/permissions/${id}`)
}

export function createPermission(data: CreatePermissionRequest) {
  return request.post<ApiResponse<PermissionResponse>>('/permissions', data)
}

export function updatePermission(id: string, data: UpdatePermissionRequest) {
  return request.put<ApiResponse<PermissionResponse>>(`/permissions/${id}`, data)
}

export function deletePermission(id: string) {
  return request.delete(`/permissions/${id}`)
}
