pub mod plugins;
pub mod releases;
pub mod types;

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;
use worker::*;

use crate::{
    AppState,
    api::types::{BUCKET_NAME, DownloadStat, FileType},
};

pub async fn health() -> &'static str {
    "Hi from jilebi!"
}

#[debug_handler]
#[worker::send]
pub async fn download_file(
    Path((file_type, name)): Path<(FileType, String)>,
    State(state): State<Arc<AppState>>,
) -> std::result::Result<impl IntoResponse, Html<String>> {
    let internal_server_error = |message: &str, e: Error| -> Html<String> {
        tracing::error!("{}: {}", message, e);
        Html("Internal Server Error".to_string())
    };
    let filename = match file_type {
        FileType::Plugins => format!("plugins/{}", name),
        FileType::Bin => format!("bin/{}", name),
        FileType::Templates => format!("templates/{}", name),
    };
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| internal_server_error("Failed to get bucket: ", e))?;

    let db = state
        .env
        .d1("jilebi")
        .map_err(|e| internal_server_error("Failed to get database: ", e))?;

    let download_stat = db
        .prepare("SELECT * FROM download_stats WHERE name = ?1 AND type = ?2")
        .bind(&[(&name).into(), file_type.to_string().into()])
        .map_err(|e| internal_server_error("Failed to bind select params: ", e))?
        .run()
        .await
        .map_err(|e| internal_server_error("Failed to execute select query: ", e))?
        .results::<DownloadStat>()
        .unwrap_or_default();

    let mut count = download_stat.first().map(|item| item.count).unwrap_or(0);

    let file = bucket
        .get(&filename)
        .execute()
        .await
        .map_err(|e| internal_server_error("Failed to get file from bucket: ", e))?;
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
                .map_err(|e| internal_server_error("Failed to bind insert params: ", e))?
                .run()
                .await
                .map_err(|e| internal_server_error("Failed to execute insert query: ", e))?;
            let bytes = file
                .body()
                .unwrap()
                .bytes()
                .await
                .map_err(|e| internal_server_error("Failed to get file from bucket: ", e))?;
            let content_type = file
                .http_metadata()
                .content_type
                .unwrap_or("application/octet-stream".to_string());

            Ok(([(header::CONTENT_TYPE, &content_type)], bytes).into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "File not found").into_response()),
    }
}
