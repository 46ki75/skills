---
name: a2a-knowledge
description: >
  Expert guidance for the A2A (Agent2Agent) Protocol — an open HTTP-based
  protocol for independent, opaque AI agents to collaborate as peers.
  Covers v1.0: data model (Agent Card, Task,
  Message, Part, Artifact, Extension); operations (`SendMessage`,
  `SendStreamingMessage`, `GetTask`, `ListTasks`, `CancelTask`,
  `SubscribeToTask`, the `*TaskPushNotificationConfig` family,
  `GetExtendedAgentCard`); task lifecycle and multi-turn flows;
  three protocol bindings (JSON-RPC, gRPC, HTTP+JSON/REST) and
  custom bindings; agent discovery (well-known URI, registries, signed
  Agent Cards); enterprise security (TLS, OAuth 2.0 / OIDC / mTLS,
  in-task authorization); SSE streaming and webhook push notifications;
  the extension framework; A2A↔MCP relationship; and the six SDKs
  (`a2a-python`, `a2a-js`, `a2a-java`, `a2a-go`, `a2a-dotnet`, `a2a-rs`).
  Always invoke for A2A, Agent2Agent, agent card, A2A tasks / messages /
  artifacts / parts, push notification configs, A2A extensions, Agent
  Card signing, or any of the six SDK package names.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.3.0"
---

# A2A Skill

You are an expert in the **Agent2Agent (A2A) Protocol** — an open standard for
communication and interoperability between independent, opaque AI agents.
A2A defines how agents discover each other, exchange messages, share long-running
stateful tasks, stream incremental results, and authenticate — over standard HTTP
with pluggable wire formats (JSON-RPC 2.0, gRPC, HTTP+JSON/REST).

## What A2A Is

A2A is the **peer-to-peer** layer of the AI ecosystem. Where MCP standardizes how a
single agent talks to its tools and resources (vertical), A2A standardizes how
two agents talk to each other (horizontal). Agents are treated as opaque systems:
a client agent never sees the remote agent's internal memory, tools, or chain of
thought — only its published `AgentCard`, the `Message` and `Artifact` objects
it returns, and the lifecycle of any `Task` it accepts.

**Three actors** participate in every A2A interaction:

- **User** — a human or automated initiator with a goal
- **A2A Client (Client Agent)** — software acting on the user's behalf, opening
  the A2A request
- **A2A Server (Remote Agent)** — an HTTP service that implements A2A and
  processes the request

## Version

A2A is at **v1.0.0** (current and stable). Older versions `0.3.0`, `0.2.6`, and
`0.1.0` exist as historical references on `a2a-protocol.org`. v1.0 introduced
breaking changes from 0.3: enum values switched from `kebab-case` to
`SCREAMING_SNAKE_CASE`, compound IDs (`tasks/{id}`) were replaced with plain
UUIDs, polymorphism dropped the `kind` discriminator in favor of JSON
member-based detection, and OAuth implicit / password flows were removed in
favor of Authorization Code + PKCE and Device Code (RFC 8628). Use **v1.0** for
all new work. Detailed migration notes live in `references/whats-new-v1.md` and
Appendix A of `references/specification.md`.

## Core Concepts

- **Agent Card** — JSON metadata describing the agent's identity, endpoint URL,
  declared protocol bindings, supported skills, authentication requirements,
  and any extensions. Its `supportedInterfaces` field is a v1.0 `AgentInterface`
  list, one entry per reachable endpoint with that interface's `protocolBinding`
  and `protocolVersion` (§4.4.6 / §5.2). Functions as a digital business card;
  discoverable via a well-known URI, a registry, or a signed JWS for
  trust-bootstrapping.
- **Task** — a stateful, server-owned unit of work with a UUID and a lifecycle
  whose states are `SCREAMING_SNAKE_CASE` (`TASK_STATE_SUBMITTED` →
  `TASK_STATE_WORKING` → `TASK_STATE_INPUT_REQUIRED` /
  `TASK_STATE_AUTH_REQUIRED` → `TASK_STATE_COMPLETED` / `TASK_STATE_FAILED` /
  `TASK_STATE_CANCELED` / `TASK_STATE_REJECTED`). Long-running work is modeled
  as a task; short Q&A is just a message.
- **Message** — one turn of communication. Has a `role` (`user` or `agent`), a
  `messageId`, and one or more `Part` objects. Sent via `SendMessage` (unary)
  or `SendStreamingMessage` (SSE).
- **Part** — the content container inside Messages and Artifacts. Holds exactly
  one of: `text` (string), `raw` (inline bytes), `url` (external reference), or
  `data` (structured JSON). Also carries `mediaType`, `filename`, `metadata`.
- **Artifact** — a tangible output produced during a task (a document, image,
  table, code blob). Has its own `artifactId`, a name, one or more Parts, and
  can be streamed incrementally.
- **Extension** — a declared, versioned addition to the protocol. Four kinds
  exist (data-only, profile, method, state-machine). Declared in the Agent
  Card; negotiated via the `A2A-Extensions` request header.
- **Push Notification Config** — a per-task webhook the server will POST to
  when a long-running task changes state and the client is disconnected.
- **Context (`contextId`)** — server-generated identifier that groups related
  tasks across a multi-step user goal.

## Reference Files

Each file under `references/` is a verbatim copy of the upstream A2A
documentation at `submodules/A2A/docs/`. The only modification is a generated
table-of-contents prepended to `references/specification.md`.

### Root references

| File                          | Content                                                                                                                                                                |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `references/specification.md` | Full v1.0 normative spec (~3,600 lines). 14 numbered sections plus migration and MCP-relationship appendices. Has a TOC at the top — jump to sections via the anchors. |
| `references/definitions.md`   | Pointer to the canonical Protobuf schema and JSON Schema. The `--8<--` directives are mkdocs snippets and do not inline content here; read the proto file directly.    |
| `references/whats-new-v1.md`  | v1.0 release notes: maturity, type safety, developer experience, enterprise features, breaking changes vs v0.3.                                                        |

The canonical Protobuf schema lives at `submodules/A2A/specification/a2a.proto`.

### Conceptual topics

| File                                                    | Content                                                                            |
| :------------------------------------------------------ | :--------------------------------------------------------------------------------- |
| `references/topics/what-is-a2a.md`                      | High-level overview, problems A2A solves, example use cases.                       |
| `references/topics/key-concepts.md`                     | Glossary walkthrough of actors, Agent Card, Task, Message, Part, Artifact.         |
| `references/topics/life-of-a-task.md`                   | Task lifecycle and state machine; multi-turn `input-required` resumes.             |
| `references/topics/streaming-and-async.md`              | SSE streaming, polling, and push-notification trade-offs.                          |
| `references/topics/agent-discovery.md`                  | Well-known URI, registries, programmatic discovery, retrieval and caching.         |
| `references/topics/enterprise-ready.md`                 | TLS, server identity, OAuth 2.0 / OIDC / API key / mTLS, monitoring.               |
| `references/topics/extensions.md`                       | Extension framework: data-only, profile, method, state-machine kinds; negotiation. |
| `references/topics/extension-and-binding-governance.md` | Governance model for extensions and new protocol bindings.                         |
| `references/topics/custom-protocol-bindings.md`         | Rules for implementing a binding beyond JSON-RPC / gRPC / REST.                    |
| `references/topics/a2a-and-mcp.md`                      | A2A vs MCP comparison, complementarity, integration patterns.                      |

### Python tutorial (read in numeric order)

| File                                                       | Content                                                             |
| :--------------------------------------------------------- | :------------------------------------------------------------------ |
| `references/tutorials/python/1-introduction.md`            | Goals and prerequisites.                                            |
| `references/tutorials/python/2-setup.md`                   | Environment and `a2a-python` installation.                          |
| `references/tutorials/python/3-agent-skills-and-card.md`   | Authoring the Agent Card and declaring skills.                      |
| `references/tutorials/python/4-agent-executor.md`          | The executor that handles `SendMessage` and emits task updates.     |
| `references/tutorials/python/5-start-server.md`            | Wiring the Starlette server and starting it.                        |
| `references/tutorials/python/6-interact-with-server.md`    | Writing a client; fetching Agent Card; unary send; reading results. |
| `references/tutorials/python/7-streaming-and-multiturn.md` | `SendStreamingMessage` SSE; resuming `TASK_STATE_INPUT_REQUIRED`.   |
| `references/tutorials/python/8-next-steps.md`              | Pointers to further samples and framework integrations.             |

### SDK

| File                              | Content                                                      |
| :-------------------------------- | :----------------------------------------------------------- |
| `references/sdk/index.md`         | Matrix of the 6 official SDKs with repository links.         |
| `references/sdk/python.md`        | One-line redirect stub from the upstream mkdocs build.       |
| `references/sdk/python/index.rst` | Sphinx root for the Python API reference.                    |
| `references/sdk/python/conf.py`   | Sphinx config used to generate the upstream Python API HTML. |

## When to Read Which Files

Use this matrix as the first stop. Most questions resolve in one or two files.

| User is asking about...                                     | Read                                                                   |
| :---------------------------------------------------------- | :--------------------------------------------------------------------- |
| What A2A is, why it exists, when to use it                  | `topics/what-is-a2a.md` then `topics/key-concepts.md`                  |
| Glossary / core objects (Task, Message, Part, Artifact)     | `topics/key-concepts.md` then `specification.md` §4                    |
| Agent Card structure and fields                             | `specification.md` §8 (and §4.4 for the schema)                        |
| Well-known URI / agent registries / discovery               | `topics/agent-discovery.md` then `specification.md` §8.2               |
| Agent Card signing (JWS, RFC 8785 canonicalization)         | `specification.md` §8.4                                                |
| Task lifecycle / state transitions / multi-turn             | `topics/life-of-a-task.md` then `specification.md` §3.3–§3.4           |
| Streaming with SSE                                          | `topics/streaming-and-async.md` then `specification.md` §3.5           |
| Resubscribing to a task's event stream (`SubscribeToTask`)  | `specification.md` §3.1 and §9.4.6                                     |
| Authenticated extended Agent Card (`GetExtendedAgentCard`)  | `specification.md` §9.4.8 and §13.3                                    |
| Push notifications (webhooks, signing, replay protection)   | `specification.md` §4.3 and §13.2                                      |
| Authentication / OAuth 2.0 / OIDC / mTLS / API keys         | `topics/enterprise-ready.md` then `specification.md` §7                |
| In-task authorization, scoped consent                       | `specification.md` §7.6                                                |
| JSON-RPC binding (methods, error codes)                     | `specification.md` §9                                                  |
| gRPC binding (service definition, streaming)                | `specification.md` §10                                                 |
| HTTP+JSON/REST binding (URL patterns, query params)         | `specification.md` §11                                                 |
| Choosing or migrating between bindings                      | `specification.md` §5 then §9 / §10 / §11                              |
| Writing a custom protocol binding                           | `topics/custom-protocol-bindings.md` then `specification.md` §12       |
| Extensions: declaring, kinds, negotiation                   | `topics/extensions.md` then `specification.md` §4.6                    |
| Extension or binding governance                             | `topics/extension-and-binding-governance.md`                           |
| Security considerations / threat model                      | `specification.md` §13                                                 |
| IANA registrations (media types, headers, well-known URI)   | `specification.md` §14                                                 |
| Migrating from v0.3 or earlier                              | `whats-new-v1.md` then `specification.md` Appendix A                   |
| Worked examples (basic, streaming, multi-turn, push, files) | `specification.md` §6                                                  |
| Canonical Protobuf or JSON schema                           | `submodules/A2A/specification/a2a.proto` (indexed by `definitions.md`) |
| Relationship with MCP                                       | `topics/a2a-and-mcp.md` then `specification.md` Appendix B             |
| Building a Python agent from scratch                        | `tutorials/python/1-introduction.md` (read 1–8 in order)               |
| Python API reference                                        | `sdk/python/index.rst` (or the rendered upstream docs)                 |
| Which SDK to use / language matrix                          | `sdk/index.md`                                                         |

Paths in this table are relative to `references/`.

## Language SDK Guides

A2A maintains six official SDKs. Their depth in this skill differs: Python has
authored API-reference scaffolding (the upstream Sphinx config is included), and
the other five are indexed as pointers to their upstream repositories. Treat
each SDK's repo README and issue tracker as the source of truth for ergonomics;
the A2A spec itself is what guarantees on-the-wire interoperability between them.

| SDK / language          | Package      | Repository                                 | Depth in this skill      |
| :---------------------- | :----------- | :----------------------------------------- | :----------------------- |
| Python                  | `a2a-python` | <https://github.com/a2aproject/a2a-python> | API reference + tutorial |
| JavaScript / TypeScript | `a2a-js`     | <https://github.com/a2aproject/a2a-js>     | Pointer only             |
| Java                    | `a2a-java`   | <https://github.com/a2aproject/a2a-java>   | Pointer only             |
| Go                      | `a2a-go`     | <https://github.com/a2aproject/a2a-go>     | Pointer only             |
| C# / .NET               | `a2a-dotnet` | <https://github.com/a2aproject/a2a-dotnet> | Pointer only             |
| Rust                    | `a2a-rs`     | <https://github.com/a2aproject/a2a-rs>     | Pointer only             |

For Python specifically, the tutorial under `references/tutorials/python/` is
the fastest end-to-end path; the Sphinx-generated API reference lives in
`references/sdk/python/`. Cross-language samples (multi-agent setups, framework
integrations such as ADK, LangGraph, CrewAI, Semantic Kernel) live at
<https://github.com/a2aproject/a2a-samples>.

## Relationship with MCP

A2A and MCP are **complementary, not competing**. They sit at different layers
of an agent stack:

- **MCP** standardizes how a single agent reaches **down** to tools, resources,
  and prompts. Agent ↔ tool / data source.
- **A2A** standardizes how agents reach **across** to each other as opaque peers.
  Agent ↔ agent.

A production system frequently uses both: each agent exposes MCP servers
inward to its tools and an A2A endpoint outward to other agents. For the full
comparison and integration patterns see `references/topics/a2a-and-mcp.md` and
Appendix B of `references/specification.md`. The companion skill `mcp-knowledge`
in this repo covers MCP in equivalent depth.
