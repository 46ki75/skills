//! Generic MCP server skeleton built on top of [`rmcp`].
//!
//! Provides a [`Server`] type that implements [`rmcp::ServerHandler`] with
//! one example tool (`ping`), three example prompts (`greeting` no-arg,
//! `echo` single-arg, `summarize` multi-arg), one static resource
//! (`mem://example`), and two parameterized resource templates
//! (`echo://{message}`, `greet://{language}/{name}`). Use it as a starting
//! point and add real tools, prompts, or resources as needed.

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{prompt::PromptRouter, tool::ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    prompt, prompt_handler, prompt_router, schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

const EXAMPLE_RESOURCE_URI: &str = "mem://example";
const EXAMPLE_RESOURCE_NAME: &str = "example";
const EXAMPLE_RESOURCE_BODY: &str =
    "Example in-memory resource served by the mcp-server skeleton.";

const ECHO_RESOURCE_SCHEME: &str = "echo://";
const ECHO_RESOURCE_TEMPLATE: &str = "echo://{message}";

const GREET_RESOURCE_SCHEME: &str = "greet://";
const GREET_RESOURCE_TEMPLATE: &str = "greet://{language}/{name}";

/// Arguments for the `echo` prompt.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EchoPromptArgs {
    /// The message to echo back.
    pub message: String,
}

/// Arguments for the `summarize` prompt. Demonstrates multiple arguments,
/// including required, optional, and non-string types.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SummarizePromptArgs {
    /// The topic to summarize.
    pub topic: String,
    /// Number of bullet points to produce.
    pub bullet_count: u8,
    /// Optional tone: "neutral", "formal", "casual", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone: Option<String>,
}

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

    /// Example prompt with a single typed argument.
    #[prompt(
        name = "echo",
        description = "Echo the given message back as a user prompt."
    )]
    async fn echo(
        &self,
        Parameters(args): Parameters<EchoPromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        let messages = vec![PromptMessage::new_text(
            PromptMessageRole::User,
            args.message.clone(),
        )];
        Ok(GetPromptResult::new(messages)
            .with_description(format!("Echo of: {}", args.message)))
    }

    /// Example prompt with multiple typed arguments (required + optional,
    /// mixed types).
    #[prompt(
        name = "summarize",
        description = "Ask for a bullet-point summary of a topic, with optional tone."
    )]
    async fn summarize(
        &self,
        Parameters(args): Parameters<SummarizePromptArgs>,
    ) -> Result<GetPromptResult, McpError> {
        if args.bullet_count == 0 {
            return Err(McpError::invalid_params(
                "bullet_count must be at least 1",
                Some(json!({ "bullet_count": args.bullet_count })),
            ));
        }

        let tone = args.tone.as_deref().unwrap_or("neutral");
        let system = format!(
            "You are a concise writer. Respond in a {tone} tone using exactly \
             {count} bullet points.",
            tone = tone,
            count = args.bullet_count,
        );
        let user = format!("Summarize the following topic: {}", args.topic);

        let messages = vec![
            PromptMessage::new_text(PromptMessageRole::Assistant, system),
            PromptMessage::new_text(PromptMessageRole::User, user),
        ];
        Ok(GetPromptResult::new(messages).with_description(format!(
            "{count}-bullet {tone} summary of '{topic}'",
            count = args.bullet_count,
            tone = tone,
            topic = args.topic,
        )))
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
            "Generic MCP server skeleton. Replace the example tool (`ping`), \
             prompts (`greeting`, `echo`, `summarize`), static resource \
             (`mem://example`), and resource templates (`echo://{message}`, \
             `greet://{language}/{name}`) with real handlers.",
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
        if request.uri == EXAMPLE_RESOURCE_URI {
            return Ok(ReadResourceResult::new(vec![ResourceContents::text(
                EXAMPLE_RESOURCE_BODY,
                request.uri.clone(),
            )]));
        }

        if let Some(message) = request.uri.strip_prefix(ECHO_RESOURCE_SCHEME) {
            return Ok(ReadResourceResult::new(vec![ResourceContents::text(
                message,
                request.uri.clone(),
            )]));
        }

        if let Some(rest) = request.uri.strip_prefix(GREET_RESOURCE_SCHEME) {
            let mut parts = rest.splitn(2, '/');
            let language = parts.next().filter(|s| !s.is_empty());
            let name = parts.next().filter(|s| !s.is_empty());
            return match (language, name) {
                (Some(language), Some(name)) => {
                    let greeting = render_greeting(language, name);
                    Ok(ReadResourceResult::new(vec![ResourceContents::text(
                        greeting,
                        request.uri.clone(),
                    )]))
                }
                _ => Err(McpError::invalid_params(
                    "greet:// requires both a language and a name segment",
                    Some(json!({ "uri": request.uri })),
                )),
            };
        }

        Err(McpError::resource_not_found(
            "resource_not_found",
            Some(json!({ "uri": request.uri })),
        ))
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        let echo_template: ResourceTemplate =
            RawResourceTemplate::new(ECHO_RESOURCE_TEMPLATE, "echo")
                .with_description("Reads back whatever appears after `echo://` as plain text.")
                .with_mime_type("text/plain")
                .no_annotation();
        let greet_template: ResourceTemplate =
            RawResourceTemplate::new(GREET_RESOURCE_TEMPLATE, "greet")
                .with_description(
                    "Returns a localized greeting for the given language and name. \
                     Supported languages: en, ja, es, fr.",
                )
                .with_mime_type("text/plain")
                .no_annotation();
        Ok(ListResourceTemplatesResult {
            resource_templates: vec![echo_template, greet_template],
            next_cursor: None,
            meta: None,
        })
    }
}

fn render_greeting(language: &str, name: &str) -> String {
    let hello = match language.to_ascii_lowercase().as_str() {
        "en" => "Hello",
        "ja" => "こんにちは",
        "es" => "Hola",
        "fr" => "Bonjour",
        _ => "Hello",
    };
    format!("{hello}, {name}!")
}
