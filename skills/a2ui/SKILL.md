---
name: a2ui
description: >
  Expert guidance for building agent-driven interfaces with A2UI — covering the
  JSONL streaming protocol, surfaces, components, data binding, catalogs,
  actions, and transports for both v0.8 (stable) and v0.9 (draft). Use this
  skill whenever someone is implementing A2UI, generating A2UI JSON from an LLM
  agent, building an A2UI renderer, configuring a catalog, working with
  surfaceUpdate / dataModelUpdate / beginRendering (v0.8) or createSurface /
  updateComponents / updateDataModel (v0.9), migrating from v0.8 to v0.9, or
  asking how agent-generated UI messages flow to a client. Always invoke this
  skill for any question mentioning A2UI, surfaceUpdate, createSurface,
  beginRendering, A2UI renderer, A2UI catalog, or agent-to-user interface, even
  if the question seems simple.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0"
---

# A2UI Skill

You are an expert in A2UI, the open standard that lets AI agents generate rich,
interactive UIs and stream them to clients as declarative JSON — without
executing arbitrary code.

## What A2UI Is

A2UI separates **UI generation** from **UI execution**:

1. An **agent** generates A2UI messages describing the interface (structure + data).
2. Messages **stream** to a client application over SSE / WebSockets / A2A.
3. The **A2UI renderer** on the client maps abstract component descriptions to
   its native widgets (Angular, Flutter, Lit, React, etc.).

Core properties:

- **Secure** — declarative data, not executable code; agents can only use
  pre-approved components from a catalog.
- **LLM-friendly** — flat adjacency list; easy to generate incrementally.
- **Framework-agnostic** — one JSON payload renders on any client.
- **Progressive** — stream components as they are generated.

## Specification Versions

| Version  | Status     | Key characteristics                                                                                                              |
| -------- | ---------- | -------------------------------------------------------------------------------------------------------------------------------- |
| **v0.8** | **Stable** | Production-ready. `surfaceUpdate`, `dataModelUpdate`, `beginRendering`. Typed adjacency-list data model.                         |
| **v0.9** | **Draft**  | Prompt-first redesign. `createSurface`, `updateComponents`, `updateDataModel`. Flat component format. Plain-JSON data model.     |

**Which version to use:**

- Default to **v0.8** for production work unless the user explicitly targets v0.9.
- Use **v0.9** when the user asks for the newer/draft API, mentions `createSurface`, or is migrating.

## Core Concepts

### Surfaces

A **surface** is an independent UI region identified by a `surfaceId`. Each
surface has its own component tree and data model. A single agent stream can
control many surfaces simultaneously.

### Adjacency List Model

Components are a **flat list** with ID references, not a nested tree.
This lets the LLM emit components one at a time and update any node by ID.

```text
root → card → column → [name-text, email-text]
```

Every component entry: `{ "id": "...", "component": { ... } }` (v0.8)\
or `{ "id": "...", "component": "TypeName", ...props }` (v0.9)

### Data Binding

Components reference data model values via JSON Pointer paths (`/user/name`).
When the data model changes, bound components re-render automatically.
The UI structure is sent once; subsequent updates are small data patches.

### Catalogs

A **catalog** defines which components the agent may use. The client declares
which catalogs it supports; the agent picks one per surface. Standard catalog
IDs:

- v0.8: `https://a2ui.org/specification/v0_8/standard_catalog_definition.json`
- v0.9: `https://a2ui.org/specification/v0_9/basic_catalog.json`

## Message Flow

### v0.8

```text
Agent → Client:
  surfaceUpdate     # add/update components
  dataModelUpdate   # update data
  beginRendering    # signal: start rendering (specifies root + catalogId)
  deleteSurface     # remove a surface

Client → Agent:
  userAction        # user interaction event
  error             # client-side error
```

### v0.9

```text
Agent → Client (each with "version": "v0.9"):
  createSurface     # create surface, declare catalogId, optional sendDataModel
  updateComponents  # add/update components
  updateDataModel   # update data (plain JSON at a path)
  deleteSurface     # remove a surface

Client → Agent:
  action            # user interaction event
  error             # VALIDATION_FAILED or other client error
```

## Minimal Working Examples

### v0.8 — Profile card

```jsonl
{"surfaceUpdate": {"surfaceId": "main", "components": [{"id": "root", "component": {"Column": {"children": {"explicitList": ["card"]}}}}]}}
{"surfaceUpdate": {"surfaceId": "main", "components": [{"id": "card", "component": {"Card": {"child": "name"}}}]}}
{"surfaceUpdate": {"surfaceId": "main", "components": [{"id": "name", "component": {"Text": {"text": {"path": "/user/name"}, "usageHint": "h2"}}}]}}
{"dataModelUpdate": {"surfaceId": "main", "contents": [{"key": "user", "valueMap": [{"key": "name", "valueString": "Alice"}]}]}}
{"beginRendering": {"surfaceId": "main", "root": "root"}}
```

### v0.9 — Profile card

```jsonl
{"version": "v0.9", "createSurface": {"surfaceId": "main", "catalogId": "https://a2ui.org/specification/v0_9/basic_catalog.json"}}
{"version": "v0.9", "updateComponents": {"surfaceId": "main", "components": [{"id": "root", "component": "Column", "children": ["card"]}, {"id": "card", "component": "Card", "child": "name"}, {"id": "name", "component": "Text", "text": {"path": "/user/name"}, "variant": "h2"}]}}
{"version": "v0.9", "updateDataModel": {"surfaceId": "main", "path": "/user", "value": {"name": "Alice"}}}
```

## Key Differences at a Glance

| Aspect              | v0.8                                     | v0.9                             |
| ------------------- | ---------------------------------------- | -------------------------------- |
| Component type      | `"component": {"Text": {...}}`           | `"component": "Text", ...props`  |
| Literal string      | `{"literalString": "Hi"}`                | `"Hi"`                           |
| Children list       | `{"explicitList": ["a","b"]}`            | `["a", "b"]`                     |
| Data update         | array of `{key, valueString/…}`          | plain JSON at a `path`           |
| Surface init        | `beginRendering` with `root`             | `createSurface` with `catalogId` |
| Button style        | `"primary": true`                        | `"variant": "primary"`           |
| Layout props        | `distribution`, `alignment`              | `justify`, `align`               |
| Choice widget       | `MultipleChoice`                         | `ChoicePicker`                   |
| TextField value     | `text`                                   | `value`                          |
| Action payload      | `{"userAction": {...}}`                  | `{"action": {...}}`              |

## Reference Files

Read the relevant reference file for deep technical details:

### v0.8 References

- **`references/v0.8/protocol.md`** — Complete v0.8 protocol spec: surfaces,
  message types, catalog negotiation, data model format, streaming lifecycle,
  schema examples.
- **`references/v0.8/components.md`** — All standard components with v0.8 JSON
  examples: layout (Row, Column, List), display (Text, Image, Icon, Divider),
  interactive (Button, TextField, CheckBox, Slider, DateTimeInput,
  MultipleChoice), container (Card, Modal, Tabs).
- **`references/v0.8/a2a-extension.md`** — How to use A2UI over the A2A
  protocol: extension URI, agent card, client capabilities, DataPart encoding.

### v0.9 References

- **`references/v0.9/protocol.md`** — Complete v0.9 protocol spec: new message
  types, flattened schema, unified catalog, validation feedback loop.
- **`references/v0.9/components.md`** — All components with v0.9 JSON examples
  and the v0.8 → v0.9 property-rename quick reference.
- **`references/v0.9/evolution-guide.md`** — Step-by-step migration from v0.8
  to v0.9: philosophy shift, every changed message type, component renames,
  side-by-side before/after examples.
- **`references/v0.9/actions.md`** — Actions in depth: Events vs. Functions,
  data model sync (`sendDataModel`), validation checks, multi-agent routing,
  security considerations.

## Tips for LLM Agents Generating A2UI

- Always include a component with `"id": "root"` — it is the render tree entry
  point.
- Use **descriptive IDs** (`"user-profile-card"` not `"c1"`).
- Send `beginRendering` (v0.8) / `createSurface` (v0.9) **after** the
  components are ready to avoid partial renders.
- For dynamic lists, bind the parent's `children` to a `template` with a
  `dataBinding` / `path` pointing to an array in the data model.
- Prefer data bindings over literal values for any content that might change.
- Keep hierarchies shallow — deep nesting is harder to update incrementally.
