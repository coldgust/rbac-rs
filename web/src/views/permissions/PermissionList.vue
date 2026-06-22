<template>
  <div class="page-card">
    <div class="page-header">
      <h2>{{ t('permission.title') }}</h2>
      <el-button v-permission="'perm:create'" type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>{{ t('permission.createPermission') }}
      </el-button>
    </div>

    <!-- Search form -->
    <el-form :inline="true" class="search-form" @submit.prevent="fetchData">
      <el-form-item :label="t('permission.resource')">
        <el-input v-model="query.resource" :placeholder="t('permission.resource')" clearable />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="fetchData">{{ t('common.search') }}</el-button>
        <el-button @click="resetQuery">{{ t('common.reset') }}</el-button>
      </el-form-item>
    </el-form>

    <!-- Data table -->
    <el-table v-loading="loading" :data="tableData" stripe>
      <el-table-column prop="id" :label="t('permission.id')" width="280" show-overflow-tooltip />
      <el-table-column prop="resource" :label="t('permission.resource')" width="120" />
      <el-table-column prop="action" :label="t('permission.action')" width="120" />
      <el-table-column prop="name" :label="t('permission.name')" width="200" />
      <el-table-column prop="description" :label="t('permission.description')" min-width="200" show-overflow-tooltip />
      <el-table-column prop="resource_path" :label="t('permission.resourcePath')" width="200" show-overflow-tooltip />
      <el-table-column prop="http_method" :label="t('permission.httpMethod')" width="100" />
      <el-table-column :label="t('common.actions')" width="220" fixed="right">
        <template #default="{ row }">
          <el-button v-permission="'perm:read'" size="small" @click="viewPermission((row as PermissionResponse).id)">
            {{ t('common.detail') }}
          </el-button>
          <el-button v-permission="'perm:update'" size="small" type="primary" @click="editPermission(row as PermissionResponse)">
            {{ t('common.edit') }}
          </el-button>
          <el-button v-permission="'perm:delete'" size="small" type="danger" @click="handleDelete((row as PermissionResponse).id)">
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

    <!-- Create permission dialog -->
    <el-dialog v-model="showCreateDialog" :title="t('permission.createPermission')" width="500">
      <el-form ref="createFormRef" :model="createForm" :rules="createFormRules" label-position="top">
        <el-form-item :label="t('permission.resource')" prop="resource">
          <el-input v-model="createForm.resource" />
        </el-form-item>
        <el-form-item :label="t('permission.action')" prop="action">
          <el-input v-model="createForm.action" />
        </el-form-item>
        <el-form-item :label="t('permission.name')" prop="name">
          <el-input v-model="createForm.name" />
        </el-form-item>
        <el-form-item :label="t('permission.description')" prop="description">
          <el-input v-model="createForm.description" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item :label="t('permission.resourcePath')" prop="resource_path">
          <el-input v-model="createForm.resource_path" />
        </el-form-item>
        <el-form-item :label="t('permission.httpMethod')" prop="http_method">
          <el-select v-model="createForm.http_method" clearable>
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
            <el-option label="PATCH" value="PATCH" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="createLoading" @click="handleCreate">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- Edit permission dialog -->
    <el-dialog v-model="showEditDialog" :title="t('permission.editPermission')" width="500">
      <el-form ref="editFormRef" :model="editForm" :rules="editFormRules" label-position="top">
        <el-form-item :label="t('permission.name')" prop="name">
          <el-input v-model="editForm.name" />
        </el-form-item>
        <el-form-item :label="t('permission.description')" prop="description">
          <el-input v-model="editForm.description" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item :label="t('permission.resourcePath')" prop="resource_path">
          <el-input v-model="editForm.resource_path" />
        </el-form-item>
        <el-form-item :label="t('permission.httpMethod')" prop="http_method">
          <el-select v-model="editForm.http_method" clearable>
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
            <el-option label="PATCH" value="PATCH" />
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
import type { PermissionResponse, PermissionListQuery, CreatePermissionRequest, UpdatePermissionRequest } from '@/types'
import { listPermissions, createPermission, updatePermission, deletePermission } from '@/api/permission'

const router = useRouter()
const { t } = useI18n()

const loading = ref(false)
const tableData = ref<PermissionResponse[]>([])
const total = ref(0)
const query = reactive<PermissionListQuery>({ page: 1, page_size: 10 })

// Create
const showCreateDialog = ref(false)
const createFormRef = ref<FormInstance>()
const createLoading = ref(false)
const createForm = reactive<CreatePermissionRequest>({
  resource: '',
  action: '',
  name: '',
  description: '',
  resource_path: '',
  http_method: '',
})
const createFormRules = reactive<FormRules>({
  resource: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
  action: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
  name: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
})

// Edit
const showEditDialog = ref(false)
const editFormRef = ref<FormInstance>()
const editLoading = ref(false)
const editFormRules = reactive<FormRules>({})
const editForm = reactive<UpdatePermissionRequest & { id: string }>({
  id: '',
  name: '',
  description: '',
  resource_path: '',
  http_method: '',
})

async function fetchData() {
  loading.value = true
  try {
    const res = await listPermissions(query)
    const data = res.data.data!
    tableData.value = data.items
    total.value = data.total
  } catch {} finally {
    loading.value = false
  }
}

function resetQuery() {
  query.resource = undefined
  query.page = 1
  fetchData()
}

function viewPermission(id: string) {
  router.push(`/permissions/${id}`)
}

function editPermission(row: PermissionResponse) {
  editForm.id = row.id
  editForm.name = row.name
  editForm.description = row.description || ''
  editForm.resource_path = row.resource_path || ''
  editForm.http_method = row.http_method || ''
  showEditDialog.value = true
}

async function handleCreate() {
  const valid = await createFormRef.value?.validate().catch(() => false)
  if (!valid) return
  createLoading.value = true
  try {
    await createPermission(createForm)
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
    await updatePermission(editForm.id, {
      name: editForm.name,
      description: editForm.description,
      resource_path: editForm.resource_path,
      http_method: editForm.http_method,
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
    await deletePermission(id)
    ElMessage.success(t('common.deleteSuccess'))
    fetchData()
  } catch {}
}

onMounted(() => fetchData())
</script>