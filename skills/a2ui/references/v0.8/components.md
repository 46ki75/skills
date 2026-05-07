# A2UI Component Gallery — v0.8

All standard components from the v0.8 catalog with full JSON examples.

Schema: `https://a2ui.org/specification/v0_8/standard_catalog_definition.json`

## Table of Contents

- [Common Properties](#common-properties)
- [Layout: Row](#row)
- [Layout: Column](#column)
- [Layout: List](#list)
- [Display: Text](#text)
- [Display: Image](#image)
- [Display: Icon](#icon)
- [Display: Divider](#divider)
- [Interactive: Button](#button)
- [Interactive: TextField](#textfield)
- [Interactive: CheckBox](#checkbox)
- [Interactive: Slider](#slider)
- [Interactive: DateTimeInput](#datetimeinput)
- [Interactive: MultipleChoice](#multiplechoice)
- [Container: Card](#card)
- [Container: Modal](#modal)
- [Container: Tabs](#tabs)

---

## Common Properties

All components share:

- `id` (required) — unique identifier within the surface.
- `accessibility` — `{ "label": "...", "role": "..." }`.
- `weight` — flex-grow value when inside a Row or Column.

---

## Row

Horizontal layout container. Children are arranged left-to-right.

**Properties:** `children` (`explicitList` or `template`), `distribution`, `alignment`

`distribution` values: `start`, `end`, `center`, `spaceBetween`, `spaceAround`, `spaceEvenly`\
`alignment` values: `start`, `end`, `center`, `stretch`, `baseline`

```json
{
  "id": "toolbar",
  "component": {
    "Row": {
      "children": { "explicitList": ["btn1", "btn2", "btn3"] },
      "distribution": "spaceBetween",
      "alignment": "center"
    }
  }
}
```

---

## Column

Vertical layout container. Children are arranged top-to-bottom.

**Properties:** `children` (`explicitList` or `template`), `distribution`, `alignment`

```json
{
  "id": "content",
  "component": {
    "Column": {
      "children": { "explicitList": ["header", "body", "footer"] },
      "distribution": "start",
      "alignment": "stretch"
    }
  }
}
```

---

## List

Scrollable list. Supports static children and dynamic templates.

**Properties:** `children` (`explicitList` or `template`), `direction`, `alignment`

```json
{
  "id": "message-list",
  "component": {
    "List": {
      "children": {
        "template": {
          "dataBinding": "/messages",
          "componentId": "message-item"
        }
      },
      "direction": "vertical"
    }
  }
}
```

---

## Text

Display text with optional styling.

**Properties:** `text` (BoundValue), `usageHint`

`usageHint` values: `h1`, `h2`, `h3`, `h4`, `h5`, `caption`, `body`

```json
{
  "id": "title",
  "component": {
    "Text": {
      "text": { "literalString": "Welcome to A2UI" },
      "usageHint": "h1"
    }
  }
}
```

Data-bound example:

```json
{
  "id": "username",
  "component": {
    "Text": {
      "text": { "path": "/user/name" }
    }
  }
}
```

---

## Image

Display an image from a URL.

**Properties:** `url` (BoundValue), `fit`, `usageHint`

`fit` values: `cover`, `contain`, `fill`, `none`

```json
{
  "id": "hero",
  "component": {
    "Image": {
      "url": { "literalString": "https://example.com/hero.png" },
      "fit": "cover",
      "usageHint": "hero"
    }
  }
}
```

---

## Icon

Display a named icon from the catalog's standard icon set.

**Properties:** `name` (BoundValue)

```json
{
  "id": "check-icon",
  "component": {
    "Icon": {
      "name": { "literalString": "check" }
    }
  }
}
```

---

## Divider

Visual separator line.

**Properties:** `axis`

`axis` values: `horizontal`, `vertical`

```json
{
  "id": "separator",
  "component": {
    "Divider": {
      "axis": "horizontal"
    }
  }
}
```

---

## Button

Clickable button that triggers a `userAction`.

**Properties:** `child` (component ID), `primary` (boolean), `action`

```json
{
  "id": "submit-btn",
  "component": {
    "Button": {
      "child": "submit-text",
      "primary": true,
      "action": {
        "name": "submit_form",
        "context": [
          { "key": "formId", "value": { "literalString": "booking" } }
        ]
      }
    }
  }
}
```

`action.context` items have `key` and `value` (BoundValue).

---

## TextField

Text input field.

**Properties:** `label` (BoundValue), `text` (BoundValue), `textFieldType`, `validationRegexp`

`textFieldType` values: `shortText`, `longText`, `number`, `obscured`, `date`

```json
{
  "id": "email-input",
  "component": {
    "TextField": {
      "label": { "literalString": "Email Address" },
      "text": { "path": "/user/email" },
      "textFieldType": "shortText"
    }
  }
}
```

The user's input is written back to the data model path bound to `text`.

---

## CheckBox

Boolean toggle.

**Properties:** `label` (BoundValue), `value` (BoundValue — boolean)

```json
{
  "id": "terms-checkbox",
  "component": {
    "CheckBox": {
      "label": { "literalString": "I agree to the terms" },
      "value": { "path": "/form/agreedToTerms" }
    }
  }
}
```

---

## Slider

Numeric range input.

**Properties:** `value` (BoundValue — number), `minValue`, `maxValue`

```json
{
  "id": "volume",
  "component": {
    "Slider": {
      "value": { "path": "/settings/volume" },
      "minValue": 0,
      "maxValue": 100
    }
  }
}
```

---

## DateTimeInput

Date and/or time picker.

**Properties:** `label` (BoundValue), `value` (BoundValue), `enableDate`, `enableTime`

```json
{
  "id": "date-picker",
  "component": {
    "DateTimeInput": {
      "label": { "literalString": "Select Date" },
      "value": { "path": "/booking/date" },
      "enableDate": true,
      "enableTime": false
    }
  }
}
```

---

## MultipleChoice

Select one or more options from a list.

**Properties:** `options` (array), `selections` (BoundValue), `maxAllowedSelections`

Each `options` item: `{ "label": BoundValue, "value": string }`

```json
{
  "id": "country-select",
  "component": {
    "MultipleChoice": {
      "options": [
        { "label": { "literalString": "USA" },    "value": "us" },
        { "label": { "literalString": "Canada" }, "value": "ca" },
        { "label": { "literalString": "UK" },     "value": "gb" }
      ],
      "selections": { "path": "/form/country" },
      "maxAllowedSelections": 1
    }
  }
}
```

Set `maxAllowedSelections` to `1` for radio-button behaviour, or higher for
multi-select checkboxes.

---

## Card

Container with elevation/border and padding. Wraps a single child.

**Properties:** `child` (component ID)

```json
{
  "id": "info-card",
  "component": {
    "Card": {
      "child": "card-content"
    }
  }
}
```

---

## Modal

Overlay dialog with an entry-point trigger and modal content.

**Properties:** `entryPointChild` (component ID), `contentChild` (component ID)

```json
{
  "id": "confirm-modal",
  "component": {
    "Modal": {
      "entryPointChild": "open-modal-btn",
      "contentChild": "modal-body"
    }
  }
}
```

The renderer renders `entryPointChild` inline. Clicking it opens the modal
overlay that displays `contentChild`.

---

## Tabs

Tabbed navigation with switchable panels.

**Properties:** `tabItems` — array of `{ title: BoundValue, child: componentId }`

```json
{
  "id": "settings-tabs",
  "component": {
    "Tabs": {
      "tabItems": [
        { "title": { "literalString": "General" }, "child": "general-tab" },
        { "title": { "literalString": "Privacy" }, "child": "privacy-tab" },
        { "title": { "literalString": "Billing" }, "child": "billing-tab" }
      ]
    }
  }
}
```
