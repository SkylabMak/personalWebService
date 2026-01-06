mod domain;
mod infrastructure;
mod delivery;
mod interface_adapters;
mod application;
mod app;
mod config;

use dotenvy::dotenv;
use tokio::net::TcpListener;
use crate::app::App;


#[tokio::main]
async fn main() {
    dotenv().ok();

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

    println!("INFO: Server starting on http://{}", addr);

    // 3. Serve
    if let Err(e) = axum::serve(listener, router).await {
        eprintln!("CRITICAL: Server error: {:?}", e);
        std::process::exit(1);
    }
}

