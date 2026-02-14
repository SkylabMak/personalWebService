use crate::application::services::profile::profile_services::ProfileServices;
use crate::application::use_cases::profile::profile::ProfileBaseUseCases;
use crate::application::use_cases::profile::life_status::LifeStatusUseCases;
use crate::application::use_cases::profile::announce::AnnounceUseCases;
use crate::application::use_cases::profile::image::ImageUseCases;
use crate::application::use_cases::profile::performance::PerformanceUseCases;

#[derive(Clone)]
pub struct ProfileUseCases {
    pub profile: ProfileBaseUseCases,
    pub life_status: LifeStatusUseCases,
    pub announce: AnnounceUseCases,
    pub image: ImageUseCases,
    pub performance: PerformanceUseCases,
}

impl ProfileUseCases {
    pub fn new(services: ProfileServices) -> Self {
        let profile = ProfileBaseUseCases::new(services.profile_get_one);
        let life_status = LifeStatusUseCases::new(services.life_status);
        let announce = AnnounceUseCases::new(services.announce);
        let image = ImageUseCases::new(
            services.image_get_all,
            services.image_get_one,
            services.image_get_usage,
            services.image_create,
            services.image_update_metadata,
            services.image_delete,
            services.image_force_delete,
            services.image_get_unused,
            services.image_delete_unused,
            services.image_track_usage,
            services.image_untrack_usage,
        );
        let performance = PerformanceUseCases::new(
            services.performance_create,
            services.performance_update,
            services.performance_delete,
            services.performance_get_content,
            services.performance_update_content,
            services.performance_get_all,
        );
        Self {
            profile,
            life_status,
            announce,
            image,
            performance,
        }
    }
}
