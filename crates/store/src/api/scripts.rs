use std::sync::Arc;

use axum::{
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};

use crate::AppState;

use super::types::{AppError, BUCKET_NAME};

async fn serve_script(
    path: &str,
    content_type: &str,
    state: Arc<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| AppError::R2Error("Failed to get bucket".to_string(), e))?;

    let file = bucket
        .get(path)
        .execute()
        .await
        .map_err(|e| AppError::R2Error("Failed to get file from bucket".to_string(), e))?;

    match file {
        Some(file) => {
            let bytes =
                file.body().unwrap().bytes().await.map_err(|e| {
                    AppError::InternalError(format!("Failed to read script: {}", e))
                })?;

            Ok((
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, content_type),
                    (header::CACHE_CONTROL, "public, max-age=3600"),
                ],
                bytes,
            )
                .into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "Script not found").into_response()),
    }
}

#[worker::send]
pub async fn install_script_sh(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    serve_script("scripts/install.sh", "text/x-shellscript", state).await
}

#[worker::send]
pub async fn install_script_ps1(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    serve_script("scripts/install.ps1", "text/plain; charset=utf-8", state).await
}

#[worker::send]
pub async fn uninstall_script_sh(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    serve_script("scripts/uninstall.sh", "text/x-shellscript", state).await
}

#[worker::send]
pub async fn uninstall_script_ps1(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    serve_script("scripts/uninstall.ps1", "text/plain; charset=utf-8", state).await
}
