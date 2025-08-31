#![deny(unused_crate_dependencies)]
use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub mod permissions;
pub mod plugin;
pub mod env;
/// a map between a URI and a plugin manifest
pub type Plugins = Arc<RwLock<HashMap<String, plugin::Manifest>>>;
