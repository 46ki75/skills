# A2UI Component Gallery — v0.9

All standard components from the v0.9 basic catalog with full JSON examples.

Schema: `https://a2ui.org/specification/v0_9/basic_catalog.json`

In v0.9 every component uses the flat discriminator format:

```json
{ "id": "my-id", "component": "TypeName", "prop1": "value", ... }
```

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
- [Interactive: ChoicePicker](#choicepicker)
- [Container: Card](#card)
- [Container: Modal](#modal)
- [Container: Tabs](#tabs)
- [v0.8 → v0.9 Property Rename Reference](#v08--v09-property-rename-reference)

---

## Common Properties

All components share:

- `id` (required) — unique identifier within the surface.
- `accessibility` — `{ "label": "...", "role": "..." }`.
- `weight` — flex-grow value when inside a Row or Column.
- `checks` — list of validation checks (see [Actions guide](actions.md)).

---

## Row

Horizontal layout container.

**Properties:** `children` (array or template), `justify`, `align`

`justify` values: `start`, `end`, `center`, `spaceBetween`, `spaceAround`, `spaceEvenly`\
`align` values: `start`, `end`, `center`, `stretch`, `baseline`

```json
{
  "id": "toolbar",
  "component": "Row",
  "children": ["btn1", "btn2", "btn3"],
  "justify": "spaceBetween",
  "align": "center"
}
```

---

## Column

Vertical layout container.

**Properties:** `children` (array or template), `justify`, `align`

```json
{
  "id": "content",
  "component": "Column",
  "children": ["header", "body", "footer"],
  "justify": "start",
  "align": "stretch"
}
```

---

## List

Scrollable list with optional template for dynamic content.

**Properties:** `children` (array or template), `direction`, `align`

```json
{
  "id": "message-list",
  "component": "List",
  "children": {
    "componentId": "message-item",
    "path": "/messages"
  },
  "direction": "vertical"
}
```

---

## Text

Display text with optional styling.

**Properties:** `text` (string or DataBinding), `variant`

`variant` values: `h1`, `h2`, `h3`, `h4`, `h5`, `caption`, `body`

```json
{
  "id": "title",
  "component": "Text",
  "text": "Welcome to A2UI",
  "variant": "h1"
}
```

Data-bound example:

```json
{
  "id": "username",
  "component": "Text",
  "text": { "path": "/user/name" }
}
```

String interpolation via `formatString`:

```json
{
  "id": "greeting",
  "component": "Text",
  "text": {
    "call": "formatString",
    "args": { "template": "Hello, ${/user/name}!" }
  }
}
```

---

## Image

Display an image from a URL.

**Properties:** `url` (string or DataBinding), `fit`, `variant`

`fit` values: `cover`, `contain`, `fill`, `none`

```json
{
  "id": "hero",
  "component": "Image",
  "url": "https://example.com/hero.png",
  "fit": "cover",
  "variant": "hero"
}
```

---

## Icon

Display a named icon.

**Properties:** `name` (string or DataBinding)

```json
{
  "id": "check-icon",
  "component": "Icon",
  "name": "check"
}
```

---

## Divider

Visual separator line.

**Properties:** `axis`

```json
{
  "id": "separator",
  "component": "Divider",
  "axis": "horizontal"
}
```

---

## Button

Clickable button that dispatches an event or calls a local function.

**Properties:** `child` (component ID), `variant`, `action`, `checks`

`variant` values: `primary`, `borderless`

```json
{
  "id": "submit-btn",
  "component": "Button",
  "child": "submit-text",
  "variant": "primary",
  "action": {
    "event": {
      "name": "submit_form",
      "context": {
        "formId": "booking",
        "date": { "path": "/reservation/date" }
      }
    }
  }
}
```

Button with a local function action:

```json
{
  "id": "help-btn",
  "component": "Button",
  "child": "help-text",
  "action": {
    "functionCall": {
      "call": "openUrl",
      "args": { "url": "https://a2ui.org/help" }
    }
  }
}
```

Button disabled until a required field is filled:

```json
{
  "id": "submit-btn",
  "component": "Button",
  "child": "submit-text",
  "checks": [
    {
      "condition": { "call": "required", "args": { "value": { "path": "/partySize" } } },
      "message": "Party size is required"
    }
  ],
  "action": { "event": { "name": "submit_booking" } }
}
```

---

## TextField

Text input field.

**Properties:** `label` (string), `value` (string or DataBinding), `textFieldType`, `validationRegexp`

`textFieldType` values: `shortText`, `longText`, `number`, `obscured`, `date`

Note: In v0.9 the property is `value` (was `text` in v0.8).

```json
{
  "id": "email-input",
  "component": "TextField",
  "label": "Email Address",
  "value": { "path": "/user/email" },
  "textFieldType": "shortText"
}
```

---

## CheckBox

Boolean toggle.

**Properties:** `label` (string), `value` (DataBinding — boolean)

```json
{
  "id": "terms-checkbox",
  "component": "CheckBox",
  "label": "I agree to the terms",
  "value": { "path": "/form/agreedToTerms" }
}
```

---

## Slider

Numeric range input.

**Properties:** `value` (DataBinding), `min`, `max`

Note: In v0.9 the properties are `min`/`max` (were `minValue`/`maxValue` in v0.8).

```json
{
  "id": "volume",
  "component": "Slider",
  "value": { "path": "/settings/volume" },
  "min": 0,
  "max": 100
}
```

---

## DateTimeInput

Date and/or time picker.

**Properties:** `label` (string), `value` (DataBinding), `enableDate`, `enableTime`

```json
{
  "id": "date-picker",
  "component": "DateTimeInput",
  "label": "Select Date",
  "value": { "path": "/booking/date" },
  "enableDate": true,
  "enableTime": false
}
```

---

## ChoicePicker

Select one or more options. Replaces v0.8's `MultipleChoice`.

**Properties:** `options` (array), `value` (DataBinding — array), `variant`

`variant` values: `mutuallyExclusive` (radio), `multipleSelection` (checkboxes)

```json
{
  "id": "country-select",
  "component": "ChoicePicker",
  "options": [
    { "label": "USA",    "value": "us" },
    { "label": "Canada", "value": "ca" },
    { "label": "UK",     "value": "gb" }
  ],
  "value": { "path": "/form/country" },
  "variant": "mutuallyExclusive"
}
```

---

## Card

Container with elevation/border and padding.

**Properties:** `child` (component ID)

```json
{
  "id": "info-card",
  "component": "Card",
  "child": "card-content"
}
```

---

## Modal

Overlay dialog.

**Properties:** `trigger` (component ID), `content` (component ID)

Note: In v0.9 the properties are `trigger`/`content` (were `entryPointChild`/`contentChild` in v0.8).

```json
{
  "id": "confirm-modal",
  "component": "Modal",
  "trigger": "open-modal-btn",
  "content": "modal-body"
}
```

---

## Tabs

Tabbed navigation.

**Properties:** `tabs` — array of `{ title: string, child: componentId }`

Note: In v0.9 the property is `tabs` (was `tabItems` in v0.8); titles are
plain strings instead of BoundValues.

```json
{
  "id": "settings-tabs",
  "component": "Tabs",
  "tabs": [
    { "title": "General", "child": "general-tab" },
    { "title": "Privacy", "child": "privacy-tab" },
    { "title": "Billing", "child": "billing-tab" }
  ]
}
```

---

## v0.8 → v0.9 Property Rename Reference

| Component        | v0.8 property      | v0.9 property        |
| ---------------- | ------------------ | -------------------- |
| Row / Column     | `distribution`     | `justify`            |
| Row / Column     | `alignment`        | `align`              |
| Modal            | `entryPointChild`  | `trigger`            |
| Modal            | `contentChild`     | `content`            |
| Tabs             | `tabItems`         | `tabs`               |
| TextField        | `text`             | `value`              |
| Text / Image     | `usageHint`        | `variant`            |
| Slider           | `minValue`         | `min`                |
| Slider           | `maxValue`         | `max`                |
| Button           | `primary: true`    | `variant: "primary"` |
| MultipleChoice   | *(renamed)*        | `ChoicePicker`       |
| Client message   | `userAction`       | `action`             |
