use std::{borrow::Cow, collections::HashMap, sync::Arc};

use derive_more::Deref;
use either::Either::{self, Left, Right};
use rmcp::model::{Prompt, RawResource, RawResourceTemplate, Resource, ResourceTemplate, Tool};
use serde::{Deserialize, Serialize};
use toml::Table;

fn mandatory_extractor(op_table: &Table, key: &String, field: &String) -> Result<String, String> {
    op_table
        .get(field)
        .ok_or(format!("Missing field {field} in resource {key}"))?
        .as_str()
        .ok_or(format!(
            "Invalid data entered for field {field} in resource {key}"
        ))
        .map(str::to_string)
}

fn optional_extractor(op_table: &Table, field: &String) -> Option<String> {
    op_table
        .get(field)
        .and_then(|i| i.as_str().map(str::to_string))
}

pub type ResourceKey = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JilebiResource {
    pub resource: Either<Resource, ResourceTemplate>,
    pub function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Resources(pub HashMap<ResourceKey, JilebiResource>);

impl TryFrom<&toml::Value> for Resources {
    type Error = String;

    fn try_from(value: &toml::Value) -> Result<Self, Self::Error> {
        match value {
            toml::Value::Table(map) => {
                let mut resource_map: HashMap<ResourceKey, JilebiResource> = HashMap::new();
                for (key, table) in map.iter() {
                    let Some(op_table) = table.as_table() else {
                        // TODO: Add docs for formats
                        return Err(format!(
                            "Invalid format for resource definition {key}, please check the toml file. Refer the docs:"
                        ));
                    };
                    let resource = if op_table.contains_key("uri_template") {
                        Right(ResourceTemplate::new(
                            RawResourceTemplate {
                                uri_template: mandatory_extractor(
                                    op_table,
                                    key,
                                    &"uri_template".to_string(),
                                )?,
                                name: mandatory_extractor(op_table, key, &"name".to_string())?,
                                description: optional_extractor(
                                    op_table,
                                    &"description".to_string(),
                                ),
                                mime_type: optional_extractor(op_table, &"mime_type".to_string()),
                            },
                            None,
                        ))
                    } else {
                        Left(Resource::new(
                            RawResource {
                                uri: mandatory_extractor(op_table, key, &"uri".to_string())?,
                                name: mandatory_extractor(op_table, key, &"name".to_string())?,
                                description: optional_extractor(
                                    op_table,
                                    &"description".to_string(),
                                ),
                                mime_type: optional_extractor(op_table, &"mime_type".to_string()),
                                size: optional_extractor(op_table, &"".to_string())
                                    .and_then(|s| s.parse::<u32>().ok()),
                            },
                            None,
                        ))
                    };
                    let jilebi_resource = JilebiResource {
                        resource,
                        function: mandatory_extractor(op_table, key, &"function".to_string())?,
                    };
                    resource_map.insert(key.to_string(), jilebi_resource);
                }
                Ok(Self(resource_map))
            }
            _ => Err("Invalid format for Resources section".to_string()),
        }
    }
}

pub type ToolKey = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JilebiTool {
    pub tool: Tool,
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
                    let Some(op_table) = table.as_table() else {
                        return Err(format!(
                            "Invalid format for resource definition {key}, check the toml"
                        ));
                    };
                    let schema = serde_json::json!(
                        op_table
                            .get("input_schema")
                            .ok_or(format!("Missing field input_schema in resource {key}"))?
                    )
                    .as_object()
                    .cloned()
                    .ok_or(format!(
                        "Invalid JSON format for the field input_schema in resource {key}"
                    ))?;
                    let tool = Tool {
                        name: Cow::from(mandatory_extractor(op_table, key, &"name".to_string())?),
                        description: optional_extractor(op_table, &"description".to_string())
                            .map(Cow::from),
                        input_schema: Arc::new(schema),
                        annotations: None,
                    };
                    tools_map.insert(key.to_string(), tool);
                }
                Ok(Self(tools_map))
            }
            _ => Err("Invalid format for Tools section".to_string()),
        }
    }
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
    pub name: String,
    // pub identifier: String,
    pub resources: Resources,
    pub tools: Tools,
    pub prompts: Prompts,
}

impl TryFrom<toml::Value> for Manifest {
    type Error = String;

    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .ok_or("The name of the plugin is mandatory".to_string())?
            .as_str()
            .ok_or("The name of the plugin is not convertible to string")?
            .to_string();
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
            name,
            resources,
            tools,
            prompts,
        })
    }
}
