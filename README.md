# rbac-rs

[English](README.md) | [中文](README_zh-CN.md)

A modern, high-performance Role-Based Access Control (RBAC) system built with Rust, providing comprehensive user management, role management, and permission control capabilities. It includes a Vue 3 admin frontend for out-of-the-box management.

## 🚀 Features

### Core Functionality
- **User Management**: Complete CRUD operations for users with soft delete support
- **Role Management**: Create, update, and manage roles with system role protection
- **Permission Control**: Fine-grained permission system based on resource-action pairs
- **JWT Authentication**: Secure token-based authentication with refresh token support
- **RBAC Middleware**: Automatic permission checking via Axum middleware
- **Pagination Support**: Efficient pagination for list endpoints

### Advanced Features
- **Password Security**: Argon2 password hashing for maximum security
- **Soft Delete**: Data preservation with logical deletion timestamps
- **Role Assignment Expiry**: Time-limited role assignments with expiration dates
- **System Roles Protection**: Built-in protection for critical system roles
- **Database Migrations**: Automated schema management with SeaORM migrations
- **OpenAPI Documentation**: Comprehensive API documentation (openapi.yaml)
- **Admin Frontend**: Vue 3 + Element Plus admin dashboard with i18n support

### API Endpoints

#### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login and receive JWT tokens

#### User Management
- `GET /api/v1/users/` - List all users (with pagination and filters)
- `GET /api/v1/users/{id}` - Get user details with roles and permissions
- `PUT /api/v1/users/{id}` - Update user information
- `DELETE /api/v1/users/{id}` - Soft delete user
- `POST /api/v1/users/{id}/roles` - Assign roles to user
- `DELETE /api/v1/users/{id}/roles/{role_id}` - Remove role from user
- `PUT /api/v1/users/change_password` - Change user password
- `GET /api/v1/users/me` - Get current user information

#### Role Management
- `GET /api/v1/roles/` - List all roles
- `POST /api/v1/roles/` - Create new role
- `GET /api/v1/roles/{id}` - Get role details with permissions
- `PUT /api/v1/roles/{id}` - Update role
- `DELETE /api/v1/roles/{id}` - Delete role (system roles protected)
- `POST /api/v1/roles/{id}/permissions` - Assign permissions to role
- `DELETE /api/v1/roles/{id}/permissions/{perm_id}` - Remove permission from role

#### Permission Management
- `GET /api/v1/permissions/` - List all permissions
- `POST /api/v1/permissions/` - Create new permission
- `GET /api/v1/permissions/{id}` - Get permission details
- `PUT /api/v1/permissions/{id}` - Update permission
- `DELETE /api/v1/permissions/{id}` - Delete permission

## 🛠️ Technology Stack

### Backend Framework
- **Axum 0.8** - Ergonomic and modular web framework built on Tokio
- **Tokio** - Asynchronous runtime for Rust

### Database & ORM
- **SeaORM 2.0** - Dynamic async ORM for Rust
- **PostgreSQL** - Primary database with advanced querying capabilities
- **SQLx** - Async SQL toolkit (embedded in SeaORM)

### Security
- **Argon2 0.5** - Password hashing algorithm (winner of Password Hashing Competition)
- **jsonwebtoken 10.4** - JWT encoding and decoding with Rust Crypto
- **UUID v4** - Unique identifier generation

### Data Validation & Serialization
- **Serde** - Serialization and deserialization framework
- **Validator 0.18** - Input validation with derive macros
- **Chrono** - Date and time handling

### Configuration & Environment
- **Config 0.15** - Hierarchical configuration management
- **Dotenvy 0.15** - Environment variable loading from .env files

### Logging & Tracing
- **Tracing 0.1** - Application-level tracing and diagnostics
- **Tracing-subscriber** - Subscriber implementation for tracing
- **Tower-http** - HTTP middleware with trace layer

### Database Migration
- **SeaORM Migration** - Type-safe database migrations

### Error Handling
- **Anyhow** - Context-rich error handling
- **Thiserror 2** - Derive macro for custom error types

## 📁 Project Structure

```
rbac-rs/
├── api/                    # Main API layer
│   ├── src/
│   │   ├── dto/           # Data Transfer Objects
│   │   │   ├── auth.rs    # Authentication DTOs
│   │   │   ├── user.rs    # User DTOs
│   │   │   ├── role.rs    # Role DTOs
│   │   │   ├── permission.rs # Permission DTOs
│   │   │   ├── pagination.rs # Pagination DTOs
│   │   │   └── response.rs # Response wrapper DTOs
│   │   ├── handler/       # Request handlers
│   │   │   ├── auth_handler.rs
│   │   │   ├── user_handler.rs
│   │   │   ├── role_handler.rs
│   │   │   └── permission_handler.rs
│   │   ├── service/       # Business logic layer
│   │   │   ├── auth_service.rs
│   │   │   ├── user_service.rs
│   │   │   ├── role_service.rs
│   │   │   └── permission_service.rs
│   │   ├── middleware/    # Axum middleware
│   │   │   ├── auth.rs    # JWT authentication middleware
│   │   │   ├── rbac.rs    # RBAC permission checking middleware
│   │   │   └── current_user.rs # Current user context middleware
│   │   ├── config.rs      # Application configuration
│   │   ├── db.rs          # Database initialization
│   │   ├── error.rs       # Error types and handling
│   │   ├── router.rs      # Route definitions
│   │   └── lib.rs         # Library entry point
│   └── Cargo.toml
├── model/                 # Database models (SeaORM entities)
│   ├── src/
│   │   ├── user.rs        # User entity
│   │   ├── role.rs        # Role entity
│   │   ├── permission.rs  # Permission entity
│   │   ├── user_role.rs   # User-Role junction entity
│   │   ├── role_permission.rs # Role-Permission junction entity
│   │   └── lib.rs
│   └── Cargo.toml
├── migration/             # Database migrations
│   ├── src/
│   │   ├── m20260619_000001_create_table.rs # Schema creation
│   │   ├── m20260620_000001_init_data.rs    # Initial data seeding
│   │   ├── m20260621_000001_init_super_admin.rs # Super admin setup
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
├── util/                  # Utility functions
│   ├── src/
│   │   ├── password.rs    # Password hashing utilities
│   │   └── lib.rs
│   └── Cargo.toml
├── settings/              # Configuration files
│   ├── default.toml       # Default configuration
│   └── {profile}.toml     # Environment-specific configs
├── src/
│   └── main.rs            # Application entry point
├── openapi.yaml           # OpenAPI 3.0 specification
├── .env                   # Environment variables
└── Cargo.toml             # Workspace configuration
```

## 🚦 Getting Started

### Prerequisites

- **Rust** (Edition 2024, version 1.85.0 or higher)
- **PostgreSQL** (latest stable version)
- **Cargo** (Rust package manager)

### Installation

1. **Clone the repository**
```bash
git clone <repository-url>
cd RBAC-RUST
```

2. **Set up environment variables**
Create a `.env` file in the project root:
```env
DATABASE_URL=postgres://admin:admin@localhost/mydb?options=--search_path=rbac
APP_PROFILE=dev
RUST_LOG=info
APP_ADMIN_NAME=admin
APP_ADMIN_PASS=superadmin
```

3. **Configure database settings**
Edit `settings/default.toml`:
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

4. **Run database migrations**
```bash
cd migration
cargo run -- up
```

5. **Start the development server**
```bash
cargo run
```

The server will start at `http://localhost:8080`

### Build for Production

```bash
cargo build --release
```

## 🗄️ Database Schema

### Tables

- **users** - User accounts with authentication credentials
- **roles** - Role definitions with optional system role flag
- **permissions** - Permission definitions (resource-action pairs)
- **user_roles** - Many-to-many relationship between users and roles
- **role_permissions** - Many-to-many relationship between roles and permissions

### Key Features

- UUID primary keys for all tables
- Soft delete support via `deleted_at` timestamp
- Timestamp tracking (`created_at`, `updated_at`)
- Foreign key constraints with cascade deletes
- Indexes for optimized query performance
- Optional role assignment expiration (`expires_at`)

## 🔐 Security Features

### Authentication Flow

1. User registers with username, email, and password
2. Password is hashed using Argon2 before storage
3. User logs in with credentials
4. Server validates credentials and issues JWT access token + refresh token
5. Access token is required for protected endpoints (Bearer authentication)
6. Middleware validates token and extracts user context

### Authorization (RBAC)

1. Each endpoint specifies required permissions (e.g., `user:read`, `role:create`)
2. RBAC middleware intercepts requests after authentication
3. System checks if user's roles grant the required permission
4. Access is granted or denied based on permission check

### Password Security

- Argon2 hashing algorithm (memory-hard function)
- Salts automatically generated
- Configurable hashing parameters

### JWT Configuration

- Configurable secret key
- Configurable expiration time (default: 300 seconds)
- Bearer token authentication scheme

## 📝 API Examples

### Register a New User

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepass123",
    "full_name": "John Doe"
  }'
```

### Login

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "password": "securepass123"
  }'
```

Response:
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

### List Users (Authenticated)

```bash
curl -X GET "http://localhost:8080/api/v1/users/?page=1&page_size=10" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

### Create a Role (Requires Permission)

```bash
curl -X POST http://localhost:8080/api/v1/roles/ \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "editor",
    "display_name": "Content Editor",
    "description": "Can edit content",
    "sort_order": 10
  }'
```

## 🧪 Testing

Run tests:
```bash
cargo test
```

## 📊 Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `APP_PROFILE` | Application profile (dev/prod) | `dev` |
| `RUST_LOG` | Log level filter | `info` |
| `DATABASE_URL` | PostgreSQL connection string | - |
| `APP_ADMIN_NAME` | Super admin username for initial migration | `admin` |
| `APP_ADMIN_PASS` | Super admin password for initial migration | `superadmin` |

### Configuration Files

Configuration is loaded in this order (later sources override earlier ones):
1. `settings/default.toml` - Base configuration
2. `settings/{APP_PROFILE}.toml` - Profile-specific overrides
3. Environment variables with `APP_` prefix

### Server Configuration

```toml
[server]
port = 8080
host = "0.0.0.0"
```

### Database Configuration

```toml
[database]
url = "postgres://user:password@localhost/dbname"
```

### JWT Configuration

```toml
[jwt]
secret = "change-this-to-a-strong-secret"
expiration_secs = 300
```

## 🔄 Database Migrations

### Run Migrations

```bash
cd migration
cargo run -- up
```

> **Note**: The migration will automatically create a super admin user using the `APP_ADMIN_NAME` and `APP_ADMIN_PASS` environment variables from your `.env` file. Make sure to set these before running migrations, and change the default credentials for production deployments.

### Rollback Migrations

```bash
cd migration
cargo run -- down
```

### Fresh Migration

```bash
cd migration
cargo run -- fresh
```

## 📖 API Documentation

The API follows RESTful conventions and returns consistent JSON responses:

```json
{
  "code": 0,
  "message": "",
  "data": { ... }
}
```

Error responses include appropriate HTTP status codes and error messages.

See `openapi.yaml` for complete API specification. You can import this file into tools like Swagger UI or Postman for interactive documentation.

## 🏗️ Architecture

### Layered Architecture

1. **Handler Layer** (`api/src/handler/`)
   - HTTP request/response handling
   - Input validation
   - Response formatting

2. **Service Layer** (`api/src/service/`)
   - Business logic
   - Transaction management
   - Data transformation

3. **Model Layer** (`model/`)
   - Database entities
   - Entity relationships
   - Query builders

4. **Middleware Layer** (`api/src/middleware/`)
   - Authentication
   - Authorization (RBAC)
   - Request context

### Design Patterns

- **Repository Pattern**: Abstracted through SeaORM entities
- **Dependency Injection**: State passed via Axum extensions
- **Middleware Chain**: Composable request processing
- **DTO Pattern**: Separation of API contracts from domain models

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the MIT License. See [LICENSE](LICENSE).

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Excellent web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) - Powerful async ORM
- [Tokio](https://tokio.rs/) - Reliable async runtime
- [Vue 3](https://vuejs.org/) & [Element Plus](https://element-plus.org/) - Frontend stack

## 📞 Support

For issues, questions, or contributions, please open an issue in the repository.

---

Built with ❤️ using Rust
