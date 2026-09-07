use std::sync::Arc;

use anyhow::Context;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
mod config;
mod errors;
mod handlers;
mod middleware;
mod routes;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file (if present — ignored in production when env vars are set directly)
    let _ = dotenv();

    // Initialise structured logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting EduKit API");

    // Load configuration
    let config = config::Config::from_env().context("Failed to load configuration")?;

    // Create database connection pool and run migrations
    let pool = infrastructure::database::create_pool(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    infrastructure::database::run_migrations(&pool)
        .await
        .context("Failed to run database migrations")?;

    // Build the application
    let app = app::build_app(pool, config.clone()).await;

    // Bind and serve
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to {addr}"))?;

    tracing::info!("EduKit API listening on http://{addr}");

    axum::serve(listener, app)
        .await
        .context("Server error")?;

    Ok(())
}
