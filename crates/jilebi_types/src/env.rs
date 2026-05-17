use std::collections::HashSet;

use strum_macros::EnumIter;

#[derive(Debug, strum_macros::Display, Clone, Default, Eq, PartialEq, Hash, EnumIter)]
pub enum EnvType {
    #[default]
    Normal,
    Secret,
}

impl TryFrom<String> for EnvType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "secret" => Ok(EnvType::Secret),
            "normal" => Ok(EnvType::Normal),
            _ => Err(format!("Invalid EnvType: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct PluginEnv {
    pub env_name: String,
    pub value: String,
    pub env_type: EnvType,
    pub schema: String,
}

pub type PluginEnvs = HashSet<PluginEnv>;

impl PluginEnv {
    pub fn new(env_name: String, value: String, env_type: EnvType, schema: String) -> Self {
        PluginEnv {
            env_name,
            value,
            env_type,
            schema,
        }
    }
}
