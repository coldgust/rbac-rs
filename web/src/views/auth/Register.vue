<template>
  <div class="auth-container">
    <div class="auth-card">
      <div class="auth-brand">
        <el-icon :size="32"><UserFilled /></el-icon>
      </div>
      <h2 class="auth-title">{{ t('auth.registerTitle') }}</h2>
      <p class="auth-subtitle">{{ t('common.appName') }}</p>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        @submit.prevent="handleRegister"
      >
        <el-form-item :label="t('auth.username')" prop="username">
          <el-input
            v-model="form.username"
            :placeholder="t('auth.username')"
            prefix-icon="User"
            size="large"
          />
        </el-form-item>

        <el-form-item :label="t('auth.email')" prop="email">
          <el-input
            v-model="form.email"
            :placeholder="t('auth.email')"
            prefix-icon="Message"
            size="large"
          />
        </el-form-item>

        <el-form-item :label="t('auth.password')" prop="password">
          <el-input
            v-model="form.password"
            type="password"
            :placeholder="t('auth.password')"
            prefix-icon="Lock"
            size="large"
            show-password
          />
        </el-form-item>

        <el-form-item :label="t('auth.confirmPassword')" prop="confirmPassword">
          <el-input
            v-model="form.confirmPassword"
            type="password"
            :placeholder="t('auth.confirmPassword')"
            prefix-icon="Lock"
            size="large"
            show-password
          />
        </el-form-item>

        <el-form-item :label="t('auth.displayName')" prop="display_name">
          <el-input
            v-model="form.display_name"
            :placeholder="t('auth.displayName')"
            size="large"
          />
        </el-form-item>

        <el-form-item>
          <el-button
            type="primary"
            size="large"
            :loading="loading"
            style="width: 100%"
            @click="handleRegister"
          >
            {{ t('auth.register') }}
          </el-button>
        </el-form-item>
      </el-form>

      <div class="auth-footer">
        {{ t('auth.login') }}?
        <router-link to="/login">{{ t('auth.login') }}</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { UserFilled } from '@element-plus/icons-vue'
import { register } from '@/api/auth'

const router = useRouter()
const { t } = useI18n()

const formRef = ref<FormInstance>()
const loading = ref(false)

const form = reactive({
  username: '',
  email: '',
  password: '',
  confirmPassword: '',
  display_name: '',
})

const validateConfirmPassword = (_rule: unknown, value: string, callback: (err?: Error) => void) => {
  if (value !== form.password) {
    callback(new Error(t('auth.passwordMismatch')))
  } else {
    callback()
  }
}

const rules = reactive<FormRules>({
  username: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { min: 3, max: 64, message: () => t('auth.usernameMin'), trigger: 'blur' },
  ],
  email: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { type: 'email', message: () => t('auth.emailInvalid'), trigger: 'blur' },
  ],
  password: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { min: 8, message: () => t('auth.passwordMin'), trigger: 'blur' },
  ],
  confirmPassword: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { validator: validateConfirmPassword, trigger: 'blur' },
  ],
})

async function handleRegister() {
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) return

  loading.value = true
  try {
    await register({
      username: form.username,
      email: form.email,
      password: form.password,
      display_name: form.display_name || undefined,
    })
    ElMessage.success(t('auth.registerSuccess'))
    router.push('/login')
  } catch {
    // Error handled by interceptor
  } finally {
    loading.value = false
  }
}
</script>

<style scoped lang="scss">
.auth-brand {
  width: 64px;
  height: 64px;
  margin: 0 auto 16px;
  border-radius: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 50%, #ec4899 100%);
  box-shadow: 0 12px 24px rgba(99, 102, 241, 0.4);
  animation: brandPulse 3s ease-in-out infinite;
}

@keyframes brandPulse {
  0%, 100% { box-shadow: 0 12px 24px rgba(99, 102, 241, 0.4); }
  50% { box-shadow: 0 12px 32px rgba(236, 72, 153, 0.5); }
}
</style>
