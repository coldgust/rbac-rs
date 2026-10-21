import axios from 'axios'
import type { ApiResponse, TokenResponse } from '@/types'
import { ElMessage } from 'element-plus'
import NProgress from 'nprogress'
import { useAuthStore } from '@/stores/auth'

const baseURL = import.meta.env.VITE_API_BASE_URL || '/api/v1'

const request = axios.create({
  baseURL,
  timeout: 15000,
})

NProgress.configure({ showSpinner: false })

let refreshPromise: Promise<TokenResponse> | null = null

async function doRefresh(): Promise<TokenResponse> {
  const authStore = useAuthStore()
  const res = await axios.post<ApiResponse<TokenResponse>>(`${baseURL}/auth/refresh`, {
    refresh_token: authStore.refreshToken,
  })
  const body = res.data
  if (body.code !== 0 || !body.data) {
    throw new Error(body.msg || 'Refresh failed')
  }
  authStore.setTokens(body.data)
  return body.data
}

export function refreshTokens(): Promise<TokenResponse> {
  if (!refreshPromise) {
    refreshPromise = doRefresh().finally(() => {
      refreshPromise = null
    })
  }
  return refreshPromise
}

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

const isAuthEndpoint = (url?: string) =>
  !!url && ['/auth/login', '/auth/register', '/auth/refresh', '/auth/logout'].some((path) => url.includes(path))

// Another tab may have already rotated the token. Read localStorage directly
// instead of trusting the possibly stale Pinia state.
function adoptLatestTokens(): boolean {
  return useAuthStore().syncFromStorage()
}

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
    // 204 No Content (e.g. logout) has no ApiResponse envelope.
    if (response.status === 204) {
      return response
    }
    const data = response.data as ApiResponse
    if (data.code !== 0) {
      ElMessage.error(data.msg || 'Request failed')
      return Promise.reject(new Error(data.msg))
    }
    return response
  },
  async (error) => {
    NProgress.done()
    const authStore = useAuthStore()
    const original = error.config
    if (!original) return Promise.reject(error)

    // Auth endpoints must not trigger another refresh (avoids loops), but their
    // backend error message still needs to be shown to the user.
    if (isAuthEndpoint(original.url)) {
      ElMessage.error(error.response?.data?.msg || error.message || 'Request failed')
      return Promise.reject(error)
    }

    const status = error.response?.status
    const code = error.response?.data?.code

    // Another tab rotated the refresh token while this request was in flight.
    if (status === 409 && code === 10007) {
      await sleep(300)
      if (adoptLatestTokens()) {
        original.headers.Authorization = `Bearer ${authStore.accessToken}`
        return request(original)
      }
      return Promise.reject(error)
    }

    if (status === 401 && !original._retry) {
      original._retry = true

      if (!authStore.refreshToken) {
        authStore.logout()
        ElMessage.error('Session expired, please login again')
        return Promise.reject(error)
      }

      try {
        await refreshTokens()
      } catch (err: any) {
        // The refresh call itself may race with another tab and get 409/10007.
        if (err.response?.status === 409 && err.response?.data?.code === 10007) {
          await sleep(300)
          if (adoptLatestTokens()) {
            original.headers.Authorization = `Bearer ${authStore.accessToken}`
            return request(original)
          }
        }
        authStore.logout()
        ElMessage.error(err.response?.data?.msg || 'Session expired, please login again')
        return Promise.reject(err)
      }

      original.headers.Authorization = `Bearer ${authStore.accessToken}`
      return request(original)
    }

    // The retried request also failed with 401: the rotated token is not usable.
    if (status === 401 && original._retry) {
      authStore.logout()
      ElMessage.error('Session expired, please login again')
      return Promise.reject(error)
    }

    if (error.response?.data?.msg) {
      ElMessage.error(error.response.data.msg)
    } else {
      ElMessage.error(error.message || 'Network error')
    }
    return Promise.reject(error)
  },
)

export default request
