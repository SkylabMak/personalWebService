use std::env;
use anyhow::{Context, Result};

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiry: u64,
    pub jwt_refresh_expiry: u64,
    pub argon2_salt: String,
    pub argon2_memory_cost: u32,
    pub argon2_iterations: u32,
    pub argon2_parallelism: u32,
    pub gcs_bucket_name: String,
    pub google_application_credentials: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL must be set in .env file or environment variables")?,
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-minimum-32-characters-long".to_string()),
            jwt_access_expiry: env::var("JWT_ACCESS_EXPIRY")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .context("JWT_ACCESS_EXPIRY must be a number")?,
            jwt_refresh_expiry: env::var("JWT_REFRESH_EXPIRY")
                .unwrap_or_else(|_| "2592000".to_string())
                .parse()
                .context("JWT_REFRESH_EXPIRY must be a number")?,
            argon2_salt: env::var("ARGON2_SALT")
                .context("ARGON2_SALT must be set")?,
            argon2_memory_cost: env::var("ARGON2_MEMORY_COST")
                .unwrap_or_else(|_| "16".to_string())
                .parse()
                .context("ARGON2_MEMORY_COST must be a number")?,
            argon2_iterations: env::var("ARGON2_ITERATIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .context("ARGON2_ITERATIONS must be a number")?,
            argon2_parallelism: env::var("ARGON2_PARALLELISM")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .context("ARGON2_PARALLELISM must be a number")?,
            gcs_bucket_name: env::var("GCS_BUCKET_NAME")
                .unwrap_or_else(|_| "my-bucket".to_string()),
            google_application_credentials: env::var("GOOGLE_APPLICATION_CREDENTIALS").ok(),
        })
    }
}
