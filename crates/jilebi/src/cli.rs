use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    time::Duration,
};
pub(crate) mod download;
pub(crate) mod env;
pub(crate) mod permissions;
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Select};
use directories::UserDirs;
use indicatif::ProgressBar;
use jilebi_types::env::PluginEnv;
use keyring::Entry;
use rusqlite::Connection;

use crate::cli::{download::remove_plugin_to_toml, env::setup_envs};

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
    /// setup a plugin in development after you've changed the permissions, envs or other manifest details
    Setup { id: String },
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
            )
            .await
            .map_err(|e| {
                tracing::error!("Could not create plugin due to {}", e);
                bar.abandon_with_message(format!("Failed to create plugin: {}", e));
                e
            })?;
            let local_plugin_path = plugin_dir.join(&plugin_name);
            #[cfg(unix)]
            symlink(&new_plugin_path, &local_plugin_path).map_err(|e| e.to_string())?;

            #[cfg(windows)]
            symlink_dir(&new_plugin_path, &local_plugin_path).map_err(|e| e.to_string())?;

            download::add_plugin_to_toml(plugin_dir, &local_plugin_path)?;
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

            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Setting up plugin...");
            env::setup_envs(&db, plugin_dir, &id)?;
            permissions::setup_permissions(&db, plugin_dir, &id, None)?;
            bar.finish_with_message(format!("Successfully set up {id}"));
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
            let envs = env::fetch_envs_from_db(&db, &id)?;
            let mut env_names = envs
                .iter()
                .map(|env| &env.env_name)
                .collect::<Vec<&String>>();

            let all = "All".to_string();
            env_names.push(&all);
            let env_selection = Select::new()
                .with_prompt("choose an ENV to update")
                .items(&env_names)
                .interact()
                .map_err(|e| e.to_string())?;

            if env_names[env_selection] == &all {
                setup_envs(&db, plugin_dir, &id)?;
            } else {
                let plugin_env = env::fetch_env(&db, &env_names[env_selection], &id)?;
                let new_value = env::query_env_from_the_user(&plugin_env)?;
                let entry =
                    Entry::new("jilebi", &plugin_env.env_name).map_err(|e| e.to_string())?;
                let new_env = PluginEnv::new(
                    plugin_env.env_name.clone(),
                    new_value,
                    plugin_env.env_type.clone(),
                    plugin_env.schema.clone(),
                );
                env::set_env(&entry, &db, &id, new_env)?;
            }
            Ok(())
        }
        PluginSubCommands::Permissions { id } => {
            let permission_requirements = permissions::fetch_permissions_for_plugin(&db, &id)?;

            let mut potential_entities = permission_requirements.keys().collect::<Vec<&String>>();

            let all_option = "All".to_string();
            potential_entities.push(&all_option);

            let entity_name = Select::new()
                .with_prompt("Select entity")
                .items(&potential_entities)
                .interact()
                .map_err(|e| e.to_string())?;
            let selected_entity = potential_entities[entity_name];

            if selected_entity == &all_option {
                permissions::setup_permissions(
                    &db,
                    plugin_dir,
                    &id,
                    Some(permission_requirements),
                )?;
            } else {
                let permissions = permission_requirements.get(selected_entity).ok_or(format!(
                    "Could not find permissions for entity {}",
                    selected_entity
                ))?;
                let new_permissions =
                    permissions::query_permissions_from_the_user(selected_entity, permissions)?;
                permissions::set_permissions(&db, &id, selected_entity, &new_permissions)?;
            };
            Ok(())
        }
        PluginSubCommands::Setup { id } => {
            env::setup_envs(&db, plugin_dir, &id)?;
            permissions::setup_permissions(&db, plugin_dir, &id, None)?;
            Ok(())
        }
    }
}
