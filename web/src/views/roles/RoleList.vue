<template>
  <div class="page-card">
    <div class="page-header">
      <h2>{{ t('role.title') }}</h2>
      <el-button v-permission="'role:create'" type="primary" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>{{ t('role.createRole') }}
      </el-button>
    </div>

    <!-- Search form -->
    <el-form :inline="true" class="search-form" @submit.prevent="fetchData">
      <el-form-item :label="t('role.name')">
        <el-input v-model="query.name" :placeholder="t('role.name')" clearable />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="fetchData">{{ t('common.search') }}</el-button>
        <el-button @click="resetQuery">{{ t('common.reset') }}</el-button>
      </el-form-item>
    </el-form>

    <!-- Data table -->
    <el-table v-loading="loading" :data="tableData" stripe>
      <el-table-column prop="id" :label="t('role.id')" width="280" show-overflow-tooltip />
      <el-table-column prop="name" :label="t('role.name')" width="150" />
      <el-table-column prop="display_name" :label="t('role.displayName')" width="200" />
      <el-table-column prop="description" :label="t('role.description')" min-width="200" show-overflow-tooltip />
      <el-table-column :label="t('role.isSystem')" width="100">
        <template #default="{ row }">
          <el-tag :type="(row as RoleResponse).is_system ? 'warning' : 'info'" size="small">
            {{ (row as RoleResponse).is_system ? t('role.yes') : t('role.no') }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="sort_order" :label="t('role.sortOrder')" width="100" />
      <el-table-column :label="t('common.actions')" width="220" fixed="right">
        <template #default="{ row }">
          <el-button v-permission="'role:read'" size="small" @click="viewRole((row as RoleResponse).id)">
            {{ t('common.detail') }}
          </el-button>
          <el-button v-permission="'role:update'" size="small" type="primary" @click="editRole(row as RoleResponse)">
            {{ t('common.edit') }}
          </el-button>
          <el-button v-permission="'role:delete'" size="small" type="danger" :disabled="(row as RoleResponse).is_system" @click="handleDelete((row as RoleResponse).id)">
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

    <!-- Create role dialog -->
    <el-dialog v-model="showCreateDialog" :title="t('role.createRole')" width="500">
      <el-form ref="createFormRef" :model="createForm" :rules="createFormRules" label-position="top">
        <el-form-item :label="t('role.name')" prop="name">
          <el-input v-model="createForm.name" />
        </el-form-item>
        <el-form-item :label="t('role.displayName')" prop="display_name">
          <el-input v-model="createForm.display_name" />
        </el-form-item>
        <el-form-item :label="t('role.description')" prop="description">
          <el-input v-model="createForm.description" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item :label="t('role.sortOrder')" prop="sort_order">
          <el-input-number v-model="createForm.sort_order" :min="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showCreateDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="createLoading" @click="handleCreate">{{ t('common.confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- Edit role dialog -->
    <el-dialog v-model="showEditDialog" :title="t('role.editRole')" width="500">
      <el-form ref="editFormRef" :model="editForm" :rules="editFormRules" label-position="top">
        <el-form-item :label="t('role.displayName')" prop="display_name">
          <el-input v-model="editForm.display_name" />
        </el-form-item>
        <el-form-item :label="t('role.description')" prop="description">
          <el-input v-model="editForm.description" type="textarea" :rows="3" />
        </el-form-item>
        <el-form-item :label="t('role.sortOrder')" prop="sort_order">
          <el-input-number v-model="editForm.sort_order" :min="0" />
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
import type { RoleResponse, RoleListQuery, CreateRoleRequest, UpdateRoleRequest } from '@/types'
import { listRoles, createRole, updateRole, deleteRole } from '@/api/role'

const router = useRouter()
const { t } = useI18n()

const loading = ref(false)
const tableData = ref<RoleResponse[]>([])
const total = ref(0)
const query = reactive<RoleListQuery>({ page: 1, page_size: 10 })

// Create
const showCreateDialog = ref(false)
const createFormRef = ref<FormInstance>()
const createLoading = ref(false)
const createForm = reactive<CreateRoleRequest>({
  name: '',
  display_name: '',
  description: '',
  sort_order: 0,
})
const createFormRules = reactive<FormRules>({
  name: [{ required: true, message: () => t('common.required'), trigger: 'blur' }],
})

// Edit
const showEditDialog = ref(false)
const editFormRef = ref<FormInstance>()
const editLoading = ref(false)
const editFormRules = reactive<FormRules>({})
const editForm = reactive<UpdateRoleRequest & { id: string }>({
  id: '',
  display_name: '',
  description: '',
  sort_order: 0,
})

async function fetchData() {
  loading.value = true
  try {
    const res = await listRoles(query)
    const data = res.data.data!
    tableData.value = data.items
    total.value = data.total
  } catch {} finally {
    loading.value = false
  }
}

function resetQuery() {
  query.name = undefined
  query.page = 1
  fetchData()
}

function viewRole(id: string) {
  router.push(`/roles/${id}`)
}

function editRole(row: RoleResponse) {
  editForm.id = row.id
  editForm.display_name = row.display_name || ''
  editForm.description = row.description || ''
  editForm.sort_order = row.sort_order
  showEditDialog.value = true
}

async function handleCreate() {
  const valid = await createFormRef.value?.validate().catch(() => false)
  if (!valid) return
  createLoading.value = true
  try {
    await createRole(createForm)
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
    await updateRole(editForm.id, {
      display_name: editForm.display_name,
      description: editForm.description,
      sort_order: editForm.sort_order,
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
    await deleteRole(id)
    ElMessage.success(t('common.deleteSuccess'))
    fetchData()
  } catch {}
}

onMounted(() => fetchData())
</script>
