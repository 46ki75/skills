# mcp-server

Generic [Model Context Protocol](https://modelcontextprotocol.io/) server
skeleton built on [`rmcp`](https://crates.io/crates/rmcp). Use it as a
starting point for project-specific MCP servers.

The crate ships a [`Server`] type that implements `rmcp::ServerHandler`
with an example of every MCP primitive — including parameterized variants:

| Primitive                  | Example                     | Notes                                                                       |
| -------------------------- | --------------------------- | --------------------------------------------------------------------------- |
| Tool                       | `ping`                      | No args, returns `"pong"`                                                   |
| Tool (task-capable)        | `slow_count`                | `target: u8`, supports async task invocation (see [tasks note](#extending)) |
| Tool (sampling)            | `ask_llm`                   | Asks the client's LLM via `sampling/createMessage`                          |
| Tool (elicitation)         | `greet_user`                | Asks the user via `elicitation/create`                                      |
| Tool (roots)               | `list_workspace_roots`      | Queries the client via `roots/list`                                         |
| Prompt                     | `greeting`                  | No args, canned exchange                                                    |
| Prompt (single arg)        | `echo`                      | Typed arg `message: string`                                                 |
| Prompt (multiple args)     | `summarize`                 | `topic: string`, `bullet_count: u8`, `tone?`                                |
| Resource                   | `mem://example`             | Static in-memory text                                                       |
| Resource template          | `echo://{message}`          | One URI variable, echoed back as content                                    |
| Resource template (2 args) | `greet://{language}/{name}` | Multi-segment template, returns a greeting                                  |

It also ships two binaries:

| Binary             | Transport       | Default endpoint            |
| ------------------ | --------------- | --------------------------- |
| `mcp-server-stdio` | stdio           | (stdin / stdout)            |
| `mcp-server-http`  | Streamable HTTP | `http://127.0.0.1:8000/mcp` |

The HTTP bind address can be overridden via the `MCP_BIND_ADDRESS`
environment variable.

## Run

```bash
# stdio (typical local MCP client integration)
cargo run -p mcp-server --bin mcp-server-stdio

# streamable HTTP
cargo run -p mcp-server --bin mcp-server-http
MCP_BIND_ADDRESS=0.0.0.0:9000 cargo run -p mcp-server --bin mcp-server-http
```

## Inspect

The official MCP Inspector works against both transports:

```bash
# stdio
npx @modelcontextprotocol/inspector cargo run -p mcp-server --bin mcp-server-stdio

# http: start the binary, then open the inspector and point it at
# http://127.0.0.1:8000/mcp
```

## Extending

Extension points are split across the `src/` modules:

| Primitive                        | File                 |
| -------------------------------- | -------------------- |
| Tools                            | `src/tools.rs`       |
| Prompts                          | `src/prompts.rs`     |
| Resources                        | `src/resources.rs`   |
| Tasks override                   | `src/tasks.rs`       |
| Glue (`Server`, `ServerHandler`) | `src/lib.rs`         |

- **Tools** — add methods inside the `#[tool_router] impl Server` block in
  `src/tools.rs`, annotated with `#[tool(description = "...")]`. The
  `#[tool_handler]` on the `ServerHandler` impl in `src/lib.rs` wires them
  in.
- **Prompts** — add methods inside the `#[prompt_router] impl Server` block
  in `src/prompts.rs`, annotated with `#[prompt(name = "...", description =
  "...")]`. The `#[prompt_handler]` on the `ServerHandler` impl in
  `src/lib.rs` wires them in. Typed arguments use `Parameters<T>` where
  `T: serde::Deserialize + schemars::JsonSchema`.
- **Resources** — edit the `list_resources`, `read_resource`, and
  `list_resource_templates` free functions in `src/resources.rs`. The
  `ServerHandler` impl in `src/lib.rs` delegates to them. There is no
  macro router for resources in `rmcp`.
- **Tasks (async tool invocation)** — annotate a tool in `src/tools.rs`
  with `execution(task_support = "optional")` (or `"required"`) and ensure
  the server struct carries a `processor: Arc<Mutex<OperationProcessor>>`
  field initialized in `new()`. The `#[task_handler]` attribute on the
  `ServerHandler` impl in `src/lib.rs` synthesizes the `tasks/enqueue`,
  `tasks/list`, `tasks/get`, `tasks/result`, and `tasks/cancel` handlers.
  The custom `list_tasks` override lives in `src/tasks.rs`.

  Note: SEP-1319 tasks are opt-in *by the client*. Clients that do not set
  a `task` field on `tools/call` (including the current MCP Inspector) will
  invoke an `"optional"` task tool synchronously and wait for the full
  result — so they can hit a request timeout (`-32001`) if the tool is
  genuinely slow. Either keep the synchronous path short, mark the tool
  `task_support = "required"`, or test the task path with a task-aware
  client (see `crates/mcp-server/tests/task.rs` and the `rmcp` task client
  example).

  Also note: the macro-generated `list_tasks` only surfaces tasks that are
  *currently running* (`OperationProcessor::list_running`). Short tools
  vanish from `tasks/list` as soon as they finish. The skeleton overrides
  `list_tasks` to also include `peek_completed()` results, so finished
  tasks remain observable as `TaskStatus::Completed` / `Failed`. If you
  prefer the default behavior, remove the override on the `ServerHandler`
  impl.
- **Server-to-client requests (sampling, elicitation, roots)** — inside a
  tool, take `ctx: RequestContext<RoleServer>` as a parameter and call:
  - `ctx.peer.create_message(CreateMessageRequestParams::new(..))` for
    sampling (asks the client's LLM to respond);
  - `ctx.peer.elicit::<T>("prompt")` for elicitation (asks the user, via
    the client, for typed input — `T` must be `JsonSchema` and registered
    with `rmcp::elicit_safe!`);
  - `ctx.peer.list_roots()` for roots (asks the client for its exposed
    filesystem/workspace roots).

  Each requires the *client* to advertise the matching capability. MCP
  Inspector supports sampling and elicitation in recent versions; roots
  support varies by client.

For richer examples (typed prompt arguments, dynamic resources, sampling,
elicitation, long-running tasks), see the `rmcp` examples under
`submodules/mcp-rust-sdk/examples/servers/`.
