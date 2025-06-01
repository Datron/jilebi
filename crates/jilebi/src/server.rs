use std::{borrow::Cow, collections::HashMap, sync::Arc};

use jilebi_types::{Plugins, plugin::Manifest};
use rmcp::{
    ServerHandler,
    model::{
        GetPromptRequestMethod, GetPromptRequestParam, GetPromptResult, Implementation,
        InitializeRequestParam, InitializeResult, ListPromptsResult, ListToolsResult,
        PaginatedRequestParam, Prompt, PromptArgument, ProtocolVersion, ServerCapabilities,
        ServerInfo, Tool,
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
                .map(|prompt| Prompt {
                    name: format!("{}.{}", plugin.name, prompt.name),
                    description: prompt.description.clone(),
                    arguments: prompt.arguments.clone().and_then(|args| {
                        let new_args = args
                            .iter()
                            .map(|arg| PromptArgument {
                                name: arg.name.clone(),
                                description: arg.description.clone(),
                                required: arg.required,
                            })
                            .collect::<Vec<_>>();
                        Some(new_args)
                    }),
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
        _request: GetPromptRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> Result<GetPromptResult, rmcp::Error> {
        Err(rmcp::Error::method_not_found::<GetPromptRequestMethod>())
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
                .map(|tool| Tool {
                    name: Cow::from(format!("{}.{}", plugin.name, tool.name)),
                    description: tool.description.clone().map(|i| Cow::from(i)),
                    input_schema: Arc::new(
                        tool.input_schema
                            .as_object()
                            .map(|s| s.to_owned())
                            .unwrap_or_default(),
                    ),
                    annotations: None,
                })
                .collect::<Vec<_>>();
            tools.append(&mut p);
        }
        Ok(ListToolsResult {
            next_cursor: None,
            tools,
        })
    }
}
