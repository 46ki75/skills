---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready
---

# ViewTransition: ready property

Baseline

2025

Newly available

Since October 2025, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransition%2Fready&level=low)

The **`ready`** read-only property of the
[`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) interface is a [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that fulfills once the pseudo-element tree is created and the transition animation is about to start.

`ready` will reject if the transition cannot begin. This can be due to misconfiguration, for example, duplicate [`view-transition-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name) s, or if the callback passed to [`Document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition) throws or returns a promise that rejects.

## In this article

- [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#value)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#see_also)

## [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#value)

A Promise.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#examples)

In the following example, `ready` is used to trigger a custom circular reveal view transition emanating from the position of the user's cursor on click, with animation provided by the [Web Animations API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API "Web Animations API").

jsCopy

```
// Store the last click event
let lastClick;
addEventListener("click", (event) => (lastClick = event));

function spaNavigate(data) {
  // Fallback for browsers that don't support this API:
  if (!document.startViewTransition) {
    updateTheDOMSomehow(data);
    return;
  }

  // Get the click position, or fallback to the middle of the screen
  const x = lastClick?.clientX ?? innerWidth / 2;
  const y = lastClick?.clientY ?? innerHeight / 2;
  // Get the distance to the furthest corner
  const endRadius = Math.hypot(
    Math.max(x, innerWidth - x),
    Math.max(y, innerHeight - y),
  );

  // Create a transition:
  const transition = document.startViewTransition(() => {
    updateTheDOMSomehow(data);
  });

  // Wait for the pseudo-elements to be created:
  transition.ready.then(() => {
    // Animate the root's new view
    document.documentElement.animate(
      {
        clipPath: [\
          `circle(0 at ${x}px ${y}px)`,\
          `circle(${endRadius}px at ${x}px ${y}px)`,\
        ],
      },
      {
        duration: 500,
        easing: "ease-in",
        // Specify which pseudo-element to animate
        pseudoElement: "::view-transition-new(root)",
      },
    );
  });
}
```

This animation also requires the following CSS, to turn off the default CSS animation and stop the old and new view states from blending in any way (the new state "wipes" right over the top of the old state, rather than transitioning in):

cssCopy

```
::view-transition-image-pair(root) {
  isolation: auto;
}

::view-transition-old(root),
::view-transition-new(root) {
  animation: none;
  mix-blend-mode: normal;
  display: block;
}
```

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#specifications)

| Specification                                                                                                                                  |
| ---------------------------------------------------------------------------------------------------------------------------------------------- |
| [CSS View Transitions Module Level 1\<br>\# dom-viewtransition-ready](https://drafts.csswg.org/css-view-transitions/#dom-viewtransition-ready) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransition.json "File: api/ViewTransition.json")

|         | desktop                            | mobile                         |
| ------- | ---------------------------------- | ------------------------------ | ------------------------------------ | ------------------------------- | --------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------- |
|         | Chrome                             | Edge                           | Firefox                              | Opera                           | Safari                            | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                   | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                    |
| ---     | ---                                | ---                            | ---                                  | ---                             | ---                               | ---                                                | ---                                                          | ---                                             | ---                                             | ---                                                   | ---                                                  | ---                                               |
| `ready` | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97 | Safari – Full support<br>Safari18 | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18 | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18 |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready#see_also)

- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
