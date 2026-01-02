use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use tracing_subscriber::{
    fmt::{format::Pretty, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use tracing_web::{MakeConsoleWriter, performance_layer};
use worker::*;

use crate::api::{
    health,
    plugins::{get_plugin, list_plugins},
    releases::get_latest_release,
    scripts::{install_script_ps1, install_script_sh, uninstall_script_ps1, uninstall_script_sh},
};

mod api;

#[derive(Debug)]
struct AppState {
    env: Env,
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
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(86400));
    Router::new()
        .route("/api/health", get(health))
        .route("/api/download/{file_type}/{name}", get(api::download_file))
        .route("/api/plugins", get(list_plugins))
        .route("/api/plugins/{name}", get(get_plugin))
        .route("/api/releases/latest", get(get_latest_release))
        .route("/install.sh", get(install_script_sh))
        .route("/install.ps1", get(install_script_ps1))
        .route("/uninstall.sh", get(uninstall_script_sh))
        .route("/uninstall.ps1", get(uninstall_script_ps1))
        .layer(cors_layer)
        .with_state(app_state)
}
