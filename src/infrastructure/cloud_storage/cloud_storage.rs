use crate::infrastructure::cloud_storage::gcs::common::gcs_repository::GcsRepository;

pub struct CloudStorage {
    pub gcs: GcsRepository,
}

impl CloudStorage {
    pub fn new(gcs: GcsRepository) -> Self {
        Self { gcs }
    }
}
