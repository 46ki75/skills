# mcp-server

Generic [Model Context Protocol](https://modelcontextprotocol.io/) server
skeleton built on [`rmcp`](https://crates.io/crates/rmcp). Use it as a
starting point for project-specific MCP servers.

The crate ships a [`Server`] type that implements `rmcp::ServerHandler` with
one example of each MCP primitive:

| Primitive | Example                                  |
| --------- | ---------------------------------------- |
| Tool      | `ping` — returns `"pong"`                |
| Prompt    | `greeting` — canned no-argument exchange |
| Resource  | `mem://example` — in-memory text blob    |

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

All extension points live in `src/lib.rs`:

- **Tools** — add methods inside the `#[tool_router] impl Server` block,
  annotated with `#[tool(description = "...")]`. The `#[tool_handler]` on
  the `ServerHandler` impl wires them in.
- **Prompts** — add methods inside the `#[prompt_router] impl Server` block,
  annotated with `#[prompt(name = "...", description = "...")]`. The
  `#[prompt_handler]` on the `ServerHandler` impl wires them in. Typed
  arguments use `Parameters<T>` where `T: serde::Deserialize + schemars::JsonSchema`.
- **Resources** — edit `list_resources`, `read_resource`, and
  `list_resource_templates` directly on the `ServerHandler` impl. There is
  no macro router for resources in `rmcp`.

For richer examples (typed prompt arguments, dynamic resources, sampling,
elicitation, long-running tasks), see the `rmcp` examples under
`submodules/mcp-rust-sdk/examples/servers/`.
