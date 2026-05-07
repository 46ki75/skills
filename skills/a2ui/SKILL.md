---
name: a2ui
description: >
  Expert guidance for implementing the A2UI (Agent to UI) protocol — covering
  surfaces, components, data models, catalogs, data binding, message streams
  (surfaceUpdate/dataModelUpdate/beginRendering for v0.8;
  createSurface/updateComponents/updateDataModel for v0.9), A2A extension
  integration, custom catalogs, and client/renderer architecture. Use this
  skill whenever someone is building or integrating an A2UI agent, renderer,
  or client; asking how to generate or consume A2UI JSON; working with the
  A2UI protocol specification; implementing catalog negotiation; or migrating
  from v0.8 to v0.9. Always invoke this skill for any question that mentions
  A2UI, surfaceUpdate, beginRendering, createSurface, updateComponents,
  a2uiClientCapabilities, or agent-driven UI streaming, even if the question
  seems simple.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0"
---

# A2UI Skill

You are an expert in the A2UI (Agent to UI) protocol — a declarative, streaming
JSON protocol that enables AI agents to generate rich, interactive UIs that
render natively on any platform (web, mobile, desktop) without executing
arbitrary code.

## What A2UI Is

A2UI transmits abstract component trees as JSONL streams. The agent describes
the UI structure; the client renders it using its own native widgets. Key
properties:

- **Declarative and LLM-friendly** — flat component lists with ID references
- **Platform-agnostic** — same JSON renders on Angular, Flutter, React, iOS
- **Separation of concerns** — UI structure, application state, and rendering
  are decoupled
- **Secure** — declarative data only, no code execution

## Versions

There are two public versions:

| Version | Status | Key message types |
| :------ | :----- | :---------------- |
| **v0.8** | Stable (recommended for production) | `surfaceUpdate`, `dataModelUpdate`, `beginRendering`, `deleteSurface` |
| **v0.9** | Draft | `createSurface`, `updateComponents`, `updateDataModel`, `deleteSurface` |

v0.9 is a "prompt-first" redesign — flatter component syntax, standard JSON
objects instead of key-value arrays, and a unified catalog format. Use v0.8 for
production; v0.9 for new projects that can tolerate draft-spec risk.

## Core Concepts

- **Surface** — a named UI region (dialog, sidebar, chat bubble). Each surface
  has its own component tree and data model.
- **Component** — an abstract UI element (Text, Button, Row, Card…) drawn from
  a catalog.
- **Data Model** — a JSON object holding dynamic state. Components bind to paths
  within it.
- **Catalog** — a JSON Schema document defining the available component and
  function types for a client.
- **A2A Extension** — how A2UI is advertised and activated when transported over
  the A2A protocol.

## Reference Files

Read the appropriate reference files as needed:

### v0.8 (Stable)

- **`references/v0.8/protocol.md`** — Full v0.8 protocol specification:
  message schemas, data binding (`literalString` / `dataBinding`), component
  tree structure, catalog negotiation, `beginRendering`, `surfaceUpdate`,
  `dataModelUpdate`.
- **`references/v0.8/a2a-extension.md`** — How to advertise and activate A2UI
  over the A2A transport; `a2uiClientCapabilities`; `DataPart` encoding.
- **`references/v0.8/custom-catalog.md`** — Per-request catalog negotiation,
  `supportedCatalogIds`, `inlineCatalogs`, and implementation guide for agent
  and renderer developers.

### v0.9 (Draft)

- **`references/v0.9/protocol.md`** — Full v0.9 protocol specification:
  `createSurface`, `updateComponents`, `updateDataModel`, flatter component
  syntax, standard JSON data model, `sendDataModel` client→server sync.
- **`references/v0.9/a2a-extension.md`** — v0.9 A2A extension URI, server
  capabilities schema, `a2uiClientCapabilities`.
- **`references/v0.9/evolution-guide.md`** — Comprehensive diff between v0.8
  and v0.9: philosophy shift, renamed messages, schema changes, migration steps.
  Read this when helping someone migrate or understand the differences.
- **`references/v0.9/custom-functions.md`** — How to define custom functions
  inside a catalog (e.g., `trim`, `getScreenResolution`).
- **`references/v0.9/basic-catalog-guide.md`** — Visual and functional
  implementation guide for every component in the Basic Catalog (Text, Button,
  Image, Row, Column, Card, etc.).
- **`references/v0.9/renderer-guide.md`** — Client/renderer architecture:
  layered design (MessageProcessor → SurfaceModel → ComponentImplementation),
  framework-specific adapters, binder layer, custom component patterns.

## When to Read Which Files

| User is asking about… | Read |
| :-------------------- | :--- |
| v0.8 message format / schema | `references/v0.8/protocol.md` |
| v0.8 A2A integration | `references/v0.8/a2a-extension.md` |
| v0.8 custom catalogs | `references/v0.8/custom-catalog.md` |
| v0.9 message format / schema | `references/v0.9/protocol.md` |
| v0.9 A2A integration | `references/v0.9/a2a-extension.md` |
| v0.8 → v0.9 migration / differences | `references/v0.9/evolution-guide.md` |
| Custom functions in v0.9 | `references/v0.9/custom-functions.md` |
| Rendering components (any framework) | `references/v0.9/basic-catalog-guide.md` |
| Building a client/renderer | `references/v0.9/renderer-guide.md` |
