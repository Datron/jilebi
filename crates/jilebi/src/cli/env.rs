use std::{collections::HashSet, fs, path::PathBuf};

use dialoguer::Input;
use jilebi_types::env::{EnvType, PluginEnv, PluginEnvs};
use keyring::Entry;
use rusqlite::Connection;
use serde_json::json;

use crate::db;

fn toml_to_plugin_env(
    env_name: &str,
    properties: &toml::Value,
    env_type: EnvType,
) -> Result<PluginEnv, String> {
    let schema = properties
        .get("schema")
        .and_then(|schema| json!(schema).to_string().into())
        .ok_or(format!(
            "Could not find schema for env {} in manifest",
            env_name
        ))?;
    tracing::info!("Found env {} in manifest with schema: {}", env_name, schema);
    let default_value = properties
        .get("default")
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string();
    Ok(PluginEnv::new(
        env_name.to_string(),
        default_value,
        env_type,
        schema,
    ))
}

pub fn query_env_from_the_user(env: &PluginEnv) -> Result<String, String> {
    let new_value = Input::new()
        .with_prompt(format!("Enter value for env {}", env.env_name))
        .validate_with(|input: &String| -> Result<(), &str> {
            let schema_json = serde_json::from_str::<serde_json::Value>(&env.schema).map_err(|e| {
				tracing::error!("Failed to parse the schema associated with this env: {}", e);
				"Could not parse the schema associated with this env, please contact the plugin developer"
			})?;
            let schema_type = schema_json
                .get("type")
                .and_then(|t| t.as_str())
                .ok_or("Schema does not have a type field, please contact the plugin developer")?;
            let val = match schema_type {
                "number" => input.parse::<f64>().map(|i| json!(i)).map_err(|_| {
                    tracing::error!("Expected a number but got a string");
                    "Expected a number"
                })?,
                "integer" => input.parse::<i64>().map(|i| json!(i)).map_err(|_| {
                    tracing::error!("Expected an integer but got a string");
                    "Expected an integer"
                })?,
                "boolean" => input.parse::<bool>().map(|i| json!(i)).map_err(|_| {
                    tracing::error!("Expected a bool but got a string");
                    "Expected a number"
                })?,
                _ => json!(input),
            };
            jsonschema::validate(&schema_json, &val).map_err(|e| {
                tracing::error!("The env value does not conform to the schema: {}", e);
                "Invalid value according to the schema, please try again"
            })
        })
        .default(env.value.clone())
        .interact()
        .map_err(|e| e.to_string())?;
    Ok(new_value)
}

pub fn get_envs_from_manifest(
    jilebi_plugin_dir: &PathBuf,
    plugin_name: &str,
) -> Result<PluginEnvs, String> {
    let toml_path = jilebi_plugin_dir.join(plugin_name).join("manifest.toml");
    let toml = fs::read_to_string(&toml_path).map_err(|e| e.to_string())?;
    let manifest = toml::from_str::<toml::Value>(&toml).map_err(|e| {
        format!(
            "Could not parse the plugins toml directory manifest {}",
            e.to_string()
        )
    })?;
    let mut all_envs: PluginEnvs = HashSet::new();

    if let Some(envs) = manifest
        .get("env")
        .and_then(|env_object| env_object.as_table())
    {
        for (env_name, properties) in envs.iter() {
            tracing::info!(
                "Found env {} in manifest with properties: {:?}",
                env_name,
                properties
            );
            let env: PluginEnv = toml_to_plugin_env(env_name, properties, EnvType::Normal)?;
            all_envs.insert(env);
        }
    }

    if let Some(secrets) = manifest
        .get("secrets")
        .and_then(|env_object| env_object.as_table())
    {
        for (env_name, properties) in secrets.iter() {
            let env: PluginEnv = toml_to_plugin_env(env_name, properties, EnvType::Secret)?;
            all_envs.insert(env);
        }
    }

    Ok(all_envs)
}

pub fn setup_envs(
    db: &Connection,
    jilebi_plugin_dir: &PathBuf,
    plugin_name: &str,
) -> Result<(), String> {
    let envs = get_envs_from_manifest(jilebi_plugin_dir, plugin_name)?;
    if envs.is_empty() {
        tracing::info!("No envs to setup for plugin {}", plugin_name);
        return Ok(());
    }
    for env in envs.iter() {
        let new_value = query_env_from_the_user(env)?;
        let entry = Entry::new("jilebi", &env.env_name).map_err(|e| e.to_string())?;
        let new_env = PluginEnv::new(
            env.env_name.clone(),
            new_value,
            env.env_type.clone(),
            env.schema.clone(),
        );
        db::plugin_env::set_env(&entry, db, plugin_name, new_env)?;
    }
    Ok(())
}
