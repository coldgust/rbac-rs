use crate::config::{Config, Jwt};
use crate::dto::auth::{
    Claims, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest, TokenResponse,
};
use crate::dto::user::UserRolesAndPermissions;
use crate::error::{AppError, AppResult};
use crate::redis::RedisPool;
use crate::service::token_service::{ClientContext, TokenService};
use crate::service::user_service::UserService;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use model::user::UserStatus;
use model::{User, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use util::password::PasswordUtil;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn register(db: &DatabaseConnection, req: RegisterRequest) -> AppResult<user::Model> {
        let exist = User::find_by_username(&req.username).one(db).await?;
        if exist.is_some() {
            return Err(AppError::UsernameExists);
        }

        let pass_hash = PasswordUtil::hash(&req.password).map_err(|_| AppError::Internal)?;

        let user = user::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            username: Set(req.username),
            email: Set(req.email),
            password_hash: Set(pass_hash),
            display_name: Set(req.display_name),
            status: Set(UserStatus::Active as i16),
            last_login_at: Set(None),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            deleted_at: Set(None),
        }
        .insert(db)
        .await?;

        Ok(user)
    }

    pub async fn login(
        db: &DatabaseConnection,
        redis: &RedisPool,
        config: &Config,
        req: LoginRequest,
        ctx: &ClientContext,
    ) -> AppResult<TokenResponse> {
        let user = User::find_by_username(&req.username)
            .filter(user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        if PasswordUtil::verify(&req.password, &user.password_hash).is_err() {
            return Err(AppError::InvalidCredentials);
        }
        if user.status != UserStatus::Active as i16 {
            return Err(AppError::AccountDisabled);
        }

        let access_token = Self::issue_access_token(db, &user.id, &config.jwt).await?;
        let issued = TokenService::issue(redis, &user.id, ctx, config).await?;

        Ok(TokenResponse {
            access_token,
            refresh_token: issued.token,
            token_type: "Bearer".to_string(),
            expires_in: config.jwt.access_expiration_secs,
            refresh_expires_in: issued.expires_in,
        })
    }

    pub async fn refresh(
        db: &DatabaseConnection,
        redis: &RedisPool,
        config: &Config,
        req: RefreshRequest,
        ctx: &ClientContext,
    ) -> AppResult<TokenResponse> {
        let rotated = TokenService::rotate(redis, &req.refresh_token, ctx, config).await?;

        let user = user::Entity::find_by_id(&rotated.user_id)
            .filter(user::Column::DeletedAt.is_null())
            .one(db)
            .await?;

        let Some(user) = user else {
            TokenService::revoke_family(redis, rotated.family_id).await?;
            return Err(AppError::InvalidRefreshToken);
        };

        if user.status != UserStatus::Active as i16 {
            TokenService::revoke_family(redis, rotated.family_id).await?;
            return Err(AppError::AccountDisabled);
        }

        let access_token = Self::issue_access_token(db, &user.id, &config.jwt).await?;

        Ok(TokenResponse {
            access_token,
            refresh_token: rotated.token,
            token_type: "Bearer".to_string(),
            expires_in: config.jwt.access_expiration_secs,
            refresh_expires_in: rotated.expires_in,
        })
    }

    pub async fn logout(redis: &RedisPool, req: LogoutRequest) -> AppResult<()> {
        TokenService::revoke_by_token(redis, &req.refresh_token).await
    }

    async fn issue_access_token(
        db: &DatabaseConnection,
        user_id: &str,
        jwt: &Jwt,
    ) -> AppResult<String> {
        let UserRolesAndPermissions { roles, permissions } =
            UserService::get_user_roles_and_permissions(db, user_id).await?;

        Self::sign_access_token(
            user_id,
            roles.into_iter().map(|role| role.name).collect(),
            permissions,
            jwt,
        )
    }

    fn sign_access_token(
        user_id: &str,
        roles: Vec<String>,
        perms: Vec<String>,
        jwt: &Jwt,
    ) -> AppResult<String> {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id.to_owned(),
            roles,
            perms,
            token_use: "access".to_owned(),
            jti: Uuid::new_v4().to_string(),
            iss: jwt.issuer.clone(),
            aud: jwt.audience.clone(),
            iat: now,
            exp: now + jwt.access_expiration_secs,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt.secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal)
    }

    pub fn verify_access_token(token: &str, jwt: &Jwt) -> AppResult<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = jwt.leeway_secs;
        validation.validate_exp = true;
        validation.set_required_spec_claims(&["exp", "sub", "iss", "aud"]);
        validation.set_issuer(&[jwt.issuer.as_str()]);
        validation.set_audience(&[jwt.audience.as_str()]);

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt.secret.as_bytes()),
            &validation,
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::Unauthorized,
        })?;

        if data.claims.token_use != "access" {
            return Err(AppError::Unauthorized);
        }

        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_jwt() -> Jwt {
        Jwt {
            secret: "0123456789abcdef0123456789abcdef".to_owned(),
            access_expiration_secs: 900,
            refresh_expiration_secs: 604_800,
            refresh_absolute_secs: 2_592_000,
            issuer: "rbac-rs".to_owned(),
            audience: "rbac-rs-web".to_owned(),
            leeway_secs: 30,
        }
    }

    #[test]
    fn access_token_uses_seconds_and_has_token_use() {
        let jwt = test_jwt();
        let token = AuthService::sign_access_token(
            "user-1",
            vec!["admin".to_owned()],
            vec!["user:read".to_owned()],
            &jwt,
        )
        .unwrap();

        let claims = AuthService::verify_access_token(&token, &jwt).unwrap();
        assert_eq!(claims.sub, "user-1");
        assert_eq!(claims.token_use, "access");
        assert_eq!(claims.exp - claims.iat, 900);
    }

    #[test]
    fn rejects_refresh_token_used_as_access_token() {
        let jwt = test_jwt();
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: "user-1".to_owned(),
            roles: Vec::new(),
            perms: Vec::new(),
            token_use: "refresh".to_owned(),
            jti: Uuid::new_v4().to_string(),
            iss: jwt.issuer.clone(),
            aud: jwt.audience.clone(),
            iat: now,
            exp: now + 300,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt.secret.as_bytes()),
        )
        .unwrap();

        assert!(matches!(
            AuthService::verify_access_token(&token, &jwt),
            Err(AppError::Unauthorized)
        ));
    }
}
