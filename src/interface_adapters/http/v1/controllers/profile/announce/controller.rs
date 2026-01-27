use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};

use crate::application::use_cases::profile::announce::dto::input::GetAnnounceListInput;
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::presenters::profile::announce::presenter::AnnouncePresenter;

pub async fn get_announce_list_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
) -> Response {
    let input = GetAnnounceListInput::new(profile_id);

    match state.profile.announce.get_list.execute(input).await {
        Ok(result) => AnnouncePresenter::success(result).into_response(),
        Err(e) => AnnouncePresenter::error(e).into_response(),
    }
}
