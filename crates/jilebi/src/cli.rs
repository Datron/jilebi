use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Duration,
};
pub(crate) mod download;
pub(crate) mod env;
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Select};
use directories::UserDirs;
use indicatif::ProgressBar;
use jilebi_types::env::{EnvType, PluginEnv};
use keyring::Entry;
use rusqlite::Connection;

use strum::IntoEnumIterator;

use crate::cli::download::remove_plugin_to_toml;

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
use std::os::windows::fs::symlink_dir;

const MANIFEST_CODE: &str = r#"
name = "<replace>"
version = "1.0.0"
homepage = ""
creator = ""
contact = ""

[resources]

[prompts]

[tools]
"#;

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
    /// Manage environment variables for a specific plugin
    Env { id: String },
    /// Manage permissions for a specific plugin
    Permissions { id: String },
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
    db: Connection,
) -> Result<(), String> {
    match subcommand {
        PluginSubCommands::Create => {
            let user_dirs = UserDirs::new().ok_or("Could not get user directories")?;
            let plugin_name = Input::<String>::new()
                .with_prompt("plugin name")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let manifest_code = MANIFEST_CODE.replace("<replace>", &plugin_name);
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
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Setting things up...");
            download::download_and_init_template(
                &plugin_name,
                &manifest_code,
                &PathBuf::from(&new_plugin_path),
                &language,
                complex_plugin,
                plugin_dir,
            )
            .await
            .map_err(|e| {
                tracing::error!("Could not create plugin due to {}", e);
                bar.abandon_with_message(format!("Failed to create plugin: {}", e));
                e
            })?;

            #[cfg(unix)]
            symlink(&new_plugin_path, plugin_dir.join(&plugin_name)).map_err(|e| e.to_string())?;

            #[cfg(windows)]
            symlink_dir(&new_plugin_path, plugin_dir.join(&plugin_name))
                .map_err(|e| e.to_string())?;
            bar.finish_with_message(format!(
                "Successfully created {} at {}",
                plugin_name, new_plugin_path
            ));

            Ok(())
        }
        PluginSubCommands::Add { id } => {
            let plugin_path = plugin_dir.join(&id);
            tracing::info!("Adding plugin at path: {}", plugin_path.display());
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Downloading plugin...");
            download::download_and_init_plugin(&id, &plugin_path, plugin_dir)
                .await
                .map_err(|e| {
                    tracing::error!("Could not download plugin due to {}", e);
                    bar.abandon_with_message(format!("Failed to download plugin: {}", e));
                    e
                })?;
            bar.finish_with_message(format!(
                "Successfully added {id} at {}. You can restart jilebi for it to show up.",
                plugin_path.display()
            ));
            Ok(())
        }
        PluginSubCommands::Remove { id } => {
            let confirmation = Confirm::new()
				    .with_prompt(format!("Are you sure you want to remove the plugin {id}? This will not delete any state it has set"))
				    .interact().map_err(|e| e.to_string())?;
            if !confirmation {
                return Ok(());
            }
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
        PluginSubCommands::Env { id } => {
            let env_name = Input::<String>::new()
                .with_prompt("env name")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let plugin_env = env::fetch_env(&db, &env_name, &id).unwrap_or_default();
            let plugin_defined_env = env::get_env_type_definition(plugin_dir, &id, &env_name)?;
            let schema = plugin_defined_env
                .get("schema")
                .and_then(|schema| serde_json::to_value(schema).ok())
                .ok_or(format!(
                    "Could not find env schema for env {} in plugin {}",
                    env_name, id
                ))?;
            let entry = Entry::new("jilebi", &env_name).map_err(|e| e.to_string())?;
            let new_env_value = if plugin_env.env_type == EnvType::Secret {
                let secret_value = entry.get_password().unwrap_or_default();
                Input::<String>::new()
                    .with_prompt("secret value")
                    .default(secret_value)
                    .interact_text()
                    .map_err(|e| e.to_string())?
            } else {
                let default_env = if plugin_env.value.is_empty() {
                    plugin_defined_env
                        .get("default")
                        .and_then(|def| def.as_str().map(|s| s.to_string()))
                        .unwrap_or_default()
                } else {
                    plugin_env.value
                };
                Input::<String>::new()
                    .with_prompt("env value")
                    .default(default_env)
                    .interact_text()
                    .map_err(|e| e.to_string())?
            };
            let items = EnvType::iter().collect::<Vec<_>>();
            let env_type_selection = Select::new()
                .with_prompt("What type of env is this?")
                .items(&items)
                .interact()
                .map_err(|e| e.to_string())?;

            jsonschema::validate(&schema, &serde_json::Value::String(new_env_value.clone()))
                .map_err(|e| {
                    tracing::error!("The env value does not conform to the schema: {}", e);
                    format!("The env value does not conform to the schema: {}", e)
                })?;
            env::set_env(
                &entry,
                &db,
                &id,
                PluginEnv::new(
                    env_name,
                    new_env_value,
                    items[env_type_selection].clone(),
                    schema.to_string(),
                ),
            )
            .map_err(|e| e.to_string())?;
            Ok(())
        }
        PluginSubCommands::Permissions { id: _ } => todo!(),
    }
}
