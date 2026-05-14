# AG-UI Python SDK Reference

Complete reference for `ag-ui-protocol` — the official Python SDK.

```bash
pip install ag-ui-protocol
```

## Table of Contents

- [Core Types](#core-types)
- [Event Types](#event-types)
- [EventEncoder](#eventencoder)
- [FastAPI Server Pattern](#fastapi-server-pattern)
- [Complete Streaming Example](#complete-streaming-example)

---

## Core Types

### Imports

```python
from ag_ui.core import (
    RunAgentInput, Message, Tool, Context,
    UserMessage, AssistantMessage, SystemMessage, ToolMessage,
    DeveloperMessage, ReasoningMessage,
    ToolCall, Role, EventType, Event,
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
)
from ag_ui.encoder import EventEncoder
```

### RunAgentInput

```python
class RunAgentInput(ConfiguredBaseModel):
    thread_id: str
    run_id: str
    parent_run_id: Optional[str] = None
    state: Any
    messages: List[Message]
    tools: List[Tool]
    context: List[Context]
    forwarded_props: Any
    resume: Optional[List[ResumeItem]] = None
```

### Tool

```python
class Tool(ConfiguredBaseModel):
    name: str
    description: str
    parameters: dict       # JSON Schema object
```

### Context

```python
class Context(ConfiguredBaseModel):
    description: str
    value: str
```

### ToolCall

```python
class ToolCall(ConfiguredBaseModel):
    id: str
    type: Literal["function"] = "function"
    function: FunctionCall

class FunctionCall(ConfiguredBaseModel):
    name: str
    arguments: str          # JSON-serialized arguments string
```

---

## Event Types

All events use `EventType` enum values as their `type` field. See
`references/concepts.md` for full event definitions.

Quick import pattern:

```python
from ag_ui.core import EventType, RunStartedEvent, TextMessageContentEvent

event = TextMessageContentEvent(
    type=EventType.TEXT_MESSAGE_CONTENT,
    message_id="msg-1",
    delta="Hello"
)
```

The top-level `Event` type is a Pydantic discriminated union — use it to parse
incoming events from a wire format:

```python
from ag_ui.core import Event
from pydantic import TypeAdapter

adapter = TypeAdapter(Event)
parsed = adapter.validate_python({"type": "TEXT_MESSAGE_CONTENT", "messageId": "m1", "delta": "hi"})
```

---

## EventEncoder

Serializes events to SSE (Server-Sent Events) wire format.

```python
from ag_ui.encoder import EventEncoder

encoder = EventEncoder()
sse_string = encoder.encode(event)
# Output: 'data: {"type":"TEXT_MESSAGE_CONTENT","messageId":"msg_123","delta":"Hello"}\n\n'
```

### Constructor

```python
EventEncoder(accept: str = None)
```

`accept` sets the `Content-Type`/`Accept` hint. The default produces SSE.

### Method: encode

```python
encoder.encode(event: BaseEvent) -> str
```

Returns the SSE-formatted string ready to write to an HTTP response stream.

---

## FastAPI Server Pattern

Standard pattern for building an AG-UI-compatible HTTP server with FastAPI:

```python
from fastapi import FastAPI, Request
from fastapi.responses import StreamingResponse
from ag_ui.core import (
    RunAgentInput, EventType,
    RunStartedEvent, RunFinishedEvent,
    TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
)
from ag_ui.encoder import EventEncoder
import uuid

app = FastAPI()

@app.post("/")
async def run_agent(input_data: RunAgentInput, request: Request):
    async def event_stream():
        encoder = EventEncoder()
        run_id = input_data.run_id

        # Signal start
        yield encoder.encode(RunStartedEvent(
            type=EventType.RUN_STARTED,
            thread_id=input_data.thread_id,
            run_id=run_id,
        ))

        # Stream a text message
        msg_id = str(uuid.uuid4())
        yield encoder.encode(TextMessageStartEvent(
            type=EventType.TEXT_MESSAGE_START,
            message_id=msg_id,
        ))
        for chunk in ["Hello", ", ", "world", "!"]:
            yield encoder.encode(TextMessageContentEvent(
                type=EventType.TEXT_MESSAGE_CONTENT,
                message_id=msg_id,
                delta=chunk,
            ))
        yield encoder.encode(TextMessageEndEvent(
            type=EventType.TEXT_MESSAGE_END,
            message_id=msg_id,
        ))

        # Signal completion
        yield encoder.encode(RunFinishedEvent(
            type=EventType.RUN_FINISHED,
            thread_id=input_data.thread_id,
            run_id=run_id,
        ))

    return StreamingResponse(event_stream(), media_type="text/event-stream")
```

Run with:

```bash
uvicorn my_agent:app --host 0.0.0.0 --port 8000
```

---

## Complete Streaming Example

LLM integration pattern (OpenAI-style):

```python
import uuid
from openai import AsyncOpenAI
from ag_ui.core import (
    RunAgentInput, EventType,
    RunStartedEvent, RunFinishedEvent,
    TextMessageStartEvent, TextMessageContentEvent, TextMessageEndEvent,
    StateSnapshotEvent,
)
from ag_ui.encoder import EventEncoder

client = AsyncOpenAI()

async def run_agent_stream(input_data: RunAgentInput):
    encoder = EventEncoder()
    run_id = input_data.run_id

    yield encoder.encode(RunStartedEvent(
        type=EventType.RUN_STARTED,
        thread_id=input_data.thread_id,
        run_id=run_id,
    ))

    # Optionally snapshot state at start
    if input_data.state:
        yield encoder.encode(StateSnapshotEvent(
            type=EventType.STATE_SNAPSHOT,
            snapshot=input_data.state,
        ))

    msg_id = str(uuid.uuid4())
    yield encoder.encode(TextMessageStartEvent(
        type=EventType.TEXT_MESSAGE_START,
        message_id=msg_id,
    ))

    # Convert AG-UI messages to OpenAI format
    openai_messages = [
        {"role": m.role, "content": m.content}
        for m in input_data.messages
        if hasattr(m, "content") and m.content
    ]

    stream = await client.chat.completions.create(
        model="gpt-4o",
        messages=openai_messages,
        stream=True,
    )

    async for chunk in stream:
        delta = chunk.choices[0].delta.content or ""
        if delta:
            yield encoder.encode(TextMessageContentEvent(
                type=EventType.TEXT_MESSAGE_CONTENT,
                message_id=msg_id,
                delta=delta,
            ))

    yield encoder.encode(TextMessageEndEvent(
        type=EventType.TEXT_MESSAGE_END,
        message_id=msg_id,
    ))

    yield encoder.encode(RunFinishedEvent(
        type=EventType.RUN_FINISHED,
        thread_id=input_data.thread_id,
        run_id=run_id,
    ))
```
