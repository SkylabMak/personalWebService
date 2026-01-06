use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use crate::config::config::Config;

pub async fn create_mysql_pool(config: &Config) -> anyhow::Result<MySqlPool> {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to MySQL: {}", e))
}
