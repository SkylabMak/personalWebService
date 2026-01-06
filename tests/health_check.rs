use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use rust_backend::app::App;
use dotenvy::dotenv;

#[tokio::test]
async fn test_get_current_life_status_not_found() {
    dotenv().ok();
    
    // In a real scenario, you might want to use a separate test database or mock.
    // Here we just test that the router is correctly set up and returns 404 for a non-existent ID.
    let app = App::create_router().await.expect("Failed to create router");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/profiles/non-existent-id/life-status/current")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
