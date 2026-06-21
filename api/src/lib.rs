pub mod config;
pub mod db;
mod dto;
mod error;
mod handler;
mod middleware;
pub mod router;
mod service;

use crate::router::create_router;
use ::config::{Config, Environment, File};
pub use config::AppState;
pub use config::Config as AppConfig;
use dotenvy::dotenv;
use std::env;
use time::macros::format_description;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    dotenv()?;

    let timer = LocalTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
    ));

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(timer))
        .with(env_filter)
        .init();

    let profile = env::var("APP_PROFILE").unwrap_or("dev".into());
    let config: AppConfig = Config::builder()
        .add_source(File::with_name("settings/default"))
        .add_source(File::with_name(&format!("settings/{}", profile)).required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?;

    info!("{:?}", config);

    let conn = db::init_database(&config.database).await?;
    info!("db init success {:?}", config.database);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let app = create_router(conn, config).layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listen on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
