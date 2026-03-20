use async_trait::async_trait;
use crate::domain::entities::profile::profile::Profile;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::profile_repository::ProfileRepository;

use crate::domain::entities::profile::life_status::life_status::LifeStatus;

#[derive(Clone)]
pub struct ProfileRepositoryImpl {
    mysql: MySqlRepository,
}

impl ProfileRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositoryImpl {
    async fn find_by_id(&self, _id: &str) -> Result<Option<Profile>, RepositoryError> {
        unimplemented!("Use ProfileDataRepositoryImpl instead")
    }
}
