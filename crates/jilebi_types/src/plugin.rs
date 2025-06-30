use std::{borrow::Cow, collections::HashMap, sync::Arc};

use derive_more::Deref;
use rmcp::model::{
    Prompt, PromptArgument, PromptMessage, PromptMessageContent, PromptMessageRole, RawResource,
    Resource, Tool,
};
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
    pub resource: Resource,
    pub function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Resources(pub HashMap<ResourceKey, JilebiResource>);

impl Resources {
    fn try_from(value: &toml::Value, plugin_name: &String) -> Result<Self, String> {
        let map = value
            .as_table()
            .ok_or("Invalid format for Resources section".to_string())?;
        let mut resource_map: HashMap<ResourceKey, JilebiResource> = HashMap::new();
        for (key, table) in map.iter() {
            let Some(op_table) = table.as_table() else {
                // TODO: Add docs for formats
                return Err(format!(
                    "Invalid format for resource definition {key}, please check the toml file. Refer the docs:"
                ));
            };
            let resource_name = mandatory_extractor(op_table, key, &"name".to_string())?;
            let resource = Resource::new(
                RawResource {
                    uri: format!("{plugin_name}.{resource_name}"),
                    name: resource_name,
                    description: optional_extractor(op_table, &"description".to_string()),
                    mime_type: optional_extractor(op_table, &"mime_type".to_string()),
                    size: optional_extractor(op_table, &"".to_string())
                        .and_then(|s| s.parse::<u32>().ok()),
                },
                None,
            );
            let jilebi_resource = JilebiResource {
                resource,
                function: mandatory_extractor(op_table, key, &"function".to_string())?,
            };
            resource_map.insert(key.to_string(), jilebi_resource);
        }
        Ok(Self(resource_map))
    }
}

pub type ToolKey = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JilebiTool {
    pub tool: Tool,
    pub function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Tools(pub HashMap<ToolKey, JilebiTool>);

impl Tools {
    fn try_from(value: &toml::Value, plugin_name: &String) -> Result<Self, String> {
        let map = value
            .as_table()
            .ok_or("Invalid format for Tools section".to_string())?;
        let mut tools_map: HashMap<ToolKey, JilebiTool> = HashMap::new();
        for (key, table) in map.iter() {
            let Some(op_table) = table.as_table() else {
                return Err(format!(
                    "Invalid format for tool definition {key}, check the toml"
                ));
            };
            let schema = serde_json::json!(
                op_table
                    .get("input_schema")
                    .ok_or(format!("Missing field input_schema in tool {key}"))?
            )
            .as_object()
            .cloned()
            .ok_or(format!(
                "Invalid JSON format for the field input_schema in tool {key}"
            ))?;
            let tool = Tool {
                name: Cow::from(format!(
                    "{plugin_name}.{}",
                    mandatory_extractor(op_table, key, &"name".to_string())?
                )),
                description: optional_extractor(op_table, &"description".to_string())
                    .map(Cow::from),
                input_schema: Arc::new(schema),
                annotations: None,
            };
            let jilebi_tool = JilebiTool {
                tool,
                function: mandatory_extractor(op_table, key, &"function".to_string())?,
            };
            tools_map.insert(key.to_string(), jilebi_tool);
        }
        Ok(Self(tools_map))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JilebiPrompt {
    pub prompt: Prompt,
    pub content: Vec<PromptMessage>,
}

pub type PromptKey = String;

#[derive(Debug, Clone, Serialize, Deserialize, Deref)]
pub struct Prompts(pub HashMap<PromptKey, JilebiPrompt>);

impl Prompts {
    fn try_from(value: &toml::Value, plugin_name: &String) -> Result<Self, String> {
        let map = value
            .as_table()
            .ok_or("Invalid format for Prompts section".to_string())?;
        let mut prompt_map: HashMap<PromptKey, JilebiPrompt> = HashMap::new();
        for (key, table) in map.iter() {
            let Some(op_table) = table.as_table() else {
                return Err(format!(
                    "Invalid format for prompt definition {key}, check the toml"
                ));
            };
            let arguments: Option<Vec<PromptArgument>> = op_table
                .get("arguments")
                .and_then(|arg| arg.as_array())
                .map(|args| {
                    args.iter()
                        .map(|argument| {
                            let table = argument.as_table().cloned().unwrap_or_default();
                            PromptArgument {
                                name: mandatory_extractor(&table, key, &"name".to_string())
                                    .unwrap_or_default(),
                                description: optional_extractor(&table, &"description".to_string()),
                                required: optional_extractor(&table, &"required".to_string())
                                    .and_then(|s| s.parse().ok()),
                            }
                        })
                        .collect()
                });
            let prompt = Prompt {
                name: format!(
                    "{plugin_name}.{}",
                    mandatory_extractor(op_table, key, &"name".to_string())?
                ),
                description: optional_extractor(op_table, &"description".to_string()),
                arguments: arguments,
            };
            let content: Vec<PromptMessage> = op_table
                .get("messages")
                .and_then(|s| s.as_array())
                .map(|messages| {
                    messages
                        .iter()
                        .map(|message| {
                            let table = message.as_table().cloned().unwrap_or_default();
                            let role = mandatory_extractor(&table, key, &"role".to_string())
                                .ok()
                                .map(|role| match role.as_str() {
                                    "assistant" => PromptMessageRole::Assistant,
                                    _ => PromptMessageRole::User,
                                })
                                .unwrap_or(PromptMessageRole::User);
                            let text = mandatory_extractor(&table, key, &"content".to_string())
                                .unwrap_or_default();
                            let content = PromptMessageContent::text(text);
                            PromptMessage { role, content }
                        })
                        .collect()
                })
                .unwrap_or_default();
            prompt_map.insert(key.to_string(), JilebiPrompt { prompt, content });
        }
        Ok(Self(prompt_map))
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
            &name,
        )?;
        let tools = Tools::try_from(
            value
                .get("tools")
                .ok_or("The tools section is mandatory".to_string())?,
            &name,
        )?;
        let prompts = Prompts::try_from(
            value
                .get("prompts")
                .ok_or("The prompts section is mandatory".to_string())?,
            &name,
        )?;
        Ok(Self {
            name,
            resources,
            tools,
            prompts,
        })
    }
}
