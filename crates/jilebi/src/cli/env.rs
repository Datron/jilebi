use std::{collections::HashSet, fs, path::PathBuf};

use jilebi_types::env::{EnvType, PluginEnv, PluginEnvs};
use keyring::Entry;
use rusqlite::Connection;

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

pub fn fetch_envs(db: &Connection, id: &str) -> Result<PluginEnvs, String> {
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

/// Get the env definition from the plugin's manifest
pub fn get_env_type_definition(
    jilebi_plugin_dir: &PathBuf,
    id: &str,
    env_name: &str,
) -> Result<toml::Value, String> {
    let toml_path = jilebi_plugin_dir.join(id).join("manifest.toml");
    let toml = fs::read_to_string(&toml_path).map_err(|e| e.to_string())?;
    let toml = toml::from_str::<toml::Value>(&toml).map_err(|e| {
        format!(
            "Could not parse the plugins toml directory manifest {}",
            e.to_string()
        )
    })?;
    let envs = toml
        .get("env")
        .and_then(|env_object| env_object.as_table())
        .cloned()
        .ok_or(format!(
            "Could not find env declaration in {} in plugin {}",
            env_name, id
        ))?;

    let secrets = toml
        .get("secrets")
        .and_then(|env_object| env_object.as_table())
        .cloned()
        .ok_or(format!(
            "Could not find secrets declaration in {} in plugin {}",
            env_name, id
        ))?;

    match (envs.get(env_name), secrets.get(env_name)) {
        (Some(_), Some(_)) => Err(format!(
            "Env name {} is declared as both normal and secret in plugin {}",
            env_name, id
        )),
        (Some(env), None) => Ok(env.clone()),
        (None, Some(secret)) => Ok(secret.clone()),
        _ => Err(format!(
            "Could not find env declaration in {} in plugin {}",
            env_name, id
        )),
    }
}
