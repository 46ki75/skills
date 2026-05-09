---
url: https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition
---

# Document: startViewTransition() method

Baseline

2025

\*

Newly available

Since October 2025, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

\\\* Some parts of this feature may have varying levels of support.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FDocument%2FstartViewTransition&level=low)

The **`startViewTransition()`** method of the [`Document`](https://developer.mozilla.org/en-US/docs/Web/API/Document) interface starts a new same-document (SPA) [view transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API) and returns a [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) object to represent it.

When `startViewTransition()` is invoked, a sequence of steps is followed as explained in [The view transition process](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#the_view_transition_process).

## In this article

- [Syntax](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#syntax)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#see_also)

## [Syntax](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#syntax)

jsCopy

```
startViewTransition()
startViewTransition(updateCallback)
startViewTransition(options)
```

### [Parameters](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#parameters)

[`updateCallback`Optional](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#updatecallback)

An optional callback function typically invoked to update the DOM during the SPA view transition process, which returns a [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise). The callback is invoked once the API has taken a snapshot of the current page. When the promise returned by the callback fulfills, the view transition begins in the next frame. If the promise returned by the callback rejects, the transition is abandoned.

[`options`Optional](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#options)

An object containing options to configure the view transition. It can include the following properties:

[`update`Optional](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#update)

The same `updateCallback` function described above. Defaults to `null`.

[`types`Optional](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#types)

An array of strings representing the types applied to the view transition. [View transition types](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types) enable selective application of CSS styles or JavaScript logic based on the type of transition occurring. Defaults to an empty sequence.

### [Return value](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#return_value)

A [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) object instance.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#examples)

See [View transition API > Examples](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#examples) for a list of full examples.

### [Basic usage](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#basic_usage)

In this same-document view transition, we check if the browser supports view transitions.
If there's no support, we set the background color using a fallback method which is applied immediately.
Otherwise, we can safely call `document.startViewTransition()` with animation rules that we define in CSS.

htmlCopyPlay

```
<main>
  <section></section>
  <button id="change-color">Change color</button>
</main>
```

We are setting the `animation-duration` to 2 seconds using the [`::view-transition-group`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-group) pseudo-element.

cssCopyPlay

```
html {
  --bg: indigo;
}
main {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
section {
  background-color: var(--bg);
  height: 60px;
  border-radius: 5px;
}
::view-transition-group(root) {
  animation-duration: 2s;
}
```

jsCopyPlay

```
const colors = ["darkred", "darkslateblue", "darkgreen"];
const colBlock = document.querySelector("section");
let count = 0;
const updateColor = () => {
  colBlock.style = `--bg: ${colors[count]}`;
  count = count !== colors.length - 1 ? ++count : 0;
};
const changeColor = () => {
  // Fallback for browsers that don't support View Transitions:
  if (!document.startViewTransition) {
    updateColor();
    return;
  }

  // With View Transitions:
  const transition = document.startViewTransition(() => {
    updateColor();
  });
};
const changeColorButton = document.querySelector("#change-color");
changeColorButton.addEventListener("click", changeColor);
changeColorButton.addEventListener("keypress", changeColor);
```

If view transitions are supported, clicking the button will transition the color from one to another over 2 seconds.
Otherwise, the background color is set using a fallback method, without any animation.

Play

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#specifications)

| Specification                                                                                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CSS View Transitions Module Level 1\<br>\# dom-document-startviewtransition](https://drafts.csswg.org/css-view-transitions-1/#dom-document-startviewtransition) |
| [CSS View Transitions Module Level 2\<br>\# dom-document-startviewtransition](https://drafts.csswg.org/css-view-transitions-2/#dom-document-startviewtransition) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/Document.json "File: api/Document.json")

|                            | desktop                            | mobile                         |
| -------------------------- | ---------------------------------- | ------------------------------ | -------------------------------------------------------- | -------------------------------- | ----------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------- |
|                            | Chrome                             | Edge                           | Firefox                                                  | Opera                            | Safari                              | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                     | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                      |
| ---                        | ---                                | ---                            | ---                                                      | ---                              | ---                                 | ---                                                | ---                                                          | ---                                             | ---                                               | ---                                                   | ---                                                  | ---                                                 |
| `startViewTransition`      | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144                     | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |
| `options` parameter        | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147                     | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `options.types` parameter  | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147                     | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `options.update` parameter | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147                     | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `updateCallback` parameter | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Preview support<br>FirefoxNightly<br> <br>more | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – No support<br>Firefox for AndroidNo    | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

In development. Supported in a pre-release version.In development. Supported in a pre-release version.

No supportNo support

User must explicitly enable this feature.

Has more compatibility info.

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition#see_also)

- [`Document.activeViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeViewTransition)
- [`:active-view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:active-view-transition) pseudo-class
- [`:active-view-transition-type()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:active-view-transition-type) pseudo-class
- [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API)
- [Using the View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using)
- [Using view transition types](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types)
- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
