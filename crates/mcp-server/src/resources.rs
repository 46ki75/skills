//! Resource handlers (`resources/list`, `resources/read`,
//! `resources/templates/list`) for [`Server`].
//!
//! Resources are not driven by a macro router in `rmcp`; the
//! [`ServerHandler`] trait impl in [`crate`] dispatches to these free
//! functions.

use rmcp::{ErrorData as McpError, model::*};
use serde_json::json;

use crate::Server;

const EXAMPLE_RESOURCE_URI: &str = "mem://example";
const EXAMPLE_RESOURCE_NAME: &str = "example";
const EXAMPLE_RESOURCE_BODY: &str = "Example in-memory resource served by the mcp-server skeleton.";

const ECHO_RESOURCE_SCHEME: &str = "echo://";
const ECHO_RESOURCE_TEMPLATE: &str = "echo://{message}";

const GREET_RESOURCE_SCHEME: &str = "greet://";
const GREET_RESOURCE_TEMPLATE: &str = "greet://{language}/{name}";

/// Body of `resources/list`. Returns the single static example resource.
pub fn list_resources(_server: &Server) -> ListResourcesResult {
    let resource: Resource =
        RawResource::new(EXAMPLE_RESOURCE_URI, EXAMPLE_RESOURCE_NAME.to_string()).no_annotation();
    ListResourcesResult {
        resources: vec![resource],
        next_cursor: None,
        meta: None,
    }
}

/// Body of `resources/read`. Dispatches by URI scheme: `mem://example`,
/// `echo://{message}`, or `greet://{language}/{name}`.
pub fn read_resource(
    _server: &Server,
    request: ReadResourceRequestParams,
) -> Result<ReadResourceResult, McpError> {
    if request.uri == EXAMPLE_RESOURCE_URI {
        return Ok(ReadResourceResult::new(vec![ResourceContents::text(
            EXAMPLE_RESOURCE_BODY,
            request.uri.clone(),
        )]));
    }

    if let Some(message) = request.uri.strip_prefix(ECHO_RESOURCE_SCHEME) {
        // `echo://` is deliberately permissive: anything after the scheme
        // (including slashes — e.g. `echo://a/b`) is echoed verbatim. Only
        // a fully-empty payload is rejected. `greet://` below is stricter
        // because its template has a fixed `{language}/{name}` shape.
        if message.is_empty() {
            return Err(McpError::invalid_params(
                "echo:// requires a non-empty message segment",
                Some(json!({ "uri": request.uri })),
            ));
        }
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

/// Body of `resources/templates/list`. Returns the `echo://` and `greet://`
/// resource templates.
pub fn list_resource_templates(_server: &Server) -> ListResourceTemplatesResult {
    let echo_template: ResourceTemplate = RawResourceTemplate::new(ECHO_RESOURCE_TEMPLATE, "echo")
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
    ListResourceTemplatesResult {
        resource_templates: vec![echo_template, greet_template],
        next_cursor: None,
        meta: None,
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
