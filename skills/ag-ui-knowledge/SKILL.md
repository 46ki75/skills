---
name: ag-ui-knowledge
description: >
  Expert guidance for building with AG-UI (Agent–User Interaction Protocol) —
  covering the event-based streaming architecture, event types (lifecycle, text,
  tool calls, state, reasoning), message formats, state management (snapshots
  and JSON Patch deltas), tools, interrupts, capabilities, and SDKs for Python
  (`ag-ui-protocol`) and JavaScript/TypeScript (`@ag-ui/client`, `@ag-ui/core`).
  Use this skill whenever someone is implementing an AG-UI server or middleware
  integration, consuming the AG-UI Python or JS SDK, handling AG-UI events or
  RunAgentInput, working with state snapshots or STATE_DELTA JSON patches,
  implementing human-in-the-loop interrupts, using HttpAgent or AbstractAgent,
  building a frontend that connects to an AG-UI-compatible agent, or asking any
  question that mentions AG-UI, RunAgentInput, RunStartedEvent, RunFinishedEvent,
  STATE_SNAPSHOT, STATE_DELTA, ToolCallStart, TextMessageStart, ag-ui-protocol,
  or @ag-ui/client. Always invoke this skill even for seemingly simple AG-UI
  questions.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0"
---

# AG-UI Skill

You are an expert in the **AG-UI (Agent–User Interaction Protocol)**, the open
standard that defines how AI agents and user-facing frontend applications
communicate in real time.

## What AG-UI Is

AG-UI is the boundary layer where agents and users meet. It standardizes how
agent state, tool calls, text streaming, and user interactions flow between an
agent runtime and an application frontend — over a streaming event bus.

Key design principles:

- **Event-driven** — all communication is a stream of typed, discriminated events.
- **Framework-agnostic** — works with any agent backend (LangGraph, CrewAI,
  Mastra, custom) and any frontend stack.
- **Bidirectional state** — agents and frontends share state through snapshot
  and delta events.
- **Human-in-the-loop** — first-class interrupt/resume lifecycle for approval
  flows and structured user input.

## Core Concepts (quick reference)

### Run lifecycle

Every agent interaction is a **run** on a **thread**:

1. Client sends `RunAgentInput` (POST to the agent's HTTP endpoint).
2. Agent emits `RUN_STARTED`, streams events, then emits `RUN_FINISHED`.
3. `RUN_FINISHED` carries an optional `outcome`:
   - `{ type: "success" }` — normal completion
   - `{ type: "interrupt", interrupts: [...] }` — paused for user input

### Event categories

| Category   | Events                                                                                                       |
| ---------- | ------------------------------------------------------------------------------------------------------------ |
| Lifecycle  | `RUN_STARTED`, `RUN_FINISHED`, `RUN_ERROR`, `STEP_STARTED`, `STEP_FINISHED`                                  |
| Text       | `TEXT_MESSAGE_START`, `TEXT_MESSAGE_CONTENT`, `TEXT_MESSAGE_END`                                             |
| Tool calls | `TOOL_CALL_START`, `TOOL_CALL_ARGS`, `TOOL_CALL_END`, `TOOL_CALL_RESULT`                                     |
| State      | `STATE_SNAPSHOT`, `STATE_DELTA`, `MESSAGES_SNAPSHOT`                                                         |
| Reasoning  | `REASONING_START`, `REASONING_MESSAGE_START/CONTENT/END/CHUNK`, `REASONING_END`, `REASONING_ENCRYPTED_VALUE` |
| Other      | `RAW`, `CUSTOM`, `ACTIVITY_SNAPSHOT`, `ACTIVITY_DELTA`                                                       |

### Message roles

`user`, `assistant`, `system`, `tool`, `developer`, `activity`, `reasoning`

### State management

- `STATE_SNAPSHOT` — replace the entire frontend state.
- `STATE_DELTA` — apply RFC 6902 JSON Patch operations incrementally.

### Tools

Tools are defined by the **frontend** and passed in `RunAgentInput.tools`. The
agent calls them via `TOOL_CALL_*` events; the frontend executes them and sends
results back.

### Interrupts

When an agent needs human input mid-run it emits `RUN_FINISHED` with
`outcome.type = "interrupt"`. The client resolves each interrupt and starts a
new run with `RunAgentInput.resume`.

## Integration types

| Type           | When to use                                                             |
| -------------- | ----------------------------------------------------------------------- |
| **Server**     | Building a new agent from scratch; maximum control over emitted events  |
| **Middleware** | Wrapping an existing protocol or framework; translating to AG-UI events |

Both types expose the same HTTP endpoint that accepts `RunAgentInput` and
returns an SSE stream of AG-UI events.

## SDKs

| Language      | Package                        | Install                      |
| ------------- | ------------------------------ | ---------------------------- |
| Python        | `ag-ui-protocol`               | `pip install ag-ui-protocol` |
| TypeScript/JS | `@ag-ui/core`, `@ag-ui/client` | `npm install @ag-ui/client`  |

## Reference files

Read the relevant reference file for deep technical details:

- **`references/concepts.md`** — Complete event type definitions, RunAgentInput
  schema, message types, state management, tools, interrupts, and capabilities
  interface.
- **`references/python-sdk.md`** — Python SDK types, events, encoder, and
  FastAPI/HTTP server patterns.
- **`references/js-sdk.md`** — TypeScript SDK: `AbstractAgent`, `HttpAgent`,
  middleware, `AgentSubscriber`, core event types.
- **`references/integrations.md`** — Step-by-step server and middleware
  integration guides with code examples.
