use std::{collections::HashMap, sync::Arc};

use jilebi_types::{Plugins, plugin::Manifest};
use rmcp::{
    ServerHandler,
    model::{
        GetPromptRequestMethod, GetPromptRequestParam, GetPromptResult, Implementation,
        InitializeRequestParam, InitializeResult, ListPromptsResult, PaginatedRequestParam, Prompt,
        PromptArgument, ProtocolVersion, ServerCapabilities, ServerInfo,
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
                .enable_prompts_list_changed()
                .build(),
            server_info: Implementation {
                name: "Jilebi".into(),
                version: "aplha-1".into(),
            },
            instructions: None,
        }
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<ListPromptsResult, rmcp::Error>> + Send + '_ {
        let mut prompts: Vec<Prompt> = Vec::new();
        let plugins = self.plugins.blocking_read();
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
        std::future::ready(Ok(ListPromptsResult {
            next_cursor: None,
            prompts,
        }))
    }

    fn get_prompt(
        &self,
        _request: GetPromptRequestParam,
        _context: RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<GetPromptResult, rmcp::Error>> + Send + '_ {
        std::future::ready(Err(
            rmcp::Error::method_not_found::<GetPromptRequestMethod>(),
        ))
    }
}
