import {createRouter, createWebHistory} from 'vue-router'
import NProgress from 'nprogress'
import {useAuthStore} from '@/stores/auth'

NProgress.configure({showSpinner: false})

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/login',
            name: 'Login',
            component: () => import('@/views/auth/Login.vue'),
            meta: {requiresAuth: false},
        },
        {
            path: '/register',
            name: 'Register',
            component: () => import('@/views/auth/Register.vue'),
            meta: {requiresAuth: false},
        },
        {
            path: '/',
            component: () => import('@/layouts/MainLayout.vue'),
            meta: {requiresAuth: true},
            children: [
                {
                    path: '',
                    name: 'Dashboard',
                    component: () => import('@/views/Dashboard.vue'),
                },
                {
                    path: 'users',
                    name: 'Users',
                    component: () => import('@/views/users/UserList.vue'),
                    meta: {permission: 'user:read'},
                },
                {
                    path: 'users/:id',
                    name: 'UserDetail',
                    component: () => import('@/views/users/UserDetail.vue'),
                    meta: {permission: 'user:read'},
                },
                {
                    path: 'roles',
                    name: 'Roles',
                    component: () => import('@/views/roles/RoleList.vue'),
                    meta: {permission: 'role:read'},
                },
                {
                    path: 'roles/:id',
                    name: 'RoleDetail',
                    component: () => import('@/views/roles/RoleDetail.vue'),
                    meta: {permission: 'role:read'},
                },
                {
                    path: 'permissions',
                    name: 'Permissions',
                    component: () => import('@/views/permissions/PermissionList.vue'),
                    meta: {permission: 'perm:read'},
                },
                {
                    path: 'permissions/:id',
                    name: 'PermissionDetail',
                    component: () => import('@/views/permissions/PermissionDetail.vue'),
                    meta: {permission: 'perm:read'},
                },
                {
                    path: 'profile',
                    name: 'Profile',
                    component: () => import('@/views/Profile.vue'),
                },
            ],
        },
    ],
})

router.beforeEach(async (to, _from, next) => {
    NProgress.start()
    const authStore = useAuthStore()

    // Not logged in → redirect to login
    if (to.meta.requiresAuth !== false && !authStore.accessToken) {
        next({name: 'Login', query: {redirect: to.fullPath}})
        return
    }

    // Logged in but user info not loaded (page refresh) → fetch first
    if (authStore.isLoggedIn && !authStore.currentUser) {
        await authStore.fetchCurrentUser()
    }

    // Check route-level permission
    if (to.meta.permission && !authStore.hasPermission(to.meta.permission as string)) {
        next({name: 'Dashboard'})
        return
    }

    next()
})

router.afterEach(() => {
    NProgress.done()
})

export default router