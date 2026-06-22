<template>
  <div class="page-card profile-page">
    <div class="page-header">
      <h2>{{ t('profile.title') }}</h2>
    </div>

    <!-- 用户信息 -->
    <el-descriptions v-loading="loading" :column="2" border>
      <el-descriptions-item :label="t('user.id')">{{ currentUser?.id }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.username')">{{ currentUser?.username }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.email')">{{ currentUser?.email }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.displayName')">{{ currentUser?.display_name || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.status')">
        <el-tag :type="currentUser?.status === 1 ? 'success' : 'info'" effect="light" round>
          {{ currentUser?.status === 1 ? t('user.statusActive') : t('user.statusInactive') }}
        </el-tag>
      </el-descriptions-item>
      <el-descriptions-item :label="t('user.lastLogin')">
        {{ currentUser?.last_login_at ? new Date(currentUser.last_login_at).toLocaleString() : '-' }}
      </el-descriptions-item>
    </el-descriptions>

    <!-- 角色 -->
    <div class="section-block">
      <div class="section-header">
        <h3>{{ t('user.roles') }}</h3>
        <span class="section-count">{{ currentUser?.roles?.length || 0 }}</span>
      </div>
      <el-space wrap>
        <el-tag v-for="role in currentUser?.roles || []" :key="role.id" type="primary" size="large" effect="light" round>
          {{ role.display_name || role.name }}
        </el-tag>
        <span v-if="!currentUser?.roles?.length" class="empty-hint">-</span>
      </el-space>
    </div>

    <!-- 权限 -->
    <div class="section-block">
      <div class="section-header">
        <h3>{{ t('user.permissions') }}</h3>
        <span class="section-count">{{ currentUser?.permissions?.length || 0 }}</span>
      </div>
      <el-space wrap>
        <el-tag v-for="perm in currentUser?.permissions || []" :key="perm" type="info" effect="plain">
          {{ perm }}
        </el-tag>
        <span v-if="!currentUser?.permissions?.length" class="empty-hint">-</span>
      </el-space>
    </div>

    <!-- 修改密码 -->
    <div class="section-block">
      <div class="section-header">
        <h3>{{ t('profile.changePassword') }}</h3>
      </div>
      <el-form ref="pwdFormRef" :model="pwdForm" :rules="pwdRules" label-position="top" style="max-width: 440px">
        <el-form-item :label="t('profile.currentPassword')" prop="old_password">
          <el-input v-model="pwdForm.old_password" type="password" show-password />
        </el-form-item>
        <el-form-item :label="t('profile.newPassword')" prop="new_password">
          <el-input v-model="pwdForm.new_password" type="password" show-password />
        </el-form-item>
        <el-form-item :label="t('profile.confirmPassword')" prop="confirmPassword">
          <el-input v-model="pwdForm.confirmPassword" type="password" show-password />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="pwdLoading" @click="handleChangePassword">
            {{ t('profile.changePassword') }}
          </el-button>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { useAuthStore } from '@/stores/auth'
import { changePassword } from '@/api/user'

const { t } = useI18n()
const authStore = useAuthStore()
const currentUser = computed(() => authStore.currentUser)

const loading = ref(false)
const pwdFormRef = ref<FormInstance>()
const pwdLoading = ref(false)
const pwdForm = reactive({
  old_password: '',
  new_password: '',
  confirmPassword: '',
})

const validateConfirmPassword = (_rule: unknown, value: string, callback: (err?: Error) => void) => {
  if (value !== pwdForm.new_password) {
    callback(new Error(t('auth.passwordMismatch')))
  } else {
    callback()
  }
}

const pwdRules = reactive<FormRules>({
  old_password: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
  new_password: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { min: 8, message: () => t('auth.passwordMin'), trigger: 'blur' },
  ],
  confirmPassword: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { validator: validateConfirmPassword, trigger: 'blur' },
  ],
})

async function handleChangePassword() {
  const valid = await pwdFormRef.value?.validate().catch(() => false)
  if (!valid) return
  pwdLoading.value = true
  try {
    await changePassword({
      old_password: pwdForm.old_password,
      new_password: pwdForm.new_password,
    })
    ElMessage.success(t('profile.passwordChanged'))
    pwdFormRef.value?.resetFields()
  } catch {} finally {
    pwdLoading.value = false
  }
}

// Fetch current user info if not loaded
if (!currentUser.value) {
  loading.value = true
  authStore.fetchCurrentUser().finally(() => { loading.value = false })
}
</script>

<style scoped lang="scss">
.profile-page {
  .section-block {
    margin-top: 28px;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 16px;

    h3 {
      margin: 0;
      font-size: 16px;
      font-weight: 600;
      color: var(--rbac-text-primary);
    }
  }

  .section-count {
    font-size: 13px;
    font-weight: 600;
    color: var(--rbac-primary);
    background: rgba(99, 102, 241, 0.1);
    padding: 2px 10px;
    border-radius: 10px;
  }

  .empty-hint {
    color: var(--rbac-text-secondary);
    font-size: 14px;
  }
}
</style>
