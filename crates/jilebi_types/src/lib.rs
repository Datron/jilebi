#![deny(unused_crate_dependencies)]
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

pub mod env;
pub mod manifest;
pub mod permissions;
/// a map between a URI and a plugin manifest
pub type Plugins = Arc<RwLock<HashMap<String, manifest::Manifest>>>;

#[derive(Debug, Clone, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum PluginOrigin {
    Local,
    Jilebi,
}

#[derive(Debug, Clone, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum PluginState {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct PluginMetaData {
    pub name: String,
    pub path: PathBuf,
    pub version: String,
    pub origin: PluginOrigin,
    pub state: PluginState,
    pub date_installed: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
