use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
mod cli;
mod server;
use clap::Parser;
use directories::ProjectDirs;
use jilebi_types::plugin::Manifest;
use rmcp::{ServiceExt, transport::stdio};
use rusqlite::Connection;
use server::JilebiMcpServer;
use tracing::{Level, error, event, info, span};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{JilebiCli, plugin_command_handler, read_log_file};

const PLUGIN_MANIFEST_FORMAT: &str = r#"
manifest = []
"#;

fn generate_path(base_path: &Path, new_folder: &str, is_dir: bool) -> Result<PathBuf, String> {
    let path = base_path.join(Path::new(new_folder));
    if !path.exists() && is_dir {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    Ok(path)
}

fn load_plugins(dir: &Path) -> Result<(PathBuf, HashMap<String, Manifest>), String> {
    let default_plugin_path = generate_path(dir, "plugins", true)?;

    let plugin_directory = dotenvy::var("PLUGIN_DIR")
        .map(PathBuf::from)
        .unwrap_or(default_plugin_path);
    info!("Plugin directory being used -> {:?}", plugin_directory);
    let plugin_toml_path = plugin_directory.join(Path::new("plugins.toml"));
    if !plugin_toml_path.exists() {
        fs::write(&plugin_toml_path, PLUGIN_MANIFEST_FORMAT).map_err(|e| e.to_string())?;
    }
    let plugin_toml = fs::read_to_string(plugin_toml_path).map_err(|e| {
        format!(
            "Failed while loading plugins from dir -> {:?}, error -> {}",
            plugin_directory,
            e.to_string()
        )
    })?;
    let plugin_toml = toml::from_str::<toml::Value>(&plugin_toml).map_err(|e| {
        format!(
            "Could not parse the plugins toml directory manifest {}",
            e.to_string()
        )
    })?;
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
                    let manifest_pathbuf = plugin_directory.join(manifest_path);
                    tracing::info!("Loading manifest file from path: {:?}", manifest_pathbuf);
                    let manifest =
                        fs::read_to_string(manifest_pathbuf).expect("Manifest file not found");
                    let manifest = toml::from_str::<toml::Value>(&manifest)
                        .expect("Could not parse toml file");
                    tracing::info!("Toml file loaded for path {manifest_path}: {manifest:#?}");
                    Manifest::try_from(manifest).map_err(|e| error!(e)).ok()
                })
                .expect(format!("Could not parse manifest file {:?}", m).as_str());
            (manifest.name.clone(), manifest)
        })
        .collect::<HashMap<String, Manifest>>();
    Ok((plugin_directory, manifests))
}

fn setup_database(dir: &Path) -> Result<(), String> {
    let db_path = generate_path(dir, "jilebi.db3", false)?;
    let connection = Connection::open(db_path).map_err(|e| e.to_string())?;
    connection.execute_batch(
		"BEGIN;
		CREATE TABLE IF NOT EXISTS plugin_state (id TEXT NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, PRIMARY KEY (id, key));
		COMMIT;"
	).map_err(|e| e.to_string())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenvy::dotenv()
        .map_err(|e| format!("error while loading the dotenv file: {}", e.to_string()))
        .unwrap_or_default();

    let base_jilebi_dir = ProjectDirs::from("ai", "jilebi", "jilebi-server")
        .ok_or(String::from("Could not create ProjectDirs struct"))?;

    let log_folder_path = generate_path(base_jilebi_dir.data_dir(), "logs", true)?;
    let log_file_path = generate_path(Path::new(&log_folder_path), "jilebi.log", false)?;

    let log_path = dotenvy::var("LOG_PATH")
        .map(PathBuf::from)
        .unwrap_or(log_folder_path);
    let log_file = dotenvy::var("LOG_FILE")
        .map(PathBuf::from)
        .unwrap_or(log_file_path);

    let file_appender = tracing_appender::rolling::never(&log_path, &log_file);
    let (non_blocking_log_writer, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking_log_writer))
        .with(EnvFilter::from_default_env())
        .init();

    let (dir, plugins) = load_plugins(base_jilebi_dir.data_dir())?;
    let jilebi_cli = JilebiCli::parse();

    match jilebi_cli.subcommand {
        cli::SubCommands::Stdio => {
            setup_database(base_jilebi_dir.data_dir())?;

            let main_span = span!(Level::INFO, "Jilebi Server started");
            info!("Starting Jilebi Server, loading plugins...");
            let _guard = main_span.enter();

            event!(Level::INFO, ?plugins, "Plugins and manifests loaded");
            let server = JilebiMcpServer::new(plugins, dir, log_path);

            let service = server.serve(stdio()).await.map_err(|e| {
                tracing::error!("Serving error: {:?}", e);
                e.to_string()
            })?;
            service.waiting().await.map_err(|e| e.to_string())?;
            Ok(())
        }
        cli::SubCommands::Plugins { subcommand } => {
            plugin_command_handler(subcommand, &log_file, &dir).await
        }
        cli::SubCommands::Log => {
            read_log_file(&log_file);
            Ok(())
        }
    }
}

// TOP PRIORITY
// TODO: support SSE, HTTP and Authentication
// TODO: Pass ENVs and secrets through env variables (needed for github)
// TODO: Let users also specify permissions
// TODO: add compile time flags for using postgres (saas) vs sqlite (stdio)
// TODO: Document it all with website + docusaurus
// TODO: Finish plugin store + login + management dashboard + API keys + payments

// Jilebi MVP
// MCP Server
// TODO: Error handling
// TODO: support Resource Template
// TODO: add pagination support
// plugins
// TODO: Add regex support to permissions
// TODO: Add `jilebi plugin publish` publish a plugin, everything is public for now
// TODO: Write 10 most popular MCPs as plugins
//			- Exa search -> important
//			- github -> https://github.com/github/github-mcp-server
//			- Fetch -> important
//			- Git
//			- Playwright = https://github.com/microsoft/playwright-mcp -> important
//			- https://github.com/awslabs/mcp/tree/main/src/aws-documentation-mcp-server
//			- https://github.com/abhiemj/manim-mcp-server

// installing plugins + CLI
// TODO: let folks specify mc plugins from github or file system or URL

// Beyond MVP
// plugin management
// TODO: make it easy to manage with a store
// UI
// TODO: think about a UI (leptos) and TUI over ssh (ratatui) for remote stuff in the CLI
// plugins
// TODO: implement the permissions module
// - Let users choose which resources, prompts and tools can be shown to the llm
// - Let users choose the permissions allowed - which files, directories can be accessed
// - Let users choose the permissions allowed - which domains can be hit
// TODO: show plugin logs and configs in the UI
