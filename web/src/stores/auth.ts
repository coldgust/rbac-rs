import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { TokenResponse, UserDetail } from '@/types'
import { getMe } from '@/api/user'
import { logout as apiLogout } from '@/api/auth'
import router from '@/router'

export const useAuthStore = defineStore('auth', () => {
  const accessToken = ref(localStorage.getItem('access_token') || '')
  const refreshToken = ref(localStorage.getItem('refresh_token') || '')
  const currentUser = ref<UserDetail | null>(null)
  const permissions = ref<string[]>([])

  const isLoggedIn = computed(() => !!accessToken.value)

  function setTokens(tokens: TokenResponse) {
    accessToken.value = tokens.access_token
    refreshToken.value = tokens.refresh_token
    localStorage.setItem('access_token', tokens.access_token)
    localStorage.setItem('refresh_token', tokens.refresh_token)
  }

  function hasPermission(perm: string): boolean {
    return permissions.value.includes(perm)
  }

  async function fetchCurrentUser() {
    try {
      const res = await getMe()
      currentUser.value = res.data.data as UserDetail
      permissions.value = (res.data.data as UserDetail).permissions || []
    } catch {
      logout()
    }
  }

  function logout() {
    if (refreshToken.value) {
      apiLogout(refreshToken.value).catch(() => {})
    }
    accessToken.value = ''
    refreshToken.value = ''
    currentUser.value = null
    permissions.value = []
    localStorage.removeItem('access_token')
    localStorage.removeItem('refresh_token')
    router.push('/login')
  }

  return {
    accessToken,
    refreshToken,
    currentUser,
    permissions,
    isLoggedIn,
    setTokens,
    hasPermission,
    fetchCurrentUser,
    logout,
  }
})
