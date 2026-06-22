# rbac-rs

[English](README.md) | [中文](README_zh-CN.md)

一个基于 Rust 构建的现代化、高性能基于角色的访问控制（RBAC）系统，提供全面的用户管理、角色管理和权限控制能力。内置 Vue 3 管理后台前端，开箱即用。

## 🚀 功能特性

### 核心功能
- **用户管理**: 完整的用户 CRUD 操作，支持软删除
- **角色管理**: 创建、更新和管理角色，具备系统角色保护机制
- **权限控制**: 基于资源-动作对的细粒度权限系统
- **JWT 认证**: 安全的令牌基础认证，支持刷新令牌
- **RBAC 中间件**: 通过 Axum 中间件自动进行权限检查
- **分页支持**: 列表接口的高效分页功能

### 高级特性
- **密码安全**: Argon2 密码哈希，确保最高安全性
- **软删除**: 通过逻辑删除时间戳保留数据
- **角色分配过期**: 支持带过期日期的限时角色分配
- **系统角色保护**: 内置关键系统角色保护机制
- **数据库迁移**: 使用 SeaORM 迁移自动化模式管理
- **OpenAPI 文档**: 完整的 API 文档（openapi.yaml）
- **管理后台前端**: Vue 3 + Element Plus 管理后台，支持国际化（中/英）

### API 端点

#### 认证
- `POST /api/v1/auth/register` - 注册新用户
- `POST /api/v1/auth/login` - 登录并获取 JWT 令牌

#### 用户管理
- `GET /api/v1/users/` - 列出所有用户（支持分页和过滤）
- `GET /api/v1/users/{id}` - 获取用户详情（包含角色和权限）
- `PUT /api/v1/users/{id}` - 更新用户信息
- `DELETE /api/v1/users/{id}` - 软删除用户
- `POST /api/v1/users/{id}/roles` - 为用户分配角色
- `DELETE /api/v1/users/{id}/roles/{role_id}` - 从用户移除角色
- `PUT /api/v1/users/change_password` - 修改用户密码
- `GET /api/v1/users/me` - 获取当前用户信息

#### 角色管理
- `GET /api/v1/roles/` - 列出所有角色
- `POST /api/v1/roles/` - 创建新角色
- `GET /api/v1/roles/{id}` - 获取角色详情（包含权限）
- `PUT /api/v1/roles/{id}` - 更新角色
- `DELETE /api/v1/roles/{id}` - 删除角色（系统角色受保护）
- `POST /api/v1/roles/{id}/permissions` - 为角色分配权限
- `DELETE /api/v1/roles/{id}/permissions/{perm_id}` - 从角色移除权限

#### 权限管理
- `GET /api/v1/permissions/` - 列出所有权限
- `POST /api/v1/permissions/` - 创建新权限
- `GET /api/v1/permissions/{id}` - 获取权限详情
- `PUT /api/v1/permissions/{id}` - 更新权限
- `DELETE /api/v1/permissions/{id}` - 删除权限

## 🛠️ 技术栈选型

### Web 框架
- **Axum 0.8** - 基于 Tokio 构建的人体工程学模块化 Web 框架
- **Tokio** - Rust 异步运行时

### 数据库与 ORM
- **SeaORM 2.0** - Rust 动态异步 ORM
- **PostgreSQL** - 主数据库，具备高级查询能力
- **SQLx** - 异步 SQL 工具包（嵌入在 SeaORM 中）

### 安全
- **Argon2 0.5** - 密码哈希算法（密码哈希竞赛获胜者）
- **jsonwebtoken 10.4** - 使用 Rust Crypto 进行 JWT 编码和解码
- **UUID v4** - 唯一标识符生成

### 数据验证与序列化
- **Serde** - 序列化和反序列化框架
- **Validator 0.18** - 使用派生宏进行输入验证
- **Chrono** - 日期和时间处理

### 配置与环境
- **Config 0.15** - 分层配置管理
- **Dotenvy 0.15** - 从 .env 文件加载环境变量

### 日志与追踪
- **Tracing 0.1** - 应用程序级追踪和诊断
- **Tracing-subscriber** - Tracing 的订阅者实现
- **Tower-http** - 带追踪层的 HTTP 中间件

### 数据库迁移
- **SeaORM Migration** - 类型安全的数据库迁移

### 错误处理
- **Anyhow** - 上下文丰富的错误处理
- **Thiserror 2** - 自定义错误类型的派生宏

## 📁 项目结构

```
rbac-rs/
├── api/                    # 主要 API 层
│   ├── src/
│   │   ├── dto/           # 数据传输对象
│   │   │   ├── auth.rs    # 认证 DTO
│   │   │   ├── user.rs    # 用户 DTO
│   │   │   ├── role.rs    # 角色 DTO
│   │   │   ├── permission.rs # 权限 DTO
│   │   │   ├── pagination.rs # 分页 DTO
│   │   │   └── response.rs # 响应包装 DTO
│   │   ├── handler/       # 请求处理器
│   │   │   ├── auth_handler.rs
│   │   │   ├── user_handler.rs
│   │   │   ├── role_handler.rs
│   │   │   └── permission_handler.rs
│   │   ├── service/       # 业务逻辑层
│   │   │   ├── auth_service.rs
│   │   │   ├── user_service.rs
│   │   │   ├── role_service.rs
│   │   │   └── permission_service.rs
│   │   ├── middleware/    # Axum 中间件
│   │   │   ├── auth.rs    # JWT 认证中间件
│   │   │   ├── rbac.rs    # RBAC 权限检查中间件
│   │   │   └── current_user.rs # 当前用户上下文中间件
│   │   ├── config.rs      # 应用配置
│   │   ├── db.rs          # 数据库初始化
│   │   ├── error.rs       # 错误类型和处理
│   │   ├── router.rs      # 路由定义
│   │   └── lib.rs         # 库入口点
│   └── Cargo.toml
├── model/                 # 数据库模型（SeaORM 实体）
│   ├── src/
│   │   ├── user.rs        # 用户实体
│   │   ├── role.rs        # 角色实体
│   │   ├── permission.rs  # 权限实体
│   │   ├── user_role.rs   # 用户-角色关联实体
│   │   ├── role_permission.rs # 角色-权限关联实体
│   │   └── lib.rs
│   └── Cargo.toml
├── migration/             # 数据库迁移
│   ├── src/
│   │   ├── m20260619_000001_create_table.rs # 创建表结构
│   │   ├── m20260620_000001_init_data.rs    # 初始化数据
│   │   ├── m20260621_000001_init_super_admin.rs # 初始化管理员
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
├── util/                  # 工具函数
│   ├── src/
│   │   ├── password.rs    # 密码哈希工具
│   │   └── lib.rs
│   └── Cargo.toml
├── settings/              # 配置文件
│   ├── default.toml       # 默认配置
│   └── {profile}.toml     # 环境特定配置
├── src/
│   └── main.rs            # 应用入口点
├── openapi.yaml           # OpenAPI 3.0 规范
├── .env                   # 环境变量
└── Cargo.toml             # 工作区配置
```

## 🚦 快速开始

### 前置要求

- **Rust** (Edition 2024, 版本 1.85.0 或更高)
- **PostgreSQL** (最新稳定版本)
- **Cargo** (Rust 包管理器)

### 安装步骤

1. **克隆仓库**
```bash
git clone <repository-url>
cd RBAC-RUST
```

2. **设置环境变量**
在项目根目录创建 `.env` 文件：
```env
DATABASE_URL=postgres://admin:admin@localhost/mydb?options=--search_path=rbac
APP_PROFILE=dev
RUST_LOG=info
APP_ADMIN_NAME=admin
APP_ADMIN_PASS=superadmin
```

3. **配置数据库设置**
编辑 `settings/default.toml`：
```toml
[server]
port = 8080
host = "0.0.0.0"

[database]
url = "postgres://admin:admin@localhost/mydb?options=--search_path=rbac"

[jwt]
secret = "your-secret-key-here"
expiration_secs = 300
```

4. **运行数据库迁移**
```bash
cd migration
cargo run -- up
```

5. **启动开发服务器**
```bash
cargo run
```

服务器将在 `http://localhost:8080` 启动

### 生产环境构建

```bash
cargo build --release
```

## 🗄️ 数据库架构

### 数据表

- **users** - 用户账户，包含认证凭证
- **roles** - 角色定义，可选系统角色标志
- **permissions** - 权限定义（资源-动作对）
- **user_roles** - 用户和角色之间的多对多关系
- **role_permissions** - 角色和权限之间的多对多关系

### 关键特性

- 所有表使用 UUID 主键
- 通过 `deleted_at` 时间戳支持软删除
- 时间戳追踪（`created_at`, `updated_at`）
- 带级联删除的外键约束
- 优化查询性能的索引
- 可选的角色分配过期时间（`expires_at`）

## 🔐 安全特性

### 认证流程

1. 用户使用用户名、邮箱和密码注册
2. 密码在存储前使用 Argon2 进行哈希
3. 用户使用凭据登录
4. 服务器验证凭据并颁发 JWT 访问令牌 + 刷新令牌
5. 受保护的端点需要访问令牌（Bearer 认证）
6. 中间件验证令牌并提取用户上下文

### 授权（RBAC）

1. 每个端点指定所需权限（例如：`user:read`, `role:create`）
2. RBAC 中间件在认证后拦截请求
3. 系统检查用户的角色是否授予所需权限
4. 根据权限检查结果授予或拒绝访问

### 密码安全

- Argon2 哈希算法（内存硬函数）
- 自动生成盐值
- 可配置的哈希参数

### JWT 配置

- 可配置的密钥
- 可配置的过期时间（默认：300 秒）
- Bearer 令牌认证方案

## 📝 API 示例

### 注册新用户

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepass123",
    "full_name": "张三"
  }'
```

### 登录

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "password": "securepass123"
  }'
```

响应：
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 300
  }
}
```

### 列出用户（已认证）

```bash
curl -X GET "http://localhost:8080/api/v1/users/?page=1&page_size=10" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

### 创建角色（需要权限）

```bash
curl -X POST http://localhost:8080/api/v1/roles/ \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "editor",
    "display_name": "内容编辑",
    "description": "可以编辑内容",
    "sort_order": 10
  }'
```

## 🧪 测试

运行测试：
```bash
cargo test
```

## 📊 配置说明

### 环境变量

| 变量 | 描述 | 默认值 |
|------|------|--------|
| `APP_PROFILE` | 应用配置文件（dev/prod） | `dev` |
| `RUST_LOG` | 日志级别过滤器 | `info` |
| `DATABASE_URL` | PostgreSQL 连接字符串 | - |
| `APP_ADMIN_NAME` | 初始迁移时超级管理员用户名 | `admin` |
| `APP_ADMIN_PASS` | 初始迁移时超级管理员密码 | `superadmin` |

### 配置文件

配置按以下顺序加载（后面的源覆盖前面的源）：
1. `settings/default.toml` - 基础配置
2. `settings/{APP_PROFILE}.toml` - 配置文件特定覆盖
3. 带有 `APP_` 前缀的环境变量

### 服务器配置

```toml
[server]
port = 8080
host = "0.0.0.0"
```

### 数据库配置

```toml
[database]
url = "postgres://user:password@localhost/dbname"
```

### JWT 配置

```toml
[jwt]
secret = "change-this-to-a-strong-secret"
expiration_secs = 300
```

## 🔄 数据库迁移

### 运行迁移

```bash
cd migration
cargo run -- up
```

> **注意**：迁移会自动使用 `.env` 文件中的 `APP_ADMIN_NAME` 和 `APP_ADMIN_PASS` 环境变量创建超级管理员用户。请确保在运行迁移前设置这些变量，并在生产环境中修改默认凭据。

### 回滚迁移

```bash
cd migration
cargo run -- down
```

### 全新迁移

```bash
cd migration
cargo run -- fresh
```

## 📖 API 文档

API 遵循 RESTful 约定并返回一致的 JSON 响应：

```json
{
  "code": 0,
  "message": "",
  "data": { ... }
}
```

错误响应包含适当的 HTTP 状态码和错误消息。

查看 `openapi.yaml` 获取完整的 API 规范。您可以将此文件导入 Swagger UI 或 Postman 等工具以获取交互式文档。

## 🏗️ 架构设计

### 分层架构

1. **处理器层** (`api/src/handler/`)
   - HTTP 请求/响应处理
   - 输入验证
   - 响应格式化

2. **服务层** (`api/src/service/`)
   - 业务逻辑
   - 事务管理
   - 数据转换

3. **模型层** (`model/`)
   - 数据库实体
   - 实体关系
   - 查询构建器

4. **中间件层** (`api/src/middleware/`)
   - 认证
   - 授权（RBAC）
   - 请求上下文

### 设计模式

- **仓储模式**: 通过 SeaORM 实体抽象
- **依赖注入**: 通过 Axum 扩展传递状态
- **中间件链**: 可组合的请求处理
- **DTO 模式**: API 契约与领域模型分离

## 🤝 贡献指南

欢迎贡献！请随时提交 Pull Request。

## 📄 许可证

本项目是开源的，采用 MIT 许可证。详见 [LICENSE](LICENSE)。

## 🙏 致谢

- [Axum](https://github.com/tokio-rs/axum) - 优秀的 Web 框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) - 强大的异步 ORM
- [Tokio](https://tokio.rs/) - 可靠的异步运行时
- [Vue 3](https://vuejs.org/) & [Element Plus](https://element-plus.org/) - 前端技术栈

## 📞 支持

如有问题、疑问或贡献，请在仓库中开启 Issue。

---

使用 Rust 构建 ❤️
