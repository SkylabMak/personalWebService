use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repository_impl::application::feature_status::repository::AppRepositoryImpl;

pub struct WebsiteRepositories {
    pub repository: AppRepositoryImpl,
}

impl WebsiteRepositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            repository: AppRepositoryImpl::new(dbs.mysql.clone()),
        }
    }
}
