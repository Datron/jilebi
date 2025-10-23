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

pub fn list_local_plugins(db: &Connection) -> Result<Vec<(String, PluginState)>, String> {
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
            let state_str: String = row.get(1)?;
            let state = match state_str.as_str() {
                "enabled" => PluginState::Enabled,
                "disabled" => PluginState::Disabled,
                _ => PluginState::Disabled, // Default case
            };
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