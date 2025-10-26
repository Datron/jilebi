use chrono::Utc;
use jilebi_types::PluginState;
use rusqlite::Connection;

/// Update the state of a plugin in the database
pub fn update_plugin_state(
    db: &Connection,
    plugin_name: &str,
    new_state: PluginState,
) -> Result<(), String> {
    let mut statement = db
        .prepare("UPDATE plugins SET state = ?1, last_updated = ?2 WHERE name = ?3")
        .map_err(|err| {
            tracing::error!(
                "Could not prepare statement to update plugin state because of: {:?}",
                err
            );
            "Could not update plugin state. Check logs to see why".to_string()
        })?;

    let datetime = Utc::now().to_rfc2822();
    statement
        .execute(rusqlite::params![
            new_state.to_string(),
            datetime,
            plugin_name
        ])
        .map_err(|err| {
            tracing::error!("Could not update plugin state in DB: {:?}", err);
            "Could not update plugin state in DB, please check logs".to_string()
        })?;

    Ok(())
}

pub fn list_local_plugins(db: &Connection) -> Result<Vec<(String, String)>, String> {
    let mut statement = db
        .prepare("SELECT name, state FROM plugins")
        .map_err(|err| {
            tracing::error!(
                "Could not prepare statement to list plugins because of: {:?}",
                err
            );
            "Could not list plugins. Check logs to see why".to_string()
        })?;

    let plugin_iter = statement
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let state: String = row.get(1)?;
            Ok((name, state))
        })
        .map_err(|err| {
            tracing::error!("Could not query plugins from DB: {:?}", err);
            "Could not query plugins from DB, please check logs".to_string()
        })?;

    let mut plugins = Vec::new();
    for plugin in plugin_iter {
        plugins.push(plugin.map_err(|e| e.to_string())?);
    }

    Ok(plugins)
}

pub async fn list_remote_plugins() -> Result<Vec<(String, String)>, String> {
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
            let download_count = plugin.get("download_count")?.as_i64()?.to_string();
            Some((name, download_count))
        })
        .collect::<Vec<(String, String)>>();
    Ok(plugins)
}
