mod config;
mod error;
mod handlers;
mod models;
mod router;
mod utils;

use crate::config::AppConfig;
use crate::models::User;
use crate::router::get_router;
use anyhow::Result;
use axum::serve;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::try_load()?;

    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on:{}", addr);

    let app = get_router(config);
    serve(listener, app.into_make_service()).await?;
    Ok(())
}
