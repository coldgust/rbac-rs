<template>
  <div class="page-card">
    <div class="page-header">
      <h2>{{ t('role.title') }} - {{ roleDetail?.name }}</h2>
      <el-button @click="router.push('/roles')">{{ t('common.back') }}</el-button>
    </div>

    <el-descriptions v-loading="loading" :column="2" border>
      <el-descriptions-item :label="t('role.id')">{{ roleDetail?.id }}</el-descriptions-item>
      <el-descriptions-item :label="t('role.name')">{{ roleDetail?.name }}</el-descriptions-item>
      <el-descriptions-item :label="t('role.displayName')">{{ roleDetail?.display_name || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('role.description')">{{ roleDetail?.description || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('role.isSystem')">
        <el-tag :type="roleDetail?.is_system ? 'warning' : 'info'" size="small">
          {{ roleDetail?.is_system ? t('role.yes') : t('role.no') }}
        </el-tag>
      </el-descriptions-item>
      <el-descriptions-item :label="t('role.sortOrder')">{{ roleDetail?.sort_order }}</el-descriptions-item>
      <el-descriptions-item :label="t('role.createdAt')">
        {{ roleDetail?.created_at ? formatTimestamp(roleDetail.created_at) : '-' }}
      </el-descriptions-item>
      <el-descriptions-item :label="t('role.updatedAt')">
        {{ roleDetail?.updated_at ? formatTimestamp(roleDetail.updated_at) : '-' }}
      </el-descriptions-item>
    </el-descriptions>

    <!-- Permissions section -->
    <div style="margin-top: 24px">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px">
        <h3 style="margin: 0">{{ t('role.permissions') }}</h3>
        <el-button v-permission="'role:assign_perm'" type="primary" size="small" @click="showAssignPermDialog = true">
          {{ t('role.assignPermissions') }}
        </el-button>
      </div>
      <el-table :data="roleDetail?.permissions || []" stripe style="width: 100%">
        <el-table-column prop="id" :label="t('permission.id')" width="auto" min-width="25%" show-overflow-tooltip />
        <el-table-column prop="resource" :label="t('permission.resource')" width="auto" min-width="15%" />
        <el-table-column prop="action" :label="t('permission.action')" width="auto" min-width="15%" />
        <el-table-column prop="name" :label="t('permission.name')" width="auto" min-width="25%" />
        <el-table-column :label="t('common.actions')" width="100" fixed="right">
          <template #default="{ row }">
            <el-button v-permission="'role:assign_perm'" size="small" type="danger" @click="handleRemovePerm(row.id)">
              {{ t('common.remove') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- Assign permissions dialog -->
    <el-dialog v-model="showAssignPermDialog" :title="t('role.assignPermissions')" width="500">
      <el-select v-model="selectedPermIds" multiple :placeholder="t('role.assignPermissions')" style="width: 100%">
        <el-option v-for="perm in allPermissions" :key="perm.id" :label="`${perm.resource}:${perm.action} - ${perm.name}`" :value="perm.id" />
      </el-select>
      <template #footer>
        <el-button @click="showAssignPermDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="assignLoading" @click="handleAssignPerms">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { RoleDetail, PermissionResponse } from '@/types'
import { getRole, assignPermissions, removePermission } from '@/api/role'
import { listPermissions } from '@/api/permission'

const route = useRoute()
const router = useRouter()
const { t } = useI18n()

const roleId = route.params.id as string
const loading = ref(false)
const roleDetail = ref<RoleDetail | null>(null)

// Assign permissions
const showAssignPermDialog = ref(false)
const assignLoading = ref(false)
const selectedPermIds = ref<string[]>([])
const allPermissions = ref<PermissionResponse[]>([])

function formatTimestamp(ts: number) {
  return new Date(ts).toLocaleString()
}

async function fetchRole() {
  loading.value = true
  try {
    const res = await getRole(roleId)
    roleDetail.value = res.data.data as RoleDetail
  } catch {} finally {
    loading.value = false
  }
}

async function fetchAllPermissions() {
  try {
    const res = await listPermissions({ page: 1, page_size: 200 })
    allPermissions.value = res.data.data!.items
  } catch {}
}

async function handleAssignPerms() {
  if (!selectedPermIds.value.length) return
  assignLoading.value = true
  try {
    await assignPermissions(roleId, { permission_ids: selectedPermIds.value })
    ElMessage.success(t('common.success'))
    showAssignPermDialog.value = false
    selectedPermIds.value = []
    fetchRole()
  } catch {} finally {
    assignLoading.value = false
  }
}

async function handleRemovePerm(permId: string) {
  await ElMessageBox.confirm(t('common.deleteConfirm'), { type: 'warning' })
  try {
    await removePermission(roleId, permId)
    ElMessage.success(t('common.deleteSuccess'))
    fetchRole()
  } catch {}
}

onMounted(() => {
  fetchRole()
  fetchAllPermissions()
})
</script>
