use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
mod download;
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use directories::UserDirs;

use crate::cli::download::remove_plugin_to_toml;

#[derive(Debug, Clone, Subcommand)]
pub enum SubCommands {
    /// Run jilebi as a process
    Stdio,
    /// Commands for managing plugins
    Plugins {
        #[command(subcommand)]
        subcommand: PluginSubCommands,
    },
    /// Read jilebi server logs
    Log,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PluginSubCommands {
    /// Create a new TS or JS plugin with some useful defaults
    Create,
    /// Add/Install a plugin to use with Jilebi
    Add { id: String },
    /// Remove a plugin from jilebi and delete its state
    Remove { id: String },
    /// Show logs for a specific plugin
    Log { id: String },
}

/// Jilebi MCP server and command line interface for plugin management
#[derive(Debug, Clone, Parser)]
pub struct JilebiCli {
    #[command(subcommand)]
    pub subcommand: SubCommands,
}

pub fn read_log_file(path: &PathBuf) {
    let log_file_content = File::open(path)
        .unwrap_or_else(|err| panic!("Failed to open log file at {}: {}", path.display(), err));
    let reader = BufReader::new(log_file_content);
    for log_line in reader.lines() {
        match log_line {
            Ok(line) => println!("{}", line),
            Err(err) => panic!("Error reading log line: {}", err),
        }
    }
}

pub async fn plugin_command_handler(
    subcommand: PluginSubCommands,
    log_file: &PathBuf,
    plugin_dir: &PathBuf,
) -> Result<(), String> {
    match subcommand {
        PluginSubCommands::Create => {
            let manifest_code = r#"
			name = "<replace>"
			version = "1.0.0"
			homepage = ""
			creator = ""
			contact = ""

			[resources]

			[prompts]

			[tools]
			"#;
            let user_dirs = UserDirs::new().ok_or("Could not get user directories")?;
            let plugin_name = Input::<String>::new()
                .with_prompt("plugin name")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let manifest_code = manifest_code.replace("<replace>", &plugin_name);
            let new_plugin_path = Input::<String>::new()
                .with_prompt("path for the plugin")
                .default(
                    user_dirs
                        .home_dir()
                        .join(&plugin_name)
                        .display()
                        .to_string(),
                )
                .interact_text()
                .map_err(|e| e.to_string())?;
            let languages = ["JavaScript", "TypeScript"];
            let language = Select::new()
                .with_prompt("Select programming language")
                .items(&languages)
                .interact()
                .map_err(|e| e.to_string())?;
            let language = languages[language].to_lowercase();
            let complex_plugin = Select::new()
                .with_prompt(
                    "Include Rollup for builds? Use this if you are writing a complex plugin",
                )
                .items(&["Yes", "No"])
                .interact()
                .map_err(|e| e.to_string())?
                == 0;
            download::download_and_init_template(
                &plugin_name,
                &manifest_code,
                &PathBuf::from(new_plugin_path),
                &language,
                complex_plugin,
                plugin_dir,
            )
            .await?;
            Ok(())
        }
        PluginSubCommands::Add { id } => {
            let plugin_path = plugin_dir.join(&id);
            tracing::info!("Adding plugin at path: {}", plugin_path.display());
            download::download_and_init_plugin(&id, &plugin_path, plugin_dir).await?;
            Ok(())
        }
        PluginSubCommands::Remove { id } => {
            let plugin_path = plugin_dir.join(&id);
            tracing::info!("Removing plugin at path: {}", plugin_path.display());
            if plugin_path.exists() {
                fs::remove_dir_all(plugin_path).map_err(|e| e.to_string())?;
            }
            remove_plugin_to_toml(plugin_dir, &id)
        }
        PluginSubCommands::Log { id } => {
            read_log_file(&log_file.parent().unwrap().join(format!("{}.logs", id)));
            Ok(())
        }
    }
}
