# A2UI Actions Guide — v0.9

User interactions, local functions, data model sync, multi-agent routing, and
security considerations.

## Table of Contents

1. [Action Types](#action-types)
2. [Events (Agent)](#events-agent)
3. [Functions (Local)](#functions-local)
4. [Validation Checks](#validation-checks)
5. [User Interaction Flow](#user-interaction-flow)
6. [Data Model Sync](#data-model-sync)
7. [Renderer Capabilities Advertisement](#renderer-capabilities-advertisement)
8. [Multi-Agent Routing](#multi-agent-routing)
9. [Security Considerations](#security-considerations)

---

## Action Types

An `action` on a component can be one of two types:

| Type         | Keyword        | Executed by                   | When to use                                      |
| ------------ | -------------- | ----------------------------- | ------------------------------------------------ |
| **Event**    | `event`        | Agent (network round-trip)    | Submit a form, navigate, trigger agent logic     |
| **Function** | `functionCall` | Renderer (local)              | Open a URL, toggle UI state, validate inputs     |

---

## Events (Agent)

Events send data to the agent for processing. They are defined inside
`action.event`.

```json
{
  "id": "submit-btn",
  "component": "Button",
  "child": "btn-text",
  "action": {
    "event": {
      "name": "submit_reservation",
      "context": {
        "time":  { "path": "/reservationTime" },
        "party": { "path": "/partySize" }
      }
    }
  }
}
```

- **`name`** — stable identifier; the agent switches on this.
- **`context`** — a map of values to send. Each value may be a literal or a
  JSON Pointer path resolved against the current data model at the moment the
  user interacts.

The agent receives an `action` payload:

```json
{
  "version": "v0.9",
  "action": {
    "name": "submit_reservation",
    "surfaceId": "booking-surface",
    "sourceComponentId": "submit-btn",
    "timestamp": "2026-02-25T10:40:00Z",
    "context": {
      "time":  "7:00 PM",
      "party": 4
    }
  }
}
```

**Context vs. Data Model**: `context` is a hand-picked subset of state for this
specific event. It simplifies the agent's job by providing exactly the values
needed — without requiring the agent to navigate the full data model.

---

## Functions (Local)

Functions execute on the renderer without a network round-trip. The agent is
not informed when a local function fires.

```json
{
  "id": "help-btn",
  "component": "Button",
  "child": "help-text",
  "action": {
    "functionCall": {
      "call": "openUrl",
      "args": { "url": "https://a2ui.org/help" }
    }
  }
}
```

Common built-in functions: `openUrl`, `toggleVisibility`, `scrollTo`.

Custom functions can be registered in the renderer's function registry and
advertised in the catalog.

---

## Validation Checks

Interactive components support a `checks` list. If any check fails, the
component (e.g., a Button) is **automatically disabled** by the renderer before
the user can trigger an action.

```json
{
  "id": "submit-btn",
  "component": "Button",
  "child": "btn-text",
  "checks": [
    {
      "condition": {
        "call": "required",
        "args": { "value": { "path": "/partySize" } }
      },
      "message": "Party size is required"
    },
    {
      "condition": {
        "call": "minLength",
        "args": { "value": { "path": "/name" }, "min": 2 }
      },
      "message": "Name must be at least 2 characters"
    }
  ],
  "action": { "event": { "name": "submit_booking" } }
}
```

Checks are **UX-focused** — they prevent invalid interactions before the user
tries to submit. They do not replace server-side data integrity checks.

---

## User Interaction Flow

When the user interacts with a component:

1. **Write** — the renderer immediately writes any new input value to the local
   data model (synchronous, no network).
2. **Check** — validation checks re-evaluate; the component enables/disables.
3. **Resolve** — all `path` references in `context` are resolved against the
   current local data model.
4. **Dispatch** — the `action` payload is sent to the agent via the transport
   (A2A, WebSockets, etc.).

Because local writes are synchronous, there are no race conditions between
typing and clicking — the "write" always commits before `context` paths are
resolved.

---

## Data Model Sync

v0.9 introduced automatic data model synchronization, where the renderer sends
the **entire surface data model** in the metadata of every outgoing message.

### Enable sync

Set `sendDataModel: true` when creating a surface:

```json
{
  "version": "v0.9",
  "createSurface": {
    "surfaceId": "booking-surface",
    "catalogId": "https://a2ui.org/specification/v0_9/basic_catalog.json",
    "sendDataModel": true
  }
}
```

### On the wire (A2A)

The data model is attached to the `metadata` field of the A2A envelope — not
as a separate A2UI message:

```json
{
  "parts": [{ "text": "Submit the reservation" }],
  "metadata": {
    "a2uiClientDataModel": {
      "version": "v0.9",
      "surfaces": {
        "booking-surface": {
          "reservationTime": "7:00 PM",
          "partySize": 4,
          "notes": "Window seat preferred"
        }
      }
    }
  }
}
```

### Why use data model sync?

- **Stateless agents**: The agent receives full current context with every
  interaction — no need to maintain per-session state.
- **Simpler wiring**: No need to map every input field into button `context`.
- **Verbal shortcuts**: Users can say "okay submit" (text or voice) without
  clicking a button; the agent sees the full data model in metadata and acts.

---

## Renderer Capabilities Advertisement

Before an agent can send UI, the renderer must advertise which catalogs it
supports. This object is placed in the `metadata` of every client → agent A2A
message:

```json
{
  "v0.9": {
    "supportedCatalogIds": [
      "https://a2ui.org/specification/v0_9/basic_catalog.json",
      "https://my-company.com/catalogs/v1/custom.json"
    ],
    "inlineCatalogs": []
  }
}
```

`inlineCatalogs` allows sending the full catalog schema inline (useful in
development).

---

## Multi-Agent Routing

In multi-agent systems an **Orchestrator** routes `action` messages to the
correct sub-agent based on which agent owns the surface.

### Recording ownership

When a sub-agent emits `createSurface`, the orchestrator records the mapping:

```python
def on_surface_created(surface_id, agent_name, session):
    session.state[f"owner_of_{surface_id}"] = agent_name
```

### Routing events

```python
async def handle_incoming_action(payload, session):
    surface_id = payload["action"]["surfaceId"]
    target_agent = session.state.get(f"owner_of_{surface_id}")
    if target_agent:
        return transfer_to(target_agent)
```

### Data model isolation

When `sendDataModel` is enabled, the orchestrator **must strip** the metadata
to include only surfaces owned by the target sub-agent before forwarding:

```python
async def intercept(self, request_payload, target_agent, session):
    data_model = request_payload["params"]["message"].get("metadata", {}).get("a2uiClientDataModel")
    if data_model:
        data_model["surfaces"] = {
            sid: state
            for sid, state in data_model["surfaces"].items()
            if session.state.get(f"owner_of_{sid}") == target_agent.name
        }
    return request_payload
```

---

## Security Considerations

### Sandboxed execution

Agents can only trigger pre-registered behaviors through the `functionCall`
mechanism. Arbitrary code injection (e.g., raw JavaScript) from the agent is
not possible.

### Data model isolation

When `sendDataModel: true` is used with multiple surfaces owned by different
agents, an orchestrator **must** strip the `a2uiClientDataModel` so that each
sub-agent only receives its own surfaces' data.

Failure to do so is a **state scraping** security risk — a malicious sub-agent
could read sensitive data from another agent's surface (e.g., a weather agent
reading banking data).

### VALIDATION_FAILED error

If the agent sends A2UI JSON that violates the catalog schema, the renderer
returns a structured error:

```json
{
  "version": "v0.9",
  "error": {
    "code": "VALIDATION_FAILED",
    "surfaceId": "booking-surface",
    "path": "/components/0/text",
    "message": "Expected string, got number"
  }
}
```

This enables the LLM to self-correct in the next turn.

### Point-to-point visibility

The `a2uiClientDataModel` payload is only visible to the immediate backend
receiving the transport envelope. Use transport-level encryption (TLS) to
protect data in transit.
