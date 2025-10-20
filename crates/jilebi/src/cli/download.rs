use std::{fs, io::Write, path::PathBuf};

use chrono::Utc;
use jilebi_types::PluginOrigin;
use rusqlite::{Connection, params};

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

pub fn add_plugin_to_db(
    db: &Connection,
    plugin_name: &str,
    plugin_path: &str,
) -> Result<(), String> {
    let mut statement = db
        .prepare("INSERT OR REPLACE INTO plugins VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)")
        .map_err(|err| {
            tracing::error!("Could not insert new plugin into DB because of: {:?}", err);
            "could not register the plugin with jilebi. Check logs to see why".to_string()
        })?;
    let datetime = Utc::now().to_rfc2822();
    statement
        .execute(params![
            plugin_name.to_string(),
            plugin_path.to_string(),
            "1.0.0",
            PluginOrigin::Jilebi.to_string(),
            jilebi_types::PluginState::Enabled.to_string(),
            datetime.clone(),
            datetime,
        ])
        .map_err(|err| {
            tracing::error!("Could not insert plugins into DB: {:?}", err);
            format!("Could not insert plugins into DB, please check logs")
        })?;
    Ok(())
}

pub fn remove_plugin_in_db(db: &Connection, plugin_name: &str) -> Result<(), String> {
    let mut statement = db
        .prepare("DELETE FROM plugins WHERE name = ?1")
        .map_err(|err| {
            tracing::error!("Could not remove plugin from DB because of: {:?}", err);
            "could not remove the plugin from jilebi. Check logs to see why".to_string()
        })?;
    statement.execute([plugin_name]).map_err(|err| {
        tracing::error!("Could not remove plugin from DB: {:?}", err);
        format!("Could not remove plugin from DB, please check logs")
    })?;
    Ok(())
}

pub async fn download_and_init_plugin(
    id: &str,
    plugin_path: &PathBuf,
    db: &Connection,
) -> Result<(), String> {
    let file_name = format!("{}.zip", id);
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
    add_plugin_to_db(db, id, &plugin_path_str)?;
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
