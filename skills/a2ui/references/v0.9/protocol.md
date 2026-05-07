# A2UI Protocol v0.9 — Draft

> v0.9 is the current draft specification. For production use, consider v0.8 (Stable).

## Table of Contents

1. [Philosophy Shift from v0.8](#philosophy-shift-from-v08)
2. [Message Types](#message-types)
   - [createSurface](#createsurface)
   - [updateComponents](#updatecomponents)
   - [updateDataModel](#updatedatamodel)
   - [deleteSurface](#deletesurface)
3. [Catalog](#catalog)
4. [Component Format](#component-format)
5. [Data Model](#data-model)
6. [Data Binding](#data-binding)
7. [Dynamic Lists (Templates)](#dynamic-lists-templates)
8. [Client-to-Agent Messages](#client-to-agent-messages)
9. [Validation Feedback Loop](#validation-feedback-loop)
10. [Full Stream Example](#full-stream-example)
11. [Modular Schema Architecture](#modular-schema-architecture)

---

## Philosophy Shift from v0.8

| Feature                  | v0.8                                     | v0.9                                                    |
| ------------------------ | ---------------------------------------- | ------------------------------------------------------- |
| **Design target**        | Structured Output / Function Calling     | Prompt-First / In-Context Schema                        |
| **Component type**       | Key-based wrapper `{"Text": {...}}`      | Property discriminator `"component": "Text"`            |
| **Data model**           | Typed adjacency-list (array of KV pairs) | Plain JSON object                                       |
| **Surface init**         | `beginRendering`                         | `createSurface`                                         |
| **Catalog**              | Separate component and function catalogs | Unified `basic_catalog.json`                            |
| **Validation**           | Best-effort                              | Strict `VALIDATION_FAILED` feedback loop                |

All v0.9 messages carry a top-level `"version": "v0.9"` field.

---

## Message Types

### createSurface

Creates a new surface and prepares the client for rendering. Replaces both the
implicit surface creation from `surfaceUpdate` and the `beginRendering` signal.

```json
{
  "version": "v0.9",
  "createSurface": {
    "surfaceId": "booking-surface",
    "catalogId": "https://a2ui.org/specification/v0_9/basic_catalog.json",
    "theme": {
      "primaryColor": "#007bff"
    },
    "sendDataModel": false
  }
}
```

| Field            | Required | Description                                                                               |
| ---------------- | -------- | ----------------------------------------------------------------------------------------- |
| `surfaceId`      | yes      | Unique surface identifier.                                                                |
| `catalogId`      | yes      | URI of the catalog the agent will use.                                                    |
| `theme`          | no       | Theme overrides (`primaryColor`, etc.).                                                   |
| `sendDataModel`  | no       | When `true`, the renderer includes the full data model in every outgoing A2A message.     |

The client starts rendering as soon as it has a valid component tree with a
`"root"` component — there is no explicit "start rendering now" signal as in v0.8.

### updateComponents

Adds or updates components in a surface. `surfaceId` must match a surface
created by `createSurface`; referencing an unknown `surfaceId` is an error.

```json
{
  "version": "v0.9",
  "updateComponents": {
    "surfaceId": "booking-surface",
    "components": [
      { "id": "root",   "component": "Column",      "children": ["title", "date-field", "submit-btn", "btn-text"] },
      { "id": "title",  "component": "Text",         "text": "Book Your Table", "variant": "h1" },
      { "id": "date-field", "component": "DateTimeInput", "label": "Date", "value": { "path": "/reservation/date" }, "enableDate": true },
      { "id": "btn-text", "component": "Text",       "text": "Confirm" },
      { "id": "submit-btn", "component": "Button",   "child": "btn-text", "variant": "primary", "action": { "event": { "name": "confirm_booking" } } }
    ]
  }
}
```

### updateDataModel

Updates the surface data model at a specific path. The value is plain JSON.

```json
{
  "version": "v0.9",
  "updateDataModel": {
    "surfaceId": "booking-surface",
    "path": "/reservation",
    "value": {
      "date": "2025-12-15",
      "time": "19:00",
      "guests": 2
    }
  }
}
```

- `path` — JSON Pointer to the target location (omit for root).
- `value` — any valid JSON value. Setting `null` removes the key.
- Semantics: **upsert** — keys are created if absent, updated if present.

### deleteSurface

Removes a surface and all its state.

```json
{
  "version": "v0.9",
  "deleteSurface": {
    "surfaceId": "booking-surface"
  }
}
```

---

## Catalog

v0.9 uses a **unified catalog** that combines components and functions in a
single JSON Schema document.

Standard v0.9 catalog:
`https://a2ui.org/specification/v0_9/basic_catalog.json`

The schema is modularised into three files:

| File                     | Contents                                           |
| ------------------------ | -------------------------------------------------- |
| `common_types.json`      | Reusable primitives, paths, function call types    |
| `server_to_client.json`  | Message envelope (references `catalog.json`)       |
| `basic_catalog.json`     | Unified catalog with components and functions      |

Custom catalogs: alias `catalog.json` to your own catalog file during schema
validation without changing the envelope.

Agents include the catalog schema and a `basic_catalog_rules.txt` auxiliary
rules file in their system prompt for reliable generation.

### Client capabilities

```json
{
  "v0.9": {
    "supportedCatalogIds": [
      "https://a2ui.org/specification/v0_9/basic_catalog.json"
    ],
    "inlineCatalogs": []
  }
}
```

This object is placed in the `metadata` of every client-to-agent A2A message.

---

## Component Format

v0.9 uses a **flat discriminator** format instead of key-based wrappers:

```json
{
  "id": "welcome",
  "component": "Text",
  "text": "Hello",
  "variant": "h1"
}
```

Compare with v0.8:

```json
{
  "id": "welcome",
  "component": { "Text": { "text": { "literalString": "Hello" }, "usageHint": "h1" } }
}
```

All component properties are top-level fields alongside `id` and `component`.

---

## Data Model

v0.9 uses **plain JSON** for the data model — no typed wrappers.

```json
{
  "version": "v0.9",
  "updateDataModel": {
    "surfaceId": "main",
    "path": "/",
    "value": {
      "user": {
        "name": "Alice",
        "age": 30
      },
      "cart": {
        "items": [
          { "name": "Widget", "price": 9.99 }
        ],
        "total": 9.99
      }
    }
  }
}
```

Partial update at a sub-path:

```json
{
  "version": "v0.9",
  "updateDataModel": {
    "surfaceId": "main",
    "path": "/user/name",
    "value": "Bob"
  }
}
```

---

## Data Binding

Same JSON Pointer paths as v0.8, but with implicit typing:

**Literal (inline string):**

```json
{ "id": "title", "component": "Text", "text": "Welcome" }
```

**Data-bound:**

```json
{ "id": "username", "component": "Text", "text": { "path": "/user/name" } }
```

### String Interpolation

v0.9 supports `formatString` for interpolating paths into strings:

```json
{
  "id": "greeting",
  "component": "Text",
  "text": {
    "call": "formatString",
    "args": {
      "template": "Hello, ${/user/name}! You have ${/cart/count} items."
    }
  }
}
```

`${...}` interpolation is **only** supported inside `formatString`.

---

## Dynamic Lists (Templates)

Use a `template` child to render arrays. In v0.9 the template uses `path`
instead of v0.8's `dataBinding`:

```json
{
  "id": "product-list",
  "component": "Column",
  "children": {
    "componentId": "product-card",
    "path": "/products"
  }
}
```

Inside the template, paths are scoped to each array item:

```json
{
  "id": "product-card",
  "component": "Text",
  "text": { "path": "/name" }
}
```

---

## Client-to-Agent Messages

### action (replaces userAction)

```json
{
  "version": "v0.9",
  "action": {
    "name": "submit_reservation",
    "surfaceId": "booking-surface",
    "sourceComponentId": "submit-btn",
    "timestamp": "2026-02-25T10:40:00Z",
    "context": {
      "time": "7:00 PM",
      "partySize": 4
    }
  }
}
```

### error

```json
{
  "version": "v0.9",
  "error": {
    "code": "VALIDATION_FAILED",
    "surfaceId": "booking-surface",
    "path": "/components/0/children",
    "message": "Expected array of strings, got null."
  }
}
```

---

## Validation Feedback Loop

v0.9 introduces a **Prompt-Generate-Validate** loop. If the agent sends invalid
JSON, the renderer returns a `VALIDATION_FAILED` error with a precise path, and
the LLM self-corrects in the next turn.

Error codes: `VALIDATION_FAILED`, `UNKNOWN_SURFACE`, `RENDER_FAILED`.

---

## Full Stream Example

Reservation booking form:

```jsonl
{"version": "v0.9", "createSurface": {"surfaceId": "s1", "catalogId": "https://a2ui.org/specification/v0_9/basic_catalog.json"}}
{"version": "v0.9", "updateComponents": {"surfaceId": "s1", "components": [{"id": "root", "component": "Column", "children": ["title", "date-field", "btn-text", "submit-btn"]}]}}
{"version": "v0.9", "updateComponents": {"surfaceId": "s1", "components": [{"id": "title", "component": "Text", "text": "Book Your Table", "variant": "h1"}]}}
{"version": "v0.9", "updateComponents": {"surfaceId": "s1", "components": [{"id": "date-field", "component": "DateTimeInput", "label": "Date", "value": {"path": "/reservation/date"}, "enableDate": true}]}}
{"version": "v0.9", "updateComponents": {"surfaceId": "s1", "components": [{"id": "btn-text", "component": "Text", "text": "Confirm"}, {"id": "submit-btn", "component": "Button", "child": "btn-text", "variant": "primary", "action": {"event": {"name": "confirm_booking"}}}]}}
{"version": "v0.9", "updateDataModel": {"surfaceId": "s1", "path": "/reservation", "value": {"date": "2025-12-15"}}}
```

---

## Modular Schema Architecture

```text
server_to_client.json   ← envelope (references catalog.json)
       │
       ├── common_types.json       ← shared primitives
       └── catalog.json            ← alias → basic_catalog.json (or custom)
               │
               └── basic_catalog.json   ← components + functions
```

To use a custom catalog, replace the `catalog.json` alias during validation.
No changes to the envelope schema are needed.
