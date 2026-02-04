pub mod profile;
pub mod application;
pub mod auth;

use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::cloud_storage::cloud_storage::CloudStorage;
use crate::infrastructure::repositories::profile::profile_repositories::ProfileRepositories;
use crate::infrastructure::repositories::application::feature_status::app_repositories::WebsiteRepositories;
use crate::infrastructure::repositories::auth::auth_repositories::AuthRepositories;

pub struct Repositories {
    pub profile: ProfileRepositories,
    pub website: WebsiteRepositories,
    pub auth: AuthRepositories,
}

impl Repositories {
    pub fn new(dbs: &Databases, cloud_storage: &CloudStorage) -> Self {
        Self {
            profile: ProfileRepositories::new(dbs, cloud_storage),
            website: WebsiteRepositories::new(dbs),
            auth: AuthRepositories::new(dbs),
        }
    }
}
