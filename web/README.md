# rbac-rs Web

[English](README.md) | [中文](README_zh-CN.md)

The admin frontend for the **rbac-rs** Role-Based Access Control system. Built with Vue 3 + TypeScript + Element Plus, it provides an out-of-the-box management UI for users, roles, and permissions, with built-in Chinese / English internationalization.

## 🚀 Features

### Authentication
- **Login / Register**: JWT-based authentication with access & refresh tokens
- **Token Refresh**: Automatic refresh on 401 responses, seamless session continuity
- **Logout**: Server-side refresh token revocation

### Dashboard
- Welcome banner with the current user's roles and permissions
- Statistic cards for users, roles, permissions, and the current user's permission count

### User Management
- Paginated user list with search & filter (username, email, status)
- User detail view with assigned roles and effective permissions
- Assign / remove roles (with optional expiration time)
- Change password
- Soft-delete support

### Role Management
- Paginated role list
- Role detail view with assigned permissions
- Create / edit / delete roles (system roles are protected from deletion)
- Assign / remove permissions per role

### Permission Management
- Paginated permission list
- Create / edit / delete permissions (resource-action pairs)
- Optional resource path and HTTP method metadata

### Profile
- View current user profile
- Change current user password

### Access Control
- **Route-level guards**: routes declare required permissions (e.g. `user:read`)
- **`v-permission` directive**: declarative element-level permission control
- **Menu visibility**: sidebar items hidden when the user lacks the required permission

### Internationalization
- Built-in **Chinese (zh)** and **English (en)** locales via `vue-i18n`
- Runtime locale switcher in the header, persisted to `localStorage`

### UX
- Responsive layout with a collapsible sidebar
- NProgress top loading bar on route navigation and API requests
- Element Plus message notifications for success / error feedback
- Route transition animations

## 🛠️ Technology Stack

### Core
- **Vue 3.5** - Composition API with `<script setup>`
- **TypeScript 6** - End-to-end type safety
- **Vite 8** - Lightning-fast dev server and build tooling

### UI
- **Element Plus 2.14** - Component library
- **@element-plus/icons-vue** - Icon set
- **Sass** - SCSS styling with CSS variables for theming

### State & Routing
- **Pinia 3** - Type-friendly state management
- **Vue Router 4** - File-less route configuration with guards

### HTTP & Auth
- **Axios 1.18** - HTTP client with request / response interceptors
- **nprogress** - Slim progress bar

### Internationalization
- **vue-i18n 9** - Locale management (zh / en)

### Developer Experience
- **unplugin-auto-import** - Auto-import Vue, Vue Router, Vue I18n, Pinia APIs
- **unplugin-vue-components** - Auto-import Element Plus components on demand
- **vue-tsc** - Vue + TypeScript type checking for production builds

## 📁 Project Structure

```
web/
├── public/                     # Static assets
│   ├── favicon.svg
│   └── icons.svg
├── src/
│   ├── api/                    # API request modules
│   │   ├── auth.ts             # Auth APIs (login, register, refresh, logout)
│   │   ├── user.ts             # User APIs
│   │   ├── role.ts             # Role APIs
│   │   └── permission.ts       # Permission APIs
│   ├── directives/
│   │   └── permission.ts       # v-permission directive
│   ├── layouts/
│   │   └── MainLayout.vue      # Sidebar + header + content layout
│   ├── locales/                # i18n
│   │   ├── en/index.ts         # English messages
│   │   ├── zh/index.ts         # Chinese messages
│   │   └── index.ts            # i18n instance
│   ├── router/
│   │   └── index.ts            # Routes + navigation guards
│   ├── stores/
│   │   └── auth.ts             # Auth store (tokens, user, permissions)
│   ├── styles/
│   │   └── index.scss          # Global styles & CSS variables
│   ├── types/
│   │   └── index.ts            # Shared TypeScript types
│   ├── utils/
│   │   └── request.ts          # Axios instance & interceptors
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
│   ├── main.ts                 # App entry
│   ├── auto-imports.d.ts       # Auto-generated type declarations
│   └── components.d.ts         # Auto-generated component types
├── .env                        # Environment variables
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.app.json
├── tsconfig.node.json
└── vite.config.ts
```

## 🚦 Getting Started

### Prerequisites

- **Node.js** (LTS recommended, v20+)
- **npm** (or compatible package manager such as pnpm / yarn)
- A running **rbac-rs backend** (default: `http://localhost:8080`)

### Installation

1. **Enter the web directory**
```bash
cd web
```

2. **Install dependencies**
```bash
npm install
```

3. **Configure environment variables**
The default `.env` file points to the local backend:
```env
VITE_API_BASE_URL=/api/v1
```
Update this if your backend is hosted elsewhere. The Vite dev server proxies `/api` to `http://localhost:8080` (see `vite.config.ts`).

### Development

Start the dev server (default port: `5173`):
```bash
npm run dev
```
Open `http://localhost:5173` in your browser. The app proxies API requests to the backend configured in `vite.config.ts`.

### Production Build

Type-check and build for production:
```bash
npm run build
```
The output is generated in `dist/`.

### Preview Production Build

Locally preview the production build:
```bash
npm run preview
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `VITE_API_BASE_URL` | Base URL for API requests | `/api/v1` |

### Vite Dev Server Proxy

Defined in `vite.config.ts`:
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
Update the `target` if your backend runs on a different host or port.

### Path Alias

`@` is aliased to `src/`, so you can import modules like `import request from '@/utils/request'`.

## 🔗 Related

- [rbac-rs backend](../README.md) - The Rust + Axum + SeaORM backend for this system.
