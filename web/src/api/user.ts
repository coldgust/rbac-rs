import request from '@/utils/request'
import type {
  ApiResponse,
  PaginatedData,
  UserBrief,
  UserDetail,
  UserListQuery,
  CreateUserRequest,
  UpdateUserRequest,
  AssignRolesRequest,
  ChangePasswordRequest,
  RoleBrief,
} from '@/types'

export function listUsers(params: UserListQuery) {
  return request.get<ApiResponse<PaginatedData<UserBrief>>>('/users', { params })
}

export function getUser(id: string) {
  return request.get<ApiResponse<UserDetail>>(`/users/${id}`)
}

export function createUser(data: CreateUserRequest) {
  return request.post<ApiResponse<UserBrief>>('/users', data)
}

export function updateUser(id: string, data: UpdateUserRequest) {
  return request.put<ApiResponse<UserBrief>>(`/users/${id}`, data)
}

export function deleteUser(id: string) {
  return request.delete(`/users/${id}`)
}

export function assignRoles(id: string, data: AssignRolesRequest) {
  return request.post<ApiResponse<RoleBrief[]>>(`/users/${id}/roles`, data)
}

export function removeRole(userId: string, roleId: string) {
  return request.delete(`/users/${userId}/roles/${roleId}`)
}

export function changePassword(data: ChangePasswordRequest) {
  return request.put<ApiResponse<null>>('/users/change_password', data)
}

export function getMe() {
  return request.get<ApiResponse<UserDetail>>('/users/me')
}
