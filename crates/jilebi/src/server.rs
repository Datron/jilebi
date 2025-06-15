use std::{collections::HashMap, sync::Arc};

use jilebi_types::{Plugins, plugin::Manifest};
use rmcp::{
    ServerHandler,
    model::{
        GetPromptRequestParam, GetPromptResult, Implementation, InitializeRequestParam,
        InitializeResult, ListPromptsResult, ListResourcesResult, ListToolsResult,
        PaginatedRequestParam, Prompt, ProtocolVersion, Resource, ResourceTemplate,
        ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct JilebiMcpServer {
    pub plugins: Plugins,
}

impl JilebiMcpServer {
    pub fn new() -> Self {
        JilebiMcpServer {
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add(&mut self, plugin: Manifest) {
        self.plugins
            .write()
            .await
            .insert(plugin.name.clone(), plugin);
    }
}

impl ServerHandler for JilebiMcpServer {
    fn initialize(
        &self,
        request: InitializeRequestParam,
        context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<InitializeResult, rmcp::Error>> + Send + '_ {
        if context.peer.peer_info().is_none() {
            context.peer.set_peer_info(request);
        }
        std::future::ready(Ok(self.get_info()))
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
                .map(|jprompt| Prompt {
                    name: format!("{}.{}", plugin.name, jprompt.prompt.name),
                    ..jprompt.prompt.clone()
                })
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
        let mut identifier = request.name.split(".").into_iter();
        let (plugin_name, prompt_name) = (
            identifier
                .next()
                .map(str::to_string)
                .ok_or(rmcp::Error::invalid_request(
                    "The name of the prompt is incorrect",
                    None,
                ))?,
            identifier
                .next()
                .map(|p| p.replace(" ", "-"))
                .ok_or(rmcp::Error::invalid_request(
                    "The name of the prompt is incorrect",
                    None,
                ))?,
        );
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
        _request: rmcp::model::CallToolRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::Error> {
        Err(rmcp::Error::method_not_found::<
            rmcp::model::CallToolRequestMethod,
        >())
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
                .map(|tool| tool.tool.clone())
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
                .filter_map(|resource| resource.resource.clone().left())
                .collect::<Vec<_>>();
            resources.append(&mut p);
        }
        Ok(ListResourcesResult {
            next_cursor: None,
            resources,
        })
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListResourceTemplatesResult, rmcp::Error> {
        let mut resource_templates: Vec<ResourceTemplate> = Vec::new();
        let plugins = self.plugins.read().await;
        for (_, plugin) in plugins.iter() {
            let mut p = plugin
                .resources
                .values()
                .filter_map(|resource| resource.resource.clone().right())
                .collect::<Vec<_>>();
            resource_templates.append(&mut p);
        }
        Ok(rmcp::model::ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates,
        })
    }

    async fn read_resource(
        &self,
        _request: rmcp::model::ReadResourceRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ReadResourceResult, rmcp::Error> {
        Err(rmcp::Error::method_not_found::<
            rmcp::model::ReadResourceRequestMethod,
        >())
    }
}
