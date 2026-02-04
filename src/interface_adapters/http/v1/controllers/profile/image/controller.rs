use axum::{
    extract::{Path, Query, State, Multipart},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::application::use_cases::profile::image::dto::input::{
    GetImagesInput, GetImageInput, CreateImageInput, UpdateImageMetadataInput,
    DeleteImageInput, ForceDeleteImageInput, GetUnusedImagesInput, DeleteUnusedImagesInput, TrackImageUsageInput
};
use crate::application::use_cases::use_case::UseCase;
use crate::delivery::http::server::state::AppState;

#[derive(Deserialize)]
pub struct GetImagesQuery {
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateImageMetadataRequest {
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

#[derive(Deserialize)]
pub struct GetUnusedImagesQuery {
    pub days_old: Option<i32>,
}

#[derive(Deserialize)]
pub struct DeleteUnusedImagesQuery {
    pub confirm: bool,
    pub days_old: Option<i32>,
}

pub async fn get_images_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    Query(query): Query<GetImagesQuery>,
) -> impl IntoResponse {
    let input = GetImagesInput::new(profile_id, query.search, query.limit, query.offset);
    
    let res: Result<crate::application::services::profile::image::result::ImageListResult, crate::application::errors::ApplicationError> = state.profile.image.get_all.execute(input).await;
    res.into_response()
}

pub async fn get_image_ctrl(
    State(state): State<AppState>,
    Path((profile_id, image_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let input = GetImageInput::new(image_id, profile_id);
    
    let res: Result<crate::application::services::profile::image::result::ImageResult, crate::application::errors::ApplicationError> = state.profile.image.get_one.execute(input).await;
    res.into_response()
}

pub async fn get_image_usage_ctrl(
    State(state): State<AppState>,
    Path((profile_id, image_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let input = GetImageInput::new(image_id, profile_id);
    
    let res: Result<crate::application::services::profile::image::result::ImageUsageResult, crate::application::errors::ApplicationError> = state.profile.image.get_usage.execute(input).await;
    res.into_response()
}
pub async fn upload_image_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut original_filename = String::new();
    let mut mime_type = String::new();
    let mut image_bytes = Vec::new();
    let mut alt_text = None;
    let mut caption = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        
        match name.as_str() {
            "file" => {
                original_filename = field.file_name().unwrap_or("image.jpg").to_string();
                mime_type = field.content_type().unwrap_or("image/jpeg").to_string();
                image_bytes = field.bytes().await.unwrap_or_default().to_vec();
            },
            "alt_text" => {
                alt_text = Some(field.text().await.unwrap_or_default());
            },
            "caption" => {
                caption = Some(field.text().await.unwrap_or_default());
            },
            _ => {}
        }
    }

    if image_bytes.is_empty() {
        return crate::application::errors::ApplicationError::ValidationError { 
            message: "No image file provided".to_string() 
        }.into_response();
    }

    let input = CreateImageInput {
        profile_id,
        original_filename,
        mime_type,
        file_size: image_bytes.len() as i32,
        width: None,
        height: None,
        alt_text,
        caption,
        image_bytes,
    };

    let res: Result<crate::application::services::profile::image::result::ImageResult, crate::application::errors::ApplicationError> = state.profile.image.create.execute(input).await;
    res.into_response()
}

pub async fn update_image_metadata_ctrl(
    State(state): State<AppState>,
    Path((profile_id, image_id)): Path<(String, String)>,
    Json(payload): Json<UpdateImageMetadataRequest>,
) -> impl IntoResponse {
    let input = UpdateImageMetadataInput {
        id: image_id,
        profile_id,
        alt_text: payload.alt_text,
        caption: payload.caption,
    };

    let res: Result<crate::application::services::profile::image::result::MessageResult, crate::application::errors::ApplicationError> = state.profile.image.update_metadata.execute(input).await;
    res.into_response()
}

pub async fn delete_image_ctrl(
    State(state): State<AppState>,
    Path((profile_id, image_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let input = DeleteImageInput {
        id: image_id,
        profile_id,
    };

    let res: Result<crate::application::services::profile::image::result::MessageResult, crate::application::errors::ApplicationError> = state.profile.image.delete.execute(input).await;
    res.into_response()
}

pub async fn force_delete_image_ctrl(
    State(state): State<AppState>,
    Path((profile_id, image_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let input = ForceDeleteImageInput {
        id: image_id,
        profile_id,
    };

    let res: Result<crate::application::services::profile::image::result::MessageResult, crate::application::errors::ApplicationError> = state.profile.image.force_delete.execute(input).await;
    res.into_response()
}

pub async fn get_unused_images_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    Query(query): Query<GetUnusedImagesQuery>,
) -> impl IntoResponse {
    let input = GetUnusedImagesInput {
        profile_id,
        days_old: query.days_old.unwrap_or(30),
    };

    let res: Result<crate::application::services::profile::image::result::UnusedImagesResult, crate::application::errors::ApplicationError> = state.profile.image.get_unused.execute(input).await;
    res.into_response()
}

pub async fn delete_unused_images_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    Query(query): Query<DeleteUnusedImagesQuery>,
) -> impl IntoResponse {
    let input = DeleteUnusedImagesInput {
        profile_id,
        days_old: query.days_old.unwrap_or(0),
        confirm: query.confirm,
    };

    let res: Result<crate::application::services::profile::image::result::DeleteUnusedImagesResult, crate::application::errors::ApplicationError> = state.profile.image.delete_unused.execute(input).await;
    res.into_response()
}

#[derive(Deserialize)]
pub struct TrackImageUsageRequest {
    pub image_id: String,
    pub performance_id: String,
}

pub async fn track_image_usage_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    Json(payload): Json<TrackImageUsageRequest>,
) -> impl IntoResponse {
    let input = TrackImageUsageInput {
        profile_id,
        image_id: payload.image_id,
        performance_id: payload.performance_id,
    };

    let res: Result<crate::application::services::profile::image::result::MessageResult, crate::application::errors::ApplicationError> = state.profile.image.track_usage.execute(input).await;
    res.into_response()
}

pub async fn untrack_image_usage_ctrl(
    State(state): State<AppState>,
    Path(profile_id): Path<String>,
    Json(payload): Json<TrackImageUsageRequest>,
) -> impl IntoResponse {
    let input = TrackImageUsageInput {
        profile_id,
        image_id: payload.image_id,
        performance_id: payload.performance_id,
    };

    let res: Result<crate::application::services::profile::image::result::MessageResult, crate::application::errors::ApplicationError> = state.profile.image.untrack_usage.execute(input).await;
    res.into_response()
}
