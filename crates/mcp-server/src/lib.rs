//! Generic MCP server skeleton built on top of [`rmcp`].
//!
//! [`Server`] implements [`rmcp::ServerHandler`] and ties together examples
//! of every MCP primitive. Implementations live in submodules:
//!
//! - [`tools`] — example tools, including a task-capable one and three
//!   server-to-client request tools (sampling, elicitation, roots).
//! - [`prompts`] — no-arg, single-arg, and multi-arg prompt examples.
//! - [`resources`] — static resource + URI-templated resources, with the
//!   handlers exposed as free functions.
//! - [`tasks`] — `tasks/list` override that surfaces completed tasks.

use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::*,
    prompt_handler,
    service::RequestContext,
    task_handler,
    task_manager::OperationProcessor,
    tool_handler,
};
use tokio::sync::Mutex;

pub mod prompts;
pub mod resources;
pub mod tasks;
pub mod tools;

/// MCP server skeleton. Clone is cheap — internal state lives behind
/// [`Arc`] / [`Mutex`].
#[derive(Clone)]
pub struct Server {
    #[allow(dead_code, reason = "read by the #[tool_handler] macro")]
    tool_router: ToolRouter<Server>,
    #[allow(dead_code, reason = "read by the #[prompt_handler] macro")]
    prompt_router: PromptRouter<Server>,
    /// Task processor used by `#[task_handler]` and the custom
    /// `tasks/list` override in [`tasks`].
    pub(crate) processor: Arc<Mutex<OperationProcessor>>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    /// Construct a new server with the default tool/prompt routers and task
    /// processor.
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
            processor: Arc::new(Mutex::new(OperationProcessor::new())),
        }
    }
}

// All three handler attributes must target the same `impl ServerHandler`
// block — each one synthesizes a different set of trait methods, so
// splitting them across multiple `impl` blocks would conflict. The
// `#[task_handler]` macro reads `self.processor` by default; rename that
// field and you'll need to pass the new name to the macro.
#[tool_handler]
#[prompt_handler]
#[task_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .enable_tasks()
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_protocol_version(ProtocolVersion::LATEST)
        .with_instructions(
            "Generic MCP server skeleton. Replace the example tools (`ping`, \
             `slow_count`, `ask_llm`, `greet_user`, `list_workspace_roots`), \
             prompts (`greeting`, `echo`, `summarize`), static resource \
             (`mem://example`), and resource templates (`echo://{message}`, \
             `greet://{language}/{name}`) with real handlers. `slow_count` \
             supports task-based (async) invocation. `ask_llm` requires \
             client sampling support, `greet_user` requires client \
             elicitation support, and `list_workspace_roots` requires \
             client roots support.",
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(resources::list_resources(self))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        resources::read_resource(self, request)
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(resources::list_resource_templates(self))
    }

    /// Override the default `tasks/list` handler.
    ///
    /// See [`tasks::list_tasks`] for the rationale.
    async fn list_tasks(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListTasksResult, McpError> {
        tasks::list_tasks(self).await
    }
}
