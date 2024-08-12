use anyhow::Result;
use axum::serve;
use notify_server::get_router;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let listener = TcpListener::bind("0.0.0.0:7799").await?;
    let app = get_router();
    serve(listener, app.into_make_service()).await?;
    Ok(())
}
