---
name: mcp-knowledge
description: >
  Expert guidance for the Model Context Protocol (MCP) — a JSON-RPC 2.0 protocol
  standardizing how LLM applications integrate with external data sources and tools.
  Covers spec versions 2024-11-05 through 2025-11-25, architecture (host/client/server),
  transports (stdio, HTTP+SSE, Streamable HTTP), server features (Resources, Prompts,
  Tools), client features (Sampling, Roots, Elicitation), lifecycle, OAuth 2.1
  authorization, and utilities (Logging, Pagination, Cancellation, Ping, Progress,
  Tasks). Use when building or integrating MCP servers or clients; implementing tools,
  resources, or prompts; working with transports or authorization; handling lifecycle
  and capability negotiation; or implementing sampling, elicitation, or Tasks. Always
  invoke for questions mentioning MCP, modelcontextprotocol, tools/list, tools/call,
  resources/read, sampling/createMessage, elicitation/create, Streamable HTTP,
  Mcp-Session-Id, or JSON-RPC in AI tool integration context.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0"
---

# MCP Skill

You are an expert in the Model Context Protocol (MCP) — an open, JSON-RPC 2.0 based
protocol that standardizes how LLM applications (hosts) connect to external data sources
and tools (servers). MCP provides a stateful session protocol focused on context
exchange and sampling coordination, enabling composable integrations across the AI
ecosystem.

## What MCP Is

MCP follows a client-host-server architecture. A **host** (e.g., an IDE or chat app)
runs multiple **clients**, each maintaining a 1:1 stateful session with a **server**.
Servers expose capabilities — Resources, Prompts, Tools — and can optionally request
LLM completions back from the client (Sampling). The protocol uses JSON-RPC 2.0 for
all messages and supports pluggable transports.

## Versions

There are four public versions:

| Version        | Status        | Key additions                                                                                                                                                                                                              |
| :------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **2024-11-05** | First release | stdio + HTTP/SSE transports; Resources, Prompts, Tools, Sampling, Roots; Pagination, Logging, Cancellation, Ping, Progress, Completion                                                                                     |
| **2025-03-26** | Stable        | OAuth 2.1 authorization; Streamable HTTP transport; JSON-RPC batching; tool annotations; audio content type; completions capability                                                                                        |
| **2025-06-18** | Stable        | Elicitation; structured tool output; resource links in tool results; OAuth Resource Server classification; RFC 8707 Resource Indicators; `MCP-Protocol-Version` header; removed batching; `title` field; `_meta` expansion |
| **2025-11-25** | Latest stable | Tasks utility; OpenID Connect discovery; tool/resource/prompt icons; incremental scope consent; URL-mode elicitation; tool calling in sampling; OAuth Client ID Metadata Documents                                         |

Use **2025-11-25** for new projects. Use **2024-11-05** only when targeting legacy hosts.

## Core Concepts

- **Host** — the LLM application; manages client lifecycle and user consent
- **Client** — created by the host; holds one stateful session per server
- **Server** — exposes Resources, Prompts, or Tools; may request Sampling
- **Capability negotiation** — client and server exchange capability objects during
  `initialize`; only negotiated capabilities may be used during the session
- **Resources** — application-driven context: files, DB schemas, live data (URI-addressed)
- **Prompts** — user-triggered prompt templates with optional arguments
- **Tools** — model-controlled functions that call external systems
- **Sampling** — server-initiated LLM completions routed through the client (human in loop)
- **Roots** — client-declared filesystem boundaries the server should respect
- **Elicitation** — server requests structured input from the user via the client (2025-06-18+)
- **Tasks** — durable async state machines for long-running requests (2025-11-25+)

## Reference Files

Each reference file is the full specification document for that topic, converted from
the official MDX source at `submodules/modelcontextprotocol/docs/specification/`.

### 2024-11-05

| File                                    | Content                                                                                   |
| :-------------------------------------- | :---------------------------------------------------------------------------------------- |
| `references/2024-11-05/architecture.md` | Client-host-server architecture, design principles, message types, capability negotiation |
| `references/2024-11-05/lifecycle.md`    | Connection lifecycle: initialization, operation, shutdown; version negotiation            |
| `references/2024-11-05/transports.md`   | stdio transport; HTTP+SSE transport; custom transports                                    |
| `references/2024-11-05/resources.md`    | Resources: list, read, templates, subscriptions, URI schemes                              |
| `references/2024-11-05/prompts.md`      | Prompts: list, get, listChanged; message content types                                    |
| `references/2024-11-05/tools.md`        | Tools: list, call, listChanged; inputSchema; error handling                               |
| `references/2024-11-05/sampling.md`     | Sampling: createMessage, modelPreferences, hints, human-in-the-loop                       |
| `references/2024-11-05/roots.md`        | Roots: list, listChanged; file:// URI constraint                                          |
| `references/2024-11-05/logging.md`      | Logging: setLevel, notifications/message; syslog severity levels                          |
| `references/2024-11-05/pagination.md`   | Cursor-based pagination for list operations                                               |
| `references/2024-11-05/completion.md`   | Argument autocompletion: completion/complete, ref/prompt, ref/resource                    |
| `references/2024-11-05/cancellation.md` | Request cancellation via notifications/cancelled                                          |
| `references/2024-11-05/ping.md`         | Ping/pong for connection health                                                           |
| `references/2024-11-05/progress.md`     | Progress tracking via progressToken and notifications/progress                            |

### 2025-03-26

| File                                     | Content                                                                                               |
| :--------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `references/2025-03-26/changelog.md`     | Delta from 2024-11-05: OAuth 2.1, Streamable HTTP, tool annotations, audio, completions capability    |
| `references/2025-03-26/architecture.md`  | Architecture (unchanged)                                                                              |
| `references/2025-03-26/lifecycle.md`     | Lifecycle (unchanged)                                                                                 |
| `references/2025-03-26/transports.md`    | Streamable HTTP (POST/GET/SSE, session management, resumability, backwards compatibility)             |
| `references/2025-03-26/authorization.md` | OAuth 2.1: authorization code, PKCE, dynamic client registration, metadata discovery                  |
| `references/2025-03-26/resources.md`     | Resources (unchanged)                                                                                 |
| `references/2025-03-26/prompts.md`       | Prompts (unchanged)                                                                                   |
| `references/2025-03-26/tools.md`         | Tools: adds annotations (readOnlyHint, destructiveHint, idempotentHint, openWorldHint); audio content |
| `references/2025-03-26/sampling.md`      | Sampling: adds audio content type                                                                     |
| `references/2025-03-26/roots.md`         | Roots (unchanged)                                                                                     |
| `references/2025-03-26/logging.md`       | Logging (unchanged)                                                                                   |
| `references/2025-03-26/pagination.md`    | Pagination (unchanged)                                                                                |
| `references/2025-03-26/completion.md`    | Completion (unchanged; server must declare completions capability)                                    |
| `references/2025-03-26/cancellation.md`  | Cancellation (unchanged)                                                                              |
| `references/2025-03-26/ping.md`          | Ping (unchanged)                                                                                      |
| `references/2025-03-26/progress.md`      | Progress: adds optional message field                                                                 |

### 2025-06-18

| File                                     | Content                                                                                                                           |
| :--------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `references/2025-06-18/changelog.md`     | Delta from 2025-03-26: elicitation, structured output, resource links, OAuth RS, RFC 8707, MCP-Protocol-Version, batching removed |
| `references/2025-06-18/architecture.md`  | Architecture (unchanged)                                                                                                          |
| `references/2025-06-18/lifecycle.md`     | Lifecycle: initialized notification is now MUST                                                                                   |
| `references/2025-06-18/transports.md`    | Transports: MCP-Protocol-Version header required; batching removed                                                                |
| `references/2025-06-18/authorization.md` | Authorization: OAuth Resource Server classification; RFC 8707 resource indicators                                                 |
| `references/2025-06-18/resources.md`     | Resources: adds title field                                                                                                       |
| `references/2025-06-18/prompts.md`       | Prompts: adds title field                                                                                                         |
| `references/2025-06-18/tools.md`         | Tools: structured output (outputSchema, structuredContent); resource links; title field                                           |
| `references/2025-06-18/sampling.md`      | Sampling (unchanged)                                                                                                              |
| `references/2025-06-18/roots.md`         | Roots (unchanged)                                                                                                                 |
| `references/2025-06-18/elicitation.md`   | Elicitation: elicitation/create, requestedSchema, accept/decline/cancel                                                           |
| `references/2025-06-18/logging.md`       | Logging (unchanged)                                                                                                               |
| `references/2025-06-18/pagination.md`    | Pagination (unchanged)                                                                                                            |
| `references/2025-06-18/completion.md`    | Completion: adds context field for dependent completions                                                                          |
| `references/2025-06-18/cancellation.md`  | Cancellation (unchanged)                                                                                                          |
| `references/2025-06-18/ping.md`          | Ping (unchanged)                                                                                                                  |
| `references/2025-06-18/progress.md`      | Progress (unchanged)                                                                                                              |

### 2025-11-25

| File                                     | Content                                                                                                                      |
| :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `references/2025-11-25/changelog.md`     | Delta from 2025-06-18: Tasks, OIDC discovery, icons, URL elicitation, tool calling in sampling, Client ID Metadata Documents |
| `references/2025-11-25/architecture.md`  | Architecture (unchanged)                                                                                                     |
| `references/2025-11-25/lifecycle.md`     | Lifecycle (unchanged)                                                                                                        |
| `references/2025-11-25/transports.md`    | Transports: OIDC discovery; Client ID Metadata Documents; incremental scope consent; SSE polling                             |
| `references/2025-11-25/authorization.md` | Authorization: OIDC discovery; OAuth Client ID Metadata Documents; incremental scope consent                                 |
| `references/2025-11-25/resources.md`     | Resources: adds icon field                                                                                                   |
| `references/2025-11-25/prompts.md`       | Prompts: adds icon field                                                                                                     |
| `references/2025-11-25/tools.md`         | Tools: adds icon field; tool name guidance; input validation as tool execution errors                                        |
| `references/2025-11-25/sampling.md`      | Sampling: tool calling in sampling (tools, toolChoice); sampling.tools capability                                            |
| `references/2025-11-25/roots.md`         | Roots (unchanged)                                                                                                            |
| `references/2025-11-25/elicitation.md`   | Elicitation: URL mode; updated ElicitResult and EnumSchema                                                                   |
| `references/2025-11-25/logging.md`       | Logging: stdio servers may use stderr for all log levels                                                                     |
| `references/2025-11-25/pagination.md`    | Pagination (unchanged)                                                                                                       |
| `references/2025-11-25/completion.md`    | Completion (unchanged)                                                                                                       |
| `references/2025-11-25/cancellation.md`  | Cancellation (unchanged)                                                                                                     |
| `references/2025-11-25/ping.md`          | Ping (unchanged)                                                                                                             |
| `references/2025-11-25/progress.md`      | Progress (unchanged)                                                                                                         |
| `references/2025-11-25/tasks.md`         | Tasks (experimental): durable state machines, polling, deferred results, tasks/get, tasks/cancel                             |

## When to Read Which Files

| User is asking about...                    | Read                                                    |
| :----------------------------------------- | :------------------------------------------------------ |
| MCP architecture / design principles       | `references/{version}/architecture.md`                  |
| Connection lifecycle, initialization       | `references/{version}/lifecycle.md`                     |
| stdio transport                            | `references/{version}/transports.md`                    |
| HTTP+SSE transport                         | `references/2024-11-05/transports.md`                   |
| Streamable HTTP transport                  | `references/2025-03-26/transports.md` (or later)        |
| OAuth 2.1 authorization                    | `references/2025-03-26/authorization.md` (or later)     |
| OAuth Resource Server / RFC 8707           | `references/2025-06-18/authorization.md` (or later)     |
| OIDC discovery / Client ID Metadata        | `references/2025-11-25/authorization.md`                |
| Resources (list/read/subscribe)            | `references/{version}/resources.md`                     |
| Prompts (list/get)                         | `references/{version}/prompts.md`                       |
| Tools (list/call/annotations)              | `references/{version}/tools.md`                         |
| Structured tool output                     | `references/2025-06-18/tools.md` (or later)             |
| Resource links in tool results             | `references/2025-06-18/tools.md` (or later)             |
| Icons on tools/resources/prompts           | `references/2025-11-25/tools.md` (or resources/prompts) |
| Sampling (createMessage, modelPreferences) | `references/{version}/sampling.md`                      |
| Tool calling inside sampling               | `references/2025-11-25/sampling.md`                     |
| Roots (filesystem boundaries)              | `references/{version}/roots.md`                         |
| Elicitation (server→user input)            | `references/2025-06-18/elicitation.md` (or later)       |
| URL-mode elicitation                       | `references/2025-11-25/elicitation.md`                  |
| Logging                                    | `references/{version}/logging.md`                       |
| Pagination                                 | `references/{version}/pagination.md`                    |
| Argument autocompletion                    | `references/{version}/completion.md`                    |
| Cancellation                               | `references/{version}/cancellation.md`                  |
| Ping / connection health                   | `references/{version}/ping.md`                          |
| Progress notifications                     | `references/{version}/progress.md`                      |
| Tasks (durable async)                      | `references/2025-11-25/tasks.md`                        |
| Migrating between versions                 | `references/{target-version}/changelog.md`              |
