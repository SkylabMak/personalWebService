use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;

/// Databases acts as a container for all database connection pools.
/// This allows different features to use different database backends (MySQL, PostgreSQL, MongoDB, etc.)
pub struct Databases {
    pub mysql: MySqlRepository,
    // Future expansion:
    // pub postgres: PostgresRepository,
    // pub nosql: NoSqlRepository,
}

impl Databases {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}
