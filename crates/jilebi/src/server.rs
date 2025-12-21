use std::{borrow::Cow, collections::HashMap, fs, path::PathBuf, sync::Arc};

use dosa::run_code;
use handlebars::Handlebars;
use jilebi_types::{
    Plugins,
    manifest::{Manifest, SEPARATOR},
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rmcp::{
    ServerHandler,
    model::{
        GetPromptRequestParam, GetPromptResult, Implementation, InitializeRequestParam,
        InitializeResult, ListPromptsResult, ListResourcesResult, ListToolsResult,
        PaginatedRequestParam, Prompt, ProtocolVersion, Resource, ServerCapabilities, ServerInfo,
        Tool,
    },
    serde_json::json,
    service::RequestContext,
};
use tokio::{runtime::Handle, sync::RwLock};

use crate::{cli::permissions, utils::get_plugin_path};

#[derive(Debug, Clone, derive_more::Display, PartialEq)]
enum McpSection {
    Resource,
    Tool,
    Prompt,
}

fn get_plugin_and_section_name(
    name: &String,
    mcp_section: McpSection,
) -> Result<(String, String), rmcp::ErrorData> {
    let mut identifier = name.split(SEPARATOR).into_iter();
    Ok((
        identifier
            .next()
            .map(str::to_string)
            .ok_or(rmcp::ErrorData::invalid_request(
                "The name of the plugin is incorrect",
                None,
            ))?,
        identifier
            .next()
            .map(|p| p.replace(" ", "-"))
            .ok_or(rmcp::ErrorData::invalid_request(
                format!("The name of the {mcp_section} is incorrect"),
                None,
            ))?,
    ))
}

fn generate_plugin_environment(
    connection: &rusqlite::Connection,
    plugin_name: &str,
) -> Result<serde_json::Value, rmcp::ErrorData> {
    let envs = crate::cli::env::fetch_envs_from_db(&connection, &plugin_name).map_err(|e| {
        rmcp::ErrorData::internal_error(
            "Could not fetch environment variables from the jilebi DB",
            Some(serde_json::Value::String(e)),
        )
    })?;
    let mut env = serde_json::Map::new();
    env.insert("id".to_string(), json!(plugin_name));
    for plugin_env in envs.iter() {
        env.insert(plugin_env.env_name.clone(), json!(plugin_env.value));
    }
    Ok(json!(env))
}
#[derive(Clone, Debug)]
pub struct JilebiMcpServer {
    pub plugins: Plugins,
    pub db: Pool<SqliteConnectionManager>,
    pub plugin_log_dir: PathBuf,
}

impl JilebiMcpServer {
    pub fn new(
        plugins: HashMap<String, Manifest>,
        db: Pool<SqliteConnectionManager>,
        plugin_log_dir: PathBuf,
    ) -> Self {
        JilebiMcpServer {
            plugins: Arc::new(RwLock::new(plugins)),
            db,
            plugin_log_dir,
        }
    }
}

impl ServerHandler for JilebiMcpServer {
    async fn initialize(
        &self,
        request: InitializeRequestParam,
        context: RequestContext<rmcp::RoleServer>,
    ) -> Result<InitializeResult, rmcp::ErrorData> {
        if context.peer.peer_info().is_none() {
            context.peer.set_peer_info(request);
        }
        Ok(self.get_info())
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_tools()
                .enable_resources()
                .build(),
            server_info: Implementation {
                name: "Jilebi".into(),
                version: "alpha-2".into(),
                title: Some("Jilebi MCP Runtime".into()),
                icons: None,
                website_url: Some("https://jilebi.ai".into()),
            },
            instructions: None,
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListPromptsResult, rmcp::ErrorData> {
        let mut prompts: Vec<Prompt> = Vec::new();
        let plugins = self.plugins.read().await;
        for (_, plugin) in plugins.iter() {
            let mut p = plugin
                .prompts
                .values()
                .map(|jprompt| jprompt.prompt.clone())
                .collect::<Vec<_>>();
            prompts.append(&mut p);
        }
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        context: RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        tracing::debug!("request ID for get_prompt: {}", context.id);
        let handlerbars = Handlebars::new();
        let (plugin_name, prompt_name) =
            get_plugin_and_section_name(&request.name, McpSection::Prompt)?;
        tracing::debug!("Plugin: {} Prompt: {}", plugin_name, prompt_name);
        let plugins = self.plugins.read().await;
        let plugin = plugins
            .get(&plugin_name)
            .ok_or(rmcp::ErrorData::invalid_request(
                "The plugin name provided is either invalid or has been removed",
                None,
            ))?;
        let mut prompt =
            plugin
                .prompts
                .get(&prompt_name)
                .cloned()
                .ok_or(rmcp::ErrorData::invalid_request(
                    "The prompt name provided is either invalid or has been removed",
                    None,
                ))?;
        let prompt_arguments = request.arguments.unwrap_or_default();
        for content in prompt.content.iter_mut() {
            let new_content = match content.content.clone() {
                rmcp::model::PromptMessageContent::Text { text } => {
                    let parsed_text = handlerbars
                        .render_template(&text, &prompt_arguments)
                        .map_err(|e| {
                            tracing::error!("error parsing prompt handlerbars: {}", e);
                            rmcp::ErrorData::internal_error(
                                "error parsing prompt handlerbars",
                                None,
                            )
                        })?;
                    rmcp::model::PromptMessageContent::Text { text: parsed_text }
                }
                s => s,
            };

            *content = rmcp::model::PromptMessage {
                content: new_content,
                role: content.role.clone(),
            };
        }
        Ok(GetPromptResult {
            description: prompt.prompt.description.clone(),
            messages: prompt.content.clone(),
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let connection = self.db.get().map_err(|e| {
            tracing::error!("Could not open the jilebi DB: {}", e);
            rmcp::ErrorData::internal_error(
                "Could not open the jilebi DB",
                Some(serde_json::Value::String(e.to_string())),
            )
        })?;
        let plugins = self.plugins.read().await;

        let (plugin_name, tool_name) =
            get_plugin_and_section_name(&request.name.into_owned(), McpSection::Tool)?;
        let code_path = get_plugin_path(&connection, &plugin_name)
            .map_err(|err| {
                tracing::error!(
                    "Could not fetch path to plugin directory in tool call: {}",
                    err
                );
                rmcp::ErrorData::internal_error(
                    "Could not fetch path to plugin directory in tool call",
                    Some(serde_json::Value::String(err.to_string())),
                )
            })?
            .join("index.js");
        let code = fs::read_to_string(code_path).map_err(|e| {
            tracing::error!("Could not find JS file: {}", e);
            rmcp::ErrorData::internal_error(
                "Could not find the JS file that has the function",
                None,
            )
        })?;
        let plugin = plugins
            .get(&plugin_name)
            .cloned()
            .ok_or(rmcp::ErrorData::invalid_request(
                "The plugin name provided is either invalid or has been removed",
                None,
            ))?;
        let tool =
            plugin
                .tools
                .get(&tool_name)
                .cloned()
                .ok_or(rmcp::ErrorData::invalid_request(
                    "The tool name provided is either invalid or has been removed",
                    None,
                ))?;

        let handle = Handle::current();
        let env = generate_plugin_environment(&connection, &plugin_name)?;
        let permissions =
            permissions::fetch_permissions_for_entity(&connection, &plugin_name, &tool_name)
                .map_err(|e| {
                    tracing::error!("Could not fetch permissions for tool {}: {}", tool_name, e);
                    rmcp::ErrorData::internal_error(
                        "Could not fetch permissions for tool",
                        Some(serde_json::Value::String(e.to_string())),
                    )
                })?;
        let log_path = self.plugin_log_dir.clone();
        let result = handle
            .spawn_blocking(move || {
                let args = request.arguments.unwrap_or_default();
                run_code::<rmcp::model::CallToolResult>(
                    &plugin_name,
                    &code,
                    &tool.function,
                    json!(args),
                    env,
                    &permissions,
                    &log_path,
                )
                .map_err(|e| {
                    rmcp::ErrorData::internal_error(
                        "The function call for this tool failed",
                        Some(json!(e)),
                    )
                })
            })
            .await
            .map_err(|e| {
                tracing::error!("An error occurred while joining the thread: {e}");
                rmcp::ErrorData::internal_error("The function call for this tool failed", None)
            })??;

        Ok(result)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
        let mut tools: Vec<Tool> = Vec::new();
        tracing::trace!("Listing tools");
        let plugins = self.plugins.read().await;
        tracing::trace!("Plugins loaded: {:?}", *plugins);
        for (_, plugin) in plugins.iter() {
            let mut p = plugin
                .tools
                .values()
                .map(|tool| Tool {
                    name: Cow::from(tool.tool.name.clone()),
                    ..tool.tool.clone()
                })
                .collect::<Vec<_>>();
            tools.append(&mut p);
        }
        Ok(ListToolsResult {
            next_cursor: None,
            tools,
            meta: None,
        })
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        let mut resources: Vec<Resource> = Vec::new();
        let plugins = self.plugins.read().await;
        for (_, plugin) in plugins.iter() {
            let mut p = plugin
                .resources
                .values()
                .map(|resource| resource.resource.clone())
                .collect::<Vec<_>>();
            resources.append(&mut p);
        }
        Ok(ListResourcesResult {
            next_cursor: None,
            resources,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ReadResourceResult, rmcp::ErrorData> {
        let connection = self.db.get().map_err(|e| {
            tracing::error!("Could not open the jilebi DB: {}", e);
            rmcp::ErrorData::internal_error(
                "Could not open the jilebi DB",
                Some(serde_json::Value::String(e.to_string())),
            )
        })?;
        let plugins = self.plugins.read().await;

        let (plugin_name, resource_name) =
            get_plugin_and_section_name(&request.uri, McpSection::Resource)?;
        let code_path = get_plugin_path(&connection, &plugin_name)
            .map_err(|err| {
                tracing::error!(
                    "Could not fetch path to plugin directory in tool call: {}",
                    err
                );
                rmcp::ErrorData::internal_error(
                    "Could not fetch path to plugin directory in tool call",
                    Some(serde_json::Value::String(err.to_string())),
                )
            })?
            .join("index.js");

        let code = fs::read_to_string(&code_path).map_err(|e| {
            tracing::error!("Could not find JS file at {}: {}", &code_path.display(), e);
            rmcp::ErrorData::internal_error(
                "Could not find the JS file that has the function",
                None,
            )
        })?;
        let plugin = plugins
            .get(&plugin_name)
            .cloned()
            .ok_or(rmcp::ErrorData::invalid_request(
                "The plugin name provided is either invalid or has been removed",
                None,
            ))?;
        let resource = plugin.resources.get(&resource_name).cloned().ok_or(
            rmcp::ErrorData::invalid_request(
                "The resource name provided is either invalid or has been removed",
                None,
            ),
        )?;
        let env = generate_plugin_environment(&connection, &plugin_name)?;
        let permissions =
            permissions::fetch_permissions_for_entity(&connection, &plugin_name, &resource_name)
                .map_err(|e| {
                    tracing::error!(
                        "Could not fetch permissions for resource {}: {}",
                        resource_name,
                        e
                    );
                    rmcp::ErrorData::internal_error(
                        "Could not fetch permissions for resource",
                        Some(serde_json::Value::String(e.to_string())),
                    )
                })?;
        let handle = Handle::current();
        let log_path = self.plugin_log_dir.clone();
        let result = handle
            .spawn_blocking(move || {
                run_code::<rmcp::model::ReadResourceResult>(
                    &plugin_name,
                    &code,
                    &resource.function,
                    json!({}),
                    env,
                    &permissions,
                    &log_path,
                )
                .map_err(|e| {
                    tracing::error!("Error while running the resource function {}", e);
                    rmcp::ErrorData::internal_error(
                        "The function call for this resource failed",
                        Some(json!(e)),
                    )
                })
            })
            .await
            .map_err(|e| {
                tracing::error!("An error occurred while joining the thread: {e}");
                rmcp::ErrorData::internal_error("The function call for this resource failed", None)
            })??;

        Ok(result)
    }
}
