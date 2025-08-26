use deno_core::{extension, op2};
use directories::ProjectDirs;
use rusqlite::{Connection, params};

#[op2(fast)]
fn set_state(#[string] id: &str, #[string] key: &str, #[string] value: &str) -> bool {
    let Some(base_jilebi_dir) = ProjectDirs::from("ai", "jilebi", "jilebi-server") else {
        tracing::error!("Could not create ProjectDirs struct");
        return false;
    };
    let db_file = base_jilebi_dir.data_dir().join("jilebi.db3");
    let connection = match Connection::open(db_file) {
        Ok(conn) => conn,
        Err(err) => {
            tracing::error!("Plugin state could not be fetched {}", err);
            return false;
        }
    };

    let Ok(mut statement) = connection
        .prepare("INSERT OR REPLACE INTO plugin_state(id, key, value) VALUES(?1, ?2, ?3)")
    else {
        tracing::error!("SQL statement could not prepared");
        return false;
    };

    match statement.execute(params![id, key, value]) {
        Ok(_) => true,
        Err(err) => {
            tracing::error!(
                "Could not execute set_state prepared statement for {id} {key} and value {value} due to {}",
                err
            );
            return false;
        }
    }
}

#[op2]
#[string]
fn get_state(#[string] id: &str, #[string] key: &str) -> String {
    let Some(base_jilebi_dir) = ProjectDirs::from("ai", "jilebi", "jilebi-server") else {
        tracing::error!("Could not create ProjectDirs struct");
        return String::from("null");
    };
    let db_file = base_jilebi_dir.data_dir().join("jilebi.db3");
    let connection = match Connection::open(db_file) {
        Ok(conn) => conn,
        Err(err) => {
            tracing::error!("Plugin state could not be fetched {}", err);
            return String::from("null");
        }
    };
    let Ok(mut statement) =
        connection.prepare("SELECT value FROM plugin_state WHERE key = ?1 AND id = ?2")
    else {
        tracing::error!("SQL statement could not prepared");
        return String::from("null");
    };

    match statement.query_row([key, id], |row| row.get(0)) {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("could not fetch the state for plugin {id} {key} due to error {err}");
            return String::from("null");
        }
    }
}

#[op2(fast)]
fn delete_state(#[string] id: &str, #[string] key: &str) -> bool {
    let Some(base_jilebi_dir) = ProjectDirs::from("ai", "jilebi", "jilebi-server") else {
        tracing::error!("Could not create ProjectDirs struct");
        return false;
    };
    let db_file = base_jilebi_dir.data_dir().join("jilebi.db3");
    let connection = match Connection::open(db_file) {
        Ok(conn) => conn,
        Err(err) => {
            tracing::error!("Plugin state could not be fetched {}", err);
            return false;
        }
    };
    let Ok(mut statement) =
        connection.prepare("DELETE FROM plugin_state WHERE key = ?1 AND id = ?2")
    else {
        tracing::error!("SQL statement could not prepared");
        return false;
    };

    match statement.execute([key, id]) {
        Ok(_) => true,
        Err(err) => {
            tracing::error!("could not delete the state for plugin {id}, {key} due to error {err}");
            return false;
        }
    }
}

extension!(
    state,
    ops = [set_state, get_state, delete_state],
    esm_entry_point = "ext:state/state.js",
    esm = [ dir "extensions", "state.js" ],
);
