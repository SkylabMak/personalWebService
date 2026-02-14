use axum::extract::{Path, State};
use axum::response::IntoResponse;
use crate::application::use_cases::profile::profile::dto::input::GetProfileInput;
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;

pub async fn get_profile_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
) -> impl IntoResponse {
    let input = GetProfileInput::new(profile_id);

    state.profile.profile.get_one.execute(input).await.into_response()
}
