use std::env;
use anyhow::{Context, Result};

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL must be set in .env file or environment variables")?,
        })
    }
}
