use std::{fs, path::Path};
mod server;
use jilebi_types::plugin::Manifest;
use rmcp::{ServiceExt, transport::stdio};
use server::JilebiMcpServer;

#[tokio::main]
async fn main() -> Result<(), String> {
    let toml_file_path =
        Path::new("/home/kartik/jilebi/examples/ts-simple-computer-use/manifest.toml");
    let manifest = fs::read_to_string(toml_file_path).expect("Manifest file not found");
    let manifest = toml::from_str::<toml::Value>(&manifest).expect("Could not parse toml file");
    let manifest = Manifest::try_from(manifest)?;

    // println!("{:#?}", manifest);
    let mut server = JilebiMcpServer::new();
    server.add(manifest).await;

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
// TODO: write logs to a file for debugging
// TODO: Error handling
// TODO: add pagination support
// plugins
// TODO: validate names to not include _
// TODO: support dynamic file loads (I've hardcoded the path to the JS file)
// TODO: support async functions
// TODO: implement custom logging functions
// TODO: implement custom file read/write functions
// TODO: implement custom network functions

// TODO: make it easy to manage with a store
// TODO: let folks specify mc plugins from github or file system or URL

// Beyond MVP
// UI
// TODO: think about a UI (leptos) and TUI over ssh (ratatui) for remote stuff
