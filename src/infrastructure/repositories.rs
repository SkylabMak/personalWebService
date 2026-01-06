pub mod profile;

use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repositories::profile::profile_repositories::ProfileRepositories;

pub struct Repositories {
    pub profile: ProfileRepositories,
}

impl Repositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            profile: ProfileRepositories::new(dbs),
        }
    }
}
