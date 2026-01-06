use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::presenters::profile::life_status::presenter::LifeStatusPresenter;


pub async fn get_current_life_status_ctrl(
    State(state): State<AppState>,          // ← Axum injects AppState here
    Path(profile_id): Path<String>,
) -> Response {
    match state
        .profile
        .life_status
        .get_current                         // ← use case from State
        .execute(profile_id)
        .await
    {
        Ok(result) => LifeStatusPresenter::success(result).into_response(),
        Err(error) => LifeStatusPresenter::error(error).into_response(),
    }
}
