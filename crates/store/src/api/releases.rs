use std::sync::Arc;

use axum::{
    extract::State,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use worker::*;

use crate::{
    AppState,
    api::types::{AppError, BUCKET_NAME},
};

fn get_latest_release_version(all_release_objects: &Objects) -> Result<String, AppError> {
    all_release_objects
        .objects()
        .iter()
        .filter(|obj| obj.key().split('/').count() > 2)
        .filter_map(|obj| {
            let key = obj.key();
            key.split('/')
                .into_iter()
                .nth(1)
                .and_then(|version| semver::Version::parse(version).ok())
        })
        .max()
        .map(|v| v.to_string())
        .ok_or(AppError::InternalError(
            "Failed to parse release version".to_string(),
        ))
}

#[debug_handler]
#[worker::send]
pub async fn get_latest_release(
    State(state): State<Arc<AppState>>,
) -> std::result::Result<impl IntoResponse, AppError> {
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| AppError::R2Error("Failed to get bucket".to_string(), e))?;

    let all_objects = bucket
        .list()
        .prefix("bin")
        .execute()
        .await
        .map_err(|e| AppError::R2Error("Failed to list releases".to_string(), e))?;

    let version = get_latest_release_version(&all_objects)?;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(axum::body::Body::from(version))
        .map_err(|e| {
            tracing::error!("Response parsing error: {}", e);
            AppError::InternalError("Failed to build response".to_string())
        })
}
