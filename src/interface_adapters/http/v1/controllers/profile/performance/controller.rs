use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::application::use_cases::profile::performance::dto::input::{
    CreatePerformanceInput, UpdatePerformanceInput, DeletePerformanceInput,
    GetPerformanceContentInput, UpdatePerformanceContentInput
};
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;

#[derive(Deserialize)]
pub struct CreatePerformanceRequest {
    pub category_id: String,
    pub visibility_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub location: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePerformanceRequest {
    pub category_id: String,
    pub visibility_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub location: Option<String>,
    pub close: bool,
}

#[derive(Deserialize)]
pub struct UpdatePerformanceContentRequest {
    pub content_markdown: String,
}

pub async fn create_performance_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    Json(payload): Json<CreatePerformanceRequest>,
) -> impl IntoResponse {
    let input = CreatePerformanceInput {
        profile_id,
        category_id: payload.category_id,
        visibility_id: payload.visibility_id,
        title: payload.title,
        summary: payload.summary,
        start_date: payload.start_date,
        end_date: payload.end_date,
        location: payload.location,
    };

    state.profile.performance.create.execute(input).await.into_response()
}

pub async fn update_performance_ctrl(
    State(state): State<AppState>,
    Path((profile_id, performance_id)): Path<(String, String)>,
    Json(payload): Json<UpdatePerformanceRequest>,
) -> impl IntoResponse {
    let input = UpdatePerformanceInput {
        id: performance_id,
        profile_id,
        category_id: payload.category_id,
        visibility_id: payload.visibility_id,
        title: payload.title,
        summary: payload.summary,
        start_date: payload.start_date,
        end_date: payload.end_date,
        location: payload.location,
        close: payload.close,
    };

    state.profile.performance.update.execute(input).await.into_response()
}

pub async fn delete_performance_ctrl(
    State(state): State<AppState>,
    Path((_profile_id, performance_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let input = DeletePerformanceInput {
        id: performance_id,
        profile_id: _profile_id,
    };

    state.profile.performance.delete.execute(input).await.into_response()
}

pub async fn get_performance_content_ctrl(
    State(state): State<AppState>,
    Path((profile_id, performance_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let input = GetPerformanceContentInput {
        performance_id,
        profile_id,
    };

    state.profile.performance.get_content.execute(input).await.into_response()
}

pub async fn update_performance_content_ctrl(
    State(state): State<AppState>,
    Path((profile_id, performance_id)): Path<(String, String)>,
    Json(payload): Json<UpdatePerformanceContentRequest>,
) -> impl IntoResponse {
    let input = UpdatePerformanceContentInput {
        performance_id,
        profile_id,
        content_markdown: payload.content_markdown,
    };

    state.profile.performance.update_content.execute(input).await.into_response()
}
