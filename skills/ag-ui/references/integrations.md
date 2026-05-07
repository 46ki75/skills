# AG-UI Integrations Reference

Step-by-step guides for building AG-UI-compatible **server** and **middleware**
integrations.

## Table of Contents

- [Integration Types](#integration-types)
- [Server Integration (Python)](#server-integration-python)
- [Middleware Integration (TypeScript)](#middleware-integration-typescript)
- [Dojo: Local Development Environment](#dojo-local-development-environment)

---

## Integration Types

| Type | Description | When to use |
| --- | --- | --- |
| **Server** | Your agent emits AG-UI events directly over HTTP (SSE). Written in Python or any language. | New agent from scratch; maximum control; exposing as standalone API. |
| **Middleware** | TypeScript class that wraps an existing agent/protocol and translates its output to AG-UI events. | Adapting existing framework (OpenAI, Anthropic, custom); in-process integration. |

Both expose the same interface: `POST /` accepts `RunAgentInput`, responds with
`text/event-stream` of AG-UI events.

---

## Server Integration (Python)

### 1. Prerequisites

```bash
pip install ag-ui-protocol fastapi uvicorn openai
```

### 2. Minimal server scaffold

```python
# agent_server/main.py
from fastapi import FastAPI, Request
from fastapi.responses import StreamingResponse
from ag_ui.core import RunAgentInput, EventType, RunStartedEvent, RunFinishedEvent
from ag_ui.encoder import EventEncoder
import uuid

app = FastAPI()

@app.post("/")
async def run_agent(input_data: RunAgentInput, request: Request):
    async def event_stream():
        encoder = EventEncoder()

        yield encoder.encode(RunStartedEvent(
            type=EventType.RUN_STARTED,
            thread_id=input_data.thread_id,
            run_id=input_data.run_id,
        ))

        # --- your agent logic here ---

        yield encoder.encode(RunFinishedEvent(
            type=EventType.RUN_FINISHED,
            thread_id=input_data.thread_id,
            run_id=input_data.run_id,
        ))

    return StreamingResponse(event_stream(), media_type="text/event-stream")
```

```bash
uvicorn agent_server.main:app --reload --port 8000
```

### 3. Adding tool call support

```python
import json
from ag_ui.core import (
    ToolCallStartEvent, ToolCallArgsEvent, ToolCallEndEvent,
    ToolCallResultEvent, EventType,
)

async def handle_tool_call(encoder, tool_call_id, tool_name, args_dict):
    # Emit tool call start
    yield encoder.encode(ToolCallStartEvent(
        type=EventType.TOOL_CALL_START,
        tool_call_id=tool_call_id,
        tool_call_name=tool_name,
    ))
    # Stream args as JSON
    args_json = json.dumps(args_dict)
    yield encoder.encode(ToolCallArgsEvent(
        type=EventType.TOOL_CALL_ARGS,
        tool_call_id=tool_call_id,
        delta=args_json,
    ))
    yield encoder.encode(ToolCallEndEvent(
        type=EventType.TOOL_CALL_END,
        tool_call_id=tool_call_id,
    ))
```

### 4. Sharing state with the frontend

```python
from ag_ui.core import StateSnapshotEvent, StateDeltaEvent, EventType

# Send full state snapshot at run start
yield encoder.encode(StateSnapshotEvent(
    type=EventType.STATE_SNAPSHOT,
    snapshot={"step": 1, "results": []},
))

# Send incremental update via JSON Patch (RFC 6902)
yield encoder.encode(StateDeltaEvent(
    type=EventType.STATE_DELTA,
    delta=[
        {"op": "replace", "path": "/step", "value": 2},
        {"op": "add", "path": "/results/-", "value": "item"},
    ],
))
```

### 5. Emitting an interrupt

```python
from ag_ui.core import RunFinishedEvent, EventType
import uuid

yield encoder.encode(RunFinishedEvent(
    type=EventType.RUN_FINISHED,
    thread_id=input_data.thread_id,
    run_id=input_data.run_id,
    outcome={
        "type": "interrupt",
        "interrupts": [
            {
                "id": str(uuid.uuid4()),
                "reason": "approval_required",
                "message": "Do you approve deploying to production?",
                "responseSchema": {
                    "type": "object",
                    "properties": {"approved": {"type": "boolean"}},
                    "required": ["approved"],
                },
            }
        ],
    },
))
```

After the user resolves, the next `RunAgentInput` will contain `resume` with
the interrupt response. Check it:

```python
if input_data.resume:
    for item in input_data.resume:
        if item.status == "resolved":
            approved = item.payload.get("approved")
```

---

## Middleware Integration (TypeScript)

Middleware runs **in-process** alongside your frontend and wraps an existing
agent or API.

### 1. Prerequisites

```bash
npm install @ag-ui/client @ag-ui/core openai
```

### 2. Extend AbstractAgent

```typescript
import { AbstractAgent, AgentConfig, RunAgent } from "@ag-ui/client"
import {
  RunAgentInput, BaseEvent,
  EventType,
  RunStartedEvent, RunFinishedEvent,
  TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
} from "@ag-ui/core"
import { Observable } from "rxjs"
import OpenAI from "openai"
import { v4 as uuidv4 } from "uuid"

const openai = new OpenAI()

export class OpenAIAgent extends AbstractAgent {
  protected run(input: RunAgentInput): RunAgent {
    return new Observable<BaseEvent>((observer) => {
      ;(async () => {
        const runId = input.runId

        observer.next({
          type: EventType.RUN_STARTED,
          threadId: input.threadId,
          runId,
        } as RunStartedEvent)

        const messageId = uuidv4()
        observer.next({
          type: EventType.TEXT_MESSAGE_START,
          messageId,
        } as TextMessageStartEvent)

        const stream = await openai.chat.completions.create({
          model: "gpt-4o",
          messages: input.messages.map((m: any) => ({
            role: m.role,
            content: m.content ?? "",
          })),
          stream: true,
        })

        for await (const chunk of stream) {
          const delta = chunk.choices[0]?.delta?.content ?? ""
          if (delta) {
            observer.next({
              type: EventType.TEXT_MESSAGE_CONTENT,
              messageId,
              delta,
            } as TextMessageContentEvent)
          }
        }

        observer.next({
          type: EventType.TEXT_MESSAGE_END,
          messageId,
        } as TextMessageEndEvent)

        observer.next({
          type: EventType.RUN_FINISHED,
          threadId: input.threadId,
          runId,
        } as RunFinishedEvent)

        observer.complete()
      })().catch((err) => observer.error(err))
    })
  }
}
```

### 3. Use the agent in your frontend

```typescript
const agent = new OpenAIAgent({
  initialMessages: [],
})

// React / Next.js example
agent.subscribe({
  onTextMessageContent(event) {
    setStreamedText((prev) => prev + event.delta)
  },
  onStateSnapshot(event) {
    setAppState(event.snapshot)
  },
  onRunFinished(event) {
    if (event.outcome?.type === "interrupt") {
      setInterrupts(event.outcome.interrupts)
    }
  },
})

await agent.runAgent({
  tools: [myConfirmTool],
})
```

### 4. Handling tool calls in middleware

```typescript
import {
  ToolCallStartEvent, ToolCallArgsEvent, ToolCallEndEvent,
  ToolCallResultEvent, EventType,
} from "@ag-ui/core"

// Inside your Observable run():
observer.next({
  type: EventType.TOOL_CALL_START,
  toolCallId: "tc-1",
  toolCallName: "searchWeb",
} as ToolCallStartEvent)

observer.next({
  type: EventType.TOOL_CALL_ARGS,
  toolCallId: "tc-1",
  delta: JSON.stringify({ query: "AG-UI protocol" }),
} as ToolCallArgsEvent)

observer.next({
  type: EventType.TOOL_CALL_END,
  toolCallId: "tc-1",
} as ToolCallEndEvent)

// After frontend executes the tool:
observer.next({
  type: EventType.TOOL_CALL_RESULT,
  messageId: uuidv4(),
  toolCallId: "tc-1",
  role: "tool",
  content: JSON.stringify({ results: ["..."] }),
} as ToolCallResultEvent)
```

---

## Dojo: Local Development Environment

The **Dojo** is the official AG-UI demo app for testing integrations locally.

### Setup

```bash
git clone git@github.com:ag-ui-protocol/ag-ui.git
cd ag-ui

# Install JS dependencies
pnpm install

# Start the dojo dev server
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) and select your integration
from the dropdown.

### Registering a new integration in the Dojo

1. Add to `apps/dojo/src/menu.ts`:

```typescript
export const menuIntegrations: MenuIntegrationConfig[] = [
  {
    id: "my-agent",
    name: "My Agent",
    features: ["agentic_chat"],
  },
]
```

1. Add to `apps/dojo/src/agents.ts`:

```typescript
import { MyAgent } from "@ag-ui/my-agent"

export const agentsIntegrations: AgentIntegrationConfig[] = [
  {
    id: "my-agent",
    agents: async () => ({
      agentic_chat: new MyAgent(),
    }),
  },
]
```

1. Add the package to `apps/dojo/package.json` dependencies:

```json
{
  "dependencies": {
    "@ag-ui/my-agent": "workspace:*"
  }
}
```

### Available features (Dojo capability IDs)

| Feature ID | Description |
| --- | --- |
| `agentic_chat` | Basic streaming chat |
| `agentic_generative_ui` | Generative / declarative UI rendering |
| `human_in_the_loop` | Interrupt/resume flows |
| `tool_based_generative_ui` | Tool-driven UI updates |
| `shared_state` | Bidirectional state synchronization |
| `predictive_state_updates` | Optimistic state updates |
| `agentic_chat_with_tools` | Chat with tool call support |
