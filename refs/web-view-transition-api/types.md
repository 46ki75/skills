---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types
---

# ViewTransition: types property

Baseline

2026

Newly available

Since January 2026, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransition%2Ftypes&level=low)

The **`types`** read-only property of the
[`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) interface is a [`ViewTransitionTypeSet`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet) that allows the [types](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types) set on the view transition to be accessed and modified.

## In this article

- [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#value)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#see_also)

## [Value](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#value)

A [`ViewTransitionTypeSet`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet). This is a [Set-like object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set#set-like_browser_apis), which means you can modify the types applied to a view transition on-the-fly using methods available on it such as `clear()`, `add()`, and `delete()`.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#examples)

### [Basic usage](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#basic_usage)

This example includes a basic document that transitions between two different pieces of content. We provide three buttons, each of which triggers the transition, but with a different `type` to apply a different kind of animation to the transition.

#### HTML

The markup includes a single [`<p>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/p) element to contain the content and three [`<button>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button) elements to trigger the view transition.

htmlCopyPlay

```
<p>This is my first piece of content. I hope you like it!</p>
<div>
  <button id="default">Default</button>
  <button id="slide">Slide</button>
  <button id="flip">Flip</button>
</div>
```

#### JavaScript

In our script, we create references to the buttons and the content paragraph, and then store our two different pieces of content in two constants.

jsCopyPlay

```
const defaultBtn = document.getElementById("default");
const slideBtn = document.getElementById("slide");
const flipBtn = document.getElementById("flip");
const content = document.querySelector("p");

const first = "This is my first piece of content. I hope you like it!";
const second =
  "This is my second piece of content. Is it better than the first?";
```

Next, we add `click` event listeners to the buttons; when they are clicked, the `changeContent()` function is run.

jsCopyPlay

```
defaultBtn.addEventListener("click", changeContent);
slideBtn.addEventListener("click", changeContent);
flipBtn.addEventListener("click", changeContent);
```

Finally, we define the `changeContent()` function. We start by invoking the [`startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "startViewTransition()") method to update the content and start the transition, saving the returned `ViewTransition` object in the `vt` constant. Inside `startViewTransition()`, the `update` callback checks whether the paragraph `textContent` is equal to the `first` string. If so, we set it to the `second` string. If not, we set it to the `first` string.

In the second part of the `changeContent()` function, we check the value of the `click` event target:

- If it is the "Slide" button, we add a `slide` type to the active view transition's types using `vt.types.add("slide")`.
- If it is the "Flip" button, we add a `flip` type to the active view transition's types using `vt.types.add("flip")`.
- We don't do anything if the "Default" button was pressed. In this case, we want to use the default view transition cross-fade animation.

jsCopyPlay

```
function changeContent(e) {
  const vt = document.startViewTransition({
    update() {
      content.textContent === first
        ? (content.textContent = second)
        : (content.textContent = first);
    },
  });

  if (e.target === slideBtn) {
    vt.types.add("slide");
  } else if (e.target === flipBtn) {
    vt.types.add("flip");
  }
}
```

#### CSS

In our styles, we start off by creating a set of nested rules using the `:active-view-transition` pseudo-class. These styles will be applied whenever a view transition is active, regardless of the their types. We apply a [`view-transition-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name) of `none` to the [`:root`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:root), as we don't want any elements captured and animated on transition except for the `<p>` element, which is given a `view-transition-name` of `content`.

cssCopyPlay

```
html,
body {
  height: 100%;
}

body {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

div {
  display: flex;
  width: 60%;
  justify-content: space-between;
}

p {
  font-size: 1.7em;
  width: 60%;
  color: blue;
  background-color: white;
  margin-top: 0;
}
```

cssCopyPlay

```
html:active-view-transition {
  :root {
    view-transition-name: none;
  }
  p {
    view-transition-name: content;
  }
}
```

Next, we use the `:active-view-transition-type()` pseudo-class to create two blocks of nested styles, the first of which is only applied when the active view transition has a type of `slide`, and the second of which is only applied when the active view transition has a type of `flip`. In each block, we use the [`::view-transition-old()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-old) and [`::view-transition-new()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-new) pseudo-elements to apply custom [`animation-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/animation-name) values to the `slide` capture group's outgoing and incoming views.

As a result:

- When the transition type is `slide`, the old content view slides out to the left, and the new content view slides in from the right.
- When the transition type is `flip`, the old content view rotates horizontally to 90 degrees so it is no longer visible, and the new content view rotates back in.
- In any other case, the default cross-fade transition animations are used.

cssCopyPlay

```
html:active-view-transition-type(slide) {
  &::view-transition-old(content) {
    animation-name: slide-out-to-left;
  }
  &::view-transition-new(content) {
    animation-name: slide-in-from-right;
  }
}

html:active-view-transition-type(flip) {
  &::view-transition-old(content) {
    animation-name: flip-out;
  }
  &::view-transition-new(content) {
    animation-name: flip-in;
    animation-delay: 0.6s;
  }
}
```

Finally, we use [`@keyframes`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@keyframes) animation blocks to define the animations referenced previously. We also set a custom [`animation-duration`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/animation-duration) on all capture groups, to slow the transition animations down slightly.

cssCopyPlay

```
@keyframes slide-out-to-left {
  to {
    translate: -100vw 0;
  }
}
@keyframes slide-in-from-right {
  from {
    translate: 100vw 0;
  }
}

@keyframes flip-out {
  to {
    rotate: x 90deg;
  }
}
@keyframes flip-in {
  from {
    rotate: x -90deg;
  }
}

::view-transition-group(*) {
  animation-duration: 0.6s;
}
```

#### Result

The example is rendered like so:

Play

This is my first piece of content. I hope you like it!

DefaultSlideFlip

Try clicking each button and note how the DOM changes are identical in each case, but the animation is different. This is because a different transition type is set depending on which button is pressed (or no transition type in the "Default" case).

### [Applied example](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#applied_example)

Check out our [MPA multiple transition types example](https://mdn.github.io/dom-examples/view-transitions/mpa-chapter-nav-multiple-transition-types/ "External link (opens in new tab)") ( [source code](https://github.com/mdn/dom-examples/tree/main/view-transitions/mpa-chapter-nav-multiple-transition-types "External link (opens in new tab)")), which demonstrates how to apply different animations to cross-document view transitions depending on the navigation type, represented by different transition types. The transition type is determined on-the-fly with JavaScript during the navigation.

[Applying different cross-document types using pageswap and pagereveal events](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types#applying_different_cross-document_types_using_pageswap_and_pagereveal_events) provides a walkthrough of this example.

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#specifications)

| Specification                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------ |
| [CSS View Transitions Module Level 2\<br>\# dom-viewtransition-types](https://drafts.csswg.org/css-view-transitions-2/#dom-viewtransition-types) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransition.json "File: api/ViewTransition.json")

|         | desktop                            | mobile                         |
| ------- | ---------------------------------- | ------------------------------ | ------------------------------------ | -------------------------------- | ----------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------- |
|         | Chrome                             | Edge                           | Firefox                              | Opera                            | Safari                              | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                     | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                      |
| ---     | ---                                | ---                            | ---                                  | ---                              | ---                                 | ---                                                | ---                                                          | ---                                             | ---                                               | ---                                                   | ---                                                  | ---                                                 |
| `types` | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types#see_also)

- [`ViewTransitionTypeSet`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet)
- [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API)
- [Using the View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using)
- [Using view transition types](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types)
- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
