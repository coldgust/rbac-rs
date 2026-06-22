<template>
  <div class="page-card">
    <div class="page-header">
      <h2>{{ t('user.title') }} - {{ userDetail?.username }}</h2>
      <el-button @click="router.push('/users')">{{ t('common.back') }}</el-button>
    </div>

    <el-descriptions v-loading="loading" :column="2" border>
      <el-descriptions-item :label="t('user.id')">{{ userDetail?.id }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.username')">{{ userDetail?.username }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.email')">{{ userDetail?.email }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.displayName')">{{ userDetail?.display_name || '-' }}</el-descriptions-item>
      <el-descriptions-item :label="t('user.status')">
        <el-tag :type="statusTagType(userDetail?.status ?? 0)">
          {{ statusLabel(userDetail?.status ?? 0) }}
        </el-tag>
      </el-descriptions-item>
      <el-descriptions-item :label="t('user.lastLogin')">
        {{ userDetail?.last_login_at ? formatTimestamp(userDetail.last_login_at) : '-' }}
      </el-descriptions-item>
      <el-descriptions-item :label="t('user.createdAt')">
        {{ userDetail?.created_at ? formatTimestamp(userDetail.created_at) : '-' }}
      </el-descriptions-item>
      <el-descriptions-item :label="t('user.updatedAt')">
        {{ userDetail?.updated_at ? formatTimestamp(userDetail.updated_at) : '-' }}
      </el-descriptions-item>
    </el-descriptions>

    <!-- Roles section -->
    <div style="margin-top: 24px">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px">
        <h3 style="margin: 0">{{ t('user.roles') }}</h3>
        <el-button v-permission="'user:assign_role'" type="primary" size="small" @click="showAssignRoleDialog = true">
          {{ t('user.assignRoles') }}
        </el-button>
      </div>
      <el-table :data="userDetail?.roles || []" stripe style="width: 100%">
        <el-table-column prop="id" :label="t('role.id')" width="auto" min-width="20%" show-overflow-tooltip />
        <el-table-column prop="name" :label="t('role.name')" width="auto" min-width="25%" />
        <el-table-column prop="display_name" :label="t('role.displayName')" width="auto" min-width="30%" />
        <el-table-column :label="t('common.actions')" width="100" fixed="right">
          <template #default="{ row }">
            <el-button v-permission="'user:assign_role'" size="small" type="danger" @click="handleRemoveRole(row.id)">
              {{ t('common.remove') }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- Permissions section -->
    <div style="margin-top: 24px">
      <h3 style="margin-bottom: 12px">{{ t('user.permissions') }}</h3>
      <el-space wrap>
        <el-tag v-for="perm in userDetail?.permissions || []" :key="perm" type="info">
          {{ perm }}
        </el-tag>
      </el-space>
    </div>

    <!-- Assign roles dialog -->
    <el-dialog v-model="showAssignRoleDialog" :title="t('user.assignRoles')" width="500">
      <el-select v-model="selectedRoleIds" multiple :placeholder="t('user.assignRoles')" style="width: 100%">
        <el-option v-for="role in allRoles" :key="role.id" :label="role.display_name || role.name" :value="role.id" />
      </el-select>
      <template #footer>
        <el-button @click="showAssignRoleDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="assignLoading" @click="handleAssignRoles">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { UserDetail, RoleResponse } from '@/types'
import { getUser, assignRoles, removeRole } from '@/api/user'
import { listRoles } from '@/api/role'

const route = useRoute()
const router = useRouter()
const { t } = useI18n()

const userId = route.params.id as string
const loading = ref(false)
const userDetail = ref<UserDetail | null>(null)

// Assign roles
const showAssignRoleDialog = ref(false)
const assignLoading = ref(false)
const selectedRoleIds = ref<string[]>([])
const allRoles = ref<RoleResponse[]>([])

function statusLabel(status: number) {
  switch (status) {
    case 1: return t('user.statusActive')
    case 0: return t('user.statusInactive')
    case 2: return t('user.statusLocked')
    default: return String(status)
  }
}

function statusTagType(status: number): 'success' | 'info' | 'danger' | 'warning' | 'primary' {
  switch (status) {
    case 1: return 'success'
    case 0: return 'info'
    case 2: return 'danger'
    default: return 'info'
  }
}

function formatTimestamp(ts: number) {
  return new Date(ts).toLocaleString()
}

async function fetchUser() {
  loading.value = true
  try {
    const res = await getUser(userId)
    userDetail.value = res.data.data as UserDetail
  } catch {} finally {
    loading.value = false
  }
}

async function fetchAllRoles() {
  try {
    const res = await listRoles({ page: 1, page_size: 200 })
    allRoles.value = res.data.data!.items
  } catch {}
}

async function handleAssignRoles() {
  if (!selectedRoleIds.value.length) return
  assignLoading.value = true
  try {
    await assignRoles(userId, { role_ids: selectedRoleIds.value })
    ElMessage.success(t('common.success'))
    showAssignRoleDialog.value = false
    selectedRoleIds.value = []
    fetchUser()
  } catch {} finally {
    assignLoading.value = false
  }
}

async function handleRemoveRole(roleId: string) {
  await ElMessageBox.confirm(t('common.deleteConfirm'), { type: 'warning' })
  try {
    await removeRole(userId, roleId)
    ElMessage.success(t('common.deleteSuccess'))
    fetchUser()
  } catch {}
}

onMounted(() => {
  fetchUser()
  fetchAllRoles()
})
</script>
