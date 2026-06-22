<template>
  <div class="auth-container">
    <div class="auth-card">
      <div class="auth-brand">
        <el-icon :size="32"><Lock /></el-icon>
      </div>
      <h2 class="auth-title">{{ t('auth.loginTitle') }}</h2>
      <p class="auth-subtitle">{{ t('common.appName') }}</p>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        @submit.prevent="handleLogin"
      >
        <el-form-item :label="t('auth.username')" prop="username">
          <el-input
            v-model="form.username"
            :placeholder="t('auth.username')"
            prefix-icon="User"
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
            @keyup.enter="handleLogin"
          />
        </el-form-item>

        <el-form-item>
          <el-button
            type="primary"
            size="large"
            :loading="loading"
            style="width: 100%"
            @click="handleLogin"
          >
            {{ t('auth.login') }}
          </el-button>
        </el-form-item>
      </el-form>

      <div class="auth-footer">
        {{ t('auth.register') }}?
        <router-link to="/register">{{ t('auth.register') }}</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { Lock } from '@element-plus/icons-vue'
import { login } from '@/api/auth'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()
const authStore = useAuthStore()

const formRef = ref<FormInstance>()
const loading = ref(false)

const form = reactive({
  username: '',
  password: '',
})

const rules = reactive<FormRules>({
  username: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { min: 3, message: () => t('auth.usernameMin'), trigger: 'blur' },
  ],
  password: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { min: 8, message: () => t('auth.passwordMin'), trigger: 'blur' },
  ],
})

async function handleLogin() {
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) return

  loading.value = true
  try {
    const res = await login(form)
    authStore.setTokens(res.data.data!)
    await authStore.fetchCurrentUser()
    ElMessage.success(t('auth.loginSuccess'))
    const redirect = (route.query.redirect as string) || '/'
    router.push(redirect)
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
