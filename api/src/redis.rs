use crate::error::AppResult;
use deadpool_redis::{Config, Pool, Runtime};
use serde::Deserialize;

pub type RedisPool = Pool;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

pub async fn init_redis_pool(redis_config: &RedisConfig) -> AppResult<RedisPool> {
    let pool = Config::from_url(&redis_config.url).create_pool(Some(Runtime::Tokio1))?;
    Ok(pool)
}
