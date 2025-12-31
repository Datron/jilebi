use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{
    extract::{Path, Query, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use axum_macros::debug_handler;
use serde_json::json;
use worker::*;

use crate::{
    AppState,
    api::types::{AppError, BUCKET_NAME, DownloadStat, FileType, VersionQuery},
};

pub fn parse_latest_plugin_version(
    plugin_name: &str,
    all_plugin_objects: &Objects,
) -> Result<String, AppError> {
    all_plugin_objects
        .objects()
        .iter()
        .filter(|obj| obj.key().starts_with(&format!("plugins/{}", plugin_name)))
        .filter_map(|obj| {
            let key = obj.key();
            key.split('/')
                .into_iter()
                .nth(2)
                .and_then(|version| semver::Version::parse(version).ok())
        })
        .max()
        .map(|v| v.to_string())
        .ok_or(AppError::InternalError(
            "Failed to parse plugin version".to_string(),
        ))
}

pub async fn get_latest_plugin_version(
    plugin_name: &str,
    state: Arc<AppState>,
) -> Result<String, AppError> {
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| AppError::R2Error("Failed to get bucket".to_string(), e))?;

    let all_objects = bucket
        .list()
        .prefix("plugins")
        .execute()
        .await
        .map_err(|e| AppError::R2Error("Failed to list plugins".to_string(), e))?;

    parse_latest_plugin_version(plugin_name, &all_objects)
}

#[debug_handler]
#[worker::send]
pub async fn list_plugins(
    State(state): State<Arc<AppState>>,
) -> std::result::Result<impl IntoResponse, AppError> {
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| AppError::R2Error("Failed to get bucket".to_string(), e))?;

    let db = state
        .env
        .d1("jilebi")
        .map_err(|e| AppError::D1Error("Failed to get database".to_string(), e))?;

    let all_objects = bucket
        .list()
        .prefix("plugins")
        .execute()
        .await
        .map_err(|e| AppError::R2Error("Failed to list plugins".to_string(), e))?;

    let plugins = all_objects
        .objects()
        .iter()
        .filter(|obj| obj.key().split('/').count() > 2)
        .filter_map(|obj| obj.key().split('/').nth(1).map(|s| s.to_string()))
        .collect::<HashSet<String>>();

    tracing::info!(
        "Fetched list of available plugins from bucket: {:?}",
        plugins
    );
    let download_stat = db
        .prepare("SELECT * FROM download_stats WHERE type = ?1 ORDER BY count DESC")
        .bind(&[FileType::Plugins.to_string().into()])
        .map_err(|e| AppError::D1Error("Failed to bind select params".to_string(), e))?
        .run()
        .await
        .map_err(|e| AppError::D1Error("Failed to execute select query".to_string(), e))?
        .results::<DownloadStat>()
        .unwrap_or_default()
        .into_iter()
        .map(|stat| (stat.name, stat.count))
        .collect::<HashMap<String, i64>>();
    let mut plugin_data = Vec::new();
    for plugin in plugins {
        let version = parse_latest_plugin_version(&plugin, &all_objects)?;
        let download_count = download_stat
            .get(&format!("{}.zip", &plugin))
            .cloned()
            .unwrap_or(0);
        plugin_data.push(json!({
            "name": plugin,
            "version": version,
            "download_count": download_count,
        }));
    }

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(
            serde_json::to_string(&plugin_data).map_err(|e| {
                tracing::error!("Response parsing error: {}", e);
                AppError::InternalError("Failed to serialize manifests".to_string())
            })?,
        ))
        .map_err(|e| {
            tracing::error!("Response parsing error: {}", e);
            AppError::InternalError("Failed to build response".to_string())
        })
}

#[debug_handler]
#[worker::send]
pub async fn get_plugin(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(VersionQuery { version }): Query<VersionQuery>,
) -> std::result::Result<impl IntoResponse, AppError> {
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| AppError::R2Error("Could not get bucket".to_string(), e))?;

    let all_objects = bucket
        .list()
        .prefix("plugins")
        .execute()
        .await
        .map_err(|e| AppError::R2Error("Failed to list plugins".to_string(), e))?;

    let db = state
        .env
        .d1("jilebi")
        .map_err(|e| AppError::D1Error("Failed to get database".to_string(), e))?;

    let plugin_download_count = db
        .prepare("SELECT * FROM download_stats WHERE type = ?1 AND name = ?2")
        .bind(&[
            FileType::Plugins.to_string().into(),
            format!("{}.zip", name).into(),
        ])
        .map_err(|e| AppError::D1Error("Failed to bind select params".to_string(), e))?
        .run()
        .await
        .map_err(|e| AppError::D1Error("Failed to execute select query".to_string(), e))?
        .results::<DownloadStat>()
        .unwrap_or_default()
        .first()
        .map(|item| item.count)
        .unwrap_or_default();

    let version = match version {
        Some(v) => v,
        None => parse_latest_plugin_version(&name, &all_objects)?,
    };

    let manifest_object = bucket
        .get(format!("plugins/{}/{}/manifest.toml", name, version))
        .execute()
        .await
        .map_err(|e| {
            AppError::R2Error("Either the plugin or version does not exist".to_string(), e)
        })?
        .ok_or(AppError::PluginNotFound(
            "Either the plugin or version does not exist",
        ))?;
    let plugin_manifest = manifest_object.body().ok_or(AppError::PluginNotFound(
        "manifest not found in plugin package",
    ))?;

    let manifest_data = plugin_manifest
        .bytes()
        .await
        .and_then(|b| String::from_utf8(b).map_err(|e| Error::RustError(e.to_string())))
        .map_err(|e| AppError::R2Error("Failed to read plugin manifest body".to_string(), e))?;

    let mut manifest = toml::from_str::<toml::Value>(&manifest_data)
        .map(|m| m.as_table().cloned())
        .map_err(|e| {
            AppError::InternalError(format!("Failed to parse plugin manifest TOML: {}", e))
        })?
        .ok_or(AppError::InternalError(
            "could not parse to hashmap".to_string(),
        ))?;

    manifest.insert(
        "downloads".to_string(),
        toml::Value::Integer(plugin_download_count),
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(
            serde_json::to_string(&manifest).map_err(|e| {
                AppError::InternalError(format!("Failed to serialize manifests, {}", e))
            })?,
        ))
        .map_err(|e| AppError::InternalError(format!("Failed to build response, {}", e)))
}
