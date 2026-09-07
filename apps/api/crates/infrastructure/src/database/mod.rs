use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Create and return a PostgreSQL connection pool.
///
/// Runs any pending migrations on startup.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    tracing::info!("Database connection pool established");
    Ok(pool)
}

/// Run SQLx migrations from the `migrations/` directory.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("../../migrations").run(pool).await?;
    tracing::info!("Database migrations applied successfully");
    Ok(())
}
