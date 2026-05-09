---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition
---

# ViewTransition

Baseline

2025

\*

Newly available

Since October 2025, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

\\\* Some parts of this feature may have varying levels of support.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransition&level=low)

The **`ViewTransition`** interface of the [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API "View Transition API") represents an active view transition, and provides functionality to react to the transition reaching different states (e.g., ready to run the animation, or animation finished) or skip the transition altogether.

This object type is made available in the following ways:

- Via the [`Document.activeViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeViewTransition) property. This provides a consistent way to access the active view transition in any context, without having to worry about saving it for easy access later on.
- In the case of same-document (SPA) transitions, it is also returned by the [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") method.
- In the case of cross-document (MPA) transitions, it is also made available:
  - In the outgoing page via the [`pageswap`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageswap_event "pageswap") event object's [`PageSwapEvent.viewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent/viewTransition) property.
  - In the inbound page via the [`pagereveal`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pagereveal_event "pagereveal") event object's [`PageRevealEvent.viewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PageRevealEvent/viewTransition) property.

When a view transition is triggered by a `startViewTransition()` call (or a page navigation in the case of MPA transitions), a sequence of steps is followed as explained in [The view transition process](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#the_view_transition_process). This also explains when the different promises fulfill.

## In this article

- [Instance properties](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#instance_properties)
- [Instance methods](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#instance_methods)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#see_also)

## [Instance properties](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#instance_properties)

[`ViewTransition.finished`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished) Read only

A [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that fulfills once the transition animation is finished, and the new page view is visible and interactive to the user.

[`ViewTransition.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready) Read only

A [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that fulfills once the pseudo-element tree is created and the transition animation is about to start.

[`ViewTransition.types`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types) Read only

A [`ViewTransitionTypeSet`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet) that allows the types set on the view transition to be accessed and modified.

[`ViewTransition.updateCallbackDone`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone) Read only

A [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) that fulfills when the promise returned by the [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") method's callback fulfills.

## [Instance methods](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#instance_methods)

[`skipTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition "skipTransition()")

Skips the animation part of the view transition, but doesn't skip running the [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") callback that updates the DOM.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#examples)

In the following SPA example, the [`ViewTransition.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready) promise is used to trigger a custom circular reveal view transition emanating from the position of the user's cursor on click, with animation provided by the [Web Animations API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API "Web Animations API").

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

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#specifications)

| Specification                                                                                                              |
| -------------------------------------------------------------------------------------------------------------------------- |
| [CSS View Transitions Module Level 1\<br>\# viewtransition](https://drafts.csswg.org/css-view-transitions/#viewtransition) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransition.json "File: api/ViewTransition.json")

|                                                                                                            | desktop                            | mobile                         |
| ---------------------------------------------------------------------------------------------------------- | ---------------------------------- | ------------------------------ | ------------------------------------ | -------------------------------- | ----------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------- |
|                                                                                                            | Chrome                             | Edge                           | Firefox                              | Opera                            | Safari                              | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                     | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                      |
| ---                                                                                                        | ---                                | ---                            | ---                                  | ---                              | ---                                 | ---                                                | ---                                                          | ---                                             | ---                                               | ---                                                   | ---                                                  | ---                                                 |
| `ViewTransition`                                                                                           | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |
| [`finished`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished)                     | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |
| [`ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready)                           | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |
| [`skipTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition)         | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |
| `transitionRoot`<br>Experimental                                                                           | Chrome – Full support<br>Chrome147 | Edge – Full support<br>Edge147 | Firefox – No support<br>FirefoxNo    | Opera – Full support<br>Opera131 | Safari – No support<br>SafariNo     | Chrome Android – Full support<br>Chrome Android147 | Firefox for Android – No support<br>Firefox for AndroidNo    | Opera Android – Full support<br>Opera Android98 | Safari on iOS – No support<br>Safari on iOSNo     | Samsung Internet – No support<br>Samsung InternetNo   | WebView Android – Full support<br>WebView Android147 | WebView on iOS – No support<br>WebView on iOSNo     |
| [`types`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types)                           | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| [`updateCallbackDone`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone) | Chrome – Full support<br>Chrome111 | Edge – Full support<br>Edge111 | Firefox – Full support<br>Firefox144 | Opera – Full support<br>Opera97  | Safari – Full support<br>Safari18   | Chrome Android – Full support<br>Chrome Android111 | Firefox for Android – Full support<br>Firefox for Android144 | Opera Android – Full support<br>Opera Android75 | Safari on iOS – Full support<br>Safari on iOS18   | Samsung Internet – Full support<br>Samsung Internet22 | WebView Android – Full support<br>WebView Android111 | WebView on iOS – Full support<br>WebView on iOS18   |
| `waitUntil`<br>Experimental                                                                                | Chrome – Full support<br>Chrome144 | Edge – Full support<br>Edge144 | Firefox – No support<br>FirefoxNo    | Opera – Full support<br>Opera128 | Safari – No support<br>SafariNo     | Chrome Android – Full support<br>Chrome Android144 | Firefox for Android – No support<br>Firefox for AndroidNo    | Opera Android – Full support<br>Opera Android95 | Safari on iOS – No support<br>Safari on iOSNo     | Samsung Internet – No support<br>Samsung InternetNo   | WebView Android – Full support<br>WebView Android144 | WebView on iOS – No support<br>WebView on iOSNo     |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

No supportNo support

Experimental. Expect behavior to change in the future.

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition#see_also)

- [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API)
- [Using the View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using)
- [Using view transition types](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types)
- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
