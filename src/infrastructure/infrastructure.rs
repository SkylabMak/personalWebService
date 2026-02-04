use crate::config::config::Config;
use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::infrastructure::db::mysql::setup::create_mysql_pool;
use crate::infrastructure::cloud_storage::cloud_storage::CloudStorage;
use crate::infrastructure::cloud_storage::gcs::setup::setup_gcs;
use crate::infrastructure::repositories::Repositories;

/// Infrastructure layer orchestrates the setup of all external dependencies.
/// It initializes database connections and creates the repositories that use them.
pub struct Infrastructure {
    pub repositories: Repositories,
    pub config: Config,
}

impl Infrastructure {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        // 1. Setup all database connections (MySQL, PostgreSQL, etc.)
        let dbs = Self::setup_databases(config).await?;

        // 2. Setup cloud storage
        let cloud_storage = Self::setup_cloud_storage(config).await?;

        // 3. Setup all repositories using the database handles and cloud storage
        let repositories = Repositories::new(&dbs, &cloud_storage);

        Ok(Self {
            repositories,
            config: config.clone(),
        })
    }

    /// Internal method to initialize all database connection pools.
    /// New database backends (like Postgres or NoSQL) should be added here.
    async fn setup_databases(config: &Config) -> anyhow::Result<Databases> {
        // Setup MySQL
        let mysql_pool = create_mysql_pool(config).await?;
        let mysql_repo = MySqlRepository::new(mysql_pool);
        
        // Setup PostgreSQL (if needed in the future)
        // let pg_pool = create_postgres_pool(config).await?;
        
        Ok(Databases::new(mysql_repo))
    }

    /// Internal method to initialize cloud storage.
    async fn setup_cloud_storage(config: &Config) -> anyhow::Result<CloudStorage> {
        let gcs_repo = setup_gcs(config).await?;
        Ok(CloudStorage::new(gcs_repo))
    }
}
