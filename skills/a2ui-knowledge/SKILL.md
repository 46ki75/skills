---
name: a2ui-knowledge
description: >
  Expert guidance for implementing the A2UI (Agent to UI) protocol — covering
  surfaces, components, data binding, catalogs, message streams (v0.9:
  createSurface / updateComponents / updateDataModel / deleteSurface /
  sendDataModel; v0.8: surfaceUpdate / dataModelUpdate / beginRendering /
  deleteSurface), the A2A extension binding, custom component catalogs
  (defining schemas, registering renderers, catalog negotiation, versioning,
  graceful degradation, two-phase validation), client/renderer architecture
  (MessageProcessor / SurfaceModel / ComponentImplementation / Binder layer),
  custom functions, and v0.8 → v0.9 migration. Always invoke this skill for
  any question that mentions A2UI, surfaceUpdate, beginRendering,
  createSurface, updateComponents, updateDataModel, sendDataModel,
  a2uiClientCapabilities, supportedCatalogIds, catalogId, basic_catalog,
  DynamicComponent, formatString, ChildList, ComponentId, or agent-driven UI
  streaming — even if the question seems simple.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.1.0"
---

# A2UI Skill

You are an expert in the A2UI (Agent to UI) protocol — a declarative,
streaming JSON protocol that lets AI agents generate rich, interactive UIs
which render natively on any platform (web, mobile, desktop) without
executing arbitrary code.

## What A2UI Is

A2UI transmits abstract component trees as streams of JSON messages. The
agent describes UI structure; the client renders it with its own native
widgets. Key properties:

- **Declarative and LLM-friendly** — flat component lists keyed by id
  (adjacency-list model) so the LLM doesn't have to generate deeply nested
  JSON in a single pass.
- **Platform-agnostic** — the same JSON renders on React, Angular, Lit,
  Flutter, iOS, Android.
- **Separation of concerns** — UI structure, application state, and
  rendering are decoupled.
- **Secure** — declarative data only, no code execution. Component types
  are restricted to an allowlist (the active catalog).

## Versions

Per upstream `submodules/A2UI/docs/roadmap.md`:

| Version      | Status                                          | Key server-to-client message types                                      |
| :----------- | :---------------------------------------------- | :---------------------------------------------------------------------- |
| v0.8         | Prior — initial public release, minimal support | `surfaceUpdate`, `dataModelUpdate`, `beginRendering`, `deleteSurface`   |
| v0.9         | Current — feature complete, supported           | `createSurface`, `updateComponents`, `updateDataModel`, `deleteSurface` |
| v0.10 / v1.0 | Draft                                           | (in development)                                                        |

Default to v0.9 unless the user is working on or migrating from a v0.8
deployment.

v0.9 is a **prompt-first** redesign: the schema is meant to live in the LLM's
system prompt rather than rely solely on structured output. It has a flatter
component syntax, standard JSON for data models, a unified catalog
(components + functions), explicit client-to-server data sync via
`sendDataModel`, and a structured `VALIDATION_FAILED` feedback loop. v0.8
was optimized for strict structured output / function calling.

## Core Concepts

- **Surface** — a named UI region (chat bubble, side panel, dialog). Has
  its own component tree and data model. One stream can drive many.
- **Component** — an abstract UI element (`Text`, `Button`, `Row`, `Card`,
  …) drawn from a catalog. Components reference children by id, not by
  nesting.
- **Data Model** — a JSON object per surface; components bind to JSON
  Pointer paths within it. v0.9 supports relative paths inside templates.
- **Catalog** — a JSON Schema document defining the available component and
  function types. The Basic Catalog is the spec's reference; production apps
  usually define their own.
- **A2A Extension** — how A2UI is advertised and activated when transported
  over the A2A protocol. v0.8 URI: `…/a2a-extension/a2ui/v0.8`. v0.9 URI:
  `…/a2a-extension/a2ui/v0.9`.

## Reference Files

Read the appropriate reference as needed.

### v0.9 (current)

- **`references/v0.9/protocol.md`** — Full v0.9 protocol: `createSurface`,
  `updateComponents`, `updateDataModel`, `deleteSurface`, `sendDataModel`,
  flat component syntax with `"component": "Type"`, JSON-Pointer binding,
  root/relative scope, `formatString`, the prompt → generate → validate
  loop, and capability metadata.
- **`references/v0.9/a2a-extension.md`** — v0.9 extension URI, Agent Card
  params, list-of-messages DataPart encoding, metadata fields
  (`a2uiClientCapabilities`, `a2uiClientDataModel`).
- **`references/v0.9/evolution-guide.md`** — Comprehensive v0.8 → v0.9
  diff: philosophy shift, renamed messages, schema changes, component
  property renames, and a migration checklist. Read this when helping
  someone migrate or explaining why v0.9 looks different.
- **`references/v0.9/custom-functions.md`** — How to define custom
  functions inside a catalog, expose them via `anyFunction`, and have
  validators recognize them (e.g. `trim`, `getScreenResolution`).
- **`references/v0.9/basic-catalog-guide.md`** — Per-component rendering
  guidance for every Basic Catalog component and function, plus the
  Leaf-Margin spacing strategy and color/contrast inheritance pattern.
- **`references/v0.9/renderer-guide.md`** — Client/renderer architecture:
  the agnostic data layer (`MessageProcessor`, `SurfaceModel`,
  `DataModel`, `ComponentContext`), the catalog API, three binder
  strategies (direct, binder layer, generic), lifecycle/memory rules, and
  the step-by-step build plan.
- **`references/v0.9/custom-catalog-guide.md`** — End-to-end custom
  catalog workflow: defining a schema, extending or cherry-picking from
  the Basic Catalog, bundling with `assemble_catalog.py`, the four-step
  authoring loop (schema → implement → register → invoke), the
  catalog-negotiation handshake, versioning + breaking-change rules,
  two-phase validation, graceful degradation, security, and agent-side
  ADK integration (`A2uiSchemaManager`, `SendA2uiToClientToolset`,
  `A2uiEventConverter`).

### v0.8 (prior)

- **`references/v0.8/protocol.md`** — Full v0.8 protocol: message schemas
  (`surfaceUpdate`, `dataModelUpdate`, `beginRendering`, `deleteSurface`),
  the `BoundValue` typed-literal pattern (`literalString` / `path` /
  initialization shorthand), key-wrapped component objects
  (`{"Text": {...}}`), `explicitList` vs `template` for container
  children, and the canonical client-side architecture.
- **`references/v0.8/a2a-extension.md`** — v0.8 extension URI, Agent Card
  declaration, activation, single-message DataPart encoding,
  `a2uiClientCapabilities` in metadata.
- **`references/v0.8/custom-catalog.md`** — Per-request catalog
  negotiation (the change that landed in v0.8), `supportedCatalogIds`,
  `inlineCatalogs`, `acceptsInlineCatalogs`, per-surface `catalogId` in
  `beginRendering`, and the implementation guide for agent and renderer
  developers.

## When to Read Which Files

| User is asking about…                                              | Read                                      |
| :----------------------------------------------------------------- | :---------------------------------------- |
| v0.9 message format / schema                                       | `references/v0.9/protocol.md`             |
| v0.9 A2A integration                                               | `references/v0.9/a2a-extension.md`        |
| v0.8 → v0.9 migration / differences / which version to use         | `references/v0.9/evolution-guide.md`      |
| Custom functions (defining, validating, registering)               | `references/v0.9/custom-functions.md`     |
| Rendering specific components (Text, Button, Card, Modal, …)       | `references/v0.9/basic-catalog-guide.md`  |
| Building a client / renderer / MessageProcessor                    | `references/v0.9/renderer-guide.md`       |
| Custom component catalog (define / register / negotiate / version) | `references/v0.9/custom-catalog-guide.md` |
| `supportedCatalogIds` / catalog negotiation handshake              | `references/v0.9/custom-catalog-guide.md` |
| Catalog versioning, breaking changes, migration                    | `references/v0.9/custom-catalog-guide.md` |
| Agent-side ADK integration (`SendA2uiToClientToolset`, etc.)       | `references/v0.9/custom-catalog-guide.md` |
| Two-phase validation, graceful degradation                         | `references/v0.9/custom-catalog-guide.md` |
| Leaf-Margin spacing strategy / color inheritance                   | `references/v0.9/basic-catalog-guide.md`  |
| `formatString`, `${...}` interpolation                             | `references/v0.9/protocol.md`             |
| `sendDataModel`, two-way binding, data sync                        | `references/v0.9/protocol.md`             |
| v0.8 message format / schema                                       | `references/v0.8/protocol.md`             |
| v0.8 A2A integration                                               | `references/v0.8/a2a-extension.md`        |
| v0.8 custom catalogs / `inlineCatalogs`                            | `references/v0.8/custom-catalog.md`       |

## Working Tips

- When a user mentions `createSurface`, `updateComponents`, or
  `sendDataModel`, you're in v0.9 territory. When they mention
  `surfaceUpdate`, `dataModelUpdate`, `beginRendering`, or `literalString`,
  it's v0.8. When unclear, ask which version they're targeting before
  recommending syntax.
- Catalog schemas must use `common_types.json` references for ids and
  child lists (`ComponentId`, `ChildList`) — raw `"type": "string"` makes
  validators silently miss broken references.
- Production catalogs must be **freestanding** (no external `$ref`s) and
  **pre-compiled** into the client/agent. The bundler is
  `tools/build_catalog/assemble_catalog.py` in the A2UI submodule.
- The v0.9 `${...}` string-interpolation syntax is valid **only** inside
  the `formatString` function. Don't suggest it in arbitrary string
  properties.
- v0.9 A2A `DataPart.data` is a **list** of A2UI messages, processed
  sequentially with per-message atomicity. v0.8 was a single message.
- Source files in this skill end with a `Source:` footer pointing back to
  the upstream `.md` in the A2UI submodule, so you can verify or re-sync
  against the spec.
