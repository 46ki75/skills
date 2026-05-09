# View Transition API — API Reference

> Source: <https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API>

## Browser support

**Baseline 2025** — Chrome 111+, Edge 111+, Firefox 144+, Safari 18+.

`@view-transition` CSS at-rule (MPA): Chrome 126+, Edge 126+, Safari 18.2+.
Firefox does not yet support the at-rule.

`types` option / `ViewTransition.types`: Chrome 125+, Edge 125+, Firefox 147+,
Safari 18.2+.

Always provide a graceful fallback: check `document.startViewTransition` before
calling it.

---

## JavaScript API

### `Document.startViewTransition()`

Starts a same-document (SPA) view transition and returns a `ViewTransition`
object.

```js
// Simple callback form
document.startViewTransition(updateCallback)

// Options object form (Chrome 125+, Firefox 147+, Safari 18.2+)
document.startViewTransition({
  update: updateCallback,   // same as the callback form
  types: ['slide', 'hero'], // optional — see ViewTransition.types
})
```

- `updateCallback` — a function (may be async/return a Promise) that performs
  the DOM update. Called after the browser captures the old-state snapshot.
  If it rejects, the transition is abandoned.
- `types` — array of strings labelling the transition; used by CSS
  `:active-view-transition-type()` to select type-specific animations.

Returns a `ViewTransition` object.

Also accessible via `document.activeViewTransition` at any time during an
active transition.

---

### `ViewTransition` object

Represents the active view transition. Obtained from:

- `document.startViewTransition()` return value (SPA)
- `document.activeViewTransition` (any context)
- `PageSwapEvent.viewTransition` on the `pageswap` event (MPA outgoing page)
- `PageRevealEvent.viewTransition` on the `pagereveal` event (MPA incoming page)

#### Promises

| Property | Fulfills when | Rejects when |
| --- | --- | --- |
| `updateCallbackDone` | The DOM update callback resolved | The callback threw or rejected |
| `ready` | Pseudo-element tree created, animation about to start | Duplicate `view-transition-name`; callback failure |
| `finished` | Animation complete, new view is visible and interactive | (SPA only) callback threw or rejected |

`finished` still fulfills even if the animation was skipped via
`skipTransition()`.

```js
const vt = document.startViewTransition(updateFn);

vt.updateCallbackDone.then(() => { /* DOM is updated */ });
vt.ready.then(() => { /* run custom animation here */ });
vt.finished.then(() => { /* cleanup after animation */ });
```

#### `ViewTransition.types`

A `ViewTransitionTypeSet` — a Set-like object of strings representing the
active types. Can be read and modified while the transition is in progress.

```js
// Read
vt.types.has('slide'); // boolean
vt.types.size;         // number

// Modify on-the-fly
vt.types.add('backwards');
vt.types.delete('forwards');
vt.types.clear();

// Iterate
vt.types.forEach((type) => console.log(type));
```

Types set here are reflected in the CSS `:active-view-transition-type()`
selector.

#### `ViewTransition.skipTransition()`

Skips the animation portion of the transition. The DOM update callback still
runs; `finished` still fulfills.

```js
const vt = document.startViewTransition(updateFn);
vt.skipTransition(); // animate out; DOM update still applies
```

---

### MPA events

#### `pageswap` event (outgoing page)

Fires on `window` when a same-origin cross-document navigation is about to
unload the current page. The event object (`PageSwapEvent`) has:

- `e.viewTransition` — the `ViewTransition` object (or `null` if no transition)
- `e.activation` — a `NavigationActivation` with `from` and `entry` history
  entries (or `null` if cross-origin redirect in chain)

Use this to set `view-transition-name` on elements that should participate
in the transition from the old page.

#### `pagereveal` event (incoming page)

Fires on `window` when the new document is first rendered (network load,
bfcache restore, or prerender activation). The event object (`PageRevealEvent`)
has:

- `e.viewTransition` — the `ViewTransition` object (or `null`)

Use `navigation.activation.from` and `.entry` to determine where the user
navigated from.

---

## CSS additions

### At-rule: `@view-transition`

Opts a document into cross-document (MPA) view transitions. Must be in both
the outgoing and destination documents.

```css
@view-transition {
  navigation: auto;         /* required to enable MPA transitions */
  types: slide, chapter;    /* optional — static type labels */
}
```

- `navigation: auto` — enables the transition for same-origin navigations.
- `types` — comma-separated list of type labels, applied to every cross-document
  transition from/to this page (use `pageswap`/`pagereveal` for dynamic types).

### Property: `view-transition-name`

Assigns a named snapshot group to an element, so it animates separately from
the `root` group.

```css
.hero {
  view-transition-name: hero-image; /* any unique <custom-ident> */
}
```

Special values:

- `none` — element does not participate in its own snapshot (uses parent's).
- `match-element` — browser auto-assigns unique names to all matched elements.

**Names must be unique** per rendered frame. Duplicate names cause `ready` to
reject and the transition to be skipped.

### Property: `view-transition-class`

An additional class label for pseudo-element targeting (alternative to name).

### Pseudo-classes

| Selector | Matches when |
| -------- | ------------ |
| `:active-view-transition` | Any view transition is active |
| `:active-view-transition-type(type1, type2)` | Active transition has one of the listed types |

These are typically applied to `html` or `:root` and used for nesting.

```css
html:active-view-transition {
  /* styles applied during any active transition */
  .card { view-transition-name: card; }
}

html:active-view-transition-type(slide) {
  &::view-transition-old(card) { animation-name: slide-out; }
  &::view-transition-new(card) { animation-name: slide-in; }
}
```

### Pseudo-elements

The browser constructs this tree during a transition:

```text
::view-transition
└─ ::view-transition-group(root)         ← one per named element
   └─ ::view-transition-image-pair(root)
      ├─ ::view-transition-old(root)     ← static snapshot, animates out
      └─ ::view-transition-new(root)     ← live DOM region, animates in
```

| Pseudo-element | Description |
| -------------- | ----------- |
| `::view-transition` | Root overlay, sits above all page content |
| `::view-transition-group(name)` | Container per named snapshot; transitions `width`, `height`, `transform` between old/new states |
| `::view-transition-image-pair(name)` | Contains old + new snapshots |
| `::view-transition-old(name)` | Static image of the old state; animates out |
| `::view-transition-new(name)` | Live DOM region of the new state; animates in |

Use `*` as the name argument to target all groups at once:

```css
::view-transition-group(*) { animation-duration: 0.4s; }
```

Default animations: cross-fade (opacity) for most elements; `width`/`height`
scale and `transform` transitions for positional changes.

To override, target `::view-transition-old` and `::view-transition-new`
directly, or set duration/easing on `::view-transition-group` (inherited by
its children):

```css
::view-transition-group(root) {
  animation-duration: 0.5s;
  animation-timing-function: ease-in-out;
}
```

### HTML: `<link rel="expect">`

Render-blocks the page until a specific element is parsed, ensuring a
consistent starting state for MPA transitions:

```html
<link rel="expect" href="#main-content" blocking="render" />
```

---

## Interfaces summary

| Interface / Addition | Description |
| --- | --- |
| `ViewTransition` | Represents the active transition; exposes `ready`, `finished`, `updateCallbackDone`, `types`, `skipTransition()` |
| `ViewTransitionTypeSet` | Set-like object for managing transition type strings (`add`, `delete`, `clear`, `has`, `forEach`, …) |
| `Document.startViewTransition()` | Triggers an SPA transition |
| `document.activeViewTransition` | Current active `ViewTransition` (or `null`) |
| `PageSwapEvent` | Event object for `pageswap`; has `viewTransition` and `activation` |
| `PageRevealEvent` | Event object for `pagereveal`; has `viewTransition` |
| `CSSViewTransitionRule` | CSSOM representation of `@view-transition` |
| `@view-transition` | CSS at-rule to opt MPA pages in |
| `view-transition-name` | CSS property to assign element to a named snapshot group |
| `view-transition-class` | CSS property for additional pseudo-element targeting |
| `:active-view-transition` | CSS pseudo-class: active when any transition is running |
| `:active-view-transition-type()` | CSS pseudo-class: active for transitions with a matching type |
| `::view-transition` | CSS pseudo-element: transition overlay root |
| `::view-transition-group()` | CSS pseudo-element: per-element transition container |
| `::view-transition-image-pair()` | CSS pseudo-element: old + new snapshot container |
| `::view-transition-old()` | CSS pseudo-element: static old-state snapshot |
| `::view-transition-new()` | CSS pseudo-element: live new-state DOM region |
| `<link rel="expect">` | HTML: render-block until critical content is parsed |
