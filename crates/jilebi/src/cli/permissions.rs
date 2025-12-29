use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use dialoguer::{Confirm, Input};
use directories::UserDirs;
use jilebi_types::permissions::JilebiPermissions;
use rusqlite::Connection;

use crate::db;

pub fn set_default_permissions(
    permissions: &JilebiPermissions,
) -> Result<JilebiPermissions, String> {
    let hosts = if permissions.hosts.contains("user_defined") {
        HashSet::from([String::from("*")])
    } else {
        permissions.hosts.clone()
    };
    let user_directory = UserDirs::new()
        .and_then(|dirs| dirs.home_dir().to_str().map(|s| s.to_string()))
        .unwrap_or_default();
    let read_dirs = if permissions.read_dirs.contains("user_defined") {
        HashSet::from([String::from(&user_directory)])
    } else {
        permissions.read_dirs.clone()
    };
    let write_dirs = if permissions.write_dirs.contains("user_defined") {
        HashSet::from([String::from(&user_directory)])
    } else {
        permissions.write_dirs.clone()
    };
    let urls = if permissions.urls.contains("user_defined") {
        HashSet::from([String::from("*")])
    } else {
        permissions.urls.clone()
    };
    Ok(JilebiPermissions {
        hosts,
        read_dirs,
        write_dirs,
        urls,
        read_files: HashSet::new(),
        write_files: HashSet::new(),
    })
}

pub fn query_permissions_from_the_user(
    entity: &String,
    permissions: &JilebiPermissions,
) -> Result<JilebiPermissions, String> {
    let hosts = if permissions.hosts.contains("user_defined") {
        let user_defined: String = Input::new()
            .with_prompt(format!(
                "Enter hosts for {}, use a comma to separate multiple entries",
                entity
            ))
            .interact_text()
            .map_err(|e| e.to_string())?;
        user_defined
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let mut allowed_hosts = HashSet::new();
        for host in &permissions.hosts {
            if Confirm::new()
                .with_prompt(format!(
                    "Allow {} to make requests to the host {}?",
                    entity, host
                ))
                .interact()
                .unwrap()
            {
                allowed_hosts.insert(host.clone());
            }
        }
        allowed_hosts.clone()
    };
    let read_dirs = if permissions.read_dirs.contains("user_defined") {
        let user_defined: String = Input::new()
						.with_prompt(format!("Enter directories that the plugin is allowed to read from for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
        user_defined
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let mut allowed_read_dirs = HashSet::new();
        for dir in &permissions.read_dirs {
            if Confirm::new()
                .with_prompt(format!(
                    "Allow {} to read from the directory {}?",
                    dir, entity
                ))
                .interact()
                .unwrap()
            {
                allowed_read_dirs.insert(dir.clone());
            }
        }
        allowed_read_dirs.clone()
    };
    let write_dirs = if permissions.write_dirs.contains("user_defined") {
        let user_defined: String = Input::new()
						.with_prompt(format!("Enter directories that the plugin is allowed to write to for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
        user_defined
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let mut allowed_write_dirs = HashSet::new();
        for dir in &permissions.write_dirs {
            if Confirm::new()
                .with_prompt(format!(
                    "Allow {} to write to the directory {}?",
                    dir, entity
                ))
                .interact()
                .unwrap()
            {
                allowed_write_dirs.insert(dir.clone());
            }
        }
        allowed_write_dirs.clone()
    };

    let urls = if permissions.urls.contains("user_defined") {
        let user_defined: String = Input::new()
						.with_prompt(format!("Enter URLs that the plugin is allowed to access for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
        user_defined
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let mut allowed_urls = HashSet::new();
        for url in &permissions.urls {
            if Confirm::new()
                .with_prompt(format!("Allow {} to access the URL {}?", url, entity))
                .interact()
                .unwrap()
            {
                allowed_urls.insert(url.clone());
            }
        }
        allowed_urls.clone()
    };

    let read_files = if permissions.read_files.contains("user_defined") {
        let user_defined: String = Input::new()
						.with_prompt(format!("Enter files that the plugin is allowed to read from for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
        user_defined
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let mut allowed_read_files = HashSet::new();
        for file in &permissions.read_files {
            if Confirm::new()
                .with_prompt(format!("Allow {} to read the file {}?", file, entity))
                .interact()
                .unwrap()
            {
                allowed_read_files.insert(file.clone());
            }
        }
        allowed_read_files.clone()
    };

    let write_files = if permissions.write_files.contains("user_defined") {
        let user_defined: String = Input::new()
						.with_prompt(format!("Enter files that the plugin is allowed to write to for {}, use a comma to separate multiple entries", entity))
						.interact_text()
						.map_err(|e| e.to_string())?;
        user_defined
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        let mut allowed_write_files = HashSet::new();
        for file in &permissions.write_files {
            if Confirm::new()
                .with_prompt(format!("Allow {} to write to the file {}?", file, entity))
                .interact()
                .unwrap()
            {
                allowed_write_files.insert(file.clone());
            }
        }
        allowed_write_files.clone()
    };
    Ok(JilebiPermissions {
        hosts,
        read_dirs,
        write_dirs,
        urls,
        read_files,
        write_files,
    })
}

pub fn get_permissions_from_manifest(
    plugin_dir: &PathBuf,
    plugin_name: &str,
) -> Result<HashMap<String, JilebiPermissions>, String> {
    let manifest = crate::utils::get_plugin_manifest(plugin_dir, plugin_name)?;
    let mut permission_requirements: HashMap<String, JilebiPermissions> = HashMap::new();
    for (name, resource) in manifest.resources.iter() {
        if let Some(ref perms) = resource.permissions {
            permission_requirements.insert(name.clone(), perms.clone());
        }
    }

    for (name, tool) in manifest.tools.iter() {
        if let Some(ref perms) = tool.permissions {
            permission_requirements.insert(name.clone(), perms.clone());
        }
    }

    Ok(permission_requirements)
}

pub fn setup_permissions(
    db: &Connection,
    plugin_dir: &PathBuf,
    plugin_name: &str,
    permissions: Option<HashMap<String, JilebiPermissions>>,
    auto_accept_permissions: bool,
) -> Result<(), String> {
    let permission_requirements = if permissions.is_some() {
        permissions.unwrap()
    } else {
        get_permissions_from_manifest(plugin_dir, &plugin_name)?
    };
    for (entity, existing_permissions) in permission_requirements.iter() {
        let new_permissions = if auto_accept_permissions {
            set_default_permissions(existing_permissions)?
        } else {
            query_permissions_from_the_user(entity, existing_permissions)?
        };
        db::plugin_permissions::set_permissions(&db, &plugin_name, entity, &new_permissions)?;
    }
    Ok(())
}
