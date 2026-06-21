use crate::dto::auth::Claims;
use crate::error::AppError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

pub struct CurrentUser(pub Claims);

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .ok_or(AppError::Unauthorized)?
            .clone();
        Ok(CurrentUser(claims))
    }
}
