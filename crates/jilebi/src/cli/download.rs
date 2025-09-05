use std::{fs, io::Write, path::PathBuf};

const DOWNLOAD_BUCKET: &str = "https://pub-94ee61872c1341e1baa984ebe1d235ec.r2.dev";

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

pub fn add_plugin_to_toml(jilebi_plugin_dir: &PathBuf, plugin_path: &PathBuf) -> Result<(), String> {
    let toml_path = jilebi_plugin_dir.join("plugins.toml");
    let toml = fs::read_to_string(&toml_path).map_err(|e| e.to_string())?;
    let mut toml = toml::from_str::<toml::Value>(&toml).map_err(|e| {
        format!(
            "Could not parse the plugins toml directory manifest {}",
            e.to_string()
        )
    })?;
    toml.get_mut("manifest")
        .and_then(|v| v.as_array_mut())
        .map(|arr| {
            arr.push(plugin_path.join("manifest.toml").display().to_string().into());
        })
        .ok_or("Could not properly parse the plugin registry")?;
    fs::write(
        &toml_path,
        toml::to_string_pretty(&toml).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

pub fn remove_plugin_to_toml(jilebi_plugin_dir: &PathBuf, plugin_name: &str) -> Result<(), String> {
    let toml_path = jilebi_plugin_dir.join("plugins.toml");
    let toml = fs::read_to_string(&toml_path).map_err(|e| e.to_string())?;
    let mut toml = toml::from_str::<toml::Value>(&toml).map_err(|e| {
        format!(
            "Could not parse the plugins toml directory manifest {}",
            e.to_string()
        )
    })?;
    toml.get_mut("manifest")
        .and_then(|v| v.as_array_mut())
        .map(|arr| {
            arr.retain(|item| item.as_str().map_or(true, |s| !s.contains(plugin_name)));
        })
        .ok_or("Could not properly parse the plugin registry")?;
    fs::write(
        &toml_path,
        toml::to_string_pretty(&toml).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

pub async fn download_and_init_plugin(
    id: &str,
    plugin_path: &PathBuf,
    jilebi_plugin_dir: &PathBuf,
) -> Result<(), String> {
    let file_name = format!("{}.zip", id);
    let url = format!("{}/plugins/{}", DOWNLOAD_BUCKET, file_name);
    tracing::debug!("Downloading plugin from URL: {}", url);
	if !plugin_path.exists() {
        fs::create_dir_all(&plugin_path).map_err(|e| e.to_string())?;
    }
    download_and_extract(&url, &plugin_path.join(&file_name)).await?;
    add_plugin_to_toml(jilebi_plugin_dir, plugin_path)?;
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
    let url = format!("{}/templates/{}", DOWNLOAD_BUCKET, file_name);
    fs::create_dir_all(new_plugin_path).map_err(|e| e.to_string())?;
    download_and_extract(&url, &new_plugin_path.join(&file_name)).await?;
    if complex_plugin {
        let rollup = if language == "javascript" {
            "rollup.config.js"
        } else {
            "rollup.config.ts"
        };
        let url = format!("{}/templates/{}", DOWNLOAD_BUCKET, rollup);
        download(&url, &new_plugin_path.join(rollup)).await?;
    }
    let package_json =
        fs::read_to_string(new_plugin_path.join("package.json")).map_err(|e| e.to_string())?;
    let package_json = package_json.replace("replace", plugin_name);
    let package_json = if language == "typescript" && complex_plugin {
        package_json.replace("command", "rollup -c")
    } else {
        package_json.replace("command", "tsc")
    };
    fs::write(new_plugin_path.join("package.json"), package_json).map_err(|e| e.to_string())?;
    fs::write(new_plugin_path.join("manifest.toml"), manifest_code).map_err(|e| e.to_string())?;
    Ok(())
}
