<template>
  <div class="page-card">
    <div class="page-header">
      <h2>{{ t('user.title') }}</h2>
      <el-button v-permission="'user:create'" type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>{{ t('user.createUser') }}
      </el-button>
    </div>

    <!-- Search form -->
    <el-form :inline="true" class="search-form" @submit.prevent="fetchData">
      <el-form-item :label="t('user.username')">
        <el-input v-model="query.username" :placeholder="t('user.username')" clearable />
      </el-form-item>
      <el-form-item :label="t('user.email')">
        <el-input v-model="query.email" :placeholder="t('user.email')" clearable />
      </el-form-item>
      <el-form-item :label="t('user.status')">
        <el-select v-model="query.status" :placeholder="t('user.status')" clearable style="width: 100px">
          <el-option :label="t('user.statusActive')" :value="1" />
          <el-option :label="t('user.statusInactive')" :value="0" />
          <el-option :label="t('user.statusLocked')" :value="2" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="fetchData">{{ t('common.search') }}</el-button>
        <el-button @click="resetQuery">{{ t('common.reset') }}</el-button>
      </el-form-item>
    </el-form>

    <!-- Data table -->
    <el-table v-loading="loading" :data="tableData" stripe row-key="id">
      <el-table-column prop="id" :label="t('user.id')" width="280" show-overflow-tooltip />
      <el-table-column prop="username" :label="t('user.username')" width="120" />
      <el-table-column prop="email" :label="t('user.email')" width="200" show-overflow-tooltip />
      <el-table-column prop="display_name" :label="t('user.displayName')" width="140" />
      <el-table-column :label="t('user.status')" width="100">
        <template #default="{ row }">
          <el-tag :type="statusTagType((row as UserBrief).status ?? 1)">
            {{ statusLabel((row as UserBrief).status ?? 1) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column :label="t('user.createdAt')" width="180">
        <template #default="{ row }">
          {{ formatTimestamp((row as UserBrief).created_at) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('common.actions')" width="220" fixed="right">
        <template #default="{ row }">
          <el-button v-permission="'user:read'" size="small" @click="viewUser((row as UserBrief).id)">
            {{ t('common.detail') }}
          </el-button>
          <el-button v-permission="'user:update'" size="small" type="primary" @click="editUser(row as UserBrief)">
            {{ t('common.edit') }}
          </el-button>
          <el-button v-permission="'user:delete'" size="small" type="danger" @click="handleDelete((row as UserBrief).id)">
            {{ t('common.delete') }}
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <div class="pagination-wrapper">
      <el-pagination
        v-model:current-page="query.page"
        v-model:page-size="query.page_size"
        :total="total"
        :page-sizes="[10, 20, 50, 100]"
        layout="total, sizes, prev, pager, next"
        @size-change="fetchData"
        @current-change="fetchData"
      />
    </div>

    <!-- Create user dialog -->
    <el-dialog v-model="showCreateDialog" :title="t('user.createUser')" width="500">
      <el-form ref="createFormRef" :model="createForm" :rules="createFormRules" label-position="top">
        <el-form-item :label="t('user.username')" prop="username">
          <el-input v-model="createForm.username" />
        </el-form-item>
        <el-form-item :label="t('user.email')" prop="email">
          <el-input v-model="createForm.email" />
        </el-form-item>
        <el-form-item :label="t('auth.password')" prop="password">
          <el-input v-model="createForm.password" type="password" show-password />
        </el-form-item>
        <el-form-item :label="t('user.displayName')" prop="display_name">
          <el-input v-model="createForm.display_name" />
        </el-form-item>
        <el-form-item :label="t('user.status')" prop="status">
          <el-select v-model="createForm.status">
            <el-option :label="t('user.statusActive')" :value="1" />
            <el-option :label="t('user.statusInactive')" :value="0" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="createLoading" @click="handleCreate">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- Edit user dialog -->
    <el-dialog v-model="showEditDialog" :title="t('user.editUser')" width="500">
      <el-form ref="editFormRef" :model="editForm" :rules="editFormRules" label-position="top">
        <el-form-item :label="t('user.email')" prop="email">
          <el-input v-model="editForm.email" />
        </el-form-item>
        <el-form-item :label="t('user.displayName')" prop="display_name">
          <el-input v-model="editForm.display_name" />
        </el-form-item>
        <el-form-item :label="t('user.status')" prop="status">
          <el-select v-model="editForm.status">
            <el-option :label="t('user.statusActive')" :value="1" />
            <el-option :label="t('user.statusInactive')" :value="0" />
            <el-option :label="t('user.statusLocked')" :value="2" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showEditDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="editLoading" @click="handleEdit">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import type { UserBrief, UserListQuery, CreateUserRequest, UpdateUserRequest } from '@/types'
import { listUsers, createUser, updateUser, deleteUser } from '@/api/user'

const router = useRouter()
const { t } = useI18n()

const loading = ref(false)
const tableData = ref<UserBrief[]>([])
const total = ref(0)
const query = reactive<UserListQuery>({ page: 1, page_size: 10 })

// Create
const showCreateDialog = ref(false)
const createFormRef = ref<FormInstance>()
const createLoading = ref(false)
const createForm = reactive<CreateUserRequest>({
  username: '',
  email: '',
  password: '',
  display_name: '',
  status: 1,
})
const createFormRules = reactive<FormRules>({
  username: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
  email: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
  password: [
    { required: true, message: () => t('common.required'), trigger: 'blur' },
    { min: 8, message: () => t('auth.passwordMin'), trigger: 'blur' },
  ],
})

// Edit
const showEditDialog = ref(false)
const editFormRef = ref<FormInstance>()
const editLoading = ref(false)
const editForm = reactive<UpdateUserRequest & { id: string }>({
  id: '',
  email: '',
  display_name: '',
  status: 1,
})
const editFormRules = reactive<FormRules>({
  email: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
})

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

async function fetchData() {
  loading.value = true
  try {
    const res = await listUsers(query)
    const data = res.data.data!
    tableData.value = data.items
    total.value = data.total
  } catch {
    // handled by interceptor
  } finally {
    loading.value = false
  }
}

function resetQuery() {
  query.username = undefined
  query.email = undefined
  query.status = undefined
  query.page = 1
  fetchData()
}

function viewUser(id: string) {
  router.push(`/users/${id}`)
}

function editUser(row: UserBrief) {
  editForm.id = row.id
  editForm.email = row.email
  editForm.display_name = row.display_name || ''
  editForm.status = row.status ?? 1
  showEditDialog.value = true
}

async function handleCreate() {
  const valid = await createFormRef.value?.validate().catch(() => false)
  if (!valid) return
  createLoading.value = true
  try {
    await createUser(createForm)
    ElMessage.success(t('common.success'))
    showCreateDialog.value = false
    createFormRef.value?.resetFields()
    fetchData()
  } catch {} finally {
    createLoading.value = false
  }
}

async function handleEdit() {
  const valid = await editFormRef.value?.validate().catch(() => false)
  if (!valid) return
  editLoading.value = true
  try {
    await updateUser(editForm.id, {
      email: editForm.email,
      display_name: editForm.display_name,
      status: editForm.status,
    })
    ElMessage.success(t('common.success'))
    showEditDialog.value = false
    fetchData()
  } catch {} finally {
    editLoading.value = false
  }
}

async function handleDelete(id: string) {
  await ElMessageBox.confirm(t('common.deleteConfirm'), { type: 'warning' })
  try {
    await deleteUser(id)
    ElMessage.success(t('common.deleteSuccess'))
    fetchData()
  } catch {}
}

onMounted(() => fetchData())
</script>
