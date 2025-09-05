use std::collections::HashMap;

use jilebi_types::permissions::JilebiPermissions;
use rusqlite::{Connection, OptionalExtension};

fn sql_to_plugin_permissions(row: &rusqlite::Row) -> Result<JilebiPermissions, rusqlite::Error> {
    let value: String = row.get(0)?;
    let permissions = serde_json::from_str::<JilebiPermissions>(&value).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    Ok(permissions)
}

pub fn _fetch_permissions_for_plugin(
    connection: &Connection,
    plugin_id: &str,
    entities: &[&str],
) -> Result<HashMap<String, JilebiPermissions>, String> {
    let query = format!(
        "SELECT resource_name, value FROM plugin_permissions WHERE id = ?1 AND resource_name IN ({})",
        vec!["?"; entities.len()].join(",")
    );
    let mut stmt = connection.prepare(&query).map_err(|e| e.to_string())?;
    let mut params = vec![plugin_id];
    for resource in entities {
        params.push(resource);
    }
    let mut rows = stmt
        .query(rusqlite::params_from_iter(params))
        .map_err(|e| e.to_string())?;
    let mut permissions = HashMap::new();
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let resource_name: String = row.get(0).map_err(|e| e.to_string())?;
        let value: String = row.get(1).map_err(|e| e.to_string())?;
        let value = serde_json::from_str::<JilebiPermissions>(&value).map_err(|e| e.to_string())?;
        permissions.insert(resource_name, value);
    }
    Ok(permissions)
}

pub fn fetch_permissions_for_entity(
    connection: &Connection,
    plugin_id: &str,
    entity: &str,
) -> Result<Option<JilebiPermissions>, String> {
    let query = "SELECT value FROM plugin_permissions WHERE resource_name = ?1 AND id = ?2";
    let mut stmt = connection.prepare(query).map_err(|e| e.to_string())?;
    let permissions = stmt
        .query_one(
            rusqlite::params![entity, plugin_id],
            sql_to_plugin_permissions,
        )
		.optional()
        .map_err(|e| e.to_string())?;
    Ok(permissions)
}

pub fn set_permissions(
    db: &Connection,
    id: &str,
    entity: &str,
    new_permissions: &JilebiPermissions,
) -> Result<(), String> {
    let query =
        "INSERT OR REPLACE INTO plugin_permissions (id, resource_name, value) VALUES (?1, ?2, ?3)";
    let mut stmt = db.prepare(query).map_err(|e| e.to_string())?;
    let value = serde_json::to_string(new_permissions).map_err(|e| e.to_string())?;
    stmt.execute(rusqlite::params![id, entity, value])
        .map_err(|e| e.to_string())?;
    Ok(())
}
