use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use jilebi_types::manifest::Manifest;
use rusqlite::Connection;

pub fn generate_path(base_path: &Path, new_folder: &str, is_dir: bool) -> Result<PathBuf, String> {
    let path = base_path.join(Path::new(new_folder));
    if !path.exists() && is_dir {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    Ok(path)
}

pub fn get_plugin_manifest(
    plugin_directory: &PathBuf,
    plugin_name: &str,
) -> Result<Manifest, String> {
    let manifest_path = plugin_directory.join(plugin_name).join("manifest.toml");
    if !manifest_path.exists() {
        return Err(format!(
            "Plugin manifest not found at path: {}",
            manifest_path.display()
        ));
    }
    let manifest_str = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Could not read manifest file: {}", e.to_string()))?;
    let manifest_toml = toml::from_str::<toml::Value>(&manifest_str).map_err(|e| {
        format!(
            "Could not parse the plugin manifest toml file: {}",
            e.to_string()
        )
    })?;
    let manifest = Manifest::try_from(manifest_toml).map_err(|e| {
        format!(
            "Could not convert toml to Manifest struct: {}",
            e.to_string()
        )
    })?;
    Ok(manifest)
}

pub fn load_plugins(db: &Connection) -> Result<HashMap<String, Manifest>, String> {
    tracing::trace!("Loading plugins from DB");
    let mut plugins_query = db
        .prepare("SELECT name, path FROM plugins WHERE state = 'enabled'")
        .map_err(|e| e.to_string())?;
    let rows = plugins_query
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let path: String = row.get(1)?;
            Ok((name, PathBuf::from(path)))
        })
        .map_err(|err| {
            tracing::error!("An error occurred while querying for plugins: {}", err);
            format!("An error occurred while querying for plugins: {}", err)
        })?;
    let mut manifests: HashMap<String, Manifest> = HashMap::new();
    for row in rows {
        let (name, path) = row.map_err(|e| e.to_string())?;
        let manifest = get_plugin_manifest(&path, &name)?;
        manifests.insert(name, manifest);
    }
    Ok(manifests)
}

pub fn get_plugin_path(db: &Connection, plugin_name: &str) -> Result<PathBuf, String> {
    let mut statement = db
        .prepare("SELECT path FROM plugins WHERE name = ?1")
        .map_err(|e| e.to_string())?;
    let plugin_path: String = statement
        .query_row([plugin_name], |row| row.get(0))
        .map_err(|e| {
            tracing::error!(
                "Could not get plugin path for plugin {}: {}",
                plugin_name,
                e.to_string()
            );
            format!(
                "Could not get plugin path for plugin {}: {}",
                plugin_name,
                e.to_string()
            )
        })?;
    Ok(PathBuf::from(plugin_path).join(plugin_name))
}
