pub mod plugins;
pub mod releases;
pub mod types;

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use axum_macros::debug_handler;

use crate::{
    AppState,
    api::{
        plugins::get_latest_plugin_version,
        releases::get_latest_release_version,
        types::{AppError, BUCKET_NAME, DownloadStat, FileType, VersionQuery},
    },
};

pub async fn health() -> &'static str {
    "Hi from jilebi!"
}

#[debug_handler]
#[worker::send]
pub async fn download_file(
    Path((file_type, name)): Path<(FileType, String)>,
    Query(VersionQuery { version }): Query<VersionQuery>,
    State(state): State<Arc<AppState>>,
) -> std::result::Result<impl IntoResponse, AppError> {
    let filename = match file_type {
        FileType::Plugins => {
            let plugin_name = name.split('.').next().unwrap_or(&name);
            let v = match version {
                Some(v) => v,
                None => get_latest_plugin_version(plugin_name, state.clone()).await?,
            };
            format!("plugins/{}/{}/plugin.zip", plugin_name, v)
        }
        FileType::Bin => {
            let v = match version {
                Some(v) => v,
                None => get_latest_release_version(state.clone()).await?,
            };
            format!("bin/{}/{}", v, name)
        }
        FileType::Templates => format!("templates/{}", name),
    };
    tracing::info!("Filename being downloaded: {}", filename);
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| AppError::R2Error("Failed to get bucket".to_string(), e))?;

    let db = state
        .env
        .d1("jilebi")
        .map_err(|e| AppError::D1Error("Failed to get database".to_string(), e))?;

    let download_stat = db
        .prepare("SELECT * FROM download_stats WHERE name = ?1 AND type = ?2")
        .bind(&[(&name).into(), file_type.to_string().into()])
        .map_err(|e| AppError::D1Error("Failed to bind select params".to_string(), e))?
        .run()
        .await
        .map_err(|e| AppError::D1Error("Failed to execute select query".to_string(), e))?
        .results::<DownloadStat>()
        .unwrap_or_default();

    let mut count = download_stat.first().map(|item| item.count).unwrap_or(0);

    let file = bucket
        .get(&filename)
        .execute()
        .await
        .map_err(|e| AppError::R2Error("Failed to get file from bucket".to_string(), e))?;
    match file {
        Some(file) => {
            count += 1;
            let _ = db
                .prepare("INSERT OR REPLACE INTO download_stats(name,type,count) VALUES(?1,?2,?3)")
                .bind(&[
                    name.into(),
                    file_type.to_string().into(),
                    count.to_string().into(),
                ])
                .map_err(|e| AppError::D1Error("Failed to bind insert params".to_string(), e))?
                .run()
                .await
                .map_err(|e| AppError::D1Error("Failed to execute insert query".to_string(), e))?;
            let bytes = file.body().unwrap().bytes().await.map_err(|e| {
                AppError::InternalError(format!("Failed to get file from bucket: {}", e))
            })?;
            let content_type = file
                .http_metadata()
                .content_type
                .unwrap_or("application/octet-stream".to_string());

            Ok(([(header::CONTENT_TYPE, &content_type)], bytes).into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "File not found").into_response()),
    }
}
