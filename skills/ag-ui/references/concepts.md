# AG-UI Concepts Reference

Complete reference for AG-UI protocol concepts: events, messages, state, tools,
interrupts, and capabilities.

## Table of Contents

- [RunAgentInput](#runagentinput)
- [Event Types](#event-types)
  - [Lifecycle Events](#lifecycle-events)
  - [Text Message Events](#text-message-events)
  - [Tool Call Events](#tool-call-events)
  - [State Management Events](#state-management-events)
  - [Reasoning Events](#reasoning-events)
  - [Convenience Chunk Events](#convenience-chunk-events)
  - [Other Events](#other-events)
- [Message Types](#message-types)
- [State Management](#state-management)
- [Tools](#tools)
- [Interrupts](#interrupts)
- [Capabilities](#capabilities)

---

## RunAgentInput

The body of the `POST` request sent by the client to start or resume a run.

```typescript
interface RunAgentInput {
  threadId: string        // Conversation thread ID
  runId: string           // ID for this specific run
  parentRunId?: string    // Optional: ID of the run that spawned this one
  state: any              // Current agent state passed back from the frontend
  messages: Message[]     // Conversation history
  tools: Tool[]           // Frontend-defined tools available to the agent
  context: Context[]      // Additional context objects
  forwardedProps?: any    // Extra properties forwarded to the agent
  resume?: ResumeItem[]   // Interrupt responses (for resuming after interrupt)
}

interface ResumeItem {
  interruptId: string
  status: "resolved" | "cancelled"
  payload?: any           // Must match the interrupt's responseSchema if defined
}
```

Python equivalent:

```python
from ag_ui.core import RunAgentInput
```

---

## Event Types

All events inherit from `BaseEvent`:

```python
class BaseEvent(ConfiguredBaseModel):
    type: EventType
    timestamp: Optional[int] = None
    raw_event: Optional[Any] = None
```

### Lifecycle Events

#### RUN_STARTED

Signals the start of an agent run.

```python
class RunStartedEvent(BaseEvent):
    type: Literal[EventType.RUN_STARTED]
    thread_id: str
    run_id: str
    parent_run_id: Optional[str] = None
    input: Optional[RunAgentInput] = None
```

#### RUN_FINISHED

Signals completion or interrupt of a run.

```python
class RunFinishedEvent(BaseEvent):
    type: Literal[EventType.RUN_FINISHED]
    thread_id: str
    run_id: str
    result: Optional[Any] = None
    outcome: Optional[RunFinishedOutcome] = None
```

`outcome` is a discriminated union:

```python
# Omitted -> legacy normal completion
# { type: "success" } -> normal completion
# { type: "interrupt", interrupts: [...] } -> paused for user input
```

#### RUN_ERROR

Signals an error during the run.

```python
class RunErrorEvent(BaseEvent):
    type: Literal[EventType.RUN_ERROR]
    message: str
    code: Optional[str] = None
```

#### STEP_STARTED / STEP_FINISHED

Bracket a logical step within a run.

```python
class StepStartedEvent(BaseEvent):
    type: Literal[EventType.STEP_STARTED]
    step_name: str

class StepFinishedEvent(BaseEvent):
    type: Literal[EventType.STEP_FINISHED]
    step_name: str
```

---

### Text Message Events

Stream an assistant text message in three phases:

```python
class TextMessageStartEvent(BaseEvent):
    type: Literal[EventType.TEXT_MESSAGE_START]
    message_id: str
    role: Optional[TextMessageRole] = None  # "assistant"

class TextMessageContentEvent(BaseEvent):
    type: Literal[EventType.TEXT_MESSAGE_CONTENT]
    message_id: str
    delta: str                              # incremental text chunk

class TextMessageEndEvent(BaseEvent):
    type: Literal[EventType.TEXT_MESSAGE_END]
    message_id: str
```

---

### Tool Call Events

```python
class ToolCallStartEvent(BaseEvent):
    type: Literal[EventType.TOOL_CALL_START]
    tool_call_id: str
    tool_call_name: str
    parent_message_id: Optional[str] = None

class ToolCallArgsEvent(BaseEvent):
    type: Literal[EventType.TOOL_CALL_ARGS]
    tool_call_id: str
    delta: str                              # incremental JSON args chunk

class ToolCallEndEvent(BaseEvent):
    type: Literal[EventType.TOOL_CALL_END]
    tool_call_id: str

class ToolCallResultEvent(BaseEvent):
    type: Literal[EventType.TOOL_CALL_RESULT]
    message_id: str
    tool_call_id: str
    role: Literal["tool"]
    content: str                            # result from frontend tool execution
```

---

### State Management Events

```python
class StateSnapshotEvent(BaseEvent):
    type: Literal[EventType.STATE_SNAPSHOT]
    snapshot: Any                           # complete state; client replaces its state

class StateDeltaEvent(BaseEvent):
    type: Literal[EventType.STATE_DELTA]
    delta: List[JsonPatchOperation]         # RFC 6902 JSON Patch operations

class MessagesSnapshotEvent(BaseEvent):
    type: Literal[EventType.MESSAGES_SNAPSHOT]
    messages: List[Message]
```

JSON Patch operations (`op`): `"add"`, `"remove"`, `"replace"`, `"move"`,
`"copy"`, `"test"`.

```json
{ "op": "replace", "path": "/user/name", "value": "Alice" }
```

---

### Reasoning Events

Track chain-of-thought / thinking phases.

```python
class ReasoningStartEvent(BaseEvent):
    type: Literal[EventType.REASONING_START]
    message_id: str

class ReasoningMessageStartEvent(BaseEvent):
    type: Literal[EventType.REASONING_MESSAGE_START]
    message_id: str
    role: Literal["reasoning"] = "reasoning"

class ReasoningMessageContentEvent(BaseEvent):
    type: Literal[EventType.REASONING_MESSAGE_CONTENT]
    message_id: str
    delta: str

class ReasoningMessageEndEvent(BaseEvent):
    type: Literal[EventType.REASONING_MESSAGE_END]
    message_id: str

class ReasoningEndEvent(BaseEvent):
    type: Literal[EventType.REASONING_END]
    message_id: str

class ReasoningEncryptedValueEvent(BaseEvent):
    type: Literal[EventType.REASONING_ENCRYPTED_VALUE]
    subtype: Literal["tool-call", "message"]
    entity_id: str
    encrypted_value: str
```

> Note: `THINKING_*` events are deprecated; use `REASONING_*` instead.
> Earlier versions of the protocol used `THINKING_START`, `THINKING_END`,
> `THINKING_TEXT_MESSAGE_START`, `THINKING_TEXT_MESSAGE_CONTENT`, and
> `THINKING_TEXT_MESSAGE_END`. These map 1-to-1 to their `REASONING_*`
> equivalents and will be removed in version 1.0.0.

---

### Convenience Chunk Events

Shorthand alternatives that clients expand into start/content/end sequences:

```python
class TextMessageChunkEvent(BaseEvent):
    type: Literal[EventType.TEXT_MESSAGE_CHUNK]
    message_id: Optional[str] = None   # required on first chunk
    role: Optional[TextMessageRole] = None
    delta: Optional[str] = None

class ToolCallChunkEvent(BaseEvent):
    type: Literal[EventType.TOOL_CALL_CHUNK]
    tool_call_id: Optional[str] = None       # required on first chunk
    tool_call_name: Optional[str] = None     # required on first chunk
    parent_message_id: Optional[str] = None
    delta: Optional[str] = None

class ReasoningMessageChunkEvent(BaseEvent):
    type: Literal[EventType.REASONING_MESSAGE_CHUNK]
    message_id: Optional[str] = None         # required on first chunk
    delta: Optional[str] = None
```

---

### Other Events

```python
class RawEvent(BaseEvent):
    type: Literal[EventType.RAW]
    event: Any

class CustomEvent(BaseEvent):
    type: Literal[EventType.CUSTOM]
    name: str
    value: Any
```

---

## Message Types

Messages represent conversation history sent in `RunAgentInput.messages`.

```typescript
interface BaseMessage {
  id: string
  role: string
  content?: string
  name?: string
  encryptedContent?: string   // for zero-data-retention / store:false scenarios
}
```

| Type | Role | Key fields |
| --- | --- | --- |
| `UserMessage` | `"user"` | `content: string \| InputContent[]` |
| `AssistantMessage` | `"assistant"` | `content?: string`, `toolCalls?: ToolCall[]` |
| `SystemMessage` | `"system"` | `content: string` |
| `DeveloperMessage` | `"developer"` | `content: string` |
| `ToolMessage` | `"tool"` | `content: string`, `toolCallId: string` |
| `ReasoningMessage` | `"reasoning"` | `content?: string`, `encryptedContent?: string` |

### Multimodal user content

```typescript
type InputContent =
  | { type: "text"; text: string }
  | { type: "image"; source: InputContentSource; metadata?: Record<string, unknown> }
  | { type: "audio"; source: InputContentSource; metadata?: Record<string, unknown> }
  | { type: "video"; source: InputContentSource; metadata?: Record<string, unknown> }
  | { type: "document"; source: InputContentSource; metadata?: Record<string, unknown> }

type InputContentSource =
  | { type: "data"; value: string; mimeType: string }    // base64 data
  | { type: "url"; value: string; mimeType?: string }    // remote URL
```

---

## State Management

The frontend and agent share a JSON state object.

### Receiving state updates

1. On `STATE_SNAPSHOT` → replace the entire local state with `snapshot`.
2. On `STATE_DELTA` → apply each JSON Patch operation to the current state.

### Sending state back

The current state is always sent in `RunAgentInput.state` on every request so
the agent can resume from where it left off.

---

## Tools

Tools are defined by the **frontend** and passed to the agent in
`RunAgentInput.tools`.

```typescript
interface Tool {
  name: string
  description: string
  parameters: {
    type: "object"
    properties: Record<string, unknown>  // JSON Schema
    required?: string[]
  }
}
```

### Tool call lifecycle

```text
Agent emits:
  TOOL_CALL_START   (tool_call_id, tool_call_name)
  TOOL_CALL_ARGS    (delta chunks of JSON args)
  TOOL_CALL_END

Frontend executes the tool, then sends result in next RunAgentInput or
the agent emits TOOL_CALL_RESULT when it receives the result.
```

---

## Interrupts

Interrupts pause a run for human input.

### Emitting an interrupt (agent side)

End the run with `RunFinished.outcome.type = "interrupt"`:

```python
from ag_ui.core import RunFinishedEvent, EventType

RunFinishedEvent(
    type=EventType.RUN_FINISHED,
    thread_id=thread_id,
    run_id=run_id,
    outcome={
        "type": "interrupt",
        "interrupts": [
            {
                "id": "interrupt-1",
                "reason": "approval_required",
                "message": "Do you approve this action?",
                "responseSchema": {
                    "type": "object",
                    "properties": {"approved": {"type": "boolean"}},
                    "required": ["approved"]
                }
            }
        ]
    }
)
```

### Interrupt object

| Field | Purpose |
| --- | --- |
| `id` | Correlation key for resume |
| `reason` | Categorical hint: `"approval_required"`, `"input_required"`, `"policy_decision"` |
| `message` | Human-readable prompt (fallback UI text) |
| `toolCallId` | Binds interrupt to a prior tool call |
| `responseSchema` | JSON Schema for `resume.payload` |
| `expiresAt` | ISO-8601 TTL; stale resumes produce `RUN_ERROR` |
| `metadata` | Framework-specific data |

### Resuming (client side)

```typescript
await agent.runAgent({
  // ... other params
  resume: [
    {
      interruptId: "interrupt-1",
      status: "resolved",
      payload: { approved: true }
    }
  ]
})
```

---

## Capabilities

Agents can declare runtime capabilities via `getCapabilities()`.

```typescript
interface AgentCapabilities {
  identity?: {
    name?: string
    type?: string      // "langgraph", "mastra", "crewai", etc.
    description?: string
    version?: string
    provider?: string
    documentationUrl?: string
    metadata?: Record<string, unknown>
  }
  transport?: {
    streaming?: boolean
    websocket?: boolean
    httpBinary?: boolean
    pushNotifications?: boolean
    resumable?: boolean
  }
  tools?: {
    supported?: boolean
    items?: Tool[]
    parallelCalls?: boolean
    clientProvided?: boolean
  }
  state?: {
    supported?: boolean
    deltaUpdates?: boolean
    persistence?: boolean
  }
  reasoning?: {
    supported?: boolean
    encrypted?: boolean
  }
  humanInTheLoop?: {
    supported?: boolean
    interrupts?: boolean
  }
  multiAgent?: {
    supported?: boolean
    delegation?: boolean
  }
  custom?: Record<string, unknown>
}
```

Query capabilities before connecting:

```typescript
const capabilities = await agent.getCapabilities?.()
if (capabilities?.tools?.supported) {
  // show tool UI
}
```
