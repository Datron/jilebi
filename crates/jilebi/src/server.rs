use std::{borrow::Cow, collections::HashMap, fs, sync::Arc};

use dosa::run_code;
use jilebi_types::{Plugins, plugin::Manifest};
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

#[derive(Debug, Clone, derive_more::Display, PartialEq)]
enum McpSection {
    Resource,
    Tool,
    Prompt,
}

fn get_plugin_and_section_name(
    name: &String,
    mcp_section: McpSection,
) -> Result<(String, String), rmcp::Error> {
    let mut identifier = name.split("_").into_iter();
    Ok((
        identifier
            .next()
            .map(str::to_string)
            .ok_or(rmcp::Error::invalid_request(
                "The name of the plugin is incorrect",
                None,
            ))?,
        identifier
            .next()
            .map(|p| p.replace(" ", "-"))
            .ok_or(rmcp::Error::invalid_request(
                format!("The name of the {mcp_section} is incorrect"),
                None,
            ))?,
    ))
}

#[derive(Clone, Debug, Default)]
pub struct JilebiMcpServer {
    pub plugins: Plugins,
    pub plugin_dir: String,
}

impl JilebiMcpServer {
    pub fn new(plugins: HashMap<String, Manifest>, plugin_dir: String) -> Self {
        JilebiMcpServer {
            plugins: Arc::new(RwLock::new(plugins)),
            plugin_dir,
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
    ) -> Result<InitializeResult, rmcp::Error> {
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
    ) -> Result<ListPromptsResult, rmcp::Error> {
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
    ) -> Result<GetPromptResult, rmcp::Error> {
        tracing::debug!("request ID for get_prompt: {}", context.id);
        let (plugin_name, prompt_name) =
            get_plugin_and_section_name(&request.name, McpSection::Prompt)?;
        tracing::debug!("Plugin: {} Prompt: {}", plugin_name, prompt_name);
        let plugins = self.plugins.read().await;
        let plugin = plugins
            .get(&plugin_name)
            .ok_or(rmcp::Error::invalid_request(
                "The plugin name provided is either invalid or has been removed",
                None,
            ))?;
        let prompt = plugin
            .prompts
            .get(&prompt_name)
            .ok_or(rmcp::Error::invalid_request(
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
    ) -> Result<rmcp::model::CallToolResult, rmcp::Error> {
        let plugins = self.plugins.read().await;

        let (plugin_name, tool_name) =
            get_plugin_and_section_name(&request.name.into_owned(), McpSection::Tool)?;
        let code_path = format!("{}/{}/main.js", self.plugin_dir, &plugin_name);
        let code = fs::read_to_string(code_path).map_err(|e| {
            tracing::error!("Could not find JS file: {}", e);
            rmcp::Error::internal_error("Could not find the JS file that has the function", None)
        })?;
        let plugin = plugins
            .get(&plugin_name)
            .cloned()
            .ok_or(rmcp::Error::invalid_request(
                "The plugin name provided is either invalid or has been removed",
                None,
            ))?;
        let tool = plugin
            .tools
            .get(&tool_name)
            .cloned()
            .ok_or(rmcp::Error::invalid_request(
                "The tool name provided is either invalid or has been removed",
                None,
            ))?;
        let handle = Handle::current();

        let result = handle
            .spawn_blocking(move || {
                let args = request.arguments.unwrap_or_default();
                run_code::<rmcp::model::CallToolResult>(
                    &plugin_name,
                    &code,
                    &tool.function,
                    json!(args),
                    &tool.permissions,
                )
                .map_err(|e| {
                    rmcp::Error::internal_error(
                        "The function call for this tool failed",
                        Some(json!(e)),
                    )
                })
            })
            .await
            .map_err(|e| {
                tracing::error!("An error occurred while joining the thread: {e}");
                rmcp::Error::internal_error("The function call for this tool failed", None)
            })??;

        Ok(result)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::Error> {
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
    ) -> Result<ListResourcesResult, rmcp::Error> {
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
    ) -> Result<rmcp::model::ReadResourceResult, rmcp::Error> {
        let plugins = self.plugins.read().await;

        let (plugin_name, resource_name) =
            get_plugin_and_section_name(&request.uri, McpSection::Resource)?;
        let code_path = format!("{}/{}/main.js", self.plugin_dir, &plugin_name);

        let code = fs::read_to_string(code_path).map_err(|e| {
            tracing::error!("Could not find JS file: {}", e);
            rmcp::Error::internal_error("Could not find the JS file that has the function", None)
        })?;
        let plugin = plugins
            .get(&plugin_name)
            .cloned()
            .ok_or(rmcp::Error::invalid_request(
                "The plugin name provided is either invalid or has been removed",
                None,
            ))?;
        let resource =
            plugin
                .resources
                .get(&resource_name)
                .cloned()
                .ok_or(rmcp::Error::invalid_request(
                    "The resource name provided is either invalid or has been removed",
                    None,
                ))?;
        let handle = Handle::current();

        let result = handle
            .spawn_blocking(move || {
                run_code::<rmcp::model::ReadResourceResult>(
                    &plugin_name,
                    &code,
                    &resource.function,
                    json!({}),
                    &resource.permissions,
                )
                .map_err(|e| {
                    tracing::error!("Error while running the resource function {}", e);
                    rmcp::Error::internal_error(
                        "The function call for this resource failed",
                        Some(json!(e)),
                    )
                })
            })
            .await
            .map_err(|e| {
                tracing::error!("An error occurred while joining the thread: {e}");
                rmcp::Error::internal_error("The function call for this resource failed", None)
            })??;

        Ok(result)
    }
}
