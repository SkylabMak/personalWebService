use sqlx::MySqlPool;

#[derive(Clone)]
pub struct MySqlRepository {
    pool: MySqlPool,
}

impl MySqlRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}

