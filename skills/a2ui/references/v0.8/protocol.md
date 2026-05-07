# A2UI Protocol v0.8 — Stable

> v0.8 is the current production-ready release.

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Core Architecture](#core-architecture)
3. [Message Types](#message-types)
   - [surfaceUpdate](#surfaceupdate)
   - [dataModelUpdate](#datamodelupdate)
   - [beginRendering](#beginrendering)
   - [deleteSurface](#deletesurface)
4. [Catalog Negotiation](#catalog-negotiation)
5. [Data Model](#data-model)
6. [Data Binding](#data-binding)
7. [Dynamic Lists (Templates)](#dynamic-lists-templates)
8. [Client-to-Agent Messages](#client-to-agent-messages)
9. [Full Stream Example](#full-stream-example)
10. [Incremental Updates](#incremental-updates)

---

## Design Philosophy

A2UI is designed around three core requirements:

**LLM generation** — flat adjacency list instead of deeply nested JSON; LLMs
emit components one at a time and reference them by ID.

**Progressive rendering** — JSONL over SSE; clients start rendering before the
full payload arrives.

**Platform independence** — abstract component types mapped to native widgets
by the client's catalog; the agent only needs to know component names.

---

## Core Architecture

A2UI decouples three elements:

| Element              | What it is                                          | Who owns it                                 |
| -------------------- | --------------------------------------------------- | ------------------------------------------- |
| **Component Tree**   | Flat adjacency list describing UI structure         | Sent by agent via `surfaceUpdate`           |
| **Data Model**       | JSON object holding dynamic values                  | Sent by agent via `dataModelUpdate`         |
| **Widget Catalog**   | Mapping from type names to native widgets           | Defined on the client                       |

Communication is a **JSONL stream** (one JSON object per line) sent over SSE.
Each surface has a separate component map and a separate data model.

---

## Message Types

### surfaceUpdate

Adds or updates components in a surface.

```json
{
  "surfaceUpdate": {
    "surfaceId": "main",
    "components": [
      {
        "id": "root",
        "component": {
          "Column": {
            "children": { "explicitList": ["profile-card"] }
          }
        }
      }
    ]
  }
}
```

- `surfaceId` — target surface (created implicitly on first use).
- `components` — array of component entries; each has `id` and `component`.
- Sending a component with an existing `id` **replaces** it.

The `component` field is an object where the single key is the component type
(`"Text"`, `"Button"`, etc.) and the value holds its properties.

### dataModelUpdate

Patches the surface's JSON data model.

```json
{
  "dataModelUpdate": {
    "surfaceId": "main",
    "path": "/user",
    "contents": [
      { "key": "name",  "valueString":  "Alice" },
      { "key": "age",   "valueInt":     30 },
      { "key": "admin", "valueBoolean": false }
    ]
  }
}
```

`contents` is an adjacency-list of typed key-value pairs. Value type fields:

| Field          | JSON type                                          |
| -------------- | -------------------------------------------------- |
| `valueString`  | string                                             |
| `valueInt`     | integer                                            |
| `valueNumber`  | number                                             |
| `valueBoolean` | boolean                                            |
| `valueMap`     | nested array of the same structure                 |
| `valueList`    | array of the same structures                       |

The optional `path` field targets a sub-object in the data model (JSON Pointer).
Omit it to update the root of the data model.

### beginRendering

Signals the client to render the surface. The client buffers components and
data until this message arrives to avoid partial-content flashes.

```json
{
  "beginRendering": {
    "surfaceId": "main",
    "root": "root",
    "catalogId": "https://a2ui.org/specification/v0_8/standard_catalog_definition.json",
    "styles": {
      "primaryColor": "#007bff"
    }
  }
}
```

- `root` — ID of the root component (required).
- `catalogId` — catalog the agent used; defaults to the v0.8 standard catalog
  if omitted.
- `styles` — optional theme overrides.

### deleteSurface

Removes a surface and all its state.

```json
{
  "deleteSurface": {
    "surfaceId": "main"
  }
}
```

---

## Catalog Negotiation

### Agent advertises capabilities (Agent Card)

```json
{
  "capabilities": {
    "extensions": [{
      "uri": "https://a2ui.org/a2a-extension/a2ui/v0.8",
      "params": {
        "supportedCatalogIds": [
          "https://a2ui.org/specification/v0_8/standard_catalog_definition.json"
        ],
        "acceptsInlineCatalogs": true
      }
    }]
  }
}
```

### Client declares capabilities (every A2A message)

```json
{
  "metadata": {
    "a2uiClientCapabilities": {
      "supportedCatalogIds": [
        "https://a2ui.org/specification/v0_8/standard_catalog_definition.json",
        "https://my-company.com/catalogs/custom-v1"
      ],
      "inlineCatalogs": [
        {
          "catalogId": "https://my-company.com/inline_catalogs/temp",
          "components": { "SignaturePad": { "type": "object", "properties": {} } },
          "styles": {}
        }
      ]
    }
  }
}
```

`inlineCatalogs` may only be provided if the agent advertised
`acceptsInlineCatalogs: true`.

### Agent selects catalog

The agent picks a catalog from the client's declared list and specifies it in
`beginRendering.catalogId`. If omitted, the standard v0.8 catalog is assumed.
Each surface may use a different catalog.

---

## Data Model

Each surface has its own JSON data model. The model is built incrementally
through `dataModelUpdate` messages before (and after) `beginRendering`.

**Typed adjacency-list format** — the model is a list of key-value pairs where
each value is explicitly typed:

```json
{
  "dataModelUpdate": {
    "surfaceId": "main",
    "contents": [
      {
        "key": "reservation",
        "valueMap": [
          { "key": "date",   "valueString": "2025-12-15" },
          { "key": "time",   "valueString": "19:00" },
          { "key": "guests", "valueInt": 2 }
        ]
      },
      {
        "key": "items",
        "valueList": [
          { "valueMap": [{ "key": "name", "valueString": "Pasta" }] },
          { "valueMap": [{ "key": "name", "valueString": "Wine"  }] }
        ]
      }
    ]
  }
}
```

---

## Data Binding

Components bind to data model values with JSON Pointer paths
([RFC 6901](https://tools.ietf.org/html/rfc6901)).

**Literal value:**

```json
{ "text": { "literalString": "Welcome" } }
```

**Data-bound value:**

```json
{ "text": { "path": "/user/name" } }
```

When `/user/name` changes in the data model, the component re-renders
automatically without resending the component definition.

Available literal wrappers: `literalString`, `literalInt`, `literalNumber`,
`literalBoolean`.

---

## Dynamic Lists (Templates)

Use a `template` instead of `explicitList` to render an array from the data
model:

```json
{
  "id": "product-list",
  "component": {
    "Column": {
      "children": {
        "template": {
          "dataBinding": "/products",
          "componentId": "product-card"
        }
      }
    }
  }
}
```

Inside the template component, paths are scoped to each array item:

```json
{
  "id": "product-card",
  "component": {
    "Text": { "text": { "path": "/name" } }
  }
}
```

For data `{ "products": [{"name": "Widget"}, {"name": "Gadget"}] }`, two
`Text` components render — one per item.

---

## Client-to-Agent Messages

### userAction

Sent when the user interacts with a component (e.g., clicks a Button).

```json
{
  "userAction": {
    "name": "submit_booking",
    "surfaceId": "main",
    "sourceComponentId": "submit-btn",
    "context": [
      { "key": "time",  "value": { "literalString": "19:00" } },
      { "key": "party", "value": { "path": "/reservation/guests" } }
    ]
  }
}
```

- `name` — action identifier.
- `context` — key-value pairs; values are resolved against the current data
  model before sending.

### error

```json
{
  "error": {
    "code": "RENDER_FAILED",
    "surfaceId": "main",
    "message": "Unknown component type: Foo"
  }
}
```

---

## Full Stream Example

A complete reservation form:

```jsonl
{"surfaceUpdate": {"surfaceId": "s1", "components": [{"id": "root", "component": {"Column": {"children": {"explicitList": ["title", "date-field", "submit-btn", "submit-label"]}}}}]}}
{"surfaceUpdate": {"surfaceId": "s1", "components": [{"id": "title", "component": {"Text": {"text": {"literalString": "Book Your Table"}, "usageHint": "h1"}}}]}}
{"surfaceUpdate": {"surfaceId": "s1", "components": [{"id": "date-field", "component": {"DateTimeInput": {"label": {"literalString": "Date"}, "value": {"path": "/reservation/date"}, "enableDate": true}}}]}}
{"surfaceUpdate": {"surfaceId": "s1", "components": [{"id": "submit-label", "component": {"Text": {"text": {"literalString": "Confirm"}}}}]}}
{"surfaceUpdate": {"surfaceId": "s1", "components": [{"id": "submit-btn", "component": {"Button": {"child": "submit-label", "primary": true, "action": {"name": "confirm_booking"}}}}]}}
{"dataModelUpdate": {"surfaceId": "s1", "contents": [{"key": "reservation", "valueMap": [{"key": "date", "valueString": "2025-12-15"}]}]}}
{"beginRendering": {"surfaceId": "s1", "root": "root"}}
```

---

## Incremental Updates

After `beginRendering`, the agent may continue sending updates:

| Operation              | How                                                            |
| ---------------------- | -------------------------------------------------------------- |
| **Update a component** | `surfaceUpdate` with the same `id`                             |
| **Add a component**    | `surfaceUpdate` with a new `id`; update the parent's `children`|
| **Remove a component** | Update the parent's `children` list to exclude the ID          |
| **Update data**        | `dataModelUpdate` with a specific `path`                       |
| **Remove surface**     | `deleteSurface`                                                |
