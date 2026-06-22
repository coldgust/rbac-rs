import request from '@/utils/request'
import type { ApiResponse, LoginRequest, RegisterRequest, TokenResponse, RefreshRequest } from '@/types'

export function login(data: LoginRequest) {
  return request.post<ApiResponse<TokenResponse>>('/auth/login', data)
}

export function register(data: RegisterRequest) {
  return request.post<ApiResponse<null>>('/auth/register', data)
}

export function refreshToken(data: RefreshRequest) {
  return request.post<ApiResponse<TokenResponse>>('/auth/refresh', data)
}

export function logout(refreshToken: string) {
  return request.post<ApiResponse<null>>('/auth/logout', { refresh_token: refreshToken })
}
