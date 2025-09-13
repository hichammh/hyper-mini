use anyhow::*;
use tracing_subscriber::{EnvFilter, fmt};
use axum::Router;
use tokio::net::TcpListener;

mod api;
mod vm;

#[tokio::main]
async fn main() -> Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let app = Router::new().merge(api::router());

    let addr = "0.0.0.0:8080";
    tracing::info!("Starting hyper-mini API at {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
