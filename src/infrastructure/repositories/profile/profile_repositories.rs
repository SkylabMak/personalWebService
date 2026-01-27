use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;
use crate::infrastructure::repository_impl::profile::announce::repository::AnnounceRepositoryImpl;

pub struct ProfileRepositories {
    pub life_status: LifeStatusRepositoryImpl,
    pub announce: AnnounceRepositoryImpl,
}

impl ProfileRepositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            life_status: LifeStatusRepositoryImpl::new(dbs.mysql.clone()),
            announce: AnnounceRepositoryImpl::new(dbs.mysql.clone()),
        }
    }
}
