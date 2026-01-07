pub mod profile;
pub mod website;

use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repositories::profile::profile_repositories::ProfileRepositories;
use crate::infrastructure::repositories::website::feature_status::website_repositories::WebsiteRepositories;

pub struct Repositories {
    pub profile: ProfileRepositories,
    pub website: WebsiteRepositories,
}

impl Repositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            profile: ProfileRepositories::new(dbs),
            website: WebsiteRepositories::new(dbs),
        }
    }
}
