use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub mod permissions;
pub mod plugin;
/// a map between a URI and a plugin manifest
pub type Plugins = Arc<RwLock<HashMap<String, plugin::Manifest>>>;
