use axum::response::IntoResponse;

pub trait PresenterOutput {
    fn into_response(self) -> impl IntoResponse;
}
