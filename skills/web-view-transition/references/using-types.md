# Using View Transition Types

> Source: <https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types>

Transition types let you apply **different CSS animations to the same elements
depending on context** — for example, sliding left vs. right in a gallery
depending on which button the user pressed, or different animations for
chapter-forward vs. chapter-backward navigation in a book app.

Types are arbitrary strings you define. You set them on the transition and then
select them in CSS with `:active-view-transition-type()`.

## Table of contents

1. [SPA: setting types on `startViewTransition`](#1-spa-setting-types-on-startviewtransition)
2. [Applying type-specific CSS animations](#2-applying-type-specific-css-animations)
3. [Modifying types dynamically via `ViewTransition.types`](#3-modifying-types-dynamically-via-viewtransitiontypes)
4. [MPA: setting types in `@view-transition`](#4-mpa-setting-types-in-view-transition)
5. [MPA: setting types dynamically via `pageswap` and `pagereveal`](#5-mpa-setting-types-dynamically-via-pageswap-and-pagereveal)

---

## 1. SPA: setting types on `startViewTransition`

Pass a `types` array in the options object form of `startViewTransition`:

```js
// "Next" button clicked
document.startViewTransition({
  update() { showNextImage(); },
  types: ['forwards'],
});

// "Previous" button clicked
document.startViewTransition({
  update() { showPrevImage(); },
  types: ['backwards'],
});

// Thumbnail clicked — a third distinct animation
document.startViewTransition({
  update() { showImage(id); },
  types: ['upwards'],
});
```

Multiple types can be active simultaneously:

```js
document.startViewTransition({
  update: updateFn,
  types: ['slide', 'hero'],
});
```

---

## 2. Applying type-specific CSS animations

Use `:active-view-transition` for styles common to **all** transitions,
then `:active-view-transition-type()` for **type-specific** overrides.

```css
/* ── Common: assign view-transition-name to animated elements ── */
html:active-view-transition {
  :root { view-transition-name: none; }   /* opt root out */
  .gallery-img { view-transition-name: image; }
  figcaption   { view-transition-name: caption; }

  /* Caption always fades, regardless of direction */
  &::view-transition-old(caption) { animation-name: fade-out; }
  &::view-transition-new(caption) { animation-name: fade-in; animation-delay: 0.6s; }
}

/* ── Forwards: slide left ── */
html:active-view-transition-type(forwards) {
  &::view-transition-old(image) { animation-name: slide-out-to-left; }
  &::view-transition-new(image) { animation-name: slide-in-from-right; }
}

/* ── Backwards: slide right ── */
html:active-view-transition-type(backwards) {
  &::view-transition-old(image) { animation-name: slide-out-to-right; }
  &::view-transition-new(image) { animation-name: slide-in-from-left; }
}

/* ── Upwards: slide from bottom ── */
html:active-view-transition-type(upwards) {
  &::view-transition-old(image) { animation-name: slide-out-to-top; }
  &::view-transition-new(image) { animation-name: slide-in-from-top; animation-delay: 0.6s; }
}

/* ── Duration for all groups ── */
::view-transition-group(*) { animation-duration: 0.6s; }
```

---

## 3. Modifying types dynamically via `ViewTransition.types`

`startViewTransition()` returns a `ViewTransition` whose `.types` is a
`ViewTransitionTypeSet` (Set-like). You can add types **after** starting the
transition, up until `ready` resolves:

```js
function changeContent(direction) {
  // Start without specifying types yet
  const vt = document.startViewTransition({ update: updateFn });

  // Decide the type based on runtime conditions
  if (direction === 'next') vt.types.add('forwards');
  if (direction === 'prev') vt.types.add('backwards');
  // If neither, the default cross-fade applies
}
```

Methods available: `add(type)`, `delete(type)`, `clear()`, `has(type)`,
`size`, `forEach()`, `values()`, `entries()`.

---

## 4. MPA: setting types in `@view-transition`

Add a `types` descriptor to the `@view-transition` at-rule for a **static**
type applied to every cross-document transition on that page:

```css
@view-transition {
  navigation: auto;
  types: slide;
}
```

Multiple types:

```css
@view-transition {
  navigation: auto;
  types: chapter-forward, fade;
}
```

The CSS pseudo-classes respond exactly as in the SPA case:

```css
html:active-view-transition-type(chapter-forward) {
  section { view-transition-name: chapter; }
  &::view-transition-old(chapter) { animation-name: slide-out-to-left; }
  &::view-transition-new(chapter) { animation-name: slide-in-from-right; }
}
```

---

## 5. MPA: setting types dynamically via `pageswap` and `pagereveal`

For **context-dependent** MPA types (e.g. forwards vs. backwards), determine
the type in JavaScript and apply it via `viewTransition.types.add()` inside
the `pageswap` and `pagereveal` event handlers.

Remove the static `types` from `@view-transition` when using dynamic types —
keep only `navigation: auto`:

```css
@view-transition { navigation: auto; }
```

### Determine direction from URL

```js
function getTransitionType(fromEntry, toEntry) {
  // Compare chapter numbers extracted from URLs
  const fromNum = extractPageNumber(fromEntry.url);
  const toNum   = extractPageNumber(toEntry.url);

  if (fromNum > toNum) return 'backwards';
  if (fromNum < toNum) return 'forwards';
  return undefined; // same page (e.g. reload)
}
```

### Apply in `pageswap` (outgoing page)

```js
window.addEventListener('pageswap', (e) => {
  const type = getTransitionType(e.activation.from, e.activation.entry);
  if (type) e.viewTransition?.types.add(type);
});
```

### Apply in `pagereveal` (incoming page)

```js
window.addEventListener('pagereveal', (e) => {
  const type = getTransitionType(
    navigation.activation.from,
    navigation.activation.entry
  );
  if (type) e.viewTransition?.types.add(type);
});
```

### Type-specific CSS on the destination page

```css
html:active-view-transition {
  nav     { view-transition-name: none; }
  section { view-transition-name: chapter; }
}

html:active-view-transition-type(forwards) {
  &::view-transition-old(chapter) { animation-name: slide-out-to-left; }
  &::view-transition-new(chapter) { animation-name: slide-in-from-right; }
}

html:active-view-transition-type(backwards) {
  &::view-transition-old(chapter) { animation-name: slide-out-to-right; }
  &::view-transition-new(chapter) { animation-name: slide-in-from-left; }
}
```

> **Note:** `navigation.activation.from` can be `null` if the user arrives
> directly at the page. Guard against this before calling
> `getTransitionType()`.
