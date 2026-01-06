pub mod domain;
pub mod infrastructure;
pub mod delivery;
pub mod interface_adapters;
pub mod application;
pub mod app;
pub mod config;

use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::fmt;
use crate::app::App;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");

    info!("INFO: Shutdown signal received");
}

pub async fn run() {
    dotenv().ok();

    // Initialize tracing
    fmt::init();

    // 1. Build router via Application Composition Root
    let router = match App::create_router().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("CRITICAL: Failed to initialize application: {:?}", e);
            std::process::exit(1);
        }
    };

    // 2. Bind listener
    let addr = "0.0.0.0:3000";
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("CRITICAL: Failed to bind to {}: {:?}", addr, e);
            std::process::exit(1);
        }
    };

    info!("INFO: Server starting on http://{}", addr);

    // 3. Serve
    if let Err(e) = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        eprintln!("CRITICAL: Server error: {:?}", e);
        std::process::exit(1);
    }

    info!("INFO: Server stopped gracefully");

}