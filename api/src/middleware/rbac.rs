use crate::error::AppError;
use crate::dto::auth::Claims;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;

pub async fn require_permission(
    perm: State<String>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;
    if claims.perms.contains(&*perm) {
        Ok(next.run(req).await)
    } else {
        Err(AppError::Forbidden)
    }
}
