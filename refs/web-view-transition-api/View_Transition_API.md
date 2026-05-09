---
url: https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API
---

# View Transition API

The **View Transition API** provides a mechanism for easily creating animated transitions between different website views. This includes animating between DOM states in a single-page app (SPA), and animating the navigation between documents in a multi-page app (MPA).

## In this article

- [Concepts and usage](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#concepts_and_usage)
- [Interfaces](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#interfaces)
- [Extensions to other interfaces](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#extensions_to_other_interfaces)
- [HTML additions](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#html_additions)
- [CSS additions](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#css_additions)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#see_also)

## [Concepts and usage](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#concepts_and_usage)

View transitions are a popular design choice for reducing users' cognitive load, helping them stay in context, and reducing perceived loading latency as they move between states or views of an application.

However, creating view transitions on the web has historically been difficult:

- Transitions between states in single-page apps (SPAs) tend to involve writing significant CSS and JavaScript to:
  - Handle the loading and positioning of the old and new content.
  - Animate the old and new states to create the transition.
  - Stop accidental user interactions with the old content from causing problems.
  - Remove the old content once the transition is complete.
    Accessibility issues like loss of reading position, focus confusion, and strange live region announcement behavior can also result from having the new and old content both present in the DOM at once.
- Cross-document view transitions (i.e., across navigations between different pages in MPAs) have historically been impossible.

The View Transition API provides an easy way of handling the required view changes and transition animations for both the above use cases.

Creating a view transition that uses the browser's default transition animations is very quick to do, and there are features that allow you to both customize the transition animation and manipulate the view transition itself (for example specify circumstances under which the animation is skipped), for both SPA and MPA view transitions.

See [Using the View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using) for more information.

## [Interfaces](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#interfaces)

[`CSSViewTransitionRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSViewTransitionRule)

Represents a [`@view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@view-transition) [at-rule](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/At-rules).

[`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition)

Represents a view transition, and provides functionality to react to the transition reaching different states (e.g., ready to run the animation, or animation finished) or skip the transition altogether.

[`ViewTransitionTypeSet`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet)

A [set-like object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set#set-like_browser_apis) representing the types of an active view transition, which enables the types to be queried or modified on-the-fly during a transition.

## [Extensions to other interfaces](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#extensions_to_other_interfaces)

[`Document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)

Starts a new same-document (SPA) view transition and returns a [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) object to represent it.

[`PageRevealEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageRevealEvent)

The event object for the [`pagereveal`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pagereveal_event "pagereveal") event. During a cross-document navigation, it allows you to manipulate the related view transition (providing access to the relevant [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) object) from the document being navigated _to_, if a view transition was triggered by the navigation.

[`PageSwapEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent)

The event object for the [`pageswap`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageswap_event "pageswap") event. During a cross-document navigation, it allows you to manipulate the related view transition (providing access to the relevant [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) object) from the document being navigated _from_, if a view transition was triggered by the navigation. It also provides access to information on the navigation type and current and destination document history entries.

The [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window) [`pagereveal`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pagereveal_event "pagereveal") event

Fired when a document is first rendered, either when loading a fresh document from the network or activating a document (either from [back/forward cache](https://developer.mozilla.org/en-US/docs/Glossary/bfcache) (bfcache) or [prerender](https://developer.mozilla.org/en-US/docs/Glossary/Prerender)).

The [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window) [`pageswap`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageswap_event "pageswap") event

Fired when a document is about to be unloaded due to a navigation.

## [HTML additions](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#html_additions)

[`<link rel="expect">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel#expect)

Identifies the most critical content in the associated document for the user's initial view of the page. Document rendering will be blocked until the critical content has been parsed, ensuring a consistent first paint — and therefore, view transition — across all supporting browsers.

## [CSS additions](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#css_additions)

### [At-rules](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#at-rules)

[`@view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@view-transition)

In the case of a cross-document navigation, `@view-transition` is used to opt in the current and destination documents to undergo a view transition.

### [Properties](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#properties)

[`view-transition-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name)

Specifies the view transition snapshot that selected elements will participate in, which enables an element to be animated separately from the rest of the page during a view transition.

[`view-transition-class`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-class)

Provides an additional method of styling selected elements that have a `view-transition-name`.

### [Pseudo-classes](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#pseudo-classes)

[`:active-view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:active-view-transition)

Matches elements when a view transition is in progress.

[`:active-view-transition-type()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:active-view-transition-type)

Matches elements when a view transition with one or more specific types is in progress.

### [Pseudo-elements](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#pseudo-elements)

[`::view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition)

The root of the view transitions overlay, which contains all view transitions and sits over the top of all other page content.

[`::view-transition-group()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-group)

The root of a single view transition.

[`::view-transition-image-pair()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-image-pair)

The container for a view transition's old and new views — before and after the transition.

[`::view-transition-old()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-old)

A static snapshot of the old view, before the transition.

[`::view-transition-new()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-new)

A live representation of the new view, after the transition.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#examples)

- [Basic View Transitions SPA demo](https://mdn.github.io/dom-examples/view-transitions/spa/): A basic image gallery demo with view transitions, featuring separate animations between old and new images, and old and new captions.
- [Basic View Transitions MPA demo](https://mdn.github.io/dom-examples/view-transitions/mpa/): A sample two-page site that demonstrates usage of cross-document (MPA) view transitions, providing a custom "swipe up" transition when the two pages are navigated between.
- [View transitions `match-element` demo](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name#using_the_match-element_value): An SPA featuring animated list items, demonstrating the use of the `match-element` value of the `view-transition-name` property to animate individual elements.
- [HTTP 203 playlist](https://http203-playlist.netlify.app/): A video player demo app that features several different SPA view transitions, many of which are explained in [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/).
- [Chrome DevRel view transitions demos](https://view-transitions.chrome.dev/): A series of View Transition API demos.

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#specifications)

| Specification                                                                           |
| --------------------------------------------------------------------------------------- |
| [CSS View Transitions Module Level 2](https://drafts.csswg.org/css-view-transitions-2/) |
| [CSS View Transitions Module Level 1](https://drafts.csswg.org/css-view-transitions/)   |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#browser_compatibility)

### [api.Document.startViewTransition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#api.Document.startViewTransition)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API# "Report an issue with this compatibility data") •
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

### [css.at-rules.view-transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#css.at-rules.view-transition)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/css/at-rules/view-transition.json "File: css/at-rules/view-transition.json")

|                    | desktop                            | mobile                         |
| ------------------ | ---------------------------------- | ------------------------------ | -------------------------------------------------- | -------------------------------- | ----------------------------------------------- | -------------------------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------------------- |
|                    | Chrome                             | Edge                           | Firefox                                            | Opera                            | Safari                                          | Chrome Android                                     | Firefox for Android                                                        | Opera Android                                   | Safari on iOS                                                 | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                                  |
| ---                | ---                                | ---                            | ---                                                | ---                              | ---                                             | ---                                                | ---                                                                        | ---                                             | ---                                                           | ---                                                   | ---                                                  | ---                                                             |
| `@view-transition` | Chrome – Full support<br>Chrome126 | Edge – Full support<br>Edge126 | Firefox – No support<br>FirefoxNo<br> <br>footnote | Opera – Full support<br>Opera112 | Safari – Full support<br>Safari18.2<br>footnote | Chrome Android – Full support<br>Chrome Android126 | Firefox for Android – No support<br>Firefox for AndroidNo<br> <br>footnote | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2<br>footnote | Samsung Internet – Full support<br>Samsung Internet28 | WebView Android – Full support<br>WebView Android126 | WebView on iOS – Full support<br>WebView on iOS18.2<br>footnote |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

No supportNo support

See implementation notes.

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#see_also)

- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/) on developer.chrome.com (2024)
- [View Transition API: Creating Smooth Page Transitions](https://stackdiary.com/view-transitions-api/) on stackdiary.com (2023)
- [View Transitions API: Single Page Apps Without a Framework](https://www.debugbear.com/blog/view-transitions-spa-without-framework) on DebugBear (2024)
