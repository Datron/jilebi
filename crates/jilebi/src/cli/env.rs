use std::{collections::HashSet, fs, path::PathBuf};

use dialoguer::Input;
use jilebi_types::env::{EnvType, PluginEnv, PluginEnvs};
use keyring::Entry;
use rusqlite::Connection;
use serde_json::json;

fn sql_to_plugin_env(row: &rusqlite::Row) -> Result<PluginEnv, rusqlite::Error> {
    let env_name: String = row.get(0)?;
    let env_type: String = row.get(2)?;
    let schema: String = row.get(3)?;
    let env_type = EnvType::try_from(env_type).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            rusqlite::types::Type::Text,
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
        )
    })?;
    let value: String = if env_type == EnvType::Normal {
        row.get(1)?
    } else {
        let entry = Entry::new("jilebi", &env_name).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            )
        })?;
        entry.get_password().unwrap_or_default()
    };
    Ok(PluginEnv::new(env_name, value, env_type, schema))
}

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
        set_env(&entry, db, plugin_name, new_env)?;
    }
    Ok(())
}

pub fn fetch_envs_from_db(db: &Connection, id: &str) -> Result<PluginEnvs, String> {
    let mut statement = db
        .prepare("SELECT env_name, value, type, schema FROM plugin_env WHERE id = ?")
        .map_err(|e| {
            tracing::error!(
                "Failed to prepare statement while fetching existing envs: {}",
                e
            );
            format!("Failed to query jilebis internal state: {}", e)
        })?;
    let rows = statement
        .query_and_then([id], sql_to_plugin_env)
        .map_err(|e| {
            tracing::error!("Failed to fetch existing envs: {}", e);
            format!("Failed to fetch existing envs: {}", e)
        })?;
    let mut envs = HashSet::new();
    for row in rows {
        envs.insert(row.map_err(|e| e.to_string())?);
    }
    Ok(envs)
}

pub fn fetch_env(db: &Connection, env_name: &str, id: &str) -> Result<PluginEnv, String> {
    let mut statement = db
        .prepare(
            "SELECT env_name, value, type, schema FROM plugin_env WHERE env_name = ? AND id = ?",
        )
        .map_err(|e| {
            tracing::error!(
                "Failed to prepare statement while fetching existing envs: {}",
                e
            );
            format!(
                "Failed to prepare statement while fetching existing envs: {}",
                e
            )
        })?;
    let row = statement
        .query_one([env_name, id], sql_to_plugin_env)
        .map_err(|e| e.to_string())?;
    Ok(row)
}

pub fn set_env(keystore: &Entry, db: &Connection, id: &str, env: PluginEnv) -> Result<(), String> {
    let mut statement = db
        .prepare("INSERT OR REPLACE INTO plugin_env (id, type, env_name, value, schema) VALUES (?1, ?2, ?3, ?4, ?5)")
        .map_err(|e| {
            tracing::error!("Failed to prepare statement while setting env: {}", e);
            format!("Failed to prepare statement while setting env: {}", e)
        })?;
    let env_value = if env.env_type == EnvType::Secret {
        keystore.set_password(&env.value).map_err(|e| {
            tracing::error!("Failed to set secret env value: {}", e);
            format!("Failed to set secret env value: {}", e)
        })?;
        "set-in-credential-manager"
    } else {
        &env.value
    };
    statement
        .execute([
            id,
            &env.env_type.to_string().to_lowercase(),
            &env.env_name,
            &env_value,
            &env.schema,
        ])
        .map_err(|e| {
            tracing::error!("Failed to execute statement while setting env: {}", e);
            format!("Failed to execute statement while setting env: {}", e)
        })?;
    Ok(())
}
