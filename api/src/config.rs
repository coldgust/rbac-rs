use crate::db::DbConfig;
use crate::redis::{RedisConfig, RedisPool};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Jwt {
    pub secret: String,

    /// Access Token 有效期（秒）。
    pub access_expiration_secs: i64,

    /// Refresh Token 滑动有效期（秒）。
    #[serde(default = "default_refresh_expiration")]
    pub refresh_expiration_secs: i64,

    /// 会话族绝对有效期（秒）。
    #[serde(default = "default_refresh_absolute")]
    pub refresh_absolute_secs: i64,

    #[serde(default = "default_issuer")]
    pub issuer: String,

    #[serde(default = "default_audience")]
    pub audience: String,

    #[serde(default = "default_leeway")]
    pub leeway_secs: u64,
}

impl fmt::Debug for Jwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Jwt")
            .field("secret", &"***")
            .field("access_expiration_secs", &self.access_expiration_secs)
            .field("refresh_expiration_secs", &self.refresh_expiration_secs)
            .field("refresh_absolute_secs", &self.refresh_absolute_secs)
            .field("issuer", &self.issuer)
            .field("audience", &self.audience)
            .field("leeway_secs", &self.leeway_secs)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    #[serde(default = "default_grace")]
    pub refresh_reuse_grace_secs: i64,

    #[serde(default = "default_max_sessions")]
    pub max_sessions_per_user: usize,

    /// 阶段 2 的 HttpOnly Cookie 模式开关；阶段 1 保留但不生效。
    #[serde(default)]
    pub refresh_cookie_enabled: bool,
}

impl Default for Auth {
    fn default() -> Self {
        Self {
            refresh_reuse_grace_secs: default_grace(),
            max_sessions_per_user: default_max_sessions(),
            refresh_cookie_enabled: false,        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: DbConfig,
    pub jwt: Jwt,
    /// 旧配置文件没有 [auth] 段时使用默认值。
    #[serde(default)]
    pub auth: Auth,
    pub redis: RedisConfig,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.jwt.secret.len() < 32 {
            anyhow::bail!("jwt.secret must be at least 32 bytes");
        }
        if self.jwt.access_expiration_secs <= 0 {
            anyhow::bail!("jwt.access_expiration_secs must be positive");
        }
        if self.jwt.refresh_expiration_secs <= 0 {
            anyhow::bail!("jwt.refresh_expiration_secs must be positive");
        }
        if self.jwt.refresh_absolute_secs < self.jwt.refresh_expiration_secs {
            anyhow::bail!("jwt.refresh_absolute_secs must be >= jwt.refresh_expiration_secs");
        }
        if self.jwt.issuer.is_empty() || self.jwt.audience.is_empty() {
            anyhow::bail!("jwt.issuer and jwt.audience must not be empty");
        }
        if self.auth.refresh_reuse_grace_secs < 0 {
            anyhow::bail!("auth.refresh_reuse_grace_secs must not be negative");
        }
        if self.auth.max_sessions_per_user < 1 {
            anyhow::bail!("auth.max_sessions_per_user must be at least 1");
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: Arc<RedisPool>,
    pub config: Arc<Config>,
}

fn default_refresh_expiration() -> i64 {
    604_800
}

fn default_refresh_absolute() -> i64 {
    2_592_000
}

fn default_issuer() -> String {
    "rbac-rs".to_owned()
}

fn default_audience() -> String {
    "rbac-rs-web".to_owned()
}

fn default_leeway() -> u64 {
    30
}

fn default_grace() -> i64 {
    30
}

fn default_max_sessions() -> usize {
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config as ConfigBuilder, File};
    use std::path::PathBuf;

    #[test]
    fn default_settings_deserialize_and_validate() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../settings/default.toml");
        let config: Config = ConfigBuilder::builder()
            .add_source(File::from(path))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();

        config.validate().unwrap();
    }
}
