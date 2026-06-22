# rbac-rs Web

[English](README.md) | [中文](README_zh-CN.md)

**rbac-rs** 基于角色的访问控制系统的管理后台前端。基于 Vue 3 + TypeScript + Element Plus 构建，提供开箱即用的用户、角色、权限管理界面，内置中英文国际化支持。

## 🚀 主要功能

### 认证
- **登录 / 注册**：基于 JWT 的认证，支持访问令牌和刷新令牌
- **令牌刷新**：401 响应时自动刷新令牌，会话无缝衔接
- **登出**：服务端撤销刷新令牌

### 仪表盘
- 欢迎横幅，展示当前用户的角色和权限
- 用户数、角色数、权限数及当前用户权限数的统计卡片

### 用户管理
- 分页用户列表，支持按用户名、邮箱、状态搜索过滤
- 用户详情视图，展示已分配角色和有效权限
- 分配 / 移除角色（支持设置过期时间）
- 修改密码
- 支持软删除

### 角色管理
- 分页角色列表
- 角色详情视图，展示已分配权限
- 创建 / 编辑 / 删除角色（系统角色受保护，不可删除）
- 为角色分配 / 移除权限

### 权限管理
- 分页权限列表
- 创建 / 编辑 / 删除权限（资源-动作对）
- 可选的资源路径和 HTTP 方法元数据

### 个人中心
- 查看当前用户资料
- 修改当前用户密码

### 访问控制
- **路由级守卫**：路由声明所需权限（如 `user:read`）
- **`v-permission` 指令**：声明式的元素级权限控制
- **菜单可见性**：当用户缺少所需权限时，侧边栏菜单项自动隐藏

### 国际化
- 通过 `vue-i18n` 内置 **中文（zh）** 和 **英文（en）** 两种语言
- 顶栏运行时切换语言，并持久化到 `localStorage`

### 用户体验
- 响应式布局，侧边栏可折叠
- 路由跳转和 API 请求时显示 NProgress 顶部进度条
- Element Plus 消息通知，反馈成功 / 错误
- 路由切换过渡动画

## 🛠️ 技术栈

### 核心
- **Vue 3.5** - 使用 `<script setup>` 的组合式 API
- **TypeScript 6** - 端到端类型安全
- **Vite 8** - 极速的开发服务器和构建工具

### UI
- **Element Plus 2.14** - 组件库
- **@element-plus/icons-vue** - 图标库
- **Sass** - 使用 CSS 变量主题化的 SCSS 样式

### 状态与路由
- **Pinia 3** - 类型友好的状态管理
- **Vue Router 4** - 无文件式路由配置 + 守卫

### HTTP 与认证
- **Axios 1.18** - 带请求 / 响应拦截器的 HTTP 客户端
- **nprogress** - 轻量进度条

### 国际化
- **vue-i18n 9** - 多语言管理（zh / en）

### 开发体验
- **unplugin-auto-import** - 自动导入 Vue、Vue Router、Vue I18n、Pinia 的 API
- **unplugin-vue-components** - 按需自动导入 Element Plus 组件
- **vue-tsc** - 生产构建时的 Vue + TypeScript 类型检查

## 📁 项目结构

```
web/
├── public/                     # 静态资源
│   ├── favicon.svg
│   └── icons.svg
├── src/
│   ├── api/                    # API 请求模块
│   │   ├── auth.ts             # 认证 API（登录、注册、刷新、登出）
│   │   ├── user.ts             # 用户 API
│   │   ├── role.ts             # 角色 API
│   │   └── permission.ts       # 权限 API
│   ├── directives/
│   │   └── permission.ts       # v-permission 指令
│   ├── layouts/
│   │   └── MainLayout.vue      # 侧边栏 + 顶栏 + 内容区布局
│   ├── locales/                # 国际化
│   │   ├── en/index.ts         # 英文文案
│   │   ├── zh/index.ts         # 中文文案
│   │   └── index.ts            # i18n 实例
│   ├── router/
│   │   └── index.ts            # 路由 + 导航守卫
│   ├── stores/
│   │   └── auth.ts             # 认证 store（令牌、用户、权限）
│   ├── styles/
│   │   └── index.scss          # 全局样式 & CSS 变量
│   ├── types/
│   │   └── index.ts            # 共享的 TypeScript 类型
│   ├── utils/
│   │   └── request.ts          # Axios 实例 & 拦截器
│   ├── views/
│   │   ├── auth/
│   │   │   ├── Login.vue
│   │   │   └── Register.vue
│   │   ├── users/
│   │   │   ├── UserList.vue
│   │   │   └── UserDetail.vue
│   │   ├── roles/
│   │   │   ├── RoleList.vue
│   │   │   └── RoleDetail.vue
│   │   ├── permissions/
│   │   │   ├── PermissionList.vue
│   │   │   └── PermissionDetail.vue
│   │   ├── Dashboard.vue
│   │   └── Profile.vue
│   ├── App.vue
│   ├── main.ts                 # 应用入口
│   ├── auto-imports.d.ts       # 自动生成的类型声明
│   └── components.d.ts         # 自动生成的组件类型
├── .env                        # 环境变量
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.app.json
├── tsconfig.node.json
└── vite.config.ts
```

## 🚦 快速开始

### 前置要求

- **Node.js**（推荐 LTS 版本，v20+）
- **npm**（或兼容的包管理器，如 pnpm / yarn）
- 已运行的 **rbac-rs 后端**（默认：`http://localhost:8080`）

### 安装步骤

1. **进入 web 目录**
```bash
cd web
```

2. **安装依赖**
```bash
npm install
```

3. **配置环境变量**
默认 `.env` 文件指向本地后端：
```env
VITE_API_BASE_URL=/api/v1
```
如果后端部署在其他位置，请修改此变量。Vite 开发服务器会将 `/api` 代理到 `http://localhost:8080`（见 `vite.config.ts`）。

### 开发模式

启动开发服务器（默认端口：`5173`）：
```bash
npm run dev
```
在浏览器中打开 `http://localhost:5173`。应用会将 API 请求代理到 `vite.config.ts` 中配置的后端地址。

### 生产环境构建

类型检查并构建生产版本：
```bash
npm run build
```
构建产物输出到 `dist/` 目录。

### 预览生产构建

本地预览生产构建：
```bash
npm run preview
```

## ⚙️ 配置说明

### 环境变量

| 变量 | 描述 | 默认值 |
|------|------|--------|
| `VITE_API_BASE_URL` | API 请求的基础 URL | `/api/v1` |

### Vite 开发服务器代理

在 `vite.config.ts` 中定义：
```ts
server: {
  port: 5173,
  proxy: {
    '/api': {
      target: 'http://localhost:8080',
      changeOrigin: true,
    },
  },
}
```
如果后端运行在不同的主机或端口，请修改 `target`。

### 路径别名

`@` 被别名为 `src/`，可以这样导入模块：`import request from '@/utils/request'`。

## 🔗 相关链接

- [rbac-rs 后端](../README_zh-CN.md) - 本系统的 Rust + Axum + SeaORM 后端。

