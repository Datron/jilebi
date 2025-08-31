use std::{borrow::Cow, collections::HashMap, fs, path::PathBuf, sync::Arc};

use dosa::run_code;
use jilebi_types::{
    Plugins,
    plugin::{Manifest, SEPARATOR},
};
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

use crate::generate_path;

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
    base_dir: &PathBuf,
    plugin_name: &str,
) -> Result<serde_json::Value, rmcp::ErrorData> {
    let db_path =
        generate_path(base_dir, "jilebi.db3", false).map_err(|e| {
            tracing::error!("Could not generate the path for the jilebi DB: {}", e);
            rmcp::ErrorData::internal_error(
                "Could not fetch environment variables from the jilebi DB",
                None,
            )
        })?;
    let connection = rusqlite::Connection::open(db_path).map_err(|e| {
        tracing::error!("Could not open the jilebi DB: {}", e);
        rmcp::ErrorData::internal_error(
            "Could not open the jilebi DB",
            Some(serde_json::Value::String(e.to_string())),
        )
    })?;
    let envs = crate::cli::env::fetch_envs(&connection, &plugin_name).map_err(|e| {
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
#[derive(Clone, Debug, Default)]
pub struct JilebiMcpServer {
    pub plugins: Plugins,
    pub plugin_dir: PathBuf,
    pub plugin_log_dir: PathBuf,
    pub jilebi_base_dir: PathBuf,
}

impl JilebiMcpServer {
    pub fn new(
        plugins: HashMap<String, Manifest>,
        plugin_dir: PathBuf,
        plugin_log_dir: PathBuf,
		jilebi_base_dir: PathBuf,
    ) -> Self {
        JilebiMcpServer {
            plugins: Arc::new(RwLock::new(plugins)),
            plugin_dir,
            plugin_log_dir,
            jilebi_base_dir,
        }
    }

    // pub async fn add(&mut self, plugin: Manifest) {
    //     self.plugins
    //         .write()
    //         .await
    //         .insert(plugin.name.clone(), plugin);
    // }
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
                version: "aplha-1".into(),
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
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        context: RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        tracing::debug!("request ID for get_prompt: {}", context.id);
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
        let prompt = plugin
            .prompts
            .get(&prompt_name)
            .ok_or(rmcp::ErrorData::invalid_request(
                "The prompt name provided is either invalid or has been removed",
                None,
            ))?;
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
        let plugins = self.plugins.read().await;

        let (plugin_name, tool_name) =
            get_plugin_and_section_name(&request.name.into_owned(), McpSection::Tool)?;
        let code_path = self.plugin_dir.join(&plugin_name).join("index.js");
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
        let env = generate_plugin_environment(&self.jilebi_base_dir, &plugin_name)?;
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
                    &tool.permissions,
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
        tracing::info!("Listing tools");
        let plugins = self.plugins.read().await;
        tracing::info!("Plugins loaded: {:?}", *plugins);
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
        })
    }

    async fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ReadResourceResult, rmcp::ErrorData> {
        let plugins = self.plugins.read().await;

        let (plugin_name, resource_name) =
            get_plugin_and_section_name(&request.uri, McpSection::Resource)?;
        let code_path = self.plugin_dir.join(&plugin_name).join("index.js");

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
        let env = generate_plugin_environment(&self.jilebi_base_dir, &plugin_name)?;
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
                    &resource.permissions,
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
