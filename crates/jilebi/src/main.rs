#![deny(unused_crate_dependencies)]
use std::path::{Path, PathBuf};
mod cli;
mod server;
mod utils;
mod db;
use clap::Parser;
use directories::ProjectDirs;
use rmcp::{ServiceExt, transport::stdio};
use rusqlite::Connection;
use server::JilebiMcpServer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{
    JilebiCli, application_context_command_handler, plugin_command_handler, read_log_file,
};

fn setup_database(dir: &Path) -> Result<Connection, String> {
    let db_path = utils::generate_path(dir, "jilebi.db3", false)?;
    let connection = Connection::open(db_path).map_err(|e| e.to_string())?;
    connection
        .execute_batch(
            "BEGIN;
		CREATE TABLE IF NOT EXISTS plugin_state (
			id TEXT NOT NULL,
			key TEXT NOT NULL,
			value TEXT NOT NULL,
			PRIMARY KEY (id, key)
		);
		CREATE TABLE IF NOT EXISTS plugin_env(
			id TEXT NOT NULL,
			type TEXT NOT NULL CHECK(type IN ('normal', 'secret')),
			env_name TEXT NOT NULL,
			schema TEXT NOT NULL,
			value TEXT NOT NULL,
			PRIMARY KEY (id, env_name)
		);
		CREATE TABLE IF NOT EXISTS plugin_permissions(
			id TEXT NOT NULL,
			resource_name TEXT NOT NULL,
			value TEXT NOT NULL,
			PRIMARY KEY (id, resource_name)
		);
		CREATE TABLE IF NOT EXISTS plugins(
			name TEXT NOT NULL,
			path TEXT NOT NULL,
			version TEXT NOT NULL,
			origin TEXT NOT NULL CHECK(origin IN ('local', 'jilebi')),
			state TEXT NOT NULL CHECK(state IN ('enabled', 'disabled')),
			date_installed TEXT NOT NULL,
			last_updated TEXT NOT NULL,
			PRIMARY KEY (name)
		);
		CREATE TABLE IF NOT EXISTS application_contexts(
		    name TEXT NOT NULL PRIMARY KEY,
			plugins TEXT NOT NULL
		);
		COMMIT;",
        )
        .map_err(|e| e.to_string())?;
    Ok(connection)
}
#[tokio::main]
async fn main() -> Result<(), String> {
    dotenvy::dotenv().unwrap_or_default();

    let base_jilebi_dir = ProjectDirs::from("ai", "jilebi", "jilebi-server")
        .ok_or(String::from("Could not create ProjectDirs struct"))?;

    let log_folder_path = utils::generate_path(base_jilebi_dir.data_dir(), "logs", true)?;
    let log_file_path = utils::generate_path(Path::new(&log_folder_path), "jilebi.logs", false)?;

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

    let default_plugin_path = utils::generate_path(base_jilebi_dir.data_dir(), "plugins", true)?;

    let plugin_directory = dotenvy::var("PLUGIN_DIR")
        .map(PathBuf::from)
        .unwrap_or(default_plugin_path);

    let jilebi_cli = JilebiCli::parse();
    let current_version = env!("CARGO_PKG_VERSION");

    let db = setup_database(base_jilebi_dir.data_dir())?;
    match jilebi_cli.subcommand {
        cli::SubCommands::Stdio { name } => {
            tracing::info!("Starting Jilebi Server...");
            let plugins = utils::load_plugins(&db, name)?;
            let db_path = utils::generate_path(base_jilebi_dir.data_dir(), "jilebi.db3", false)?;
            let pool = r2d2_sqlite::SqliteConnectionManager::file(db_path);
            let db = r2d2::Pool::new(pool).map_err(|e| e.to_string())?;
            let main_span = tracing::span!(tracing::Level::INFO, "Jilebi Server started");
            let _guard = main_span.enter();
            tracing::trace!(?plugins, "Plugins and manifests loaded");
            let server = JilebiMcpServer::new(plugins, db, log_path, current_version);

            let service = server.serve(stdio()).await.map_err(|e| {
                tracing::error!("Serving error: {:?}", e);
                e.to_string()
            })?;
            service.waiting().await.map_err(|e| e.to_string())?;
            Ok(())
        }
        cli::SubCommands::Plugins { subcommand } => {
            plugin_command_handler(subcommand, &log_file, &plugin_directory, db).await
        }
        cli::SubCommands::Context { subcommand } => {
            application_context_command_handler(subcommand, &db).await
        }
        cli::SubCommands::Log => {
            read_log_file(&log_file);
            Ok(())
        }
        cli::SubCommands::Version => {
            println!("Jilebi version: {}", current_version);
            Ok(())
        }
    }
}

// TOP PRIORITY
// TODO: update plugin downloading to use versions
// TODO: self update jilebi automatically when a new version is released
// TODO: Let all CLI functionality be done via REST APIs
// TODO: support SSE, HTTP and Authentication
// TODO: update docs
// TODO: add a frontend
// 			- plugin store
// 			- login
// 			- management dashboard
// 			- API keys
// 			- show plugin logs and configs in the UI
// 			- payments

// TODO: update plugins without restarting the server
// TODO: check resources and add support for args (resource templates)
// Jilebi MVP
// MCP Server
// TODO: Error handling
// TODO: add pagination, autocomplete,  support
// TODO: Add regex support to permissions
// TODO: Add `jilebi plugin publish` publish a plugin, everything is public for now
// TODO: Write 10 most popular MCPs as plugins
//			- Exa search -> important
//			- Git
//			- Playwright = https://github.com/microsoft/playwright-mcp -> important
//			- https://github.com/awslabs/mcp/tree/main/src/aws-documentation-mcp-server
//			- https://github.com/abhiemj/manim-mcp-server

// installing plugins + CLI
// TODO: let folks specify mc plugins from github or file system or URL

// Beyond MVP
// plugin management

// UI
// TODO: think about a UI (leptos) and TUI over ssh (ratatui) for remote stuff in the CLI
// plugins
