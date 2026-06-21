use anyhow::Result;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
}

pub async fn init_database(config: &DbConfig) -> Result<DatabaseConnection> {
    let mut  opts: ConnectOptions = (&config.url).into();
    if let Some(c) = config.max_connections {
        opts.max_connections(c);
    }
    if let Some(c) = config.min_connections {
        opts.min_connections(c);
    }
    let db = Database::connect(opts).await?;
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}