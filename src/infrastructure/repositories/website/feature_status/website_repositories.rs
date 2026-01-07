use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repository_impl::website::feature_status::repository::WebsiteRepositoryImpl;

pub struct WebsiteRepositories {
    pub repository: WebsiteRepositoryImpl,
}

impl WebsiteRepositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            repository: WebsiteRepositoryImpl::new(dbs.mysql.clone()),
        }
    }
}
