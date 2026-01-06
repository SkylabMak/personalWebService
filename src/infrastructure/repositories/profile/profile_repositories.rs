use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;

pub struct ProfileRepositories {
    pub life_status: LifeStatusRepositoryImpl,
}

impl ProfileRepositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            life_status: LifeStatusRepositoryImpl::new(dbs.mysql.clone()),
        }
    }
}
