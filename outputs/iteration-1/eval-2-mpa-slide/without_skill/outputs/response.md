# MPA Slide Animations Between Pages Using the View Transitions API

The modern way to achieve slide animations between separate `.html` pages on the same origin is the **Cross-Document View Transitions API** (Level 2). It requires no JavaScript framework, no shared shell, and no service worker — just CSS and a tiny bit of meta configuration in each page.

---

## How It Works

1. The browser intercepts the navigation.
2. It takes a screenshot ("snapshot") of the old page.
3. It loads the new page.
4. It runs a CSS transition between the two snapshots.

You control the animation entirely with CSS `@keyframes` and the `::view-transition-*` pseudo-elements.

---

## Step-by-Step Implementation

### 1. Opt In to Cross-Document View Transitions

Add this to the `<head>` of **every** `.html` file:

```html
<meta name="view-transition" content="same-origin" />
```

This single line tells the browser to use view transitions when navigating between same-origin pages.

> **Browser support:** Chrome 126+, Edge 126+. Firefox and Safari support is in progress. Always check [caniuse.com](https://caniuse.com/view-transitions) before shipping.

---

### 2. Add the Slide CSS

Create a shared stylesheet (e.g. `transitions.css`) and link it from every page:

```css
/* transitions.css */

/* ─── Default (forward) navigation: slide left ─── */

/* Old page exits to the left */
@keyframes slide-out-left {
  from { transform: translateX(0); }
  to   { transform: translateX(-100%); }
}

/* New page enters from the right */
@keyframes slide-in-right {
  from { transform: translateX(100%); }
  to   { transform: translateX(0); }
}

::view-transition-old(root) {
  animation: 300ms ease-in-out both slide-out-left;
}

::view-transition-new(root) {
  animation: 300ms ease-in-out both slide-in-right;
}
```

Link it in every page's `<head>`:

```html
<link rel="stylesheet" href="transitions.css" />
```

That's the minimum — you now have a left-slide on every forward navigation.

---

### 3. Handle Back Navigation (Reverse Slide)

For back navigation the slide should go the other way (old page slides **right**, new page comes from the **left**). Use the `@media (prefers-reduced-motion)` guard and the navigation direction API:

```css
/* transitions.css — add below the forward animations */

/* ─── Back navigation: slide right ─── */
@keyframes slide-out-right {
  from { transform: translateX(0); }
  to   { transform: translateX(100%); }
}

@keyframes slide-in-left {
  from { transform: translateX(-100%); }
  to   { transform: translateX(0); }
}

/* Applied when the browser classifies navigation as "back" */
html[data-nav-direction="back"] ::view-transition-old(root) {
  animation: 300ms ease-in-out both slide-out-right;
}

html[data-nav-direction="back"] ::view-transition-new(root) {
  animation: 300ms ease-in-out both slide-in-left;
}
```

Then add a tiny inline script to each page to set that attribute **before** the transition runs:

```html
<!-- Add to <head>, after the <meta name="view-transition"> tag -->
<script>
  // Detect navigation direction via Navigation API (Chrome 102+)
  if (navigation) {
    navigation.addEventListener("navigate", (e) => {
      const direction = navigation.currentEntry
        ? (navigation.currentEntry.index < (navigation.entries().indexOf(e.destination) ?? -1)
            ? "forward"
            : "back")
        : "forward";
      document.documentElement.dataset.navDirection = direction;
    });
  }
</script>
```

> **Simpler alternative:** If you don't need directional awareness, skip this section and only implement the forward animation.

---

### 4. Respect `prefers-reduced-motion`

Always wrap motion in a media query for accessibility:

```css
/* Disable transitions for users who prefer reduced motion */
@media (prefers-reduced-motion: reduce) {
  ::view-transition-old(root),
  ::view-transition-new(root) {
    animation: none !important;
  }
}
```

---

### 5. Full Example

Here is a complete, self-contained two-page demo.

#### `page1.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <meta name="view-transition" content="same-origin" />
  <title>Page 1</title>
  <link rel="stylesheet" href="transitions.css" />
  <style>
    body { font-family: sans-serif; padding: 2rem; background: #f0f4ff; }
  </style>
</head>
<body>
  <h1>Page 1</h1>
  <p>Click the link to navigate to Page 2 with a slide animation.</p>
  <a href="page2.html">Go to Page 2 →</a>
</body>
</html>
```

#### `page2.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <meta name="view-transition" content="same-origin" />
  <title>Page 2</title>
  <link rel="stylesheet" href="transitions.css" />
  <style>
    body { font-family: sans-serif; padding: 2rem; background: #fff4e6; }
  </style>
</head>
<body>
  <h1>Page 2</h1>
  <p>You navigated here with a slide animation!</p>
  <a href="page1.html">← Back to Page 1</a>
</body>
</html>
```

#### `transitions.css` (complete file)

```css
/* Forward: old page slides out left, new page slides in from right */
@keyframes slide-out-left {
  from { transform: translateX(0); }
  to   { transform: translateX(-100%); }
}

@keyframes slide-in-right {
  from { transform: translateX(100%); }
  to   { transform: translateX(0); }
}

::view-transition-old(root) {
  animation: 300ms ease-in-out both slide-out-left;
}

::view-transition-new(root) {
  animation: 300ms ease-in-out both slide-in-right;
}

/* Back: old page slides out right, new page slides in from left */
@keyframes slide-out-right {
  from { transform: translateX(0); }
  to   { transform: translateX(100%); }
}

@keyframes slide-in-left {
  from { transform: translateX(-100%); }
  to   { transform: translateX(0); }
}

html[data-nav-direction="back"] ::view-transition-old(root) {
  animation: 300ms ease-in-out both slide-out-right;
}

html[data-nav-direction="back"] ::view-transition-new(root) {
  animation: 300ms ease-in-out both slide-in-left;
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  ::view-transition-old(root),
  ::view-transition-new(root) {
    animation: none !important;
  }
}
```

---

## Progressive Enhancement / Fallback

Browsers that don't support the View Transitions API simply perform a normal (instant) navigation — no broken experience, no JavaScript errors. No feature-detection code is needed.

If you want a JavaScript-based fallback for older browsers, you can check:

```js
if (!document.startViewTransition) {
  // Browser doesn't support View Transitions — graceful fallback
}
```

But for most cases the default fallback (instant navigation) is perfectly acceptable.

---

## Animating Specific Elements (Optional Advanced Use)

If you want individual elements (like a header or hero image) to animate independently rather than the whole page, assign them a `view-transition-name`:

```css
/* In the shared stylesheet */
.page-header {
  view-transition-name: page-header;
}
```

The browser will then create a separate `::view-transition-old(page-header)` / `::view-transition-new(page-header)` pair that you can animate independently of `root`.

> **Important:** `view-transition-name` values must be unique per page at snapshot time. Never assign the same name to two visible elements simultaneously.

---

## Summary

| What you need | Where it goes |
|---|---|
| `<meta name="view-transition" content="same-origin">` | `<head>` of every HTML page |
| `::view-transition-old/new` CSS keyframes | Shared CSS file linked on every page |
| (Optional) direction detection script | `<head>` of every HTML page |

No build step, no framework, no service worker required.
