use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use crate::application::use_cases::application::feature_status::dto::input::GetAppFeatureStatusInput;
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;
use crate::interface_adapters::http::v1::presenters::website::feature_status::presenter::WebsitePresenter;

pub async fn get_website_feature_status_ctrl(
    State(state): State<AppState>,
    Path(app_id): Path<String>,
) -> Response {
    let input = GetAppFeatureStatusInput::new(app_id);

    match state
        .website
        .feature_status
        .get_feature_status
        .execute(input)
        .await
    {
        Ok(result) => WebsitePresenter::success(result).into_response(),
        Err(error) => WebsitePresenter::error(error).into_response(),
    }
}
