use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{Path, State},
    http::{Response, StatusCode, header},
    response::{Html, IntoResponse},
    routing::get,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tower_service::Service;
use tracing_subscriber::{
    fmt::{format::Pretty, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use tracing_web::{MakeConsoleWriter, performance_layer};
use worker::*;

const BUCKET_NAME: &str = "JILEBI";

#[derive(Debug)]
struct AppState {
    env: Env,
}

#[derive(Debug, strum_macros::Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
enum FileType {
    Plugins,
    Bin,
    Templates,
}
#[derive(Debug, Serialize, Deserialize)]
struct DownloadStat {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub r#type: FileType,
}
// Multiple calls to `init` will cause a panic as a tracing subscriber is already set.
// So we use the `start` event to initialize our tracing subscriber when the worker starts.
#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[event(fetch)]
async fn main(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router(env).call(req).await?)
}

fn router(env: Env) -> Router {
    let app_state = Arc::new(AppState { env });
    Router::new()
        .route("/api/health", get(health))
        .route("/api/download/{file_type}/{name}", get(download_file))
        .route("/api/plugins", get(list_plugins))
        .with_state(app_state)
}

pub async fn health() -> &'static str {
    "Hello World!"
}

#[debug_handler]
#[worker::send]
async fn download_file(
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

#[debug_handler]
#[worker::send]
async fn list_plugins(
    State(state): State<Arc<AppState>>,
) -> std::result::Result<impl IntoResponse, Html<String>> {
    let internal_server_error = |message: &str, e: Error| -> Html<String> {
        tracing::error!("{}: {}", message, e);
        Html("Internal Server Error".to_string())
    };
    let bucket = state
        .env
        .bucket(BUCKET_NAME)
        .map_err(|e| internal_server_error("Failed to get bucket: ", e))?;

    let db = state
        .env
        .d1("jilebi")
        .map_err(|e| internal_server_error("Failed to get database: ", e))?;

    let plugins_available = bucket
        .list()
        .prefix("plugins")
        .execute()
        .await
        .map_err(|e| internal_server_error("Failed to list plugins: ", e))?;

    tracing::info!(
        "Fetched list of available plugins from bucket: {:?}",
        plugins_available
    );

    let plugins = plugins_available
        .objects()
        .iter()
        .filter_map(|obj| {
            obj.key()
                .split('/')
                .nth(1)
                .and_then(|s| s.split('.').nth(0).map(|i| i.to_string()))
        })
        .collect::<Vec<String>>();

    let download_stat = db
        .prepare("SELECT * FROM download_stats WHERE type = ?1 ORDER BY count DESC")
        .bind(&[FileType::Plugins.to_string().into()])
        .map_err(|e| internal_server_error("Failed to bind select params: ", e))?
        .run()
        .await
        .map_err(|e| internal_server_error("Failed to execute select query: ", e))?
        .results::<DownloadStat>()
        .unwrap_or_default()
        .into_iter()
        .map(|stat| (stat.name, stat.count))
        .collect::<HashMap<String, i64>>();
    let client = reqwest::Client::new();
    let mut manifests = Vec::new();
    for plugin in plugins {
        let manifest_url = format!(
            "https://raw.githubusercontent.com/Datron/jilebi-plugins/refs/heads/master/{}/manifest.toml",
            plugin
        );
        let manifest = client.get(manifest_url).send().await.map_err(|e| {
            internal_server_error(
                "Failed to get manifest from store",
                Error::RustError(e.to_string()),
            )
        })?;
        tracing::info!("Fetched manifest for plugin: {}", plugin);
        if manifest.status().is_success() {
            let manifest_text = manifest.text().await.map_err(|e| {
                internal_server_error(
                    "Failed to read manifest text",
                    Error::RustError(e.to_string()),
                )
            })?;
            let mut manifest_toml: toml::Value = toml::from_str(&manifest_text).map_err(|e| {
                internal_server_error(
                    "Failed to parse manifest toml",
                    Error::RustError(e.to_string()),
                )
            })?;
            let download_count = download_stat
                .get(&format!("{}.zip", &plugin))
                .cloned()
                .unwrap_or(0);
            manifest_toml.as_table_mut().and_then(|table| {
                table.insert(
                    "download_count".to_string(),
                    toml::Value::Integer(download_count),
                )
            });
            manifests.push(manifest_toml);
        }
    }

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(
            serde_json::to_string(&manifests).map_err(|e| {
                internal_server_error(
                    "Failed to serialize manifests",
                    Error::RustError(e.to_string()),
                )
            })?,
        ))
        .map_err(|e| {
            internal_server_error("Failed to build response", Error::RustError(e.to_string()))
        })
}
