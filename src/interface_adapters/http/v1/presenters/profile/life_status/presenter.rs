use axum::response::IntoResponse;

use crate::application::errors::ApplicationError;
use crate::interface_adapters::http::v1::presenters::common::error_presenter::ErrorPresenter;
use crate::interface_adapters::http::v1::presenters::common::presenter_output::PresenterOutput;

pub struct LifeStatusPresenter;

impl LifeStatusPresenter {
    pub fn success<T: PresenterOutput>(result: T) -> impl IntoResponse {
        result.into_response()
    }

    pub fn error(error: ApplicationError) -> impl IntoResponse {
        ErrorPresenter::present(error)
    }
}
