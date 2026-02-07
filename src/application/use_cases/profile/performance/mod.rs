pub mod dto;

use std::sync::Arc;
use crate::application::services::profile::performance::service::{
    CreatePerformanceService, UpdatePerformanceService, DeletePerformanceService
};
use crate::application::services::profile::performance::content_service::{
    GetPerformanceContentService, UpdatePerformanceContentService
};
use crate::infrastructure::repository_impl::profile::performance::repository::PerformanceRepositoryImpl;
use crate::infrastructure::repository_impl::profile::performance_content::repository::GcsPerformanceContentRepositoryImpl;

#[derive(Clone)]
pub struct PerformanceUseCases {
    pub create: Arc<CreatePerformanceService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>>,
    pub update: Arc<UpdatePerformanceService<PerformanceRepositoryImpl>>,
    pub delete: Arc<DeletePerformanceService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>>,
    pub get_content: Arc<GetPerformanceContentService<GcsPerformanceContentRepositoryImpl>>,
    pub update_content: Arc<UpdatePerformanceContentService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>>,
}

impl PerformanceUseCases {
    pub fn new(
        create: CreatePerformanceService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>,
        update: UpdatePerformanceService<PerformanceRepositoryImpl>,
        delete: DeletePerformanceService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>,
        get_content: GetPerformanceContentService<GcsPerformanceContentRepositoryImpl>,
        update_content: UpdatePerformanceContentService<PerformanceRepositoryImpl, GcsPerformanceContentRepositoryImpl>,
    ) -> Self {
        Self {
            create: Arc::new(create),
            update: Arc::new(update),
            delete: Arc::new(delete),
            get_content: Arc::new(get_content),
            update_content: Arc::new(update_content),
        }
    }
}
