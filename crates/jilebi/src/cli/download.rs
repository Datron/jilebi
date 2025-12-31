use std::{fs, io::Write, path::PathBuf};

use rusqlite::Connection;

use crate::db;

const DOWNLOAD_URL: &str = "https://jilebi.ai/api/download";

async fn download(url: &str, download_path: &PathBuf) -> Result<(), String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;
    let mut file = fs::File::create(&download_path).map_err(|e| e.to_string())?;
    file.write_all(&response).map_err(|e| e.to_string())?;
    Ok(())
}

async fn download_and_extract(url: &str, extract_path: &PathBuf) -> Result<(), String> {
    download(url, extract_path).await?;
    let file = fs::File::open(&extract_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| format!("{e:?}"))?;
    zip.extract_unwrapped_root_dir(
        extract_path.parent().unwrap(),
        zip::read::root_dir_common_filter,
    )
    .map_err(|e| e.to_string())?;
    fs::remove_file(&extract_path).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn download_and_init_plugin(
    id: &str,
    plugin_path: &PathBuf,
    db: &Connection,
) -> Result<(), String> {
    let file_name = format!("{}", id);
    let plugin_path_str = plugin_path
        .parent()
        .map(|s| s.display().to_string())
        .ok_or("Invalid plugin path")?;
    let url = format!("{}/plugins/{}", DOWNLOAD_URL, file_name);
    tracing::debug!("Downloading plugin from URL: {}", url);
    if !plugin_path.exists() {
        fs::create_dir_all(&plugin_path).map_err(|e| e.to_string())?;
    }
    download_and_extract(&url, &plugin_path.join(&file_name)).await?;
    let manifest: toml::Value = toml::from_str(
        &fs::read_to_string(plugin_path.join("manifest.toml")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let version = manifest.get("version")
        .and_then(|v| v.as_str())
        .ok_or("Version not found in manifest")?;
    tracing::debug!("Downloaded plugin version: {}", version);
    db::plugins::add_plugin_to_db(db, id, &plugin_path_str, version)?;
    Ok(())
}

pub async fn download_and_init_template(
    plugin_name: &str,
    manifest_code: &str,
    new_plugin_path: &PathBuf,
    language: &str,
    complex_plugin: bool,
) -> Result<(), String> {
    let file_name = format!("{}.zip", language);
    let url = format!("{}/templates/{}", DOWNLOAD_URL, file_name);
    fs::create_dir_all(new_plugin_path).map_err(|e| e.to_string())?;
    download_and_extract(&url, &new_plugin_path.join(&file_name)).await?;
    if complex_plugin {
        let url = format!("{}/templates/rollup.config.js", DOWNLOAD_URL);
        download(&url, &new_plugin_path.join("rollup.config.js")).await?;
        fs::remove_file(new_plugin_path.join("package.json")).map_err(|e| e.to_string())?;
        fs::rename(
            new_plugin_path.join("package.rollup.json"),
            new_plugin_path.join("package.json"),
        )
        .map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(new_plugin_path.join("package.rollup.json")).map_err(|e| e.to_string())?;
    }
    let package_json =
        fs::read_to_string(new_plugin_path.join("package.json")).map_err(|e| e.to_string())?;
    let package_json = package_json.replace("replace", plugin_name);
    fs::write(new_plugin_path.join("package.json"), package_json).map_err(|e| e.to_string())?;
    fs::write(new_plugin_path.join("manifest.toml"), manifest_code).map_err(|e| e.to_string())?;
    Ok(())
}
