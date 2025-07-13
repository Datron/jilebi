use std::{collections::HashMap, fs, path::Path};
mod server;
use jilebi_types::plugin::Manifest;
use rmcp::{ServiceExt, transport::stdio};
use server::JilebiMcpServer;
use tracing::{Level, error, event, info, span};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn load_plugins() -> Result<HashMap<String, Manifest>, String> {
    let plugin_directory = dotenvy::var("PLUGIN_DIR").unwrap_or("./".into());
    info!("Plugin directory being used -> {}", plugin_directory);
    let plugin_toml_path = plugin_directory + "/plugins.toml";
    let plugin_toml =
        fs::read_to_string(Path::new(&plugin_toml_path)).map_err(|e| e.to_string())?;
    let plugin_toml = toml::from_str::<toml::Value>(&plugin_toml).map_err(|e| e.to_string())?;
    let manifests = plugin_toml
        .get("manifest")
        .and_then(|manifests| manifests.as_array())
        .ok_or(String::from(
            "Define an array for plugin manifests. Check the docs",
        ))?
        .into_iter()
        .map(|m| {
            let manifest = m
                .as_str()
                .and_then(|manifest_path| {
                    let manifest =
                        fs::read_to_string(manifest_path).expect("Manifest file not found");
                    let manifest = toml::from_str::<toml::Value>(&manifest)
                        .expect("Could not parse toml file");
                    Manifest::try_from(manifest).map_err(|e| error!(e)).ok()
                })
                .expect("Could not parse manifest file");
            (manifest.name.clone(), manifest)
        })
        .collect::<HashMap<String, Manifest>>();
    Ok(manifests)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenvy::dotenv().map_err(|e| e.to_string())?;
    let file_appender = tracing_appender::rolling::hourly(
        dotenvy::var("LOG_PATH").unwrap_or("./logs".into()),
        dotenvy::var("LOG_FILE").unwrap_or("jilebi.log".into()),
    );
    let (non_blocking_log_writer, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking_log_writer))
        .with(EnvFilter::from_default_env())
        .init();

    let main_span = span!(Level::INFO, "Jilebi Server started");
    let _ = main_span.enter();
    let plugins = load_plugins()?;

    event!(Level::INFO, ?plugins, "Plugins and manifests loaded");
    let server = JilebiMcpServer::new(plugins);

    let service = server.serve(stdio()).await.map_err(|e| {
        tracing::error!("Serving error: {:?}", e);
        e.to_string()
    })?;
    service.waiting().await.map_err(|e| e.to_string())?;
    Ok(())
}

// Jilebi MVP
// MCP Server
// TODO: support SSE, HTTP
// TODO: support Resource Template
// TODO: Error handling
// TODO: add pagination support
// plugins
// TODO: validate names to not include _
// TODO: implement custom logging functions
// TODO: implement custom file read/write functions
// TODO: implement custom network functions

// TODO: make it easy to manage with a store
// TODO: let folks specify mc plugins from github or file system or URL

// Beyond MVP
// UI
// TODO: think about a UI (leptos) and TUI over ssh (ratatui) for remote stuff
