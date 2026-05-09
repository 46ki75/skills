# Using the View Transition API

> Source: <https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using>

## Table of contents

1. [How the transition process works](#1-how-the-transition-process-works)
2. [SPA: same-document transitions](#2-spa-same-document-transitions)
3. [MPA: cross-document transitions](#3-mpa-cross-document-transitions)
4. [Customizing animations with CSS](#4-customizing-animations-with-css)
5. [Per-element animations with `view-transition-name`](#5-per-element-animations-with-view-transition-name)
6. [JavaScript-controlled animations](#6-javascript-controlled-animations)
7. [MPA: using `pageswap` and `pagereveal` for custom transitions](#7-mpa-using-pageswap-and-pagereveal-for-custom-transitions)
8. [Stabilizing MPA page state](#8-stabilizing-mpa-page-state)

---

## 1. How the transition process works

1. Transition is **triggered** (via `startViewTransition()` for SPA, or
   navigation for MPA).
2. Browser captures **static snapshots** of elements with a `view-transition-name`.
3. The **DOM update** runs (SPA: callback; MPA: navigation completes).
   → `ViewTransition.updateCallbackDone` fulfills.
4. Browser captures **live snapshots** of the new state.
   → `ViewTransition.ready` fulfills.
5. Old snapshots **animate out**; new snapshots **animate in** (default:
   cross-fade).
6. Animation ends → `ViewTransition.finished` fulfills.

If the page is not visible (minimised, different tab), the animation is skipped
automatically; the DOM update still runs.

The pseudo-element tree that drives the animation:

```text
::view-transition
└─ ::view-transition-group(root)
   └─ ::view-transition-image-pair(root)
      ├─ ::view-transition-old(root)   ← static image, animates out
      └─ ::view-transition-new(root)   ← live DOM region, animates in
```

---

## 2. SPA: same-document transitions

Wrap the state update in `document.startViewTransition()`. Always provide a
fallback for browsers that do not support the API.

```js
function navigate(updateFn) {
  if (!document.startViewTransition) {
    updateFn();   // fallback: update immediately, no animation
    return;
  }
  document.startViewTransition(updateFn);
}
```

The callback can be async — the transition waits for the returned promise:

```js
document.startViewTransition(async () => {
  const html = await fetchPageContent(url);
  document.body.innerHTML = html;
});
```

The default transition is a cross-fade of the entire page — often all you need.

---

## 3. MPA: cross-document transitions

No JavaScript required for a basic transition. Both the outgoing and destination
pages must include the `@view-transition` at-rule with `navigation: auto`.
Pages must be **same-origin**.

```css
/* Add to the CSS of BOTH pages (or a shared stylesheet) */
@view-transition {
  navigation: auto;
}
```

That one rule is enough to get a cross-fade between every same-origin navigation
on those pages.

---

## 4. Customizing animations with CSS

Override the default cross-fade by targeting the pseudo-elements.

Prefer setting `animation-duration` / `animation-timing-function` on
`::view-transition-group()` rather than directly on `old`/`new`, because the
values cascade to both children consistently:

```css
::view-transition-group(root) {
  animation-duration: 0.4s;
  animation-timing-function: ease-in-out;
}
```

For a slide-in/out animation:

```css
@keyframes slide-out-left {
  to { transform: translateX(-100%); }
}
@keyframes slide-in-right {
  from { transform: translateX(100%); }
}

::view-transition-old(root) {
  animation: 0.4s ease-in-out both slide-out-left;
}
::view-transition-new(root) {
  animation: 0.4s ease-in-out both slide-in-right;
}
```

For MPA, the pseudo-element rules must be in the **destination** page (or a
shared stylesheet). If you want the same animation in both navigation
directions, include the rules in both pages.

Use `*` to target all named snapshot groups at once:

```css
::view-transition-group(*) { animation-duration: 0.3s; }
```

---

## 5. Per-element animations with `view-transition-name`

By default, the entire page cross-fades as one unit (`root` group). Assign
a `view-transition-name` to elements you want to animate **independently**.

```css
.hero-image {
  view-transition-name: hero;
}

figcaption {
  view-transition-name: caption;
}
```

The browser generates a separate `::view-transition-group()` branch for each
named element and automatically morphs their position and size between old and
new states.

Target these groups in CSS to apply custom animations:

```css
::view-transition-old(caption) {
  animation: 0.25s linear both shrink-x;
}
::view-transition-new(caption) {
  animation: 0.25s 0.25s linear both grow-x;
}
```

### Important: names must be unique per frame

If two visible elements share the same `view-transition-name` at the same time,
`ViewTransition.ready` rejects and the transition is skipped entirely.

Assign names dynamically and clean them up after the transition:

```js
const vt = document.startViewTransition(() => {
  card.style.viewTransitionName = 'selected';
  updateDOM();
});
// Remove the name once the transition finishes to avoid bfcache conflicts
vt.finished.then(() => {
  card.style.viewTransitionName = '';
});
```

### `match-element` keyword

Automatically assigns unique names to all matched elements — useful for lists:

```css
.list-item { view-transition-name: match-element; }
```

---

## 6. JavaScript-controlled animations

Use `ViewTransition.ready` to run a custom animation with the Web Animations
API at exactly the right moment (after snapshots are taken, before the
default CSS animation begins).

### Circular reveal from click position

```js
let lastClick;
document.addEventListener('click', (e) => (lastClick = e));

function navigate(updateFn) {
  if (!document.startViewTransition) { updateFn(); return; }

  const x = lastClick?.clientX ?? innerWidth / 2;
  const y = lastClick?.clientY ?? innerHeight / 2;
  const r = Math.hypot(
    Math.max(x, innerWidth - x),
    Math.max(y, innerHeight - y)
  );

  const vt = document.startViewTransition(updateFn);

  vt.ready.then(() => {
    document.documentElement.animate(
      { clipPath: [`circle(0 at ${x}px ${y}px)`, `circle(${r}px at ${x}px ${y}px)`] },
      { duration: 500, easing: 'ease-in', pseudoElement: '::view-transition-new(root)' }
    );
  });
}
```

Required CSS (disables the default cross-fade so it does not compete):

```css
::view-transition-image-pair(root) { isolation: auto; }
::view-transition-old(root),
::view-transition-new(root) {
  animation: none;
  mix-blend-mode: normal;
  display: block;
}
```

### Back/forward direction with class on `<html>`

A clean pattern for direction-aware transitions:

```js
async function handleNav(isBack, updateFn) {
  if (isBack) document.documentElement.classList.add('back-nav');

  const vt = document.startViewTransition(updateFn);

  try {
    await vt.finished;
  } finally {
    document.documentElement.classList.remove('back-nav');
  }
}
```

```css
.back-nav::view-transition-old(root) { animation-name: slide-out-right; }
.back-nav::view-transition-new(root) { animation-name: slide-in-left; }
```

---

## 7. MPA: using `pageswap` and `pagereveal` for custom transitions

These events give JavaScript access to the `ViewTransition` object during
cross-document navigations, enabling dynamic element naming and type assignment.

### `pageswap` — outgoing page

Fires just before the current page unloads. Use it to assign
`view-transition-name` to specific elements and set transition types.

```js
window.addEventListener('pageswap', async (e) => {
  if (!e.viewTransition) return;

  const targetUrl = new URL(e.activation.entry.url);

  if (isDetailPage(targetUrl)) {
    const id = extractId(targetUrl);

    // Name the element that should morph into the destination
    document.querySelector(`#item-${id} img`).style.viewTransitionName = 'hero';

    // Clean up after snapshots are taken (prevents bfcache naming conflicts)
    await e.viewTransition.finished;
    document.querySelector(`#item-${id} img`).style.viewTransitionName = '';
  }
});
```

> **Note:** Clean up names using `e.viewTransition.finished` (not `ready`).
> If names are left set on bfcache-restored pages, the next navigation sees
> duplicate names and the transition is skipped.

### `pagereveal` — incoming page

Fires when the new document is first rendered. Use it to name the
corresponding element on the new page.

```js
window.addEventListener('pagereveal', async (e) => {
  if (!e.viewTransition) return;

  const fromUrl = navigation.activation.from?.url;
  if (fromUrl && isListPage(new URL(fromUrl))) {
    document.querySelector('.detail-hero').style.viewTransitionName = 'hero';

    await e.viewTransition.finished;
    document.querySelector('.detail-hero').style.viewTransitionName = '';
  }
});
```

---

## 8. Stabilizing MPA page state

For consistent cross-document transitions, render-block the page until
critical content is parsed. This prevents a flash of different layout states
at the start of the transition.

Stylesheets are render-blocked by default. Block scripts and critical HTML
with the `blocking` attribute:

```html
<head>
  <!-- Render-block a critical script -->
  <script async src="layout.js" blocking="render"></script>

  <!-- Render-block until a specific element is in the DOM -->
  <link rel="expect" href="#lead-content" blocking="render" />
</head>
<body>
  <div id="lead-content">…</div>
</body>
```

Use `media` to render-block different elements on different screen sizes:

```html
<link rel="expect" href="#lead-content" blocking="render"
      media="screen and (width > 640px)" />
<link rel="expect" href="#first-section" blocking="render"
      media="screen and (width <= 640px)" />
```
