# Server: tools

Tools are the most common MCP primitive. `rmcp` exposes them through
the `#[tool]` attribute on async methods inside a `#[tool_router]
impl` block. The macros handle JSON-RPC dispatch, argument
deserialization, and JSON Schema generation.

## When to read this

- Authoring or modifying a `#[tool]` method.
- A tool needs typed arguments and you're not sure how `Parameters<T>`
  wires in.
- A tool needs to make a server-to-client request (sampling,
  elicitation, roots) and you need `RequestContext<RoleServer>`.
- You want a tool that can run synchronously *or* as an async task
  (SEP-1319).

The canonical local example is `crates/mcp-server/src/tools.rs`.

## The minimum tool

```rust
#[tool_router]
impl Server {
    #[tool(description = "Health-check tool. Returns 'pong'.")]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }
}
```

`description` shows up in `tools/list` and in MCP Inspector's tool
listing. Keep it short, action-oriented, and self-contained — a model
may pick which tool to call based on this string alone.

Tool methods take `&self` and return `Result<CallToolResult, McpError>`.
The macros take care of marshalling them to JSON-RPC. `CallToolResult`
shape:

| Field                        | Constructor                                                          |
| ---------------------------- | -------------------------------------------------------------------- |
| Successful text reply        | `CallToolResult::success(vec![Content::text("...")])`                |
| Successful image reply       | `CallToolResult::success(vec![Content::image(base64, "image/png")])` |
| Error returned to the client | `CallToolResult::error(vec![Content::text("...")])`                  |

Returning `Err(McpError::...)` from the method bubbles up as a JSON-RPC
error — usually you want `CallToolResult::error(...)` instead so the
client sees a structured error result (`is_error: true`) rather than a
protocol-level failure.

## Typed arguments with `Parameters<T>`

Wrap the argument struct in `Parameters<T>` and the macros take care of
the rest. `T` must derive `Deserialize` and `JsonSchema`:

```rust
use rmcp::{handler::server::wrapper::Parameters, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SlowCountArgs {
    /// How high to count. Each tick sleeps for SLOW_COUNT_TICK_MS.
    pub target: u8,
}

#[tool_router]
impl Server {
    #[tool(description = "Count up to `target` slowly.")]
    async fn slow_count(
        &self,
        Parameters(args): Parameters<SlowCountArgs>,
    ) -> Result<CallToolResult, McpError> {
        for _ in 1..=args.target {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        Ok(CallToolResult::success(vec![Content::text(args.target.to_string())]))
    }
}
```

The macro emits a JSON Schema for `SlowCountArgs` and registers it as
the tool's `inputSchema`. Doc-comments on the struct fields surface as
`description` entries in the schema, which clients (and LLMs) use to
understand how to call the tool. **Write them.**

Default values, optional fields, and nullable strings work the standard
serde way:

```rust
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SummarizeArgs {
    pub topic: String,
    pub bullet_count: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone: Option<String>,
}
```

## `RequestContext<RoleServer>` and server-to-client requests

Add `ctx: RequestContext<RoleServer>` as a method parameter to get
access to:

- `ctx.peer` — the `Peer<RoleServer>` that can make outgoing requests
  to the client (sampling, elicitation, roots).
- `ctx.request_id` — for correlating progress notifications.
- `ctx.meta` / `ctx.extensions` — request metadata.

The three server-to-client request types each get their own dedicated
reference file:

- **Sampling** — `ctx.peer.create_message(...)` — see
  `references/server/sampling.md`.
- **Elicitation** — `ctx.peer.elicit::<T>(...)` — see
  `references/server/elicitation.md`.
- **Roots** — `ctx.peer.list_roots()` — see
  `references/server/roots.md`.

The pattern in `crates/mcp-server/src/tools.rs` (`ask_llm`,
`greet_user`, `list_workspace_roots`) is the canonical reference for
each.

## Task-capable tools (SEP-1319)

A tool can opt in to async-task invocation by adding
`execution(task_support = "optional"|"required")` to the `#[tool]`
attribute:

```rust
#[tool(
    description = "Count up to `target` slowly (100ms per tick). Supports task-based invocation.",
    execution(task_support = "optional")
)]
async fn slow_count(/* ... */) -> Result<CallToolResult, McpError> { /* ... */ }
```

| Value        | Behavior                                                                                                  |
| ------------ | --------------------------------------------------------------------------------------------------------- |
| `"optional"` | Client *may* request task-based execution by passing `task` metadata on `tools/call`. Otherwise runs sync |
| `"required"` | Client *must* request task-based execution. Synchronous calls are rejected                                |
| (omitted)    | Synchronous only. Task-based calls are rejected                                                           |

For `"optional"` tools, the synchronous path still runs the whole tool
to completion, so clients that don't opt in to tasks (including MCP
Inspector at the time of writing) will hold the JSON-RPC request open
until the tool finishes. Keep the sync path short, or mark the tool
`"required"` so clients have to use the task path.

See `references/server/tasks.md` for the `OperationProcessor` and the
`list_tasks` override pattern.

## Calling `tools/list` programmatically

`#[tool_handler]` auto-implements `ServerHandler::list_tools`. Clients
fetch the registry via `ListToolsRequest` — there's no need to
implement it yourself. The test at
`crates/mcp-server/tests/tools.rs::list_tools_returns_the_advertised_set`
shows the round trip.

## Common patterns and gotchas

### `CallToolResult::error` vs `Err(McpError)`

Use `CallToolResult::error(vec![Content::text("...")])` for problems
the tool itself surfaces (validation failures, downstream API errors).
The client sees a successful JSON-RPC response with `is_error: true`,
which is what most MCP UIs render as "the tool ran and reported a
problem."

Use `Err(McpError::internal_error(...))` only for protocol-level
failures the client can't recover from. The client sees a JSON-RPC
error response, which most UIs render as a hard failure.

### Visibility of the router function

`#[tool_router]` generates `fn tool_router()` with the same visibility
as the `impl` block by default. If your `Server` is `pub` and you want
the router to stay `pub(crate)`, pass `vis`:

```rust
#[tool_router(vis = "pub(crate)")]
impl Server { /* ... */ }
```

The local example uses this at `crates/mcp-server/src/tools.rs:47`.

### Renaming the router field

Pass the field name to both macros if you don't want the default
`tool_router`:

```rust
#[tool_router(router = my_tools)]
impl Server { /* ... */ }

#[tool_handler(router = self.my_tools)]
impl ServerHandler for Server {}
```

### Tool annotations

Beyond `description`, `#[tool(...)]` supports `name = "..."` (override
the auto-derived snake_case name), `input_schema = ...` (provide a
hand-rolled schema), and `annotations(...)` (declare hints like
`destructive_hint = true`). The full attribute syntax lives in
`submodules/mcp-rust-sdk/crates/rmcp-macros/src/tool.rs`.

## See also

- `references/server/getting-started.md` — composing `Server` with
  multiple routers
- `references/server/tasks.md` — `execution(task_support = ...)` and
  the task processor
- `references/server/sampling.md`,
  `references/server/elicitation.md`,
  `references/server/roots.md` — the three server-to-client request
  patterns
- `crates/mcp-server/src/tools.rs` — five worked examples
  (`ping`, `slow_count`, `ask_llm`, `greet_user`, `list_workspace_roots`)
- `crates/mcp-server/tests/tools.rs` — integration tests for the same
