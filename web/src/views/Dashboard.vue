<template>
  <div class="dashboard">
    <!-- 欢迎横幅 -->
    <div class="welcome-banner">
      <div class="welcome-content">
        <h1>{{ t('nav.dashboard') }}</h1>
        <p>{{ t('common.appName') }}</p>
      </div>
      <div class="welcome-deco">
        <el-icon :size="120"><Odometer /></el-icon>
      </div>
    </div>

    <!-- 统计卡片 -->
    <div class="stat-grid">
      <div class="stat-card stat-card--indigo">
        <div class="stat-icon">
          <el-icon :size="24"><User /></el-icon>
        </div>
        <div class="stat-info">
          <div class="stat-value">{{ stat.users }}</div>
          <div class="stat-label">{{ t('nav.users') }}</div>
        </div>
      </div>

      <div class="stat-card stat-card--violet">
        <div class="stat-icon">
          <el-icon :size="24"><Stamp /></el-icon>
        </div>
        <div class="stat-info">
          <div class="stat-value">{{ stat.roles }}</div>
          <div class="stat-label">{{ t('nav.roles') }}</div>
        </div>
      </div>

      <div class="stat-card stat-card--pink">
        <div class="stat-icon">
          <el-icon :size="24"><Lock /></el-icon>
        </div>
        <div class="stat-info">
          <div class="stat-value">{{ stat.permissions }}</div>
          <div class="stat-label">{{ t('nav.permissions') }}</div>
        </div>
      </div>

      <div class="stat-card stat-card--cyan">
        <div class="stat-icon">
          <el-icon :size="24"><Key /></el-icon>
        </div>
        <div class="stat-info">
          <div class="stat-value">{{ authStore.permissions.length }}</div>
          <div class="stat-label">{{ t('user.permissions') }}</div>
        </div>
      </div>
    </div>

    <!-- 角色与权限 -->
    <div class="info-grid">
      <div class="info-card">
        <div class="info-header">
          <h3>{{ t('user.roles') }}</h3>
          <span class="info-count">{{ authStore.currentUser?.roles?.length || 0 }}</span>
        </div>
        <div class="tag-cloud">
          <el-tag
            v-for="role in authStore.currentUser?.roles || []"
            :key="role.id"
            size="large"
            effect="light"
            round
          >
            {{ role.display_name || role.name }}
          </el-tag>
          <span v-if="!authStore.currentUser?.roles?.length" class="empty-hint">-</span>
        </div>
      </div>

      <div class="info-card">
        <div class="info-header">
          <h3>{{ t('user.permissions') }}</h3>
          <span class="info-count">{{ authStore.permissions.length }}</span>
        </div>
        <div class="tag-cloud">
          <el-tag
            v-for="perm in authStore.permissions"
            :key="perm"
            type="info"
            effect="plain"
          >
            {{ perm }}
          </el-tag>
          <span v-if="!authStore.permissions.length" class="empty-hint">-</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { User, Stamp, Lock, Key, Odometer } from '@element-plus/icons-vue'

const { t } = useI18n()
const authStore = useAuthStore()

const stat = reactive({
  users: 0,
  roles: 0,
  permissions: 0,
})
</script>

<style scoped lang="scss">
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

// 欢迎横幅
.welcome-banner {
  position: relative;
  overflow: hidden;
  border-radius: var(--rbac-radius-lg);
  padding: 32px 36px;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 45%, #ec4899 100%);
  color: #fff;
  box-shadow: 0 12px 32px rgba(99, 102, 241, 0.25);
  display: flex;
  justify-content: space-between;
  align-items: center;

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    background: radial-gradient(circle at 80% 20%, rgba(255, 255, 255, 0.18) 0%, transparent 50%);
    pointer-events: none;
  }

  h1 {
    margin: 0 0 8px;
    font-size: 28px;
    font-weight: 700;
    letter-spacing: -0.5px;
  }

  p {
    margin: 0;
    font-size: 15px;
    opacity: 0.9;
  }
}

.welcome-content {
  position: relative;
  z-index: 1;
}

.welcome-deco {
  opacity: 0.25;
  position: relative;
  z-index: 0;
}

// 统计卡片网格
.stat-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
}

.stat-card {
  background: #fff;
  border-radius: var(--rbac-radius-lg);
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: var(--rbac-shadow-md);
  border: 1px solid rgba(226, 232, 240, 0.6);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3px;
    opacity: 0.9;
  }

  &:hover {
    transform: translateY(-4px);
    box-shadow: var(--rbac-shadow-lg);
  }

  &--indigo::before { background: linear-gradient(90deg, #6366f1, #818cf8); }
  &--violet::before { background: linear-gradient(90deg, #8b5cf6, #a78bfa); }
  &--pink::before   { background: linear-gradient(90deg, #ec4899, #f472b6); }
  &--cyan::before   { background: linear-gradient(90deg, #06b6d4, #22d3ee); }
}

.stat-icon {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}

.stat-card--indigo .stat-icon { background: linear-gradient(135deg, #6366f1, #818cf8); box-shadow: 0 8px 16px rgba(99, 102, 241, 0.3); }
.stat-card--violet .stat-icon { background: linear-gradient(135deg, #8b5cf6, #a78bfa); box-shadow: 0 8px 16px rgba(139, 92, 246, 0.3); }
.stat-card--pink   .stat-icon { background: linear-gradient(135deg, #ec4899, #f472b6); box-shadow: 0 8px 16px rgba(236, 72, 153, 0.3); }
.stat-card--cyan   .stat-icon { background: linear-gradient(135deg, #06b6d4, #22d3ee); box-shadow: 0 8px 16px rgba(6, 182, 212, 0.3); }

.stat-info {
  flex: 1;
  min-width: 0;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--rbac-text-primary);
  line-height: 1.2;
  letter-spacing: -0.5px;
}

.stat-label {
  font-size: 13px;
  color: var(--rbac-text-secondary);
  margin-top: 4px;
  font-weight: 500;
}

// 信息卡片
.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
}

.info-card {
  background: #fff;
  border-radius: var(--rbac-radius-lg);
  padding: 24px;
  box-shadow: var(--rbac-shadow-md);
  border: 1px solid rgba(226, 232, 240, 0.6);
}

.info-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;

  h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: var(--rbac-text-primary);
  }
}

.info-count {
  font-size: 13px;
  font-weight: 600;
  color: var(--rbac-primary);
  background: rgba(99, 102, 241, 0.1);
  padding: 2px 10px;
  border-radius: 10px;
}

.tag-cloud {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  max-height: 240px;
  overflow-y: auto;
}

.empty-hint {
  color: var(--rbac-text-secondary);
  font-size: 14px;
}

// 响应式
@media (max-width: 1200px) {
  .stat-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .stat-grid,
  .info-grid {
    grid-template-columns: 1fr;
  }
  .welcome-deco {
    display: none;
  }
}
</style>
