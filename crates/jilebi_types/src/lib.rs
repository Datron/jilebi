use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub mod plugin;
pub mod permissions;
/// a map between a URI and a plugin manifest
pub type Plugins = Arc<RwLock<HashMap<String, plugin::Manifest>>>;
