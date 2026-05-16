//! Prompt definitions for [`Server`].
//!
//! All `#[prompt]`-annotated methods live in a single `#[prompt_router] impl
//! Server` block so that the macro can generate one
//! `Server::prompt_router()` associated function.

use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
    prompt, prompt_router, schemars,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Server;

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

#[prompt_router(vis = "pub(crate)")]
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
        Ok(GetPromptResult::new(messages).with_description(format!("Echo of: {}", args.message)))
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
