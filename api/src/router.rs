use crate::AppState;
use crate::config::Config;
use crate::handler::{auth_handler, permission_handler, role_handler, user_handler};
use crate::middleware::auth::auth_middleware;
use crate::middleware::rbac::require_permission;
use axum::Router;
use axum::routing::{delete, get, post, put};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn create_router(db: DatabaseConnection, config: Config) -> Router {
    let state = Arc::new(AppState {
        db: Arc::new(db),
        config: Arc::new(config),
    });

    let auth_routers = Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login));

    let user_routers = Router::new()
        .route(
            "/",
            get(user_handler::list_users).layer(axum::middleware::from_fn_with_state(
                "user:read".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            get(user_handler::get_user).layer(axum::middleware::from_fn_with_state(
                "user:read".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            put(user_handler::update_user).layer(axum::middleware::from_fn_with_state(
                "user:update".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            delete(user_handler::delete_user).layer(axum::middleware::from_fn_with_state(
                "user:delete".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}/roles",
            post(user_handler::assign_roles).layer(axum::middleware::from_fn_with_state(
                "user:assign_role".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}/roles/{role_id}",
            delete(user_handler::remove_role).layer(axum::middleware::from_fn_with_state(
                "user:assign_role".to_string(),
                require_permission,
            )),
        )
        .route("/change_password", put(user_handler::change_password))
        .route("/me", get(user_handler::me));

    let role_routes = Router::new()
        .route(
            "/",
            get(role_handler::list_roles).layer(axum::middleware::from_fn_with_state(
                "role:read".to_string(),
                require_permission,
            )),
        )
        .route(
            "/",
            post(role_handler::create_role).layer(axum::middleware::from_fn_with_state(
                "role:create".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            get(role_handler::get_role).layer(axum::middleware::from_fn_with_state(
                "role:read".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            put(role_handler::update_role).layer(axum::middleware::from_fn_with_state(
                "role:update".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            delete(role_handler::delete_role).layer(axum::middleware::from_fn_with_state(
                "role:delete".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}/permissions",
            post(role_handler::assign_permissions).layer(axum::middleware::from_fn_with_state(
                "role:assign_perm".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}/permissions/{perm_id}",
            delete(role_handler::remove_permission).layer(axum::middleware::from_fn_with_state(
                "role:assign_perm".to_string(),
                require_permission,
            )),
        );

    let permission_routes = Router::new()
        .route(
            "/",
            get(permission_handler::list_permissions).layer(axum::middleware::from_fn_with_state(
                "perm:read".to_string(),
                require_permission,
            )),
        )
        .route(
            "/",
            post(permission_handler::create_permission).layer(
                axum::middleware::from_fn_with_state("perm:create".to_string(), require_permission),
            ),
        )
        .route(
            "/{id}",
            get(permission_handler::get_permission).layer(axum::middleware::from_fn_with_state(
                "perm:read".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            put(permission_handler::update_permission).layer(axum::middleware::from_fn_with_state(
                "perm:update".to_string(),
                require_permission,
            )),
        )
        .route(
            "/{id}",
            delete(permission_handler::delete_permission).layer(
                axum::middleware::from_fn_with_state("perm:delete".to_string(), require_permission),
            ),
        );

    let protected_routes = Router::new()
        .nest("/users", user_routers)
        .nest("/roles", role_routes)
        .nest("/permissions", permission_routes)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/api/v1/auth", auth_routers)
        .nest("/api/v1", protected_routes)
        .with_state(state)
}
