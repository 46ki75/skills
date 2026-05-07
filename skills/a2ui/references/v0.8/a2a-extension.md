# A2UI A2A Extension — v0.8

How to use A2UI over the A2A (Agent-to-Agent) protocol.

## Extension URI

```text
https://a2ui.org/a2a-extension/a2ui/v0.8
```

This is the only URI accepted for v0.8.

## Data Encoding

A2UI messages are encoded as A2A `DataPart` objects with:

- `mimeType`: `application/json+a2ui`
- `data`: the A2UI JSON message object

```json
{
  "kind": "data",
  "metadata": {
    "mimeType": "application/json+a2ui"
  },
  "data": {
    "beginRendering": {
      "surfaceId": "main",
      "root": "root"
    }
  }
}
```

## Agent Card

Agents advertise A2UI support in their `AgentCard` inside
`AgentCapabilities.extensions`:

```json
{
  "uri": "https://a2ui.org/a2a-extension/a2ui/v0.8",
  "description": "Ability to render A2UI",
  "required": false,
  "params": {
    "supportedCatalogIds": [
      "https://a2ui.org/specification/v0_8/standard_catalog_definition.json",
      "https://my-company.com/a2ui/v0.8/custom-catalog.json"
    ],
    "acceptsInlineCatalogs": true
  }
}
```

### Parameters

| Parameter               | Type       | Description                                                                               |
| ----------------------- | ---------- | ----------------------------------------------------------------------------------------- |
| `supportedCatalogIds`   | `string[]` | Catalog URIs the agent can generate. Optional.                                            |
| `acceptsInlineCatalogs` | `boolean`  | Whether the agent accepts catalog definitions inline from the client. Defaults to `false`.|

## Client Capabilities

Clients include an `a2uiClientCapabilities` object in the `metadata` of every
A2A `Message` they send:

```json
{
  "metadata": {
    "a2uiClientCapabilities": {
      "supportedCatalogIds": [
        "https://a2ui.org/specification/v0_8/standard_catalog_definition.json"
      ],
      "inlineCatalogs": [
        {
          "catalogId": "https://my-company.com/inline_catalogs/custom",
          "components": {
            "SignaturePad": {
              "type": "object",
              "properties": { "penColor": { "type": "string" } }
            }
          },
          "styles": {}
        }
      ]
    }
  },
  "message": {
    "prompt": { "text": "Show me the booking form" }
  }
}
```

`inlineCatalogs` is only allowed when the agent has set
`acceptsInlineCatalogs: true` in its Agent Card.

## Extension Activation

Clients indicate support for the A2UI extension via the transport-defined
activation mechanism:

- **HTTP / JSON-RPC**: `X-A2A-Extensions` header.
- **gRPC**: `X-A2A-Extensions` metadata value.

Activating the extension means:

- The server may send A2UI-specific messages (`surfaceUpdate`, etc.).
- The client is expected to send A2UI-specific events (`userAction`).

## Schemas

Three JSON schemas define the extension:

| Schema                           | Purpose                                                                                    |
| -------------------------------- | ------------------------------------------------------------------------------------------ |
| Catalog Definition Schema        | Library of components and styles; each catalog has one.                                    |
| Server-to-Client Message Schema  | Wire format for `surfaceUpdate`, `dataModelUpdate`, `beginRendering`, `deleteSurface`.     |
| Client-to-Server Event Schema    | Wire format for `userAction`, `error`.                                                     |

The standard v0.8 catalog schema:
`https://a2ui.org/specification/v0_8/standard_catalog_definition.json`
