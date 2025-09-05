use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use jilebi_types::plugin::Manifest;

pub const PLUGIN_MANIFEST_FORMAT: &str = r#"
manifest = []
"#;

pub fn generate_path(base_path: &Path, new_folder: &str, is_dir: bool) -> Result<PathBuf, String> {
    let path = base_path.join(Path::new(new_folder));
    if !path.exists() && is_dir {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    Ok(path)
}

pub fn get_plugin_manifest(plugin_directory: &PathBuf, plugin_name: &str) -> Result<Manifest, String> {
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
	let manifest = Manifest::try_from(manifest_toml)
		.map_err(|e| format!("Could not convert toml to Manifest struct: {}", e.to_string()))?;
	Ok(manifest)
}

pub fn load_plugins(plugin_directory: &PathBuf) -> Result<HashMap<String, Manifest>, String> {
    tracing::info!("Plugin directory being used -> {:?}", plugin_directory);
    let plugin_toml_path = plugin_directory.join(Path::new("plugins.toml"));
    if !plugin_toml_path.exists() {
        fs::write(&plugin_toml_path, PLUGIN_MANIFEST_FORMAT).map_err(|e| e.to_string())?;
    }
    let plugin_toml = fs::read_to_string(plugin_toml_path).map_err(|e| {
        format!(
            "Failed while loading plugins from dir -> {:?}, error -> {}",
            plugin_directory,
            e.to_string()
        )
    })?;
    let plugin_toml = toml::from_str::<toml::Value>(&plugin_toml).map_err(|e| {
        format!(
            "Could not parse the plugins toml directory manifest {}",
            e.to_string()
        )
    })?;
    let manifests = plugin_toml
        .get("manifest")
        .and_then(|manifests| manifests.as_array())
        .ok_or(String::from(
            "Define an array for plugin manifests. Check the docs",
        ))?
        .into_iter()
        .map(|m| {
            let manifest = m
                .as_str()
                .and_then(|manifest_path| {
                    let manifest_pathbuf = plugin_directory.join(manifest_path);
                    tracing::info!("Loading manifest file from path: {:?}", manifest_pathbuf);
                    let manifest =
                        fs::read_to_string(manifest_pathbuf).expect("Manifest file not found");
                    let manifest = toml::from_str::<toml::Value>(&manifest)
                        .expect("Could not parse toml file");
                    tracing::info!("Toml file loaded for path {manifest_path}: {manifest:#?}");
                    Manifest::try_from(manifest)
                        .map_err(|e| tracing::error!(e))
                        .ok()
                })
                .expect(format!("Could not parse manifest file {:?}", m).as_str());
            (manifest.name.clone(), manifest)
        })
        .collect::<HashMap<String, Manifest>>();
    Ok(manifests)
}
