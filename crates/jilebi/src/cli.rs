use std::{
    collections::HashMap,
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
use jilebi_types::{
    env::{EnvType, PluginEnv},
    permissions::JilebiPermissions,
};
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
    /// TODO: Setup a plugin, if the plugin was installed manually
    // Setup { id: String },
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
        PluginSubCommands::Permissions { id } => {
            let manifest = crate::utils::get_plugin_manifest(plugin_dir, &id)?;
            let mut permission_requirements: HashMap<&String, &JilebiPermissions> = HashMap::new();
            for (name, resource) in manifest.resources.iter() {
                if let Some(ref perms) = resource.permissions {
                    permission_requirements.insert(name, perms);
                }
            }

            for (name, tool) in manifest.tools.iter() {
                if let Some(ref perms) = tool.permissions {
                    permission_requirements.insert(name, perms);
                }
            }

            let mut potential_entities = permission_requirements
                .keys()
                .map(|k| *k)
                .collect::<Vec<&String>>();

            let all_option = "All".to_string();
            potential_entities.push(&all_option);

            let entity_name = Select::new()
                .with_prompt("Select entity")
                .items(&potential_entities)
                .interact()
                .map_err(|e| e.to_string())?;
            let selected_entity = potential_entities[entity_name];

            let permission_query_closure = |entity: &String,
                                            permissions: &JilebiPermissions|
             -> Result<JilebiPermissions, String> {
                let hosts = if permissions.hosts.contains("user_defined") {
                    let user_defined: String = Input::new()
                        .with_prompt(format!(
                            "Enter hosts for {}, use a comma to separate multiple entries",
                            entity
                        ))
                        .interact_text()
                        .map_err(|e| e.to_string())?;
                    user_defined
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    permissions.hosts.clone()
                };
                let read_dirs = if permissions.read_dirs.contains("user_defined") {
                    let user_defined: String = Input::new()
						.with_prompt(format!("Enter directories that the plugin is allowed to read from for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
                    user_defined
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    permissions.read_dirs.clone()
                };
                let write_dirs = if permissions.write_dirs.contains("user_defined") {
                    let user_defined: String = Input::new()
						.with_prompt(format!("Enter directories that the plugin is allowed to write to for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
                    user_defined
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    permissions.write_dirs.clone()
                };

                let urls = if permissions.urls.contains("user_defined") {
                    let user_defined: String = Input::new()
						.with_prompt(format!("Enter URLs that the plugin is allowed to access for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
                    user_defined
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    permissions.read_dirs.clone()
                };

                let read_files = if permissions.read_files.contains("user_defined") {
                    let user_defined: String = Input::new()
						.with_prompt(format!("Enter files that the plugin is allowed to read from for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
                    user_defined
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    permissions.read_files.clone()
                };

                let write_files = if permissions.write_files.contains("user_defined") {
                    let user_defined: String = Input::new()
						.with_prompt(format!("Enter files that the plugin is allowed to write to for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
                    user_defined
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    permissions.write_files.clone()
                };
                Ok(JilebiPermissions {
                    hosts,
                    read_dirs,
                    write_dirs,
                    urls,
                    read_files,
                    write_files,
                })
            };

            if selected_entity == &all_option {
                for (entity, existing_permissions) in permission_requirements.iter() {
                    let new_permissions = permission_query_closure(entity, existing_permissions)?;
                    permissions::set_permissions(&db, &id, entity, &new_permissions)?;
                }
            } else {
                let permissions = permission_requirements.get(selected_entity).ok_or(format!(
                    "Could not find permissions for entity {}",
                    selected_entity
                ))?;
                let new_permissions = permission_query_closure(selected_entity, permissions)?;
                permissions::set_permissions(&db, &id, selected_entity, &new_permissions)?;
            };
            Ok(())
        }
    }
}
