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

use crate::{cli::env::setup_envs, db};

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
    Stdio {
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Commands for managing application contexts
    Context {
        #[command(subcommand)]
        subcommand: ApplicationContextCommands,
    },
    /// Commands for managing plugins
    Plugins {
        #[command(subcommand)]
        subcommand: PluginSubCommands,
    },
    /// Read jilebi server logs
    Log {
        #[arg(short, long)]
        clear: bool,
    },
    /// Get the current version of jilebi
    Version,
    /// Update jilebi
    Update,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PluginSubCommands {
    /// Create a new TS or JS plugin with some useful defaults
    Create {
        /// name of the plugin
        #[arg(short, long)]
        name: Option<String>,
        /// path or directory to create the plugin in
        #[arg(short, long)]
        path: Option<String>,
        /// programming language to use for the plugin - Javascript or TypeScript (use these values)
        #[arg(short, long)]
        language: Option<String>,
        /// whether to include Rollup for complex plugin builds
        #[arg(short, long)]
        complex_plugin: Option<bool>,
    },
    /// Add/Install a plugin to use with Jilebi
    Add {
        ids: Vec<String>,
        #[arg(short, long)]
        accept_permissions: bool,
        #[arg(short, long)]
        version: Option<String>,
    },
    /// setup a plugin in development after you've changed the permissions, envs or other manifest details
    Setup {
        id: String,
        #[arg(short, long)]
        accept_permissions: bool,
    },
    /// List plugins
    List {
        #[arg(short, long)]
        remote: bool,
    },
    /// Enable a plugin and its state, it will start showing up in jilebi
    Enable { id: String },
    /// Disable a plugin, it will stop showing up in jilebi but its state will be preserved
    Disable { id: String },
    /// Remove a plugin from jilebi and delete its state
    Remove { id: String },
    /// Manage environment variables for a specific plugin
    Env { id: String },
    /// Manage permissions for a specific plugin
    Permissions {
        id: String,
        #[arg(short, long)]
        accept_permissions: bool,
    },
    /// Show logs for a specific plugin
    Log {
        id: String,
        #[arg(short, long)]
        clear: bool,
    },
    /// Update a particular plugin to a new version
    Update {
        id: String,
        #[arg(short, long)]
        version: Option<String>,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ApplicationContextCommands {
    /// Create a new application context
    Create { name: String, plugins: Vec<String> },
    /// Add a plugin to existing application context
    Add { name: String, plugin: String },
    /// Remove a plugin from existing application context
    Remove { name: String, plugin: String },
    /// Delete an existing application context
    Delete { name: String },
    /// List all application contexts
    List,
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

pub fn clear_log_file(path: &PathBuf) -> Result<(), String> {
    fs::write(path, "").map_err(|e| format!("Failed to clear log file: {}", e))
}

pub async fn list_remote_plugins() -> Result<Vec<(String, String, String)>, String> {
    let client = reqwest::Client::new();
    let plugins = client
        .get("https://jilebi.ai/api/plugins")
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch remote plugins: {}", e);
            "Failed to fetch remote plugins".to_string()
        })?
        .json::<Vec<serde_json::Value>>()
        .await
        .map_err(|e| {
            tracing::error!("Failed to parse remote plugins JSON: {}", e);
            "Failed to parse remote plugins JSON".to_string()
        })?;
    let plugins = plugins
        .into_iter()
        .filter_map(|plugin| {
            let name = plugin.get("name")?.as_str()?.to_string();
            let version = plugin.get("version")?.as_str()?.to_string();
            let download_count = plugin.get("download_count")?.as_i64()?.to_string();
            Some((name, version, download_count))
        })
        .collect::<Vec<(String, String, String)>>();
    Ok(plugins)
}

pub async fn plugin_command_handler(
    subcommand: PluginSubCommands,
    log_file: &PathBuf,
    plugin_dir: &PathBuf,
    db: Connection,
) -> Result<(), String> {
    match subcommand {
        PluginSubCommands::Create {
            name,
            path,
            complex_plugin,
            language,
        } => {
            let user_dirs = UserDirs::new().ok_or("Could not get user directories")?;
            let plugin_name = match name {
                Some(n) => n,
                None => Input::<String>::new()
                    .with_prompt("name of the new plugin")
                    .interact_text()
                    .map_err(|e| e.to_string())?,
            };
            let manifest_code = MANIFEST_CODE.replace("<replace>", &plugin_name);
            let new_plugin_path = match path {
                Some(p) => p,
                None => Input::<String>::new()
                    .with_prompt("path for the plugin, do not include the name")
                    .default(user_dirs.home_dir().display().to_string())
                    .interact_text()
                    .map_err(|e| e.to_string())?,
            };
            let languages = ["JavaScript", "TypeScript"];
            let language = match language {
                Some(l) => languages
                    .iter()
                    .position(|&lang| lang.to_lowercase() == l.to_lowercase())
                    .ok_or("Invalid language selected")?,
                None => Select::new()
                    .with_prompt("Select programming language")
                    .items(&languages)
                    .interact()
                    .map_err(|e| e.to_string())?,
            };
            let language = languages[language].to_lowercase();
            let complex_plugin = match complex_plugin {
                Some(c) => c,
                None => Select::new()
                    .with_prompt(
                        "Include Rollup for builds? Use this if you are writing a complex plugin",
                    )
                    .items(&["Yes", "No"])
                    .interact()
                    .map_err(|e| e.to_string())?
                    == 0,
            };
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Setting things up...");
            download::download_and_init_template(
                &plugin_name,
                &manifest_code,
                &PathBuf::from(&new_plugin_path).join(&plugin_name),
                &language,
                complex_plugin,
            )
            .await
            .map_err(|e| {
                tracing::error!("Could not create plugin due to {}", e);
                bar.abandon_with_message(format!("Failed to create plugin: {}", e));
                e
            })?;

            db::plugins::add_plugin_to_db(&db, &plugin_name, &new_plugin_path, "1.0.0")?;
            bar.finish_with_message(format!(
                "Successfully created {} at {}",
                plugin_name, new_plugin_path
            ));

            Ok(())
        }
        PluginSubCommands::Add {
            ids,
            accept_permissions,
            version: _,
        } => {
            for id in ids.iter() {
                let plugin_path = plugin_dir.join(&id);
                tracing::info!("Adding plugin at path: {}", plugin_path.display());
                let bar = ProgressBar::new_spinner();
                bar.enable_steady_tick(Duration::from_millis(100));
                bar.set_message("Downloading plugin...");
                download::download_and_init_plugin(&id, &plugin_path, &db)
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

                env::setup_envs(&db, plugin_dir, &id)?;
                permissions::setup_permissions(&db, plugin_dir, &id, None, accept_permissions)?;
            }
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
            db::plugins::remove_plugin_in_db(&db, &id)
        }
        PluginSubCommands::Log { id, clear } => {
            let file = log_file.parent().unwrap().join(format!("{}.logs", id));
            if clear {
                clear_log_file(&file)?;
            } else {
                read_log_file(&file);
            }
            Ok(())
        }
        PluginSubCommands::Env { id } => {
            let plugin = db::plugins::get_plugin(&db, &id)?;
            let envs = db::plugin_env::fetch_envs_from_db(&db, &id)?;
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
                setup_envs(&db, &plugin.path, &id)?;
            } else {
                let plugin_env = db::plugin_env::fetch_env(&db, &env_names[env_selection], &id)?;
                let new_value = env::query_env_from_the_user(&plugin_env)?;
                let entry =
                    Entry::new("jilebi", &plugin_env.env_name).map_err(|e| e.to_string())?;
                let new_env = PluginEnv::new(
                    plugin_env.env_name.clone(),
                    new_value,
                    plugin_env.env_type.clone(),
                    plugin_env.schema.clone(),
                );
                db::plugin_env::set_env(&entry, &db, &id, new_env)?;
            }
            Ok(())
        }
        PluginSubCommands::Permissions {
            id,
            accept_permissions,
        } => {
            let plugin = db::plugins::get_plugin(&db, &id)?;
            let permission_requirements =
                db::plugin_permissions::fetch_permissions_for_plugin(&db, &id)?;

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
                    &plugin.path,
                    &id,
                    Some(permission_requirements),
                    accept_permissions,
                )?;
            } else {
                let permissions = permission_requirements.get(selected_entity).ok_or(format!(
                    "Could not find permissions for entity {}",
                    selected_entity
                ))?;
                let new_permissions =
                    permissions::query_permissions_from_the_user(selected_entity, permissions)?;
                db::plugin_permissions::set_permissions(
                    &db,
                    &id,
                    selected_entity,
                    &new_permissions,
                )?;
            };
            Ok(())
        }
        PluginSubCommands::Setup {
            id,
            accept_permissions,
        } => {
            let plugin = db::plugins::get_plugin(&db, &id)?;
            env::setup_envs(&db, &plugin.path, &id)?;
            permissions::setup_permissions(&db, &plugin.path, &id, None, accept_permissions)?;
            Ok(())
        }
        PluginSubCommands::List { remote } => {
            let plugins = if remote {
                let bar = ProgressBar::new_spinner();
                bar.enable_steady_tick(Duration::from_millis(100));
                bar.set_message("Getting a list of plugins available...");
                let data = list_remote_plugins().await?;
                bar.finish();
                println!(
                    "{:<30} {:<10} {:<10}",
                    "Plugin Name", "Version", "Downloads"
                );
                data
            } else {
                println!("{:<30} {:<10} {:<10}", "Plugin Name", "Version", "State");
                let plugins = db::plugins::list_local_plugins(&db)?;
                plugins
                    .into_iter()
                    .map(|p| {
                        let state_str = p.state.to_string();
                        (p.name, p.version, state_str)
                    })
                    .collect::<Vec<(String, String, String)>>()
            };
            println!("{:-<60}", "");
            for (name, version, state) in plugins {
                let state_str = state.to_string();
                println!("{:<30} {:<10} {:<10}", name, version, state_str);
            }
            Ok(())
        }
        PluginSubCommands::Enable { id } => {
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Enabling plugin...");
            db::plugins::update_plugin_state(&db, &id, jilebi_types::PluginState::Enabled)?;
            bar.finish_with_message("Plugin Enabled");
            Ok(())
        }
        PluginSubCommands::Disable { id } => {
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Disabling plugin...");
            db::plugins::update_plugin_state(&db, &id, jilebi_types::PluginState::Disabled)?;
            bar.finish_with_message("Plugin Disabled");
            Ok(())
        }
        PluginSubCommands::Update { id, version: _ } => {
            let plugin_path = plugin_dir.join(&id);
            tracing::info!("Removing plugin at path: {}", plugin_path.display());
            if plugin_path.exists() {
                fs::remove_dir_all(&plugin_path).map_err(|e| e.to_string())?;
            }
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message("Downloading plugin...");
            download::download_and_init_plugin(&id, &plugin_path, &db)
                .await
                .map_err(|e| {
                    tracing::error!("Could not download plugin due to {}", e);
                    bar.abandon_with_message(format!("Failed to download plugin: {}", e));
                    e
                })?;
            bar.finish_with_message(format!("Successfully updated {id}"));
            Ok(())
        }
    }
}

pub async fn application_context_command_handler(
    subcommand: ApplicationContextCommands,
    db: &Connection,
) -> Result<(), String> {
    match subcommand {
        ApplicationContextCommands::Create { name, plugins } => {
            db::application_contexts::create_application_context(db, &name, &plugins)?;
            println!("Created context {}", name);
            Ok(())
        }
        ApplicationContextCommands::Add { name, plugin } => {
            db::application_contexts::add_plugin_to_application_context(db, &name, plugin.clone())?;
            println!("Plugin {} added to context {}", plugin, name);
            Ok(())
        }
        ApplicationContextCommands::Remove { name, plugin } => {
            db::application_contexts::remove_plugin_from_application_context(
                db,
                &name,
                plugin.clone(),
            )?;
            println!("Plugin {} removed from context {}", plugin, name);
            Ok(())
        }
        ApplicationContextCommands::Delete { name } => {
            db::application_contexts::delete_application_context(db, &name)?;
            println!("Deleted context {}", name);
            Ok(())
        }
        ApplicationContextCommands::List => {
            let contexts = db::application_contexts::list_application_contexts(db)?;
            println!("{:<30} {:<50}", "Context Name", "Plugins");
            println!("{:-<80}", "");
            for context in contexts {
                let plugins_str = context.plugins.join(", ");
                println!("{:<30} {:<50}", context.name, plugins_str);
            }
            Ok(())
        }
    }
}
