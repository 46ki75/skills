---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone
---

# ViewTransition: updateCallbackDone property

Baseline

2025

Newly available

Since October 2025, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransition%2FupdateCallbackDone&level=low)

The **`updateCallbackDone`** read-only property of the
[`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) interface is a [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that fulfills when the promise returned by the [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") method's callback fulfills, or rejects when it rejects.

`updateCallbackDone` is useful when you don't care about the success/failure of a same-document (SPA) view transition animation, and just want to know if and when the DOM is updated.

**Note:**
In the case of a cross-document (MPA) view transition, the `updateCallbackDone` promise of the associated `ViewTransition` is automatically fulfilled.

## In this article

- [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#value)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#see_also)

## [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#value)

A Promise.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#examples)

jsCopy

```
// start new SPA view transition
const transition = document.startViewTransition(() => displayNewImage());

transition.updateCallbackDone.then(() => {
  // Respond to the DOM being updated successfully
});
```

See [Transitions as an enhancement](https://developer.chrome.com/docs/web-platform/view-transitions/#transitions-as-an-enhancement "External link (opens in new tab)") for a useful example.

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#specifications)

| Specification                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [CSS View Transitions Module Level 1\<br>\# dom-viewtransition-updatecallbackdone](https://drafts.csswg.org/css-view-transitions/#dom-viewtransition-updatecallbackdone) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransition.json "File: api/ViewTransition.json")

|                      | desktop                            | mobile                         |
| -------------------- | ---------------------------------- | ------------------------------ | ------------------------------------ | ------------------------------- | --------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------- |
|                      | Chrome                             | Edge                           | Firefox                              | Opera                           | Safari                            | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                   | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                    |
| ---                  | ---                                | ---                            | ---                                  | ---                             | ---                               | ---                                                | ---                                                          | ---                                             | ---                                             | ---                                                   | ---                                                  | ---                                               |
| `updateCallbackDone` | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97 | Safari – Full support<br>Safari18 | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18 | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18 |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone#see_also)

- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
