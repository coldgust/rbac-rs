use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::redis::RedisPool;
use axum::http::{HeaderMap, header};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::Utc;
use rand::RngCore;
use sha2::{Digest, Sha256};
use uuid::Uuid;

const META_KEY_PREFIX: &str = "rbac:rt";
const TOKEN_PREFIX: &str = "v1";
const ROTATE_SCRIPT: &str = include_str!("../../lua/rotate_refresh_token.lua");

#[derive(Debug, Default, Clone)]
pub struct ClientContext {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

impl ClientContext {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let user_agent = headers
            .get(header::USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.chars().take(256).collect::<String>());

        // Only trust proxy headers when the service is behind a trusted proxy.
        let ip = headers
            .get("x-forwarded-for")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(',').next())
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        Self { ip, user_agent }
    }
}

#[derive(Debug)]
pub struct IssuedRefreshToken {
    pub token: String,
    pub expires_in: i64,
}

#[derive(Debug)]
pub struct RotatedRefreshToken {
    pub token: String,
    pub user_id: String,
    pub family_id: Uuid,
    pub expires_in: i64,
}

pub struct TokenService;

impl TokenService {
    pub async fn issue(
        redis: &RedisPool,
        user_id: &str,
        ctx: &ClientContext,
        config: &Config,
    ) -> AppResult<IssuedRefreshToken> {
        let now = Utc::now().timestamp();
        let abs_exp = now + config.jwt.refresh_absolute_secs;
        let exp = (now + config.jwt.refresh_expiration_secs).min(abs_exp);
        let family_id = Uuid::new_v4();
        let secret = random_secret();
        let token_hash = hash_secret(&secret);
        let meta_key = family_meta_key(family_id);
        let user_key = user_sessions_key(user_id);

        let mut conn = redis.get().await?;
        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("HSET")
            .arg(&meta_key)
            .arg("uid")
            .arg(user_id)
            .arg("gen")
            .arg(0)
            .arg("cur_th")
            .arg(&token_hash)
            .arg("prev_th")
            .arg("")
            .arg("prev_at")
            .arg(0)
            .arg("iat")
            .arg(now)
            .arg("exp")
            .arg(exp)
            .arg("created_at")
            .arg(now)
            .arg("abs_exp")
            .arg(abs_exp)
            .arg("last_used_at")
            .arg(now)
            .arg("ua")
            .arg(ctx.user_agent.as_deref().unwrap_or(""))
            .arg("ip")
            .arg(ctx.ip.as_deref().unwrap_or(""))
            .arg("revoked")
            .arg(0)
            .ignore()
            .cmd("EXPIRE")
            .arg(&meta_key)
            .arg(abs_exp - now)
            .ignore()
            .cmd("ZADD")
            .arg(&user_key)
            .arg(now)
            .arg(family_id.to_string())
            .ignore()
            .cmd("EXPIRE")
            .arg(&user_key)
            .arg(abs_exp - now)
            .ignore();
        let _: () = pipe.exec_async(&mut conn).await?;

        let sessions: i64 = redis::cmd("ZCARD")
            .arg(&user_key)
            .query_async(&mut conn)
            .await?;
        if sessions as usize > config.auth.max_sessions_per_user {
            let oldest: Vec<String> = redis::cmd("ZPOPMIN")
                .arg(&user_key)
                .query_async(&mut conn)
                .await?;
            if let Some(family_id) = oldest.first().and_then(|value| Uuid::parse_str(value).ok()) {
                drop(conn);
                Self::revoke_family(redis, family_id).await?;
            }
        }

        Ok(IssuedRefreshToken {
            token: build_refresh_token(family_id, &secret),
            expires_in: (exp - now).max(0),
        })
    }

    pub async fn rotate(
        redis: &RedisPool,
        token: &str,
        ctx: &ClientContext,
        config: &Config,
    ) -> AppResult<RotatedRefreshToken> {
        let (family_id, secret) =
            parse_refresh_token(token).ok_or(AppError::InvalidRefreshToken)?;
        let presented_hash = hash_secret(&secret);
        let new_secret = random_secret();
        let new_hash = hash_secret(&new_secret);
        let meta_key = family_meta_key(family_id);

        let mut conn = redis.get().await?;
        let result: Vec<String> = redis::Script::new(ROTATE_SCRIPT)
            .key(meta_key)
            .arg(Utc::now().timestamp())
            .arg(config.auth.refresh_reuse_grace_secs)
            .arg(config.jwt.refresh_expiration_secs)
            .arg(&presented_hash)
            .arg(&new_hash)
            .arg(ctx.user_agent.as_deref().unwrap_or(""))
            .arg(ctx.ip.as_deref().unwrap_or(""))
            .invoke_async(&mut conn)
            .await?;

        match result.first().map(String::as_str) {
            Some("ok") => {
                let user_id = result.get(1).cloned().unwrap_or_default();
                let new_exp: i64 = result
                    .get(3)
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(0);
                Ok(RotatedRefreshToken {
                    token: build_refresh_token(family_id, &new_secret),
                    user_id,
                    family_id,
                    expires_in: (new_exp - Utc::now().timestamp()).max(0),
                })
            }
            Some("retry") => Err(AppError::RefreshInProgress),
            Some("reused") => {
                let uid = result.get(1).cloned().unwrap_or_default();
                tracing::warn!(%uid, %family_id, "refresh token reuse detected");
                Err(AppError::RefreshTokenReused)
            }
            Some("expired") => Err(AppError::SessionExpired),
            Some("revoked") | Some("invalid") | _ => Err(AppError::InvalidRefreshToken),
        }
    }

    /// Revokes the family that the presented token belongs to.
    ///
    /// Invalid tokens are ignored so logout remains idempotent and cannot be
    /// used to probe whether a token exists.
    pub async fn revoke_by_token(redis: &RedisPool, token: &str) -> AppResult<()> {
        if let Some((family_id, _)) = parse_refresh_token(token) {
            Self::revoke_family(redis, family_id).await?;
        }
        Ok(())
    }

    pub async fn revoke_family(redis: &RedisPool, family_id: Uuid) -> AppResult<()> {
        let meta_key = family_meta_key(family_id);
        let mut conn = redis.get().await?;

        let uid: Option<String> = redis::cmd("HGET")
            .arg(&meta_key)
            .arg("uid")
            .query_async(&mut conn)
            .await?;
        let Some(uid) = uid else {
            return Ok(());
        };

        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("HSET")
            .arg(&meta_key)
            .arg("revoked")
            .arg(1)
            .ignore()
            .cmd("EXPIRE")
            .arg(&meta_key)
            .arg(3600)
            .ignore();
        let _: () = pipe.exec_async(&mut conn).await?;

        let _: () = redis::cmd("ZREM")
            .arg(user_sessions_key(&uid))
            .arg(family_id.to_string())
            .query_async(&mut conn)
            .await?;

        Ok(())
    }

    /// Revokes every refresh session belonging to a user.
    pub async fn revoke_all(redis: &RedisPool, user_id: &str) -> AppResult<()> {
        let user_key = user_sessions_key(user_id);
        let mut conn = redis.get().await?;

        let families: Vec<String> = redis::cmd("ZRANGE")
            .arg(&user_key)
            .arg(0)
            .arg(-1)
            .query_async(&mut conn)
            .await?;

        let mut pipe = redis::pipe();
        pipe.atomic();
        for value in &families {
            let Ok(family_id) = Uuid::parse_str(value) else {
                continue;
            };
            let meta_key = family_meta_key(family_id);
            pipe.cmd("HSET")
                .arg(&meta_key)
                .arg("revoked")
                .arg(1)
                .ignore();
            pipe.cmd("EXPIRE").arg(&meta_key).arg(3600).ignore();
        }
        pipe.cmd("DEL").arg(&user_key).ignore();

        let _: () = pipe.exec_async(&mut conn).await?;
        Ok(())
    }
}

fn family_meta_key(family_id: Uuid) -> String {
    format!("{META_KEY_PREFIX}:{{{family_id}}}:meta")
}

fn user_sessions_key(user_id: &str) -> String {
    format!("{META_KEY_PREFIX}:user:{user_id}")
}

fn random_secret() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn hash_secret(secret: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(secret.as_bytes()))
}

fn build_refresh_token(family_id: Uuid, secret: &str) -> String {
    format!("{TOKEN_PREFIX}.{family_id}.{secret}")
}

fn parse_refresh_token(token: &str) -> Option<(Uuid, String)> {
    let mut parts = token.split('.');
    if parts.next()? != TOKEN_PREFIX {
        return None;
    }
    let family_id = Uuid::parse_str(parts.next()?).ok()?;
    let secret = parts.next()?;
    if secret.len() != 43 || parts.next().is_some() {
        return None;
    }
    Some((family_id, secret.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_token_roundtrip() {
        let family_id = Uuid::new_v4();
        let secret = random_secret();
        let token = build_refresh_token(family_id, &secret);

        let (parsed_family_id, parsed_secret) = parse_refresh_token(&token).unwrap();
        assert_eq!(parsed_family_id, family_id);
        assert_eq!(parsed_secret, secret);
        assert_eq!(secret.len(), 43);
    }

    #[test]
    fn rejects_malformed_refresh_tokens() {
        assert!(parse_refresh_token("").is_none());
        assert!(parse_refresh_token("v2.00000000-0000-0000-0000-000000000000.abc").is_none());
        assert!(parse_refresh_token("v1.not-a-uuid.abc").is_none());
        assert!(parse_refresh_token("v1.00000000-0000-0000-0000-000000000000.short").is_none());
        assert!(
            parse_refresh_token(&format!("v1.{}.{}.extra", Uuid::new_v4(), random_secret()))
                .is_none()
        );
    }

    #[test]
    fn hash_is_stable_and_not_plaintext() {
        let hash = hash_secret("hello");
        assert_eq!(hash, hash_secret("hello"));
        assert_ne!(hash, hash_secret("world"));
        assert_eq!(hash.len(), 43);
        assert_ne!(hash, "hello");
    }
}
