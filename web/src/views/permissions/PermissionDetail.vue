<template>
  <div class="page-card">
    <div class="page-header">
      <h2>{{ t('permission.title') }} - {{ permission?.name }}</h2>
      <el-button @click="router.push('/permissions')">{{ t('common.back') }}</el-button>
    </div>

    <el-descriptions v-loading="loading" :column="2" border>
      <el-descriptions-item :label="t('permission.id')">{{ permission?.id }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.resource')">{{ permission?.resource }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.action')">{{ permission?.action }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.name')">{{ permission?.name }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.description')">{{ permission?.description || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.resourcePath')">{{ permission?.resource_path || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.httpMethod')">{{ permission?.http_method || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('permission.createdAt')">
        {{ permission?.created_at ? formatTimestamp(permission.created_at) : '-' }}
      </el-descriptions-item>
      <el-descriptions-item :label="t('permission.updatedAt')">
        {{ permission?.updated_at ? formatTimestamp(permission.updated_at) : '-' }}
      </el-descriptions-item>
    </el-descriptions>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { PermissionResponse } from '@/types'
import { getPermission } from '@/api/permission'

const route = useRoute()
const router = useRouter()
const { t } = useI18n()

const permId = route.params.id as string
const loading = ref(false)
const permission = ref<PermissionResponse | null>(null)

function formatTimestamp(ts: number) {
  return new Date(ts).toLocaleString()
}

async function fetchPermission() {
  loading.value = true
  try {
    const res = await getPermission(permId)
    permission.value = res.data.data as PermissionResponse
  } catch {} finally {
    loading.value = false
  }
}

onMounted(() => fetchPermission())
</script>
