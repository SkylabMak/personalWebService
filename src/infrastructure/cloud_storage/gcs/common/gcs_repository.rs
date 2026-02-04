use google_cloud_storage::client::Client;

#[derive(Clone)]
pub struct GcsRepository {
    bucket_name: String,
    client: Client,
}

impl GcsRepository {
    pub fn new(bucket_name: String, client: Client) -> Self {
        Self { bucket_name, client }
    }

    pub fn bucket_name(&self) -> &str {
        &self.bucket_name
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
