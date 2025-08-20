use std::{fs, io::Write, path::PathBuf};

const DOWNLOAD_BUCKET: &str = "https://pub-94ee61872c1341e1baa984ebe1d235ec.r2.dev";

async fn download_and_extract(url: &str, extract_path: &PathBuf) -> Result<(), String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;
    let mut file = fs::File::create(&extract_path).map_err(|e| e.to_string())?;
    file.write_all(&response).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    zip.extract(extract_path).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn download_and_init_plugin(id: &str, plugin_path: &PathBuf) -> Result<(), String> {
    let file_name = format!("{}.zip", id);
    let url = format!("{}/plugins/{}", DOWNLOAD_BUCKET, file_name);
    download_and_extract(&url, &plugin_path.join(&file_name)).await?;
    Ok(())
}

pub async fn download_and_init_template(
    plugin_name: &str,
    manifest_code: &str,
    plugin_path: &PathBuf,
    language: &str,
    complex_plugin: bool,
) -> Result<(), String> {
    let file_name = format!("{}.zip", language);
    let url = format!("{}/templates/{}", DOWNLOAD_BUCKET, file_name);
    download_and_extract(&url, &plugin_path.join(&file_name)).await?;
    if complex_plugin {
        let rollup = if language == "javascript" {
            "rollup.config.js"
        } else {
            "rollup.config.ts"
        };
        let url = format!("{}/templates/{}", DOWNLOAD_BUCKET, rollup);
        download_and_extract(&url, &plugin_path.join(rollup)).await?;
    }
    let package_json =
        fs::read_to_string(plugin_path.join("package.json")).map_err(|e| e.to_string())?;
    let package_json = package_json.replace("replace", plugin_name);
    let package_json = if language == "typescript" && complex_plugin {
        package_json.replace("commandreplace", "rollup -c")
    } else {
        package_json.replace("commandreplace", "tsc")
    };
    fs::write(plugin_path.join("package.json"), package_json).map_err(|e| e.to_string())?;
    fs::write(plugin_path.join("manifest.toml"), manifest_code).map_err(|e| e.to_string())?;
    Ok(())
}
