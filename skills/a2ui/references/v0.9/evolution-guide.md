# A2UI Evolution Guide: v0.8 → v0.9

This guide covers everything you need to know to migrate from A2UI v0.8 to v0.9.

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Message Type Changes](#message-type-changes)
3. [Component Format](#component-format)
4. [Data Model](#data-model)
5. [Data Binding](#data-binding)
6. [Component-Specific Changes](#component-specific-changes)
7. [Client-to-Agent Messages](#client-to-agent-messages)
8. [Catalog Changes](#catalog-changes)
9. [Error Handling](#error-handling)
10. [Quick Reference: Before / After](#quick-reference-before--after)

---

## Executive Summary

v0.9 shifts philosophy from **Structured Output First** to **Prompt First**:

- **v0.8** was optimised for LLMs using strict JSON mode / function calling.
  It relied on deep nesting that was definable in JSON Schema but often
  confusing for an LLM to generate.
- **v0.9** is designed to be embedded directly in an LLM system prompt. The
  schema is more human-readable, token-efficient, and uses patterns LLMs
  naturally excel at (standard JSON objects for maps, flat objects for
  components).

---

## Message Type Changes

### beginRendering → createSurface

v0.8 used `beginRendering` to signal "start rendering now" and specify the
root component and catalog.

v0.9 replaces it with `createSurface`, which is sent **before** components:

**v0.8:**

```json
{
  "beginRendering": {
    "surfaceId": "user_profile_card",
    "root": "root",
    "styles": { "primaryColor": "#007bff" }
  }
}
```

**v0.9:**

```json
{
  "version": "v0.9",
  "createSurface": {
    "surfaceId": "user_profile_card",
    "catalogId": "https://a2ui.org/specification/v0_9/basic_catalog.json",
    "theme": { "primaryColor": "#007bff" }
  }
}
```

Key differences:

- `createSurface` is sent **first**, before any components.
- `catalogId` is now **required** (was optional in `beginRendering`).
- `styles` is renamed to `theme`.
- The `root` field is removed — the component with `"id": "root"` is
  automatically the render entry point.
- The client renders as soon as it has a valid tree with a `"root"` component.

### surfaceUpdate → updateComponents

**v0.8:**

```json
{
  "surfaceUpdate": {
    "surfaceId": "main",
    "components": [
      { "id": "title", "component": { "Text": { "text": { "literalString": "Hello" } } } }
    ]
  }
}
```

**v0.9:**

```json
{
  "version": "v0.9",
  "updateComponents": {
    "surfaceId": "main",
    "components": [
      { "id": "root", "component": "Column", "children": ["title"] },
      { "id": "title", "component": "Text", "text": "Hello" }
    ]
  }
}
```

### dataModelUpdate → updateDataModel

**v0.8** used a typed adjacency-list:

```json
{
  "dataModelUpdate": {
    "surfaceId": "main",
    "contents": [
      { "key": "name", "valueString": "Alice" },
      { "key": "age",  "valueInt":    30 }
    ]
  }
}
```

**v0.9** uses plain JSON:

```json
{
  "version": "v0.9",
  "updateDataModel": {
    "surfaceId": "main",
    "path": "/",
    "value": { "name": "Alice", "age": 30 }
  }
}
```

### deleteSurface (unchanged structure, add version field)

**v0.8:**

```json
{ "deleteSurface": { "surfaceId": "main" } }
```

**v0.9:**

```json
{ "version": "v0.9", "deleteSurface": { "surfaceId": "main" } }
```

---

## Component Format

### Key-based wrapper → flat discriminator

Every component in v0.8 wrapped properties inside an object keyed by type:

```json
{ "id": "title", "component": { "Text": { "text": { "literalString": "Hi" }, "usageHint": "h1" } } }
```

In v0.9, the type is a `component` string property and all props are top-level:

```json
{ "id": "title", "component": "Text", "text": "Hi", "variant": "h1" }
```

### Children arrays

**v0.8:** `{ "children": { "explicitList": ["a", "b"] } }`\
**v0.9:** `{ "children": ["a", "b"] }`

### Template children

**v0.8:** `{ "children": { "template": { "dataBinding": "/items", "componentId": "item" } } }`\
**v0.9:** `{ "children": { "path": "/items", "componentId": "item" } }`

---

## Data Model

### Typed adjacency list → plain JSON

**v0.8 typed pairs:**

```json
"contents": [
  { "key": "name",  "valueString":  "Alice" },
  { "key": "score", "valueNumber":  9.5 },
  { "key": "active","valueBoolean": true },
  { "key": "meta",  "valueMap": [{ "key": "role", "valueString": "admin" }] }
]
```

**v0.9 plain JSON:**

```json
"value": { "name": "Alice", "score": 9.5, "active": true, "meta": { "role": "admin" } }
```

Use `null` to delete a key:

```json
{ "version": "v0.9", "updateDataModel": { "surfaceId": "main", "path": "/meta/role", "value": null } }
```

---

## Data Binding

The `path` keyword is consistent in both versions, but literal syntax changed:

**v0.8 literal:** `{ "literalString": "Hello" }`, `{ "literalInt": 42 }`, etc.\
**v0.9 literal:** just `"Hello"` or `42` — native JSON types.

**Unified in v0.9:** The term `dataBinding` (used in templates) is replaced by
`path` everywhere.

---

## Component-Specific Changes

### Button

| Aspect          | v0.8                                              | v0.9                                                        |
| --------------- | ------------------------------------------------- | ----------------------------------------------------------- |
| Primary style   | `"primary": true`                                 | `"variant": "primary"`                                      |
| Text-link style | *(not available)*                                 | `"variant": "borderless"`                                   |
| Event payload   | `{ "name": "...", "context": [KV pairs] }`        | `{ "event": { "name": "...", "context": { map } } }`        |
| Local function  | *(not available)*                                 | `{ "functionCall": { "call": "openUrl", "args": {...} } }`  |

### TextField

| Aspect          | v0.8                   | v0.9                              |
| --------------- | ---------------------- | --------------------------------- |
| Value property  | `text`                 | `value`                           |
| Type property   | `textFieldType`        | `textFieldType` (unchanged)       |
| Validation      | `validationRegexp`     | `checks` list of function calls   |

### MultipleChoice → ChoicePicker

| Aspect           | v0.8                       | v0.9                              |
| ---------------- | -------------------------- | --------------------------------- |
| Component name   | `MultipleChoice`           | `ChoicePicker`                    |
| Value property   | `selections`               | `value`                           |
| Max selections   | `maxAllowedSelections: 1`  | `variant: "mutuallyExclusive"`    |
| Multi-select     | `maxAllowedSelections: N`  | `variant: "multipleSelection"`    |

### Slider

`minValue` / `maxValue` → `min` / `max`

### Modal

`entryPointChild` / `contentChild` → `trigger` / `content`

### Tabs

`tabItems` → `tabs`; tab title is a plain string instead of a BoundValue.

### Row / Column

`distribution` / `alignment` → `justify` / `align`

### Text / Image

`usageHint` → `variant`

---

## Client-to-Agent Messages

**v0.8:** `{ "userAction": { ... } }`\
**v0.9:** `{ "version": "v0.9", "action": { ... } }`

The `context` field in a Button action changed from an array of KV pairs to a
plain JSON map:

**v0.8:**

```json
{
  "userAction": {
    "name": "book",
    "context": [
      { "key": "time",  "value": { "literalString": "19:00" } },
      { "key": "party", "value": { "path": "/reservation/guests" } }
    ]
  }
}
```

**v0.9:**

```json
{
  "version": "v0.9",
  "action": {
    "name": "book",
    "surfaceId": "...",
    "context": {
      "time": "19:00",
      "party": { "path": "/reservation/guests" }
    }
  }
}
```

---

## Catalog Changes

v0.9 introduces a **unified catalog** that merges components and functions into
a single `basic_catalog.json`. v0.8 had separate component and function
catalogs.

v0.9 also introduces `basic_catalog_rules.txt` — a plain-text prompt fragment
with rules that are hard to express in JSON Schema (e.g., "MUST provide
`action` for Button"). Include it in the agent system prompt alongside the
catalog schema.

---

## Error Handling

v0.9 introduces structured `VALIDATION_FAILED` errors for a
self-correction feedback loop:

```json
{
  "version": "v0.9",
  "error": {
    "code": "VALIDATION_FAILED",
    "surfaceId": "main",
    "path": "/components/0/text",
    "message": "Expected string, got number"
  }
}
```

The LLM receives this error in the next turn and self-corrects.

---

## Quick Reference: Before / After

| Thing                   | v0.8                             | v0.9                             |
| ----------------------- | -------------------------------- | -------------------------------- |
| Surface init            | `beginRendering` (last)          | `createSurface` (first)          |
| Component update        | `surfaceUpdate`                  | `updateComponents`               |
| Data update             | `dataModelUpdate`                | `updateDataModel`                |
| Component type syntax   | `{ "Text": { ... } }`            | `"Text"` + flat props            |
| Literal string          | `{ "literalString": "hi" }`      | `"hi"`                           |
| Children                | `{ "explicitList": [...] }`      | `[...]`                          |
| Template path           | `dataBinding`                    | `path`                           |
| Data model format       | typed adjacency list             | plain JSON                       |
| Button primary          | `primary: true`                  | `variant: "primary"`             |
| Button action           | `{ "name": "..." }`              | `{ "event": { "name": "..." } }` |
| Layout props            | `distribution`, `alignment`      | `justify`, `align`               |
| TextField value         | `text`                           | `value`                          |
| Choice widget           | `MultipleChoice`                 | `ChoicePicker`                   |
| Modal props             | `entryPointChild`, `contentChild`| `trigger`, `content`             |
| Tabs prop               | `tabItems`                       | `tabs`                           |
| Styling hint            | `usageHint`                      | `variant`                        |
| Slider range            | `minValue`, `maxValue`           | `min`, `max`                     |
| Client event            | `userAction`                     | `action`                         |
| Version field           | *(absent)*                       | `"version": "v0.9"`              |
