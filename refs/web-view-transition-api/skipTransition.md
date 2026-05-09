---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition
---

# ViewTransition: skipTransition() method

Baseline

2025

Newly available

Since October 2025, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransition%2FskipTransition&level=low)

The **`skipTransition()`** method of the
[`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) interface skips the animation part of the view transition, but doesn't skip running the associated view update.

## In this article

- [Syntax](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#syntax)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#see_also)

## [Syntax](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#syntax)

jsCopy

```
skipTransition()
```

### [Parameters](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#parameters)

None.

### [Return value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#return_value)

`undefined`.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#examples)

### [Skipping an SPA view transition](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#skipping_an_spa_view_transition)

jsCopy

```
// start new view transition
const transition = document.startViewTransition(() => displayNewImage());

// skip the animation and just update the DOM
transition.skipTransition();
```

### [Skipping an MPA view transition](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#skipping_an_mpa_view_transition)

jsCopy

```
// Fired on the current (outgoing) page
document.addEventListener("pageswap", (event) => {
  event.viewTransition?.skipTransition();
});

// Fired on the destination (inbound) page
document.addEventListener("pagereveal", (event) => {
  event.viewTransition?.skipTransition();
});
```

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#specifications)

| Specification                                                                                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CSS View Transitions Module Level 1\<br>\# dom-viewtransition-skiptransition](https://drafts.csswg.org/css-view-transitions/#dom-viewtransition-skiptransition) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransition.json "File: api/ViewTransition.json")

|                  | desktop                            | mobile                         |
| ---------------- | ---------------------------------- | ------------------------------ | ------------------------------------ | ------------------------------- | --------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------- |
|                  | Chrome                             | Edge                           | Firefox                              | Opera                           | Safari                            | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                   | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                    |
| ---              | ---                                | ---                            | ---                                  | ---                             | ---                               | ---                                                | ---                                                          | ---                                             | ---                                             | ---                                                   | ---                                                  | ---                                               |
| `skipTransition` | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97 | Safari – Full support<br>Safari18 | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18 | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18 |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition#see_also)

- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
