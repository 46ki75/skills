<!-- markdownlint-disable -->
# Custom Component Catalog Guide (v0.9)

This guide explains how to define, implement, register, and negotiate custom component
catalogs in A2UI v0.9. Use it when you need to go beyond the Basic Catalog and wire
your own design system components to A2UI agents.

---

## 1. Why Define Your Own Catalog?

Every A2UI surface is driven by a **Catalog** — a JSON Schema file that tells the agent
which components, functions, and themes exist for a given client. The Basic Catalog is a
sparse, general-purpose starting point; for production applications you should replace or
extend it with a catalog that mirrors your design system.

| Use case | Recommendation |
| :------- | :------------- |
| Mature frontend with existing design system | Define a catalog that maps directly to your components. No adapters. |
| Greenfield / prototype | Start with the Basic Catalog; evolve to your own as the app matures. |

Benefits:

- **Design system alignment** — agents use only the components that exist in your app.
- **Security** — only trusted, pre-registered components can be rendered.
- **No mappers** — a catalog that directly names your components avoids mapping Generic→Custom at runtime.

---

## 2. Catalog JSON Schema Structure

A catalog is a JSON Schema object that conforms to this shape:

```json
{
  "$id": "https://example.com/catalogs/my-catalog/v1/catalog.json",
  "components": {
    "ComponentName": { /* JSON Schema object */ }
  },
  "functions": [ /* array of FunctionDefinition */ ],
  "theme": { /* arbitrary JSON Schema for theming */ }
}
```

| Field | Required | Description |
| :---- | :------- | :---------- |
| `$id` (serves as `catalogId`) | Yes | URI used as the stable identifier for negotiation. Not fetched at runtime. |
| `components` | No | Map of component name → JSON Schema object. |
| `functions` | No | Array of function definitions callable by the agent. |
| `theme` | No | Arbitrary JSON Schema for theme properties. |

### Minimal catalog example

```json
{
  "$id": "https://github.com/myorg/myapp/catalogs/v1/catalog.json",
  "components": {
    "HelloWorldBanner": {
      "type": "object",
      "description": "A simple banner greeting.",
      "properties": {
        "message": {
          "type": "string",
          "description": "The banner text."
        },
        "backgroundColor": {
          "type": "string",
          "default": "#f0f0f0"
        }
      },
      "required": ["message"]
    }
  }
}
```

When the agent uses this catalog it generates payloads that conform strictly to those
component schemas:

```json
[
  {
    "version": "v0.9",
    "createSurface": {
      "surfaceId": "my-surface",
      "catalogId": "https://github.com/myorg/myapp/catalogs/v1/catalog.json"
    }
  },
  {
    "version": "v0.9",
    "updateComponents": {
      "surfaceId": "my-surface",
      "components": [
        {
          "id": "root",
          "component": "HelloWorldBanner",
          "message": "Hello, A2UI!",
          "backgroundColor": "#4CAF50"
        }
      ]
    }
  }
]
```

---

## 3. Building a Catalog

### Freestanding requirement

A2UI catalogs **must be standalone** (no unresolved `$ref` to external files). LLM inference
does not resolve external references at runtime. During authoring you may use `$ref` for
modularity; bundle everything before distribution using the assembly tool:

```bash
uv run tools/build_catalog/assemble_catalog.py [INPUTS ...] \
  --output-name my_catalog \
  [--catalog-id <ID>] \
  [--version 0.9] \
  [--extend-basic-catalog] \
  [--out-dir dist] \
  [--verbose]
```

Options:

| Option | Description |
| :----- | :---------- |
| `--output-name` | Base filename of the bundled catalog (`.json` appended automatically). |
| `--catalog-id` | Override the `$id` / `catalogId`. Defaults to `urn:a2ui:catalog:<base_name>`. |
| `--version` | A2UI spec version for official catalog fallbacks. `0.9` or `0.10`. Default `0.9`. |
| `--extend-basic-catalog` | Includes the entire Basic Catalog automatically. |
| `--out-dir` | Output directory. Default `dist`. |
| `--verbose` | Debug logging. |

### Extending the Basic Catalog

Import every component from the Basic Catalog and add your own:

```json
{
  "$id": "https://example.com/catalogs/extended/v1/catalog.json",
  "components": {
    "allOf": [
      { "$ref": "basic_catalog_definition.json#/components" },
      {
        "SuggestionChips": {
          "type": "object",
          "description": "A horizontal list of suggested prompts.",
          "properties": {
            "suggestions": {
              "type": "array",
              "description": "Suggested prompt strings.",
              "items": { "type": "string" }
            }
          },
          "required": ["suggestions"]
        }
      }
    ]
  }
}
```

Run `assemble_catalog.py` to resolve the `$ref` before distributing.

### Cherry-picking components

Import only specific Basic Catalog components:

```json
{
  "$id": "https://example.com/catalogs/popup/v1/catalog.json",
  "components": {
    "allOf": [
      { "$ref": "basic_catalog.json#/components/Text" },
      {
        "Popup": {
          "type": "object",
          "description": "A modal overlay that displays an icon and text.",
          "properties": {
            "text": { "$ref": "common_types.json#/$defs/ComponentId" }
          },
          "required": ["text"]
        }
      }
    ]
  }
}
```

Run `assemble_catalog.py` to resolve the `$ref` before distributing.

### Defining a richer component (with data binding)

Properties that support both literal values and data model paths use the `StringValue` /
`literalArray` + `path` pattern (same primitives as the Basic Catalog):

```json
"Chart": {
  "type": "object",
  "description": "An interactive doughnut or pie chart.",
  "properties": {
    "type": {
      "type": "string",
      "enum": ["doughnut", "pie"]
    },
    "title": {
      "type": "object",
      "properties": {
        "literalString": { "type": "string" },
        "path": { "type": "string" }
      }
    },
    "chartData": {
      "type": "object",
      "properties": {
        "literalArray": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "label": { "type": "string" },
              "value": { "type": "number" }
            },
            "required": ["label", "value"]
          }
        },
        "path": { "type": "string" }
      }
    }
  },
  "required": ["type", "chartData"]
}
```

---

## 4. Client-Side Implementation

### Step 1 — Implement the component (Angular example)

Extend `DynamicComponent` from `@a2ui/angular` to gain data binding resolution via
`resolvePrimitive`:

```typescript
import {DynamicComponent} from '@a2ui/angular';
import * as Primitives from '@a2ui/web_core/types/primitives';
import * as Types from '@a2ui/web_core/types/types';
import {Component, computed, input} from '@angular/core';

@Component({
  selector: 'hello-world-banner',
  template: `<div [style.background]="bg()"><h2>{{ message() }}</h2></div>`,
})
export class HelloWorldBanner extends DynamicComponent {
  readonly message = input<string>();
  readonly backgroundColor = input<string>('#f0f0f0');
  protected readonly bg = computed(() => this.backgroundColor() ?? '#f0f0f0');
}
```

For components with data-bound properties use `resolvePrimitive`:

```typescript
@Component({
  selector: 'a2ui-chart',
  template: `<div><h2>{{ resolvedTitle() }}</h2></div>`,
})
export class Chart extends DynamicComponent<Types.CustomNode> {
  readonly type = input.required<string>();
  readonly title = input<Primitives.StringValue | null>();
  protected readonly resolvedTitle = computed(
    () => super.resolvePrimitive(this.title() ?? null)
  );
  readonly chartData = input.required<Primitives.StringValue | null>();
}
```

### Step 2 — Register with the renderer catalog

Map component names (as the agent sends them) to lazy-loaded implementations and input bindings:

```typescript
import {Catalog, DEFAULT_CATALOG} from '@a2ui/angular';
import {inputBinding} from '@angular/core';

export const MY_CATALOG = {
  ...DEFAULT_CATALOG,  // keep Basic Catalog components
  HelloWorldBanner: {
    type: () => import('./hello_world_banner').then(r => r.HelloWorldBanner),
    bindings: ({properties}) => [
      inputBinding('message', () => properties['message'] || undefined),
      inputBinding('backgroundColor', () => properties['backgroundColor'] || undefined),
    ],
  },
  Chart: {
    type: () => import('./chart').then(r => r.Chart),
    bindings: ({properties}) => [
      inputBinding('type', () => properties['type'] || undefined),
      inputBinding('title', () => properties['title'] || undefined),
      inputBinding('chartData', () => properties['chartData'] || undefined),
    ],
  },
} as Catalog;
```

### Step 3 — Provide the catalog to `A2UI_RENDERER_CONFIG`

```typescript
import {ApplicationConfig} from '@angular/core';
import {A2UI_RENDERER_CONFIG, A2uiRendererService} from '@a2ui/angular/v0_9';
import {MY_CATALOG} from './my-catalog';

export const appConfig: ApplicationConfig = {
  providers: [
    {
      provide: A2UI_RENDERER_CONFIG,
      useValue: {
        catalogs: [MY_CATALOG],
        actionHandler: action => console.log('action:', action),
      },
    },
    A2uiRendererService,
  ],
};
```

### React renderer

For React, pass your catalog to the `MessageProcessor`:

```typescript
import {MessageProcessor} from '@a2ui/react';
import {MY_CATALOG} from './my-catalog';

const processor = new MessageProcessor({catalogs: [MY_CATALOG]});
```

---

## 5. Catalog Negotiation

Catalog negotiation is a three-step handshake between agent and client.

### Step 1 — Agent advertises supported catalogs (optional)

The agent can list supported catalogs in its A2A Agent Card. Clients can use this to check
compatibility before opening a session, but it is informational only.

```json
{
  "capabilities": {
    "extensions": [
      {
        "uri": "https://a2ui.org/a2a-extension/a2ui/v0.8",
        "params": {
          "supportedCatalogIds": [
            "https://a2ui.org/specification/v0_9/basic_catalog.json",
            "https://github.com/myorg/myapp/catalogs/v1/catalog.json"
          ]
        }
      }
    ]
  }
}
```

### Step 2 — Client advertises supported catalogs (required)

The client **must** include `a2uiClientCapabilities` in the `metadata` of **every** A2A
message it sends. The list is ordered by preference (most preferred first).

```json
{
  "parts": [{"text": "Show me the dashboard."}],
  "metadata": {
    "a2uiClientCapabilities": {
      "supportedCatalogIds": [
        "https://github.com/myorg/myapp/catalogs/v1/catalog.json",
        "https://a2ui.org/specification/v0_9/basic_catalog.json"
      ]
    }
  }
}
```

`inlineCatalogs` (optional, not recommended for production): full catalog definition objects
the client sends at runtime instead of pre-registered IDs.

### Step 3 — Agent selects a catalog per surface

When the agent creates a surface it picks the best match from the client's list. The chosen
`catalogId` is locked for the lifetime of that surface. If no match is found, the agent
sends no UI.

```json
{
  "version": "v0.9",
  "createSurface": {
    "surfaceId": "dashboard-surface",
    "catalogId": "https://github.com/myorg/myapp/catalogs/v1/catalog.json"
  }
}
```

---

## 6. Catalog Naming and Versioning

### URI convention

Use URIs as `catalogId`s to guarantee global uniqueness and human readability. The URI is
**never fetched at runtime** — it is only an identifier.

```
https://example.com/catalogs/<name>/v<MAJOR>/catalog.json
```

### Breaking vs non-breaking changes

| Change | Category | Action required |
| :----- | :------- | :-------------- |
| Add container component | Breaking | Bump major version |
| Remove container component | Breaking | Bump major version |
| Change a field type | Breaking | Bump major version |
| Add a required property without default | Breaking | Bump major version |
| Add leaf (non-container) component | Non-breaking | No version bump |
| Add optional property | Non-breaking | No version bump |
| Remove a property | Non-breaking | No version bump |
| Add new functions or styles | Non-breaking | No version bump |
| Update `description` fields | Non-breaking | No version bump |

### Migration pattern (zero downtime)

1. **Client** updates `supportedCatalogIds` to include **both** old and new versions
   (new first, old second).
2. **Agent** is rebuilt with the v2 schema; when the client advertises v2 support the agent
   prefers it.
3. **Old agents** still match v1 in the client's list — no downtime.

---

## 7. Schema Validation and Graceful Degradation

### Two-phase validation

1. **Agent-side (pre-send):** The agent runtime validates generated JSON against the catalog
   before transmitting. On failure the agent can retry or fall back to plain text.
2. **Client-side (on receive):** The client validates received JSON against its local copy of
   the catalog. On failure it reports a `VALIDATION_FAILED` error back to the agent.

### Graceful degradation

Clients **must not crash** on unknown components or properties. Instead:

- Unknown component → render a safe "not supported" placeholder or skip the node.
- Unknown property → silently ignore.
- Entire surface fails → display a generic error message or raw text.

### Client-to-server error reporting

```json
{
  "version": "v0.9",
  "error": {
    "code": "VALIDATION_FAILED",
    "surfaceId": "dashboard-surface",
    "path": "/components/Chart/chartData",
    "message": "Missing required property 'chartData' in component 'Chart'."
  }
}
```

---

## 8. Agent-Side Integration (ADK / Python)

Use `A2uiSchemaManager` to generate the system prompt with your catalog's schema and
examples, and `SendA2uiToClientToolset` to give the agent a tool for sending A2UI payloads.

```python
from a2ui.schema.constants import VERSION_0_9
from a2ui.schema.manager import A2uiSchemaManager

schema_manager = A2uiSchemaManager(
    version=VERSION_0_9,
    catalogs=[my_catalog_config],  # your CatalogConfig pointing to the JSON file
)

A2UI_INSTRUCTION = schema_manager.generate_system_prompt(
    role_description="You are a dashboard agent.",
    ui_description="Use the Chart component for all data visualizations.",
    include_schema=True,
    include_examples=True,
    validate_examples=True,
)
```

In the executor, resolve the catalog at session start and save it to session state:

```python
from a2ui.adk.send_a2ui_to_client_toolset import SendA2uiToClientToolset, A2uiEventConverter
from a2ui.a2a_extension.utils import try_activate_a2ui_extension

use_ui = try_activate_a2ui_extension(context)
if use_ui:
    catalog = schema_manager.get_selected_catalog(client_ui_capabilities=capabilities)
    agent.tools = [SendA2uiToClientToolset(a2ui_catalog=catalog, a2ui_enabled=True)]

# In A2aAgentExecutorConfig, wire the event converter
config = A2aAgentExecutorConfig(event_converter=A2uiEventConverter())
```

The `A2uiEventConverter` automatically translates `send_a2ui_json_to_client` tool calls
into A2A `DataPart` messages with the A2UI payload, so the agent does not need explicit
serialization logic.

---

## 9. Security Considerations

1. **Allowlist only trusted components.** Do not expose components that execute scripts or
   access privileged APIs.
2. **Validate all properties.** Run client-side JSON Schema validation on every incoming
   component payload.
3. **Sanitize text values.** Do not render agent-provided strings as raw HTML unless you
   have sanitized them.
4. **Pin catalog versions.** Clients should only accept `catalogId`s from their own
   pre-registered list.
