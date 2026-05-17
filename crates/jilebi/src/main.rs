#![deny(unused_crate_dependencies)]
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    time::Duration,
};
mod cli;
mod db;
mod server;
mod utils;
use axum::{
    Router,
    http::{HeaderName, HeaderValue, StatusCode},
    routing::get,
};
use clap::Parser;
use directories::ProjectDirs;
use indicatif::ProgressBar;
use rmcp::{
    ServiceExt,
    transport::{
        StreamableHttpServerConfig, stdio,
        streamable_http_server::{
            session::local::LocalSessionManager, tower::StreamableHttpService,
        },
    },
};
use rusqlite::Connection;
use server::JilebiMcpServer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    cli::{
        JilebiCli, application_context_command_handler, clear_log_file,
        download::download_and_replace_jilebi, plugin_command_handler, read_log_file,
    },
    utils::get_latest_release_version,
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

fn http_bind_addr() -> Result<SocketAddr, String> {
    let bind_address = dotenvy::var("HTTP_BIND_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = dotenvy::var("HTTP_PORT").unwrap_or("3000".to_string());
    format!("{bind_address}:{port}")
        .parse::<SocketAddr>()
        .map_err(|e| format!("Invalid HTTP bind address or port: {e}"))
}

fn mcp_path() -> Result<String, String> {
    let path = dotenvy::var("HTTP_MCP_PATH").unwrap_or("/mcp".to_string());
    if path.trim().is_empty() {
        return Err("MCP path cannot be empty".to_string());
    }
    Ok(if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    })
}

fn cors_env_values(name: &str, default: &str) -> Vec<String> {
    dotenvy::var(name)
        .unwrap_or(default.to_string())
        .split(",")
        .into_iter()
        .map(|value| value.to_string())
        .collect()
}

fn cors_layer() -> Result<CorsLayer, String> {
    let allowed_origins = cors_env_values("MCP_ALLOWED_ORIGINS", "*");

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers([HeaderName::from_static("mcp-session-id")]);

    if allowed_origins.iter().any(|value| value == "*") {
        return Ok(cors_layer.allow_origin(Any));
    }

    let origins = allowed_origins
        .into_iter()
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .map_err(|e| format!("Invalid CORS origin `{origin}`: {e}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if origins.is_empty() {
        Ok(cors_layer)
    } else {
        Ok(cors_layer.allow_origin(origins))
    }
}

async fn serve_http(server: JilebiMcpServer) -> Result<(), String> {
    let addr = http_bind_addr()?;
    let mcp_path = mcp_path()?;
    let cors_layer = cors_layer()?;
    let mcp_service: StreamableHttpService<JilebiMcpServer, LocalSessionManager> =
        StreamableHttpService::new(
            move || Ok(server.clone()),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default(),
        );

    let app = Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest_service(&mcp_path, mcp_service)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("Could not bind HTTP server on {addr}: {e}"))?;
    tracing::info!("Jilebi HTTP server listening on {}", addr);
    tracing::info!(
        "Jilebi MCP streamable HTTP endpoint mounted at {}",
        mcp_path
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            tracing::info!("Shutting down Jilebi HTTP server");
        })
        .await
        .map_err(|e| e.to_string())
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
    let current = semver::Version::parse(current_version).map_err(|e| e.to_string())?;
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
        cli::SubCommands::Http { name } => {
            tracing::info!("Starting Jilebi HTTP Server...");
            let plugins = utils::load_plugins(&db, name)?;
            let db_path = utils::generate_path(base_jilebi_dir.data_dir(), "jilebi.db3", false)?;
            let pool = r2d2_sqlite::SqliteConnectionManager::file(db_path);
            let db = r2d2::Pool::new(pool).map_err(|e| e.to_string())?;
            tracing::trace!(?plugins, "Plugins and manifests loaded");
            let server = JilebiMcpServer::new(plugins, db, log_path, current_version);
            serve_http(server).await
        }
        cli::SubCommands::Plugins { subcommand } => {
            plugin_command_handler(subcommand, &log_file, &plugin_directory, db).await
        }
        cli::SubCommands::Context { subcommand } => {
            application_context_command_handler(subcommand, &db).await
        }
        cli::SubCommands::Log { clear } => {
            if clear {
                clear_log_file(&log_file)
            } else {
                read_log_file(&log_file);
                Ok(())
            }
        }
        cli::SubCommands::Version => {
            println!(
                r#"
   $$$$$\ $$\ $$\           $$\       $$\
   \__$$ |\__|$$ |          $$ |      \__|
      $$ |$$\ $$ | $$$$$$\  $$$$$$$\  $$\
      $$ |$$ |$$ |$$  __$$\ $$  __$$\ $$ |
$$\   $$ |$$ |$$ |$$$$$$$$ |$$ |  $$ |$$ |
$$ |  $$ |$$ |$$ |$$   ____|$$ |  $$ |$$ |
\$$$$$$  |$$ |$$ |\$$$$$$$\ $$$$$$$  |$$ |
 \______/ \__|\__| \_______|\_______/ \__|

version: {}
"#,
                current_version
            );
            Ok(())
        }
        cli::SubCommands::Update => {
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("checking for updates...");
            let latest_version = get_latest_release_version().await?;
            let latest = semver::Version::parse(&latest_version).map_err(|e| e.to_string())?;
            if current < latest {
                bar.finish_and_clear();
                println!(
                    "A new version of Jilebi is available, downloading: {} -> {}",
                    current_version, latest_version
                );
                bar.enable_steady_tick(Duration::from_millis(100));
                bar.set_message("downloading latest version...");
                let target = match std::env::consts::OS {
                    "linux" => "unknown-linux-gnu",
                    "windows" => "pc-windows-msvc",
                    "macos" | "apple" => "apple-darwin",
                    other => {
                        bar.finish_and_clear();
                        return Err(format!("Unsupported OS: {}", other));
                    }
                };
                let arch = std::env::consts::ARCH;
                let file_name = format!("jilebi-{}-{}.zip", arch, target);
                download_and_replace_jilebi(&file_name).await?;
                bar.finish_and_clear();
                println!("Jilebi has been updated to version {}", latest_version);
            } else {
                bar.finish_and_clear();
                println!(
                    "You are already using the latest version of Jilebi: {}",
                    current_version
                );
            }
            Ok(())
        }
    }
}

// TOP PRIORITY
// TODO: Let all CLI functionality be done via REST APIs
// TODO: support SSE, HTTP and Authentication
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

// Blog post ideas
// - How to update your CLI remotely
// - What is wrong with current MCPs and how Jilebi solves them
// - How to develop a context server for zed
// - Developing a plugin for VS Code
// - Developing a plugin for opencode
