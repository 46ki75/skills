# A2UI Evolution Guide: v0.8 → v0.9

This guide documents every breaking and notable change between v0.8 (Stable) and v0.9 (Draft) with side-by-side examples.

## Executive summary

| Dimension | v0.8 | v0.9 |
|-----------|------|------|
| Philosophy | Structured Output / Function Calling | Prompt-First / In-Context Schema |
| Surface init | `beginRendering` sent *after* components | `createSurface` sent *first* |
| Component updates | `surfaceUpdate` | `updateComponents` |
| Data updates | `dataModelUpdate` | `updateDataModel` |
| Component type | Key wrapper `{"Text": {...}}` | Discriminator `"component": "Text"` |
| Data model payload | Typed adjacency-list array | Plain JSON object |
| Literal text | `{"literalString": "foo"}` | `"foo"` (native string) |
| Data binding | `{"path": "/foo"}` inside BoundValue | `{"path": "/foo"}` directly |
| Button context | Array of key-value pairs | Plain JSON object |
| Button variant | Boolean `primary: true` | Enum `variant: "primary"` |
| String interpolation | Not supported | `formatString` function with `${...}` |
| Catalog | Optional; separate component + style defs | Required `catalogId` URI; unified catalog |
| Version field | Not present | `"version": "v0.9"` on every message |
| Schema architecture | Monolithic tendencies | Modular: `common_types.json`, `server_to_client.json`, `basic_catalog.json` |

---

## 1. Surface lifecycle

### v0.8 — `beginRendering` comes last

In v0.8 the agent streams all components and data first, then sends `beginRendering` as a "draw now" signal. The root component ID is declared in `beginRendering`.

```jsonl
// v0.8
{"surfaceUpdate":{"surfaceId":"s1","components":[{"id":"root","component":{"Column":{"children":{"explicitList":["msg"]}}}}]}}
{"surfaceUpdate":{"surfaceId":"s1","components":[{"id":"msg","component":{"Text":{"text":{"literalString":"Hi"}}}}]}}
{"dataModelUpdate":{"surfaceId":"s1","contents":{}}}
{"beginRendering":{"surfaceId":"s1","root":"root","styles":{"primaryColor":"#007bff"}}}
```

### v0.9 — `createSurface` comes first

In v0.9 the agent sends `createSurface` immediately to allocate the surface, then streams components. Rendering starts as soon as the `root` component exists — no explicit render signal needed.

```jsonl
// v0.9
{"version":"v0.9","createSurface":{"surfaceId":"s1","catalogId":"https://a2ui.org/specification/v0_9/basic_catalog.json","theme":{"primaryColor":"#007bff"}}}
{"version":"v0.9","updateComponents":{"surfaceId":"s1","components":[{"id":"root","component":"Column","children":["msg"]},{"id":"msg","component":"Text","text":"Hi"}]}}
```

---

## 2. Component type representation

### v0.8 — key-wrapper

The component type is expressed as the single key of the `component` object:

```json
// v0.8
{
  "id": "title",
  "component": {
    "Text": {
      "text": { "literalString": "Hello" }
    }
  }
}
```

### v0.9 — discriminator field

The component type is expressed as a plain `"component"` string property; all other properties are flat siblings:

```json
// v0.9
{
  "id": "title",
  "component": "Text",
  "text": "Hello"
}
```

The flat structure is easier for LLMs to generate reliably.

---

## 3. Data model updates

### v0.8 — typed adjacency-list array

```json
// v0.8
{
  "dataModelUpdate": {
    "surfaceId": "s1",
    "path": "user",
    "contents": [
      { "key": "name",       "valueString": "Alice" },
      { "key": "age",        "valueNumber": 30 },
      { "key": "isAdmin",    "valueBoolean": false },
      { "key": "address",    "valueMap": [
          { "key": "city", "valueString": "Anytown" }
      ]}
    ]
  }
}
```

### v0.9 — plain JSON object

```json
// v0.9
{
  "version": "v0.9",
  "updateDataModel": {
    "surfaceId": "s1",
    "path": "/user",
    "value": {
      "name": "Alice",
      "age": 30,
      "isAdmin": false,
      "address": { "city": "Anytown" }
    }
  }
}
```

Key difference: v0.9 uses standard JSON (LLMs are trained on it); v0.8 required explicit type wrappers.

---

## 4. Data binding

### v0.8 — `BoundValue` wrapper objects

Static value:
```json
{ "text": { "literalString": "Hello" } }
```

Dynamic value:
```json
{ "text": { "path": "/user/name" } }
```

Initialise + bind (shorthand):
```json
{ "text": { "path": "/user/name", "literalString": "Guest" } }
```

### v0.9 — native types + path object

Static value:
```json
{ "text": "Hello" }
```

Dynamic value:
```json
{ "text": { "path": "/user/name" } }
```

v0.9 drops the `literalString` / `literalNumber` / `literalBoolean` wrappers — plain JSON types are used directly.

### v0.9 — string interpolation with `formatString`

v0.9 adds `formatString` for mixing literals and paths in one string (only valid inside the function):

```json
{
  "id": "greeting",
  "component": "Text",
  "text": {
    "call": "formatString",
    "args": { "format": "Hello, ${/user/firstName} ${/user/lastName}!" }
  }
}
```

---

## 5. Container children

### v0.8 — `explicitList` / `template` inside a `children` object

Static children:
```json
{ "children": { "explicitList": ["child_a", "child_b"] } }
```

Dynamic list:
```json
{
  "children": {
    "template": {
      "dataBinding": { "path": "/items" },
      "componentId": "item_template"
    }
  }
}
```

### v0.9 — plain array or inline template object

Static children:
```json
{ "children": ["child_a", "child_b"] }
```

Dynamic list:
```json
{
  "children": {
    "path": "/items",
    "componentId": "item_template"
  }
}
```

---

## 6. Button action context

### v0.8 — array of key-value pairs

```json
{
  "action": {
    "name": "submit_form",
    "context": [
      { "key": "userId",  "value": { "literalString": "u-42" } },
      { "key": "inputValue", "value": { "path": "/form/input" } }
    ]
  }
}
```

### v0.9 — plain JSON object

```json
{
  "action": {
    "event": {
      "name": "submit_form",
      "context": {
        "userId": "u-42",
        "inputValue": { "path": "/form/input" }
      }
    }
  }
}
```

Note: in v0.9 the action wrapper uses an `event` sub-key to distinguish server events from local `functionCall` actions.

---

## 7. Button variant

### v0.8

```json
{ "Button": { "child": "lbl", "primary": true } }
```

### v0.9

```json
{ "id": "btn", "component": "Button", "child": "lbl", "variant": "primary" }
```

---

## 8. Catalog

### v0.8

`catalogId` is optional on `beginRendering`. When omitted, the standard v0.8 catalog is assumed:
`https://a2ui.org/specification/v0_8/standard_catalog_definition.json`

Custom catalogs are declared in the client's `a2uiClientCapabilities.inlineCatalogs`.

### v0.9

`catalogId` is **required** on `createSurface`. Components and functions are unified in a single catalog file. The standard catalog URI is:
`https://a2ui.org/specification/v0_9/basic_catalog.json`

A companion `basic_catalog_rules.txt` plain-text prompt fragment can be included in the system prompt for additional validation rules.

---

## 9. Schema architecture

### v0.8

Semi-monolithic `server_to_client.json` with deep nested definitions. Components have dynamic key names, making schema tooling harder.

### v0.9

Three separate files:
- `common_types.json` — reusable primitives: `ComponentId`, `DynamicString`, `ChildList`, `FunctionCall`
- `server_to_client.json` — envelope schema using `oneOf` for strict message typing; references `catalog.json` as a swappable placeholder
- `basic_catalog.json` — unified component + function definitions

To validate against a custom catalog, alias `catalog.json` to your own catalog file without modifying the envelope schema.

---

## 10. Migration checklist: v0.8 → v0.9

- [ ] Add `"version": "v0.9"` to every message
- [ ] Move `beginRendering` → `createSurface` (send it first, include `catalogId`)
- [ ] Move `surfaceUpdate` → `updateComponents`
- [ ] Move `dataModelUpdate` → `updateDataModel`, change `contents` array → `value` JSON object
- [ ] Unwrap component type: `{"Text": {...}}` → `"component": "Text", ...` (flat)
- [ ] Replace `literalString`/`literalNumber`/`literalBoolean` with plain native values
- [ ] Replace `explicitList` array with plain array in `children`
- [ ] Replace `template.dataBinding` with `children.path`
- [ ] Replace button `context` array with plain JSON object under `action.event`
- [ ] Replace `primary: true` style booleans with `variant: "primary"` enum
- [ ] Update `catalogId` URI from v0.8 to v0.9 standard catalog
