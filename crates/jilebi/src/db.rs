use chrono::Utc;
use jilebi_types::{PluginOrigin, PluginState, permissions::JilebiPermissions};
use rusqlite::{Connection, params};
use std::collections::{HashMap, HashSet};

pub(crate) mod plugins {

    use std::{path::PathBuf, str::FromStr};

    use jilebi_types::PluginMetaData;

    use super::*;

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

    pub fn get_plugin(db: &Connection, plugin_name: &str) -> Result<PluginMetaData, String> {
        let mut statement = db
            .prepare("SELECT * FROM plugins WHERE name = ?1")
            .map_err(|err| {
                tracing::error!(
                    "Could not prepare statement to get plugin because of: {:?}",
                    err
                );
                "Could not get plugin. Check logs to see why".to_string()
            })?;

        let plugin = statement
            .query_row([plugin_name], |row| {
                let name: String = row.get(0)?;
                let path: String = row.get(1)?;
                let path = PathBuf::from(path);
                let version: String = row.get(2)?;
                let origin: String = row.get(3)?;
                let origin = PluginOrigin::from_str(&origin).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let state: String = row.get(4)?;
                let state = PluginState::from_str(&state).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
                let date_installed: String = row.get(5)?;
                let last_updated: String = row.get(6)?;
                Ok(PluginMetaData {
                    name,
                    path,
                    version,
                    origin,
                    state,
                    date_installed: chrono::DateTime::parse_from_rfc2822(&date_installed)
                        .map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                0,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?
                        .with_timezone(&Utc),
                    last_updated: chrono::DateTime::parse_from_rfc2822(&last_updated)
                        .map_err(|e| {
                            rusqlite::Error::FromSqlConversionFailure(
                                0,
                                rusqlite::types::Type::Text,
                                Box::new(e),
                            )
                        })?
                        .with_timezone(&Utc),
                })
            })
            .map_err(|err| {
                tracing::error!("Could not query plugin from DB: {:?}", err);
                "Could not query plugin from DB, please check logs".to_string()
            })?;

        Ok(plugin)
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
}

pub(crate) mod plugin_permissions {

    use super::*;
    use rusqlite::OptionalExtension;

    fn sql_to_plugin_permissions(
        row: &rusqlite::Row,
    ) -> Result<JilebiPermissions, rusqlite::Error> {
        let value: String = row.get(0)?;
        let permissions = serde_json::from_str::<JilebiPermissions>(&value).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;
        Ok(permissions)
    }

    pub fn fetch_permissions_for_plugin(
        connection: &Connection,
        plugin_id: &str,
    ) -> Result<HashMap<String, JilebiPermissions>, String> {
        let mut stmt = connection
            .prepare("SELECT resource_name, value FROM plugin_permissions WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        let params = vec![plugin_id];
        let rows = stmt
            .query_map(rusqlite::params_from_iter(params), |row| {
                let resource_name: String = row.get(0)?;
                let permissions = sql_to_plugin_permissions(row)?;
                Ok((resource_name, permissions))
            })
            .map_err(|e| e.to_string())?;
        let mut permissions: HashMap<String, JilebiPermissions> = HashMap::new();
        for row in rows {
            let (resource_name, perms) = row.map_err(|e| e.to_string())?;
            permissions.insert(resource_name, perms);
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
        let query = "INSERT OR REPLACE INTO plugin_permissions (id, resource_name, value) VALUES (?1, ?2, ?3)";
        let mut stmt = db.prepare(query).map_err(|e| e.to_string())?;
        let value = serde_json::to_string(new_permissions).map_err(|e| e.to_string())?;
        stmt.execute(rusqlite::params![id, entity, value])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub(crate) mod plugin_env {

    use super::*;
    use jilebi_types::env::{EnvType, PluginEnv, PluginEnvs};
    use keyring::Entry;

    fn sql_to_plugin_env(row: &rusqlite::Row) -> Result<PluginEnv, rusqlite::Error> {
        let env_name: String = row.get(0)?;
        let env_type: String = row.get(2)?;
        let schema: String = row.get(3)?;
        let env_type = EnvType::try_from(env_type).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            )
        })?;
        let value: String = if env_type == EnvType::Normal {
            row.get(1)?
        } else {
            let entry = Entry::new("jilebi", &env_name).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
                )
            })?;
            entry.get_password().unwrap_or_default()
        };
        Ok(PluginEnv::new(env_name, value, env_type, schema))
    }

    pub fn fetch_envs_from_db(db: &Connection, id: &str) -> Result<PluginEnvs, String> {
        let mut statement = db
            .prepare("SELECT env_name, value, type, schema FROM plugin_env WHERE id = ?")
            .map_err(|e| {
                tracing::error!(
                    "Failed to prepare statement while fetching existing envs: {}",
                    e
                );
                format!("Failed to query jilebis internal state: {}", e)
            })?;
        let rows = statement
            .query_and_then([id], sql_to_plugin_env)
            .map_err(|e| {
                tracing::error!("Failed to fetch existing envs: {}", e);
                format!("Failed to fetch existing envs: {}", e)
            })?;
        let mut envs = HashSet::new();
        for row in rows {
            envs.insert(row.map_err(|e| e.to_string())?);
        }
        Ok(envs)
    }

    pub fn fetch_env(db: &Connection, env_name: &str, id: &str) -> Result<PluginEnv, String> {
        let mut statement = db
            .prepare(
                "SELECT env_name, value, type, schema FROM plugin_env WHERE env_name = ? AND id = ?",
            )
            .map_err(|e| {
                tracing::error!(
                    "Failed to prepare statement while fetching existing envs: {}",
                    e
                );
                format!(
                    "Failed to prepare statement while fetching existing envs: {}",
                    e
                )
            })?;
        let row = statement
            .query_one([env_name, id], sql_to_plugin_env)
            .map_err(|e| e.to_string())?;
        Ok(row)
    }

    pub fn set_env(
        keystore: &Entry,
        db: &Connection,
        id: &str,
        env: PluginEnv,
    ) -> Result<(), String> {
        let mut statement = db
            .prepare("INSERT OR REPLACE INTO plugin_env (id, type, env_name, value, schema) VALUES (?1, ?2, ?3, ?4, ?5)")
            .map_err(|e| {
                tracing::error!("Failed to prepare statement while setting env: {}", e);
                format!("Failed to prepare statement while setting env: {}", e)
            })?;
        let env_value = if env.env_type == EnvType::Secret {
            keystore.set_password(&env.value).map_err(|e| {
                tracing::error!("Failed to set secret env value: {}", e);
                format!("Failed to set secret env value: {}", e)
            })?;
            "set-in-credential-manager"
        } else {
            &env.value
        };
        statement
            .execute([
                id,
                &env.env_type.to_string().to_lowercase(),
                &env.env_name,
                &env_value,
                &env.schema,
            ])
            .map_err(|e| {
                tracing::error!("Failed to execute statement while setting env: {}", e);
                format!("Failed to execute statement while setting env: {}", e)
            })?;
        Ok(())
    }
}

pub(crate) mod application_contexts {
    use super::*;
    use jilebi_types::ApplicationContext;

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
}
