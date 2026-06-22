<template>
  <div class="layout-container">
    <!-- Sidebar -->
    <div class="sidebar-container" :class="{ collapsed: isCollapsed }">
      <div class="sidebar-logo" :class="{ collapsed: isCollapsed }">
        <span v-if="!isCollapsed">{{ t('common.appName') }}</span>
        <span v-else>RB</span>
      </div>
      <el-menu
        :default-active="activeMenu"
        :collapse="isCollapsed"
        background-color="transparent"
        text-color="#ffffffa6"
        active-text-color="#fff"
        router
      >
        <el-menu-item index="/">
          <el-icon><Odometer /></el-icon>
          <template #title>{{ t('nav.dashboard') }}</template>
        </el-menu-item>
        <el-menu-item v-permission="'user:read'" index="/users">
          <el-icon><User /></el-icon>
          <template #title>{{ t('nav.users') }}</template>
        </el-menu-item>
        <el-menu-item v-permission="'role:read'" index="/roles">
          <el-icon><Stamp /></el-icon>
          <template #title>{{ t('nav.roles') }}</template>
        </el-menu-item>
        <el-menu-item v-permission="'perm:read'" index="/permissions">
          <el-icon><Lock /></el-icon>
          <template #title>{{ t('nav.permissions') }}</template>
        </el-menu-item>
      </el-menu>
    </div>

    <!-- Main area -->
    <div class="main-container">
      <!-- Header -->
      <div class="header-container">
        <div class="header-left">
          <el-icon
            :size="20"
            class="collapse-btn"
            @click="isCollapsed = !isCollapsed"
          >
            <Expand v-if="isCollapsed" />
            <Fold v-else />
          </el-icon>
        </div>
        <div class="header-right">
          <el-dropdown trigger="click" @command="switchLocale">
            <span class="locale-switch">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="14" height="14" fill="currentColor"><path d="M12.87 15.07l-2.54-2.51.03-.03A17.52 17.52 0 0014.07 6H17V4h-7V2H8v2H1v2h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-.02zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7l1.62-4.33L19.12 17h-3.24z"/></svg>
              {{ locale === 'zh' ? '中文' : 'EN' }}
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="zh">中文</el-dropdown-item>
                <el-dropdown-item command="en">English</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>

          <el-dropdown trigger="click" @command="handleCommand">
            <span class="user-dropdown">
              <el-icon><UserFilled /></el-icon>
              {{ authStore.currentUser?.username || 'User' }}
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="profile">
                  <el-icon><User /></el-icon>{{ t('nav.profile') }}
                </el-dropdown-item>
                <el-dropdown-item command="logout" divided>
                  <el-icon><SwitchButton /></el-icon>{{ t('auth.logout') }}
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </div>

      <!-- Content -->
      <div class="content-container">
        <router-view v-slot="{ Component }">
          <transition name="router-fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import {
  Odometer,
  User,
  Stamp,
  Lock,
  Expand,
  Fold,
  UserFilled,
  SwitchButton,
} from '@element-plus/icons-vue'

const route = useRoute()
const router = useRouter()
const { t, locale } = useI18n()
const authStore = useAuthStore()

const isCollapsed = ref(false)
const activeMenu = computed(() => {
  const path = route.path
  if (path.startsWith('/users')) return '/users'
  if (path.startsWith('/roles')) return '/roles'
  if (path.startsWith('/permissions')) return '/permissions'
  return path
})

// User info is loaded by router guard on page refresh
// No need to fetch here again

function switchLocale(lang: string) {
  locale.value = lang
  localStorage.setItem('locale', lang)
}

function handleCommand(command: string) {
  if (command === 'profile') {
    router.push('/profile')
  } else if (command === 'logout') {
    authStore.logout()
  }
}
</script>

<style scoped lang="scss">
.collapse-btn {
  cursor: pointer;
  color: var(--rbac-text-regular);
  padding: 8px;
  border-radius: 8px;
  transition: all 0.25s ease;
  &:hover {
    color: var(--rbac-primary);
    background: rgba(99, 102, 241, 0.08);
  }
}

.locale-switch {
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  font-weight: 500;
  color: var(--rbac-text-regular);
  padding: 6px 10px;
  border-radius: 8px;
  transition: all 0.25s ease;
  &:hover {
    color: var(--rbac-primary);
    background: rgba(99, 102, 241, 0.08);
  }
}

.user-dropdown {
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  font-weight: 500;
  color: var(--rbac-text-regular);
  padding: 6px 12px;
  border-radius: 20px;
  transition: all 0.25s ease;
  &:hover {
    color: var(--rbac-primary);
    background: rgba(99, 102, 241, 0.08);
  }
}
</style>
