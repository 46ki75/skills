---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished
---

# ViewTransition: finished property

Baseline

2025

Newly available

Since October 2025, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransition%2Ffinished&level=low)

The **`finished`** read-only property of the
[`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) interface is a [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that fulfills once the transition animation is finished, and the new page view is visible and interactive to the user.

`finished` will only reject in the case of a same-document (SPA) transition, if the callback passed to [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") throws or returns a promise that rejects. This would indicate that the new state of the page wasn't created.

If a transition animation fails to start or is skipped during the transition using [`ViewTransition.skipTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition), the end state is still reached therefore `finished` will still fulfill.

## In this article

- [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#value)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#see_also)

## [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#value)

A Promise.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#examples)

### [Different transitions for different navigations](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#different_transitions_for_different_navigations)

Sometimes certain navigations will require specifically tailored transitions, for example, a "back" navigation may want a different transition to a "forward" navigation. The best way to handle such cases is to set a class name on the `<html>` element, handle the transition — applying the correct animation using a tailored selector — and then remove the class name once the transition is finished.

jsCopy

```
async function handleTransition() {
  if (isBackNavigation) {
    document.documentElement.classList.add("back-transition");
  }

  const transition = document.startViewTransition(() =>
    updateTheDOMSomehow(data),
  );

  try {
    await transition.finished;
  } finally {
    document.documentElement.classList.remove("back-transition");
  }
}
```

**Note:**`isBackNavigation` isn't a built-in feature; it's a theoretical function that could be implemented using the [Navigation API](https://developer.mozilla.org/en-US/docs/Web/API/Navigation_API) or similar.

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#specifications)

| Specification                                                                                                                                        |
| ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CSS View Transitions Module Level 1\<br>\# dom-viewtransition-finished](https://drafts.csswg.org/css-view-transitions/#dom-viewtransition-finished) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransition.json "File: api/ViewTransition.json")

|            | desktop                            | mobile                         |
| ---------- | ---------------------------------- | ------------------------------ | ------------------------------------ | ------------------------------- | --------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------- |
|            | Chrome                             | Edge                           | Firefox                              | Opera                           | Safari                            | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                   | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                    |
| ---        | ---                                | ---                            | ---                                  | ---                             | ---                               | ---                                                | ---                                                          | ---                                             | ---                                             | ---                                                   | ---                                                  | ---                                               |
| `finished` | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97 | Safari – Full support<br>Safari18 | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18 | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18 |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished#see_also)

- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
