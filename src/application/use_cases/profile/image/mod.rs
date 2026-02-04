pub mod dto;

use std::sync::Arc;
use crate::application::services::profile::image::service::{
    GetImagesService, GetImageService, GetImageUsageService,
    CreateImageService, UpdateImageMetadataService, DeleteImageService, ForceDeleteImageService,
    GetUnusedImagesService, DeleteUnusedImagesService, TrackImageUsageService, UntrackImageUsageService
};
use crate::infrastructure::repository_impl::profile::image::repository::ImageRepositoryImpl;
use crate::infrastructure::repository_impl::profile::image::storage_repository::GcsImageStorageRepositoryImpl;

#[derive(Clone)]
pub struct ImageUseCases {
    pub get_all: Arc<GetImagesService<ImageRepositoryImpl>>,
    pub get_one: Arc<GetImageService<ImageRepositoryImpl>>,
    pub get_usage: Arc<GetImageUsageService<ImageRepositoryImpl>>,
    pub create: Arc<CreateImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>>,
    pub update_metadata: Arc<UpdateImageMetadataService<ImageRepositoryImpl>>,
    pub delete: Arc<DeleteImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>>,
    pub force_delete: Arc<ForceDeleteImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>>,
    pub get_unused: Arc<GetUnusedImagesService<ImageRepositoryImpl>>,
    pub delete_unused: Arc<DeleteUnusedImagesService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>>,
    pub track_usage: Arc<TrackImageUsageService<ImageRepositoryImpl>>,
    pub untrack_usage: Arc<UntrackImageUsageService<ImageRepositoryImpl>>,
}

impl ImageUseCases {
    pub fn new(
        get_all: GetImagesService<ImageRepositoryImpl>,
        get_one: GetImageService<ImageRepositoryImpl>,
        get_usage: GetImageUsageService<ImageRepositoryImpl>,
        create: CreateImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
        update_metadata: UpdateImageMetadataService<ImageRepositoryImpl>,
        delete: DeleteImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
        force_delete: ForceDeleteImageService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
        get_unused: GetUnusedImagesService<ImageRepositoryImpl>,
        delete_unused: DeleteUnusedImagesService<ImageRepositoryImpl, GcsImageStorageRepositoryImpl>,
        track_usage: TrackImageUsageService<ImageRepositoryImpl>,
        untrack_usage: UntrackImageUsageService<ImageRepositoryImpl>,
    ) -> Self {
        Self {
            get_all: Arc::new(get_all),
            get_one: Arc::new(get_one),
            get_usage: Arc::new(get_usage),
            create: Arc::new(create),
            update_metadata: Arc::new(update_metadata),
            delete: Arc::new(delete),
            force_delete: Arc::new(force_delete),
            get_unused: Arc::new(get_unused),
            delete_unused: Arc::new(delete_unused),
            track_usage: Arc::new(track_usage),
            untrack_usage: Arc::new(untrack_usage),
        }
    }
}
