//! Generic MCP server skeleton built on top of [`rmcp`].
//!
//! Provides a [`Server`] type that implements [`rmcp::ServerHandler`] with a
//! single example tool (`ping`), a single example prompt (`greeting`), and a
//! single example resource (`mem://example`). Use it as a starting point and
//! add real tools, prompts, or resources as needed.

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::*,
    prompt, prompt_handler, prompt_router,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde_json::json;

const EXAMPLE_RESOURCE_URI: &str = "mem://example";
const EXAMPLE_RESOURCE_NAME: &str = "example";
const EXAMPLE_RESOURCE_BODY: &str =
    "Example in-memory resource served by the mcp-server skeleton.";

/// MCP server skeleton. Clone is cheap — internal state lives behind
/// [`std::sync::Arc`] when added.
#[derive(Clone)]
pub struct Server {
    #[allow(dead_code, reason = "read by the #[tool_handler] macro")]
    tool_router: ToolRouter<Server>,
    #[allow(dead_code, reason = "read by the #[prompt_handler] macro")]
    prompt_router: PromptRouter<Server>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl Server {
    /// Construct a new server with the default tool and prompt routers.
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    /// Example tool. Replace with real tools.
    #[tool(description = "Health-check tool. Returns 'pong'.")]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }
}

#[prompt_router]
#[allow(
    missing_docs,
    reason = "the #[prompt] macro generates associated items without docs"
)]
impl Server {
    /// Example prompt. Replace with real prompts.
    #[prompt(
        name = "greeting",
        description = "A simple greeting prompt with no arguments."
    )]
    async fn greeting(&self) -> Result<GetPromptResult, McpError> {
        let messages = vec![
            PromptMessage::new_text(
                PromptMessageRole::User,
                "Hello! I'd like to start our conversation.",
            ),
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                "Hello! I'm here to help. What would you like to discuss today?",
            ),
        ];
        Ok(GetPromptResult::new(messages).with_description("Canned greeting exchange."))
    }
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_protocol_version(ProtocolVersion::LATEST)
        .with_instructions(
            "Generic MCP server skeleton. Replace the example `ping` tool, \
             `greeting` prompt, and `mem://example` resource with real handlers.",
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resource: Resource =
            RawResource::new(EXAMPLE_RESOURCE_URI, EXAMPLE_RESOURCE_NAME.to_string())
                .no_annotation();
        Ok(ListResourcesResult {
            resources: vec![resource],
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match request.uri.as_str() {
            EXAMPLE_RESOURCE_URI => Ok(ReadResourceResult::new(vec![ResourceContents::text(
                EXAMPLE_RESOURCE_BODY,
                request.uri.clone(),
            )])),
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({ "uri": request.uri })),
            )),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            resource_templates: Vec::new(),
            next_cursor: None,
            meta: None,
        })
    }
}
