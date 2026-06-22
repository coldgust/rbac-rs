use crate::db::DbConfig;
use crate::redis::{RedisConfig, RedisPool};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub expiration_secs: i64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: DbConfig,
    pub jwt: Jwt,
    pub redis: RedisConfig,
}

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis: Arc<RedisPool>,
    pub config: Arc<Config>,
}
