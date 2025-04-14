use std::collections::HashMap;

use derive_more::Deref;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Text,
    Blob,
}

pub type ResourceKey = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub uri: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub resource_type: ResourceType,
    pub function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Resources(pub HashMap<ResourceKey, Resource>);

impl TryFrom<&toml::Value> for Resources {
    type Error = String;

    fn try_from(value: &toml::Value) -> Result<Self, Self::Error> {
        match value {
            toml::Value::Table(map) => {
                let mut resource_map: HashMap<ResourceKey, Resource> = HashMap::new();
                for (key, table) in map.iter() {
                    let Ok(resource) = (*table).clone().try_into() else {
                        return Err(format!(
                            "Invalid format for resource definition {key}, check the toml"
                        ));
                    };
                    resource_map.insert(key.to_string(), resource);
                }
                Ok(Self(resource_map))
            }
            _ => Err("Invalid format for Resources section".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotations {
    pub title: Option<String>,
    pub read_only_hint: Option<bool>,
    pub destructive_hint: Option<bool>,
    pub idempotent_hint: Option<bool>,
    pub open_world_hint: Option<bool>,
}

pub type ToolKey = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
    pub annotations: Option<Annotations>,
    pub function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Tools(pub HashMap<ToolKey, Tool>);

impl TryFrom<&toml::Value> for Tools {
    type Error = String;

    fn try_from(value: &toml::Value) -> Result<Self, Self::Error> {
        match value {
            toml::Value::Table(map) => {
                let mut tools_map: HashMap<ToolKey, Tool> = HashMap::new();
                for (key, table) in map.iter() {
                    let Ok(tool) = (*table).clone().try_into() else {
                        return Err(format!(
                            "Invalid format for resource definition {key}, check the toml"
                        ));
                    };
                    tools_map.insert(key.to_string(), tool);
                }
                Ok(Self(tools_map))
            }
            _ => Err("Invalid format for Tools section".to_string()),
        }
    }
}

#[derive(Debug, Clone, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PromptContentType {
    Text {
        text: String,
    },
    Resource {
        uri: String,
        text: String,
        mime_type: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct PromptArguments(pub Vec<PromptArgument>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<PromptArguments>,
    pub role: String,
    pub content: PromptContentType,
}

pub type PromptKey = String;

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Prompts(pub HashMap<PromptKey, Prompt>);

impl TryFrom<&toml::Value> for Prompts {
    type Error = String;

    fn try_from(value: &toml::Value) -> Result<Self, Self::Error> {
        match value {
            toml::Value::Table(map) => {
                let mut prompt_map: HashMap<PromptKey, Prompt> = HashMap::new();
                for (key, table) in map.iter() {
                    let Ok(prompt) = (*table).clone().try_into() else {
                        return Err(format!(
                            "Invalid format for resource definition {key}, check the toml"
                        ));
                    };
                    prompt_map.insert(key.to_string(), prompt);
                }
                Ok(Self(prompt_map))
            }
            _ => Err("Invalid format for Resources section".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub resources: Resources,
    pub tools: Tools,
    pub prompts: Prompts,
}

impl TryFrom<toml::Value> for Manifest {
    type Error = String;

    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        let resources = Resources::try_from(
            value
                .get("resources")
                .ok_or("The resources section is mandatory".to_string())?,
        )?;
        let tools = Tools::try_from(
            value
                .get("tools")
                .ok_or("The tools section is mandatory".to_string())?,
        )?;
        let prompts = Prompts::try_from(
            value
                .get("prompts")
                .ok_or("The prompts section is mandatory".to_string())?,
        )?;
        Ok(Self {
            resources,
            tools,
            prompts,
        })
    }
}
