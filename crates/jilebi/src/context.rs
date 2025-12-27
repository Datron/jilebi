use std::collections::HashSet;

use jilebi_types::ApplicationContext;
use rusqlite::{Connection, params};

fn row_to_context(row: &rusqlite::Row) -> Result<ApplicationContext, rusqlite::Error> {
    let name: String = row.get(0)?;
    let plugins_str: String = row.get(1)?;
    let plugins: Vec<String> = plugins_str
        .split(',')
        .map(|s| s.trim().trim_matches('\'').to_string())
        .collect();
    Ok(ApplicationContext { name, plugins })
}

fn validate_plugins(db: &Connection, plugins: &[String]) -> Result<(), String> {
    let user_request_set: HashSet<String> = HashSet::from_iter(plugins.iter().cloned());
    let plugins_str = plugins
        .iter()
        .map(|p| format!("'{}'", p.trim()))
        .collect::<Vec<String>>()
        .join(",");
    let query = format!("SELECT name FROM plugins WHERE name IN ({})", plugins_str);
    let mut statement = db.prepare(&query).map_err(|err| {
        tracing::error!(
            "Could not prepare statement to validate plugins because of: {:?}",
            err
        );
        "Could not validate plugins. Check logs to see why".to_string()
    })?;
    let mut valid_plugins_set = HashSet::new();
    let rows = statement
        .query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })
        .map_err(|e| {
            tracing::error!("Could not query plugins from DB: {:?}", e);
            "Could not query plugins from DB, please check logs".to_string()
        })?;
    for row in rows {
        let plugin_name = row.map_err(|e| e.to_string())?;
        valid_plugins_set.insert(plugin_name);
    }
    let invalid_plugins = user_request_set
        .difference(&valid_plugins_set)
        .cloned()
        .collect::<Vec<String>>();
    if invalid_plugins.len() > 0 {
        return Err(format!(
            "The following plugins are not installed: {}",
            invalid_plugins.join(", ")
        ));
    }
    Ok(())
}

fn get_application_context(db: &Connection, name: &str) -> Result<ApplicationContext, String> {
    let mut statement = db
        .prepare("SELECT name, plugins FROM application_contexts WHERE name = ?1")
        .map_err(|err| {
            tracing::error!(
                "Could not prepare statement to get context because of: {:?}",
                err
            );
            "Could not get context. Check logs to see why".to_string()
        })?;
    let context = statement.query_row([name], row_to_context).map_err(|e| {
        tracing::error!("Could not query context from DB: {:?}", e);
        "Could not query context from DB, please check logs".to_string()
    })?;
    Ok(context)
}

fn update_application_context_plugins(
    db: &Connection,
    name: &str,
    plugins: HashSet<&String>,
) -> Result<(), String> {
    let plugins_str = plugins
        .iter()
        .map(|p| format!("'{}'", p.trim()))
        .collect::<Vec<String>>()
        .join(", ");
    let mut statement = db
        .prepare("UPDATE application_contexts SET plugins = ?1 WHERE name = ?2")
        .map_err(|err| {
            tracing::error!(
                "Could not prepare statement to update context because of: {:?}",
                err
            );
            "Could not update context. Check logs to see why".to_string()
        })?;
    statement.execute(params![plugins_str, name]).map_err(|e| {
        tracing::error!("Could not update context in DB: {:?}", e);
        "Could not update context in DB, please check logs".to_string()
    })?;
    Ok(())
}

pub(crate) fn create_application_context(
    db: &Connection,
    context_name: &str,
    plugins: &[String],
) -> Result<(), String> {
    validate_plugins(db, plugins)?;
    let mut statement = db
        .prepare("INSERT INTO application_contexts VALUES(?1, ?2)")
        .map_err(|err| {
            tracing::error!("Could not insert new context into DB because of: {:?}", err);
            "could not register the context with jilebi. Check logs to see why".to_string()
        })?;
    let plugins_str = plugins
        .iter()
        .map(|p| format!("'{}'", p.trim()))
        .collect::<Vec<String>>()
        .join(", ");
    statement
        .execute(params![context_name, plugins_str])
        .map_err(|e| {
            tracing::error!("Could not insert context into DB due to: {:?}", e);
            "Could not insert context into DB, please check logs".to_string()
        })?;
    Ok(())
}

pub(crate) fn add_plugin_to_application_context(
    db: &Connection,
    name: &str,
    plugin: String,
) -> Result<(), String> {
    validate_plugins(db, &[plugin.clone()])?;
    let context = get_application_context(db, name)?;
    let mut updated_plugins: HashSet<&String> = HashSet::from_iter(context.plugins.iter());
    updated_plugins.insert(&plugin);
    update_application_context_plugins(db, name, updated_plugins)?;
    Ok(())
}

pub(crate) fn remove_plugin_from_application_context(
    db: &Connection,
    name: &str,
    plugin: String,
) -> Result<(), String> {
    let context = get_application_context(db, name)?;
    let mut updated_plugins: HashSet<&String> = HashSet::from_iter(context.plugins.iter());
    updated_plugins.remove(&plugin);
    update_application_context_plugins(db, name, updated_plugins)?;
    Ok(())
}

pub(crate) fn delete_application_context(db: &Connection, name: &str) -> Result<(), String> {
    let mut statement = db
        .prepare("DELETE FROM application_contexts WHERE name = ?1")
        .map_err(|err| {
            tracing::error!("Could not insert new context into DB because of: {:?}", err);
            "could not register the context with jilebi. Check logs to see why".to_string()
        })?;
    statement.execute(params![name]).map_err(|e| {
        tracing::error!("Could not delete context due to: {:?}", e);
        "Could not delete context, please check logs".to_string()
    })?;
    Ok(())
}

pub(crate) fn list_application_contexts(
    db: &Connection,
) -> Result<Vec<ApplicationContext>, String> {
    let mut statement = db
        .prepare("SELECT name, plugins FROM application_contexts")
        .map_err(|err| {
            tracing::error!(
                "Could not prepare statement to list contexts because of: {:?}",
                err
            );
            "Could not list contexts. Check logs to see why".to_string()
        })?;
    let mut contexts = Vec::new();
    let rows = statement.query_map([], row_to_context).map_err(|e| {
        tracing::error!("Could not query contexts from DB: {:?}", e);
        "Could not query contexts from DB, please check logs".to_string()
    })?;
    for row in rows {
        contexts.push(row.map_err(|e| e.to_string())?);
    }
    Ok(contexts)
}
