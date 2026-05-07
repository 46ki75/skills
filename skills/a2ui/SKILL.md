---
name: a2ui
description: >
  Expert guidance for generating A2UI (Agent to UI) protocol JSON — the
  streaming, declarative UI protocol for AI agents. Use this skill whenever
  someone is building an AI agent that produces UI, needs to generate A2UI
  JSONL/JSON streams, asks about surfaces, components, data binding, adjacency
  list models, catalogs, or wants to render agent responses as rich interactive
  interfaces. Covers both v0.8 (Stable, structured-output optimised) and v0.9
  (Draft, prompt-first). Always invoke this skill for any question that mentions
  A2UI, surfaceUpdate, updateComponents, beginRendering, createSurface,
  dataModelUpdate, updateDataModel, adjacency list UI, or agent-generated UI,
  even if the question seems simple.
license: Apache-2.0
metadata:
  author: "Ikuma Yamashita"
  version: "1.0"
---

# A2UI Skill

You are an expert in A2UI — the open protocol for AI agents to stream rich, interactive, declarative UIs to clients. Your goal is to help users generate correct, streaming A2UI JSON that works across web, mobile, and desktop renderers.

## What A2UI is

A2UI lets an AI agent describe a UI as a stream of JSON messages instead of returning plain text or executing arbitrary code. The client renders those messages using its own native widget set (Angular, Flutter, Lit, React, etc.), keeping UI generation safe across trust boundaries.

Key properties:
- **Declarative** — agents describe *what* to show, not *how* to render it
- **Streaming (JSONL)** — messages arrive line-by-line; clients render progressively
- **Adjacency list model** — components are a flat list linked by ID references, not a deeply nested tree
- **Data separated from structure** — `dataModel` updates change values without resending component trees
- **Catalog-driven** — the set of available components is defined by the client's catalog, not the protocol

## Choosing a version

| Version | Status | Best for |
|---------|--------|----------|
| **v0.8** | **Stable** | Production use; structured-output / function-calling LLMs |
| **v0.9** | Draft | Prompt-first / in-context schema; more natural JSON patterns |

When the user hasn't specified a version, **prefer v0.8** for production and **v0.9** when they want a more natural authoring experience or the system prompt approach.

## Core concepts (both versions)

### Surfaces

A **surface** is a rectangular UI region. Each surface has:
- A unique `surfaceId`
- Its own component tree (rooted at a component with `id: "root"`)
- Its own data model (avoids key collisions across multiple surfaces)
- A catalog that defines available component types

### Adjacency list model

The UI tree is **flat**. Container components reference children by their `id` strings. The client reconstructs the tree at render time. This is LLM-friendly: the model can output one component at a time without worrying about JSON nesting depth.

```
root (Column)
 ├── card (Card)
 │    └── title (Text)
 └── button (Button)
```

All four components are output as sibling entries in a flat list.

### Data binding

Properties that display dynamic data use a **binding** rather than a literal value. The data model holds the live state; components reference it by path.

- **v0.8**: `{ "path": "/user/name" }` or `{ "literalString": "Guest" }` inside a `BoundValue` wrapper
- **v0.9**: `{ "path": "/user/name" }` or just `"Guest"` as a plain string — implicit typing

### Message flow

```
Agent  →  createSurface / beginRendering (v0.8)
Agent  →  updateComponents / surfaceUpdate (v0.8)
Agent  →  updateDataModel / dataModelUpdate (v0.8)
Client →  userAction  (button click, form submit, …)
Agent  →  updateComponents / updateDataModel  (dynamic update)
Agent  →  deleteSurface
```

## Quick examples

### v0.8 — minimal "Hello World"

```jsonl
{"surfaceUpdate":{"surfaceId":"s1","components":[{"id":"root","component":{"Column":{"children":{"explicitList":["msg"]}}}}]}}
{"surfaceUpdate":{"surfaceId":"s1","components":[{"id":"msg","component":{"Text":{"text":{"literalString":"Hello, World!"}}}}]}}
{"dataModelUpdate":{"surfaceId":"s1","contents":{}}}
{"beginRendering":{"surfaceId":"s1","root":"root"}}
```

### v0.9 — minimal "Hello World"

```jsonl
{"version":"v0.9","createSurface":{"surfaceId":"s1","catalogId":"https://a2ui.org/specification/v0_9/basic_catalog.json"}}
{"version":"v0.9","updateComponents":{"surfaceId":"s1","components":[{"id":"root","component":"Column","children":["msg"]},{"id":"msg","component":"Text","text":"Hello, World!"}]}}
```

## Version comparison cheat-sheet

| Concept | v0.8 | v0.9 |
|---------|------|------|
| Surface init | `beginRendering` (sent *after* components) | `createSurface` (sent *first*) |
| Component updates | `surfaceUpdate` | `updateComponents` |
| Data updates | `dataModelUpdate` | `updateDataModel` |
| Component type | Key-wrapper: `{"Text": {...}}` | Discriminator field: `"component":"Text"` |
| Literal text | `{"literalString":"Hello"}` | `"Hello"` (plain string) |
| Data binding | `{"path":"/foo"}` inside `BoundValue` | `{"path":"/foo"}` or `"${/foo}"` in formatString |
| Data model payload | Array of typed key-value pairs | Plain JSON object |
| Button action context | Array of key-value pairs | Plain JSON object |
| Surface catalog | Optional — defaults to standard | Required `catalogId` URI |
| Version field | Not present | `"version":"v0.9"` on every message |

## Reference files

Read these for complete message schemas, full component catalogs, and detailed examples:

- **`references/v0.8.md`** — Complete v0.8 specification: message schemas, `BoundValue`, `dataModelUpdate`, `userAction`, full stream example, standard component catalog
- **`references/v0.9.md`** — Complete v0.9 specification: `createSurface`, `updateComponents`, `updateDataModel`, simplified data binding, string interpolation, client-side functions, full stream example
- **`references/evolution-guide.md`** — Side-by-side migration guide: every v0.8 → v0.9 change with before/after examples
