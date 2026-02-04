use axum::{
    http::{StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
    extract::{Request, State},
    body::Body,
};
use crate::delivery::http::server::state::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = match request.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok()) {
            Some(h) => h,
            None => return StatusCode::UNAUTHORIZED.into_response(),
        };

    if !auth_header.starts_with("Bearer ") {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let token = &auth_header[7..];

    let claims = match state.auth.auth.jwt_service.validate_access_token(token) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    request.extensions_mut().insert(claims);

    next.run(request).await
}
