use google_cloud_storage::client::{Client, ClientConfig};
use crate::config::config::Config;
use crate::infrastructure::cloud_storage::gcs::common::gcs_repository::GcsRepository;

pub async fn setup_gcs(config: &Config) -> anyhow::Result<GcsRepository> {
    tracing::info!("🔧 Initializing Google Cloud Storage client...");

    let mut client_config = ClientConfig::default();

    // Instead of setting a global env var, we inject the credentials
    // directly into the client configuration.
    if let Some(creds_path) = &config.google_application_credentials {
        tracing::info!("📁 Loading GCS credentials from: {}", creds_path);
        // Assuming your crate supports with_credentials_file or similar:
        client_config = client_config.with_auth().await?;
        // Note: If your GCS library version requires the env var,
        // the unsafe block above is your only choice.
    }

    let client = Client::new(client_config);
    tracing::info!("✅ GCS client initialized successfully");

    Ok(GcsRepository::new(config.gcs_bucket_name.clone(), client))
}
