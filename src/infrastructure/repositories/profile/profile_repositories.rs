use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::cloud_storage::cloud_storage::CloudStorage;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;
use crate::infrastructure::repository_impl::profile::announce::repository::AnnounceRepositoryImpl;
use crate::infrastructure::repository_impl::profile::image::repository::ImageRepositoryImpl;
use crate::infrastructure::repository_impl::profile::performance::repository::PerformanceRepositoryImpl;
use crate::infrastructure::repository_impl::profile::performance_content::repository::GcsPerformanceContentRepositoryImpl;
use crate::infrastructure::repository_impl::profile::image::storage_repository::GcsImageStorageRepositoryImpl;

pub struct ProfileRepositories {
    pub life_status: LifeStatusRepositoryImpl,
    pub announce: AnnounceRepositoryImpl,
    pub image: ImageRepositoryImpl,
    pub image_storage: GcsImageStorageRepositoryImpl,
    pub performance: PerformanceRepositoryImpl,
    pub performance_content: GcsPerformanceContentRepositoryImpl,
}

impl ProfileRepositories {
    pub fn new(dbs: &Databases, cloud_storage: &CloudStorage) -> Self {
        Self {
            life_status: LifeStatusRepositoryImpl::new(dbs.mysql.clone()),
            announce: AnnounceRepositoryImpl::new(dbs.mysql.clone()),
            image: ImageRepositoryImpl::new(dbs.mysql.clone()),
            image_storage: GcsImageStorageRepositoryImpl::new(cloud_storage.gcs.clone()),
            performance: PerformanceRepositoryImpl::new(dbs.mysql.clone()),
            performance_content: GcsPerformanceContentRepositoryImpl::new(cloud_storage.gcs.clone()),
        }
    }
}
