import axios from 'axios'
import type { ApiResponse, TokenResponse } from '@/types'
import { ElMessage } from 'element-plus'
import NProgress from 'nprogress'
import { useAuthStore } from '@/stores/auth'
import router from '@/router'

const request = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || '/api/v1',
  timeout: 15000,
})

NProgress.configure({ showSpinner: false })

// Request interceptor
request.interceptors.request.use(
  (config) => {
    NProgress.start()
    const authStore = useAuthStore()
    if (authStore.accessToken) {
      config.headers.Authorization = `Bearer ${authStore.accessToken}`
    }
    return config
  },
  (error) => {
    NProgress.done()
    return Promise.reject(error)
  },
)

// Response interceptor
request.interceptors.response.use(
  (response) => {
    NProgress.done()
    const data = response.data as ApiResponse
    if (data.code !== 0) {
      ElMessage.error(data.msg || 'Request failed')
      return Promise.reject(new Error(data.msg))
    }
    return response
  },
  async (error) => {
    NProgress.done()
    if (error.response?.status === 401) {
      const authStore = useAuthStore()
      // Try to refresh token
      if (authStore.refreshToken && !error.config._retry) {
        error.config._retry = true
        try {
          const res = await axios.post(
            `${import.meta.env.VITE_API_BASE_URL || '/api/v1'}/auth/refresh`,
            { refresh_token: authStore.refreshToken },
          )
          const data = res.data as ApiResponse
          if (data.code === 0 && data.data) {
            authStore.setTokens(data.data as TokenResponse)
            error.config.headers.Authorization = `Bearer ${authStore.accessToken}`
            return request(error.config)
          }
        } catch {
          // Refresh failed, logout
          authStore.logout()
          router.push('/login')
        }
      } else {
        authStore.logout()
        router.push('/login')
      }
      ElMessage.error('Session expired, please login again')
    } else if (error.response?.data?.msg) {
      ElMessage.error(error.response.data.msg)
    } else {
      ElMessage.error(error.message || 'Network error')
    }
    return Promise.reject(error)
  },
)

export default request
