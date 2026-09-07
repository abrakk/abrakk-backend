use anyhow::{bail, Result};

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://edukit:edukit_password@localhost:5432/edukit_development".into());

        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());

        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .unwrap_or(8080);

        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET not set — using insecure default. Set this in production!");
            "change_this_in_production_please".into()
        });

        if jwt_secret.len() < 32 {
            bail!("JWT_SECRET must be at least 32 characters");
        }

        let jwt_expiration_hours: u64 = std::env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".into())
            .parse()
            .unwrap_or(24);

        Ok(Config {
            database_url,
            host,
            port,
            jwt_secret,
            jwt_expiration_hours,
        })
    }
}
