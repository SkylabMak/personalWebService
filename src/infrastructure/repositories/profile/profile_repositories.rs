use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repository_impl::profile::life_status::mysql_life_status_repo::MySqlLifeStatusRepository;

pub struct ProfileRepositories {
    pub life_status: MySqlLifeStatusRepository,
}

impl ProfileRepositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            life_status: MySqlLifeStatusRepository::new(dbs.mysql.clone()),
        }
    }
}
