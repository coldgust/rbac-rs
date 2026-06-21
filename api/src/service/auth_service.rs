use crate::config::{Config, Jwt};
use crate::dto::auth::{Claims, LoginRequest, RegisterRequest, TokenResponse};
use crate::dto::user::UserRolesAndPermissions;
use crate::error::{AppError, AppResult};
use crate::service::user_service::UserService;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use migration::prelude::Utc;
use model::user::UserStatus;
use model::{User, user};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
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
        config: &Config,
        req: LoginRequest,
    ) -> AppResult<TokenResponse> {
        let user = User::find_by_username(&req.username)
            .one(db)
            .await?
            .ok_or(AppError::InvalidCredentials)?;
        if PasswordUtil::verify(&req.password, &user.password_hash).is_err() {
            return Err(AppError::InvalidCredentials);
        }
        if user.status != UserStatus::Active as i16 {
            return Err(AppError::AccountDisabled);
        }

        let (access_token, refresh_token) = Self::generate_token(db, &user, &config.jwt).await?;
        Ok(TokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: (Utc::now() + Duration::seconds(config.jwt.expiration_secs))
                .timestamp_millis(),
        })
    }

    async fn generate_token(
        db: &DatabaseConnection,
        user: &user::Model,
        jwt: &Jwt,
    ) -> AppResult<(String, String)> {
        let UserRolesAndPermissions { roles, permissions } =
            UserService::get_user_roles_and_permissions(db, &user.id).await?;

        let now = Utc::now();
        let exp = (now + Duration::seconds(jwt.expiration_secs)).timestamp_millis();
        let claims = Claims::new(
            &user.id,
            roles.into_iter().map(|r| r.name).collect(),
            permissions,
            exp,
        );

        let access_token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt.secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal)?;

        // Refresh token valid for 7 days
        let refresh_exp = (now + Duration::days(7)).timestamp();
        let refresh_claims = Claims::new(&user.id, vec![], vec![], refresh_exp);
        let refresh_token = jsonwebtoken::encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(jwt.secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal)?;

        Ok((access_token, refresh_token))
    }

    pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|_| AppError::Unauthorized)?;
        Ok(token_data.claims)
    }
}
