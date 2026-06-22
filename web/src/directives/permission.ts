import type { App, Directive } from 'vue'
import { useAuthStore } from '@/stores/auth'

const permissionDirective: Directive = {
  mounted(el, binding) {
    const authStore = useAuthStore()
    const requiredPermission = binding.value as string | string[]

    if (!requiredPermission) return

    const hasPermission = Array.isArray(requiredPermission)
      ? requiredPermission.some((p) => authStore.hasPermission(p))
      : authStore.hasPermission(requiredPermission)

    if (!hasPermission) {
      el.style.display = 'none'
    } else {
      el.style.display = ''
    }
  },
  updated(el, binding) {
    const authStore = useAuthStore()
    const requiredPermission = binding.value as string | string[]

    if (!requiredPermission) {
      el.style.display = ''
      return
    }

    const hasPermission = Array.isArray(requiredPermission)
      ? requiredPermission.some((p) => authStore.hasPermission(p))
      : authStore.hasPermission(requiredPermission)

    if (!hasPermission) {
      el.style.display = 'none'
    } else {
      el.style.display = ''
    }
  },
}

export function setupPermissionDirective(app: App) {
  app.directive('permission', permissionDirective)
}
