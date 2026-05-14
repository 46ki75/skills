# AG-UI JavaScript / TypeScript SDK Reference

Complete reference for `@ag-ui/client` and `@ag-ui/core`.

```bash
npm install @ag-ui/client
```

## Table of Contents

- [Core Types](#core-types)
- [AbstractAgent](#abstractagent)
- [HttpAgent](#httpagent)
- [Middleware](#middleware)
- [AgentSubscriber](#agentsubscriber)
- [Event Type Definitions](#event-type-definitions)

---

## Core Types

```typescript
import {
  RunAgentInput, RunAgentParameters, RunAgentResult,
  Message, UserMessage, AssistantMessage, SystemMessage,
  ToolMessage, DeveloperMessage, ReasoningMessage,
  Tool, Context, ToolCall,
  BaseEvent, EventType,
  RunStartedEvent, RunFinishedEvent, RunErrorEvent,
  StepStartedEvent, StepFinishedEvent,
  TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
  TextMessageChunkEvent,
  ToolCallStartEvent, ToolCallArgsEvent, ToolCallEndEvent, ToolCallResultEvent,
  ToolCallChunkEvent,
  StateSnapshotEvent, StateDeltaEvent, MessagesSnapshotEvent,
  ReasoningStartEvent, ReasoningMessageStartEvent,
  ReasoningMessageContentEvent, ReasoningMessageEndEvent,
  ReasoningMessageChunkEvent, ReasoningEndEvent,
  ReasoningEncryptedValueEvent,
  CustomEvent, RawEvent,
  AgentCapabilities,
} from "@ag-ui/core"
```

### RunAgentParameters

```typescript
interface RunAgentParameters {
  runId?: string
  tools?: Tool[]
  context?: Context[]
  forwardedProps?: Record<string, any>
  resume?: ResumeItem[]
}

interface ResumeItem {
  interruptId: string
  status: "resolved" | "cancelled"
  payload?: any
}
```

### RunAgentResult

```typescript
interface RunAgentResult {
  result: any
  newMessages: Message[]
}
```

---

## AbstractAgent

Base class for all agent implementations. Extend it to build a custom agent.

```typescript
import { AbstractAgent, AgentConfig } from "@ag-ui/client"

interface AgentConfig {
  agentId?: string
  description?: string
  threadId?: string
  initialMessages?: Message[]
  initialState?: any
}

class MyAgent extends AbstractAgent {
  protected run(input: RunAgentInput): RunAgent {
    // return an Observable<BaseEvent>
  }
}
```

### Core methods

| Method | Signature | Description |
| --- | --- | --- |
| `runAgent()` | `(params?, subscriber?) => Promise<RunAgentResult>` | Execute the agent |
| `subscribe()` | `(subscriber) => { unsubscribe }` | Register a persistent event subscriber |
| `abortRun()` | `() => void` | Cancel the current run |
| `use()` | `(middleware) => this` | Add middleware to the pipeline |
| `clone()` | `() => AbstractAgent` | Clone the agent with current state |
| `getCapabilities()` | `() => Promise<AgentCapabilities \| undefined>` | Query runtime capabilities |

### Properties

```typescript
agent.messages     // Message[]   — current conversation history
agent.state        // any         — current agent state
agent.threadId     // string      — conversation thread ID
```

---

## HttpAgent

Concrete HTTP-based agent. Points at a remote AG-UI endpoint.

```typescript
import { HttpAgent } from "@ag-ui/client"

const agent = new HttpAgent({
  url: "https://my-agent.example.com/api",
  headers: {
    Authorization: "Bearer your-api-key",
  },
})

const { result, newMessages } = await agent.runAgent({
  tools: [myTool],
})
```

### HttpAgentConfig

```typescript
interface HttpAgentConfig extends AgentConfig {
  url: string
  headers?: Record<string, string>
}
```

### Protected method: requestInit

Override to customize the HTTP request:

```typescript
class MyHttpAgent extends HttpAgent {
  protected requestInit(input: RunAgentInput): RequestInit {
    return {
      ...super.requestInit(input),
      headers: {
        ...this.headers,
        "X-Custom-Header": "value",
      },
    }
  }
}
```

The default request is:

```typescript
{
  method: "POST",
  headers: {
    "Content-Type": "application/json",
    Accept: "text/event-stream",
    ...this.headers,
  },
  body: JSON.stringify(input),
  signal: this.abortController.signal,
}
```

---

## Middleware

Middleware intercepts the event stream flowing through an agent. Apply with
`agent.use(middleware)`.

### Function-based middleware

```typescript
import { EventMiddleware } from "@ag-ui/client"

const loggingMiddleware: EventMiddleware = (event, next) => {
  console.log("Event:", event.type)
  return next(event)
}

agent.use(loggingMiddleware)
```

### Class-based middleware

```typescript
import { AbstractMiddleware } from "@ag-ui/client"

class FilterMiddleware extends AbstractMiddleware {
  process(event: BaseEvent, next: (e: BaseEvent) => Observable<BaseEvent>) {
    if (event.type === EventType.CUSTOM) {
      return EMPTY   // drop CUSTOM events
    }
    return next(event)
  }
}

agent.use(new FilterMiddleware())
```

### Built-in middleware

```typescript
import { FilterToolCallsMiddleware } from "@ag-ui/client"

// Only allow specific tool calls through
agent.use(new FilterToolCallsMiddleware({ allowedTools: ["search", "calculate"] }))
```

---

## AgentSubscriber

Event-driven hooks for reacting to agent lifecycle events without modifying the
stream.

```typescript
import { AgentSubscriber } from "@ag-ui/client"

const subscriber: AgentSubscriber = {
  // Lifecycle
  onRunStarted(event: RunStartedEvent) {},
  onRunFinished(event: RunFinishedEvent) {},
  onRunError(event: RunErrorEvent) {},
  onStepStarted(event: StepStartedEvent) {},
  onStepFinished(event: StepFinishedEvent) {},

  // Text messages
  onTextMessageStart(event: TextMessageStartEvent) {},
  onTextMessageContent(event: TextMessageContentEvent) {},
  onTextMessageEnd(event: TextMessageEndEvent) {},

  // Tool calls
  onToolCallStart(event: ToolCallStartEvent) {},
  onToolCallArgs(event: ToolCallArgsEvent) {},
  onToolCallEnd(event: ToolCallEndEvent) {},
  onToolCallResult(event: ToolCallResultEvent) {},

  // State
  onStateSnapshot(event: StateSnapshotEvent) {},
  onStateDelta(event: StateDeltaEvent) {},
  onMessagesSnapshot(event: MessagesSnapshotEvent) {},

  // Reasoning
  onReasoningStart(event: ReasoningStartEvent) {},
  onReasoningEnd(event: ReasoningEndEvent) {},
}

// Register for all runs
const { unsubscribe } = agent.subscribe(subscriber)

// Or pass for a single run
await agent.runAgent(params, subscriber)
```

### State mutation control

```typescript
const subscriber: AgentSubscriber = {
  onStateSnapshot(event) {
    // Return false to prevent the default state replacement
    return false
  },
  onStateDelta(event) {
    // Return false to prevent the default patch application
    return false
  },
}
```

---

## Event Type Definitions

TypeScript interfaces for all AG-UI events.

```typescript
// All events extend BaseEvent
interface BaseEvent {
  type: EventType
  timestamp?: number
  rawEvent?: any
}

// Lifecycle
interface RunStartedEvent extends BaseEvent {
  type: EventType.RUN_STARTED
  threadId: string
  runId: string
  parentRunId?: string
  input?: RunAgentInput
}

interface RunFinishedEvent extends BaseEvent {
  type: EventType.RUN_FINISHED
  threadId: string
  runId: string
  result?: any
  outcome?: RunFinishedOutcome
}

type RunFinishedOutcome =
  | { type: "success" }
  | { type: "interrupt"; interrupts: Interrupt[] }

interface RunErrorEvent extends BaseEvent {
  type: EventType.RUN_ERROR
  message: string
  code?: string
}

// Text
interface TextMessageStartEvent extends BaseEvent {
  type: EventType.TEXT_MESSAGE_START
  messageId: string
  role?: "assistant"
}
interface TextMessageContentEvent extends BaseEvent {
  type: EventType.TEXT_MESSAGE_CONTENT
  messageId: string
  delta: string
}
interface TextMessageEndEvent extends BaseEvent {
  type: EventType.TEXT_MESSAGE_END
  messageId: string
}

// Tool calls
interface ToolCallStartEvent extends BaseEvent {
  type: EventType.TOOL_CALL_START
  toolCallId: string
  toolCallName: string
  parentMessageId?: string
}
interface ToolCallArgsEvent extends BaseEvent {
  type: EventType.TOOL_CALL_ARGS
  toolCallId: string
  delta: string
}
interface ToolCallEndEvent extends BaseEvent {
  type: EventType.TOOL_CALL_END
  toolCallId: string
}
interface ToolCallResultEvent extends BaseEvent {
  type: EventType.TOOL_CALL_RESULT
  messageId: string
  toolCallId: string
  role: "tool"
  content: string
}

// State
interface StateSnapshotEvent extends BaseEvent {
  type: EventType.STATE_SNAPSHOT
  snapshot: any
}
interface StateDeltaEvent extends BaseEvent {
  type: EventType.STATE_DELTA
  delta: JsonPatchOperation[]
}
```

> Note: TypeScript uses camelCase field names (`messageId`, `toolCallId`),
> while the Python SDK uses snake_case (`message_id`, `tool_call_id`).
> The wire format (JSON) uses camelCase.
