---
name: web-view-transition
description: >
  Expert guidance for implementing the View Transition API — covering
  same-document (SPA) transitions with `document.startViewTransition()`,
  cross-document (MPA) transitions with `@view-transition`, customizing
  animations via CSS pseudo-elements (`::view-transition-old`,
  `::view-transition-new`, `::view-transition-group`), per-element
  animations with `view-transition-name`, JavaScript control via the
  `ViewTransition` promises (`ready`, `finished`, `updateCallbackDone`),
  context-aware transition types with `:active-view-transition-type()`,
  and graceful fallbacks for unsupported browsers. Use this skill when
  someone wants page transition animations, shared-element transitions,
  slide/fade/circular-reveal effects, or asks about `startViewTransition`,
  `@view-transition`, `view-transition-name`, `::view-transition-*`
  pseudo-elements, or the `ViewTransition` object — even if they just say
  "smooth page transitions" or "animate between routes".
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# Web View Transition API

The View Transition API creates animated transitions between DOM states or page
navigations with minimal code. The browser handles snapshotting old/new states
and animating between them.

**Browser support:** Chrome 111+, Edge 111+, Firefox 144+, Safari 18+
(Baseline 2025). Always provide a no-animation fallback.

## Quick mental model

When a transition fires, the browser:

1. Captures a **static snapshot** of the current view
2. Applies your DOM change
3. Captures a **live snapshot** of the new view
4. Animates old → new using a pseudo-element tree layered above the page

The pseudo-element tree looks like:

```text
::view-transition
└─ ::view-transition-group(root)
   └─ ::view-transition-image-pair(root)
      ├─ ::view-transition-old(root)   ← static old snapshot
      └─ ::view-transition-new(root)   ← live new snapshot
```

Named elements get their own group alongside `root`.

---

## SPA (same-document) transitions

### Basic usage

Wrap your DOM update in `document.startViewTransition()`. The default
cross-fade animation applies automatically.

```js
function navigate(updateFn) {
  // Graceful fallback — the update still runs, just without animation
  if (!document.startViewTransition) {
    updateFn();
    return;
  }

  document.startViewTransition(updateFn);
}
```

The callback can be async; the transition waits for the returned promise.

```js
document.startViewTransition(async () => {
  const data = await fetchNewContent();
  renderContent(data);
});
```

### Animate specific elements separately

By default the whole page cross-fades. Assign `view-transition-name` to
elements you want to animate independently. Names must be **unique per frame**.

```css
/* CSS */
.hero-image {
  view-transition-name: hero;
}

nav {
  view-transition-name: site-nav;
}
```

This generates separate `::view-transition-group(hero)` and
`::view-transition-group(site-nav)` branches. The browser automatically
morphs position and size between old/new states for those elements.

**Avoid conflicts:** if two rendered elements share the same name, `ready`
rejects and the transition is skipped. Assign names dynamically and clean them
up:

```js
document.startViewTransition(() => {
  clickedCard.style.viewTransitionName = "selected-card";
  updateDOM();
  // Reset in the next frame to prevent bfcache conflicts
  requestAnimationFrame(() => {
    clickedCard.style.viewTransitionName = "";
  });
});
```

---

## MPA (cross-document) transitions

No JavaScript needed. Both the outgoing and destination pages must opt in with
the CSS `@view-transition` at-rule. Both pages must be **same-origin**.

```css
/* Add to CSS on BOTH pages */
@view-transition {
  navigation: auto;
}
```

That's it for a default cross-fade. Customize animations in the **destination**
page's CSS.

---

## Customizing animations

Target the pseudo-elements to override the default cross-fade. Prefer targeting
`::view-transition-group()` for duration/easing so the values cascade to
`old`/`new` consistently.

```css
/* Slow down all transitions */
::view-transition-group(root) {
  animation-duration: 0.4s;
  animation-timing-function: ease-in-out;
}

/* Custom swipe-up animation */
@keyframes slide-out {
  to {
    transform: translateY(-100%);
  }
}
@keyframes slide-in {
  from {
    transform: translateY(100%);
  }
}

::view-transition-old(root) {
  animation: 0.4s ease-in both slide-out;
}
::view-transition-new(root) {
  animation: 0.4s ease-in both slide-in;
}
```

Use `*` to target all named groups at once:

```css
::view-transition-group(*) {
  animation-duration: 0.3s;
}
```

### Circular-reveal (Web Animations API)

For programmatic animations that depend on runtime data (e.g. click position),
use the `ViewTransition.ready` promise together with the Web Animations API.

```js
let lastClick;
document.addEventListener("click", (e) => (lastClick = e));

function navigate(updateFn) {
  if (!document.startViewTransition) {
    updateFn();
    return;
  }

  const x = lastClick?.clientX ?? innerWidth / 2;
  const y = lastClick?.clientY ?? innerHeight / 2;
  const r = Math.hypot(
    Math.max(x, innerWidth - x),
    Math.max(y, innerHeight - y),
  );

  const transition = document.startViewTransition(updateFn);

  transition.ready.then(() => {
    document.documentElement.animate(
      {
        clipPath: [
          `circle(0 at ${x}px ${y}px)`,
          `circle(${r}px at ${x}px ${y}px)`,
        ],
      },
      {
        duration: 500,
        easing: "ease-in",
        pseudoElement: "::view-transition-new(root)",
      },
    );
  });
}

// Required CSS to disable default cross-fade blending
// ::view-transition-image-pair(root) { isolation: auto; }
// ::view-transition-old(root), ::view-transition-new(root) {
//   animation: none; mix-blend-mode: normal; display: block;
// }
```

---

## ViewTransition object (JavaScript control)

`document.startViewTransition()` returns a `ViewTransition` object. Access it
in MPA transitions via `PageSwapEvent.viewTransition` (outgoing page) and
`PageRevealEvent.viewTransition` (incoming page). Also available anywhere via
`document.activeViewTransition`.

| Promise              | Fulfills when                                     |
| -------------------- | ------------------------------------------------- |
| `updateCallbackDone` | The DOM update callback resolved                  |
| `ready`              | Pseudo-elements created, animation about to start |
| `finished`           | Animation completed, new view is interactive      |

```js
const transition = document.startViewTransition(updateFn);

// Know when the DOM is updated (regardless of animation outcome)
transition.updateCallbackDone.then(() => console.log("DOM updated"));

// Run custom JS animation at the right moment
transition.ready.then(() => {
  /* animate */
});

// Cleanup after animation completes
transition.finished.then(() => {
  element.style.viewTransitionName = "";
});

// Skip animation (DOM update still runs)
transition.skipTransition();
```

### Back/forward direction pattern

```js
async function handleNav(isBack) {
  if (isBack) document.documentElement.classList.add("back-nav");

  const transition = document.startViewTransition(updateFn);

  try {
    await transition.finished;
  } finally {
    document.documentElement.classList.remove("back-nav");
  }
}
```

```css
.back-nav::view-transition-old(root) {
  animation-name: slide-out-right;
}
.back-nav::view-transition-new(root) {
  animation-name: slide-in-left;
}
```

---

## Transition types (context-aware animations)

Types let you apply different animations to the same elements depending on
context (e.g. "forwards" vs "backwards" in a gallery).

### SPA — pass types to `startViewTransition`

```js
document.startViewTransition({
  update() {
    renderNextImage();
  },
  types: ["forwards"],
});
```

Modify types dynamically on the returned object:

```js
const vt = document.startViewTransition({ update: renderFn });
if (isBack) vt.types.add("backwards");
```

### MPA — set types in `@view-transition`

```css
@view-transition {
  navigation: auto;
  types: chapter-forward;
}
```

Or assign dynamically via `pageswap`/`pagereveal` events:

```js
window.addEventListener("pageswap", (e) => {
  if (e.viewTransition && goingForward(e)) {
    e.viewTransition.types.add("forwards");
  }
});
```

### Apply CSS per type

```css
/* Styles when any transition is active */
html:active-view-transition {
  :root {
    view-transition-name: none;
  }
  .card {
    view-transition-name: card;
  }
}

/* Different animations per type */
html:active-view-transition-type(forwards) {
  &::view-transition-old(card) {
    animation-name: slide-out-left;
  }
  &::view-transition-new(card) {
    animation-name: slide-in-right;
  }
}

html:active-view-transition-type(backwards) {
  &::view-transition-old(card) {
    animation-name: slide-out-right;
  }
  &::view-transition-new(card) {
    animation-name: slide-in-left;
  }
}
```

---

## MPA: pageswap / pagereveal pattern

Use these events to set `view-transition-name` dynamically on MPA pages,
enabling shared-element transitions between specific elements.

```js
// outgoing page — runs just before unload
window.addEventListener("pageswap", async (e) => {
  if (!e.viewTransition) return;

  const targetUrl = new URL(e.activation.entry.url);
  if (isDetailPage(targetUrl)) {
    const id = extractId(targetUrl);
    document.querySelector(`#item-${id} img`).style.viewTransitionName =
      "hero-img";

    // Clean up to avoid bfcache naming conflicts
    await e.viewTransition.finished;
    document.querySelector(`#item-${id} img`).style.viewTransitionName = "";
  }
});

// incoming page — runs when new page first renders
window.addEventListener("pagereveal", async (e) => {
  if (!e.viewTransition) return;

  const fromUrl = navigation.activation.from?.url;
  if (fromUrl && isListPage(new URL(fromUrl))) {
    document.querySelector(".detail-hero").style.viewTransitionName =
      "hero-img";

    await e.viewTransition.finished;
    document.querySelector(".detail-hero").style.viewTransitionName = "";
  }
});
```

---

## Common pitfalls

**Duplicate `view-transition-name`:** if two visible elements share a name,
`ViewTransition.ready` rejects and the transition is skipped silently. Clean up
names after transitions using `finished`.

**bfcache conflicts:** when the back button is pressed, the page is restored
from cache. If names were left set, the next `pagereveal` handler sets them
again → duplicate → skipped. Always remove names after `finished`.

**Hidden page skips transition:** if the page is not visible (minimised, other
tab), `startViewTransition` skips the animation automatically. This is correct
behaviour — the DOM update still runs.

**MPA cross-origin:** cross-document transitions only work between same-origin
pages. A cross-origin redirect in the chain also disables the `activation`
property (`PageSwapEvent.activation` returns `null`).

**`match-element` keyword:** you can use `view-transition-name: match-element`
to automatically assign unique names to all matched elements, useful for list
items without manual ID assignment.

---

## Accessibility

Respect the user's motion preference. When `prefers-reduced-motion: reduce` is
set, either remove the animation or make it instant.

```css
@media (prefers-reduced-motion: reduce) {
  ::view-transition-group(*),
  ::view-transition-old(*),
  ::view-transition-new(*) {
    animation-duration: 0.01ms !important;
  }
}
```

---

## Reference files

Read these when you need deeper detail than what is covered above.

- **`references/api.md`** — Complete API reference: browser support table,
  `Document.startViewTransition()` signature, all `ViewTransition` promises,
  `ViewTransitionTypeSet` methods, `pageswap`/`pagereveal` event objects, and
  every CSS addition (`@view-transition`, `view-transition-name`,
  `view-transition-class`, pseudo-classes, pseudo-elements, `<link rel="expect">`)
- **`references/using.md`** — Detailed walkthrough: how the transition process
  works step-by-step, SPA and MPA setup, CSS animation customization, per-element
  naming, the circular-reveal Web Animations API pattern, back/forward direction
  patterns, `pageswap`/`pagereveal` shared-element examples, and MPA render
  stabilization with `<link rel="expect">`
- **`references/using-types.md`** — In-depth guide to transition types: SPA
  `types` option, `ViewTransition.types.add()`, type-specific CSS with
  `:active-view-transition-type()`, static MPA types in `@view-transition`,
  and dynamic MPA types via `pageswap`/`pagereveal` with direction detection
