use crate::application::services::profile::life_status::service::GetCurrentLifeStatusService;
use crate::application::services::profile::announce::service::GetAnnounceListService;
use crate::application::services::profile::image::service::{
    GetImagesService, GetImageService, GetImageUsageService,
    CreateImageService, UpdateImageMetadataService, DeleteImageService, ForceDeleteImageService,
    GetUnusedImagesService, DeleteUnusedImagesService, TrackImageUsageService, UntrackImageUsageService
};
use crate::application::services::profile::performance::service::{
    CreatePerformanceService, UpdatePerformanceService, DeletePerformanceService
};
use crate::application::services::profile::performance::content_service::{
    GetPerformanceContentService, UpdatePerformanceContentService
};
use crate::infrastructure::repositories::Repositories;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;
use crate::infrastructure::repository_impl::profile::announce::repository::AnnounceRepositoryImpl;
use crate::infrastructure::repository_impl::profile::image::repository::ImageRepositoryImpl;
use crate::infrastructure::repository_impl::profile::image::storage_repository::GcsImageStorageRepositoryImpl;
use crate::infrastructure::repository_impl::profile::performance::repository::PerformanceRepositoryImpl;
use crate::infrastructure::repository_impl::profile::performance_content::repository::GcsPerformanceContentRepositoryImpl;

pub struct ProfileServices {
    pub life_status: GetCurrentLifeStatusService<LifeStatusRepositoryImpl>,
    pub announce: GetAnnounceListService<AnnounceRepositoryImpl>,
    pub image_get_all: GetImagesService<ImageRepositoryImpl>,
    pub image_get_one: GetImageService<ImageRepositoryImpl>,
    pub image_get_usage: GetImageUsageService<ImageRepositoryImpl>,
    pub image_create: CreateImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
    pub image_update_metadata: UpdateImageMetadataService<ImageRepositoryImpl>,
    pub image_delete: DeleteImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
    pub image_force_delete: ForceDeleteImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
    pub image_get_unused: GetUnusedImagesService<ImageRepositoryImpl>,
    pub image_delete_unused: DeleteUnusedImagesService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
    pub image_track_usage: TrackImageUsageService<ImageRepositoryImpl>,
    pub image_untrack_usage: UntrackImageUsageService<ImageRepositoryImpl>,
    pub performance_create: CreatePerformanceService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>,
    pub performance_update: UpdatePerformanceService<PerformanceRepositoryImpl>,
    pub performance_delete: DeletePerformanceService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>,
    pub performance_get_content: GetPerformanceContentService<GcsPerformanceContentRepositoryImpl>,
    pub performance_update_content: UpdatePerformanceContentService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>,
}

impl ProfileServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            life_status: GetCurrentLifeStatusService::new(repos.profile.life_status.clone()),
            announce: GetAnnounceListService::new(repos.profile.announce.clone()),
            image_get_all: GetImagesService::new(repos.profile.image.clone()),
            image_get_one: GetImageService::new(repos.profile.image.clone()),
            image_get_usage: GetImageUsageService::new(repos.profile.image.clone()),
            image_create: CreateImageService::new(repos.profile.image.clone(), repos.profile.image_storage.clone()),
            image_update_metadata: UpdateImageMetadataService::new(repos.profile.image.clone()),
            image_delete: DeleteImageService::new(repos.profile.image.clone(), repos.profile.image_storage.clone()),
            image_force_delete: ForceDeleteImageService::new(repos.profile.image.clone(), repos.profile.image_storage.clone()),
            image_get_unused: GetUnusedImagesService::new(repos.profile.image.clone()),
            image_delete_unused: DeleteUnusedImagesService::new(repos.profile.image.clone(), repos.profile.image_storage.clone()),
            image_track_usage: TrackImageUsageService::new(repos.profile.image.clone()),
            image_untrack_usage: UntrackImageUsageService::new(repos.profile.image.clone()),
            performance_create: CreatePerformanceService::new(repos.profile.performance.clone(), repos.profile.performance_content.clone()),
            performance_update: UpdatePerformanceService::new(repos.profile.performance.clone()),
            performance_delete: DeletePerformanceService::new(repos.profile.performance.clone(), repos.profile.performance_content.clone()),
            performance_get_content: GetPerformanceContentService::new((), repos.profile.performance_content.clone()),
            performance_update_content: UpdatePerformanceContentService::new(repos.profile.performance.clone(), repos.profile.performance_content.clone()),
        }
    }
}
