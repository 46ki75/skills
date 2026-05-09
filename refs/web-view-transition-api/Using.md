---
url: https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using
description: >
  The article provides an in-depth explanation of the View Transition API, detailing its workings, creation, and customization of view transitions in web applications. Key sections cover: 1) **View Transition Process**: Describes how transitions are triggered, the capturing of snapshots, and how transitions animate. 2) **Creating Basic Transitions**: Illustrates creating transitions for single-page applications (SPAs) and multi-page applications (MPAs). 3) **Customizing Animations**: Discusses how to override default animations and tailor them to specific elements. 4) **Controlling Transitions with JavaScript**: Explains how to interact with the ViewTransition object to enhance transition control. 5) **Stabilizing Page State**: Offers guidelines for ensuring consistent transitions by managing render blocking and critical styling.
---

# Using the View Transition API

This article explains the theory behind how the [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API) works, how to create view transitions and customize the transition animations, and how to manipulate active view transitions. This covers view transitions for both DOM state updates in a single-page app (SPA), and navigating between documents in a multi-page app (MPA).

## In this article

- [The view transition process](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#the_view_transition_process)
- [Creating a basic view transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#creating_a_basic_view_transition)
- [Customizing your animations](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#customizing_your_animations)
- [Different animations for different elements](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#different_animations_for_different_elements)
- [Controlling view transitions with JavaScript](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#controlling_view_transitions_with_javascript)
- [Stabilizing page state to make cross-document transitions consistent](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#stabilizing_page_state_to_make_cross-document_transitions_consistent)

## [The view transition process](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#the_view_transition_process)

Let's walk through the process by which a view transition works:

1. A view transition is triggered. How this is done depends on the type of view transition:
   - In the case of same-document transitions (SPAs), a view transition is triggered by passing the function that would trigger the view change DOM update as a callback to the [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") method.
   - In the case of cross-document transitions (MPAs), a view transition is triggered by initiating navigation to a new document. Both the current and destination documents of the navigation need to be on the same origin, and opt-in to the view transition by including a [`@view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@view-transition) at rule in their CSS with a `navigation` descriptor of `auto`.

     **Note:**
     An active view transition has an associated [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) instance (for example, returned by `startViewTransition()` in the case of same-document (SPA) transitions). The `ViewTransition` object includes several promises, allowing you to run code in response to different parts of the view transition process being reached. See [Controlling view transitions with JavaScript](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#controlling_view_transitions_with_javascript) for more information.

2. On the current (old page) view, the API captures static image **snapshots** of elements that have a [`view-transition-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name) declared on them.

3. The view change occurs:
   - In the case of same-document transitions (SPAs), the callback passed to `startViewTransition()` is invoked, which causes the DOM to change.

     When the callback has run successfully, the [`ViewTransition.updateCallbackDone`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone) promise fulfills, allowing you to respond to the DOM updating.

   - In the case of cross-document transitions (MPAs), the navigation occurs between the current and destination documents.

4. The API captures "live" snapshots (meaning, interactive DOM regions) from the new view.

At this point, the view transition is about to run, and the [`ViewTransition.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready) promise fulfills, allowing you to respond by running a custom JavaScript animation instead of the default, for example.

5. The old page snapshots animate "out", while the new view snapshots animate "in". By default, the old view snapshots animate from [`opacity`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/opacity) 1 to 0, and the new view snapshots animate from `opacity` 0 to 1, which creates a cross-fade.

6. When the transition animations have reached their end states, the [`ViewTransition.finished`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished) promise fulfills, allowing you to respond.

**Note:**
If the document's [page visibility state](https://developer.mozilla.org/en-US/docs/Web/API/Page_Visibility_API) is `hidden` (for example if the document is obscured by a window, the browser is minimized, or another browser tab is active) during a [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") call, the view transition is skipped entirely.

### [An aside on snapshots](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#an_aside_on_snapshots)

It is worth noting that when talking about view transitions, we commonly use the term _snapshot_ to refer to a part of the page that has a `view-transition-name` declared on it. These sections will be animated separately from other parts of the page with different `view-transition-name` values set on them. While the process of animating a snapshot via a view transition actually involves two separate snapshots—one of the old and one of the new UI states—we use snapshot to refer to the whole page area for simplicity.

The snapshot of the old UI state is a static image, so that the user can't interact with it as it animates "out".

The snapshot of the new UI state is an interactive DOM region, so that the user can start to interact with the new content as it animates "in".

### [The view transition pseudo-element tree](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#the_view_transition_pseudo-element_tree)

To handle creating the outbound and inbound transition animations, the API constructs a pseudo-element tree with the following structure:

```
::view-transition
└─ ::view-transition-group(root)
  └─ ::view-transition-image-pair(root)
      ├─ ::view-transition-old(root)
      └─ ::view-transition-new(root)
```

In the case of same-document transitions (SPAs), the pseudo-element tree is made available in the document. In the case of cross-document transitions (MPAs), the pseudo-element tree is made available in the destination document only.

The most interesting parts of the tree structure are as follows:

- [`::view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition) is the root of the view transitions overlay, which contains all view transition groups and sits above all other page content.

- A [`::view-transition-group()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-group) acts as a container for each view transition snapshot. The `root` argument specifies the default snapshot — the view transition animation will apply to the snapshot whose `view-transition-name` is `root`. By default, this is a snapshot of the [`:root`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/:root) element, because the default browser styles define this:

cssCopy

```
:root {
    view-transition-name: root;
}
```

Be aware however that page authors can change this by unsetting the above, and setting `view-transition-name: root` on a different element.

- [`::view-transition-old()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-old) targets the static snapshot of the old page element, and [`::view-transition-new()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-new) targets the live snapshot of the new page element. Both of these render as replaced content, in the same manner as an [`<img>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img) or [`<video>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video), meaning that they can be styled with properties like [`object-fit`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/object-fit) and [`object-position`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/object-position).

**Note:**
It is possible to target different DOM elements with different custom view transition animations by setting a different [`view-transition-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name) on each one. In such cases, a `::view-transition-group()` is created for each one. See [Different animations for different elements](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#different_animations_for_different_elements) for an example.

**Note:**
As you'll see later, to customize the outbound and inbound animations you need to target the [`::view-transition-old()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-old) and [`::view-transition-new()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-new) pseudo-elements with your animations, respectively.

## [Creating a basic view transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#creating_a_basic_view_transition)

This section illustrates how to create a basic view transition, in both the SPA and MPA case.

### [Basic SPA view transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#basic_spa_view_transition)

An SPA may include functionality to fetch new content and update the DOM in response to an event of some kind, such as a navigation link being clicked or an update being pushed from the server.

Our [View Transitions SPA demo](https://mdn.github.io/dom-examples/view-transitions/spa/ "External link (opens in new tab)") is a basic image gallery. We have a series of [`<a>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a) elements that contain thumbnail [`<img>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img) elements, dynamically generated using JavaScript. We also have a [`<figure>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure) element containing a [`<figcaption>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figcaption) and an `<img>`, which displays the full-size gallery images.

When a thumbnail is clicked, the `displayNewImage()` function is run via [`Document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition), which causes the full-size image and its associated caption to be displayed inside the `<figure>`. We've encapsulated this inside an `updateView()` function that only calls the View Transition API if the browser supports it:

jsCopy

```
function updateView(event) {
  // Handle the difference in whether the event is fired on the <a> or the <img>
  const targetIdentifier = event.target.firstChild || event.target;

  const displayNewImage = () => {
    const mainSrc = `${targetIdentifier.src.split("_th.jpg")[0]}.jpg`;
    galleryImg.src = mainSrc;
    galleryCaption.textContent = targetIdentifier.alt;
  };

  // Fallback for browsers that don't support View Transitions:
  if (!document.startViewTransition) {
    displayNewImage();
    return;
  }

  // With View Transitions:
  const transition = document.startViewTransition(() => displayNewImage());
}
```

This code is enough to handle the transition between displayed images. Supporting browsers will show the change from old to new images and captions as a smooth cross-fade (the default view transition). It will still work in non-supporting browsers but without the nice animation.

### [Basic MPA view transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#basic_mpa_view_transition)

When creating a cross-document (MPA) view transition, the process is even simpler than for SPAs. No JavaScript is required, as the view update is triggered by a cross-document, same-origin navigation rather than a JavaScript-initiated DOM change. To enable a basic MPA view transition, you need to specify a [`@view-transition`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@view-transition) at-rule in the CSS for both the current and destination documents to opt them in, like so:

cssCopy

```
@view-transition {
  navigation: auto;
}
```

Our [View Transitions MPA demo](https://mdn.github.io/dom-examples/view-transitions/mpa/ "External link (opens in new tab)") shows this at-rule in action, and additionally demonstrates how to [customize the outbound and inbound animations](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#customizing_your_animations) of the view transition.

**Note:**
Currently MPA view transitions can only be created between same-origin documents, but this restriction may be relaxed in future implementations.

## [Customizing your animations](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#customizing_your_animations)

The View Transitions pseudo-elements have default [CSS Animations](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Animations) applied (which are detailed in their [reference pages](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API#pseudo-elements)).

Most appearance transitions are given a default smooth cross-fade animation, as mentioned above. There are some exceptions:

- `height` and `width` transitions have a smooth scaling animation applied.
- `position` and `transform` transitions have a smooth movement animation applied.

You can modify the default animations in any way you want using regular CSS — target the "from" animation with [`::view-transition-old()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-old), and the "to" animation with [`::view-transition-new()`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/::view-transition-new).

For example, to change the speed of both:

cssCopy

```
::view-transition-old(root),
::view-transition-new(root) {
  animation-duration: 0.5s;
}
```

It is recommended that you target the `::view-transition-group()` with such styles in cases where you want to apply them to `::view-transition-old()` and `::view-transition-new()`. Because of the pseudo-element hierarchy and default user-agent styling, the styles will be inherited by both. For example:

cssCopy

```
::view-transition-group(root) {
  animation-duration: 0.5s;
}
```

**Note:**
This is also a good option for safeguarding your code — `::view-transition-group()` also animates and you could end up with different durations for the `group`/`image-pair` pseudo-elements versus the `old` and `new` pseudo-elements.

In the case of cross-document (MPA) transitions, the pseudo-elements need to be included in the destination document only for the view transition to work. If you want to use the view transition in both directions, you'll need to include it in both.

Our [View Transitions MPA demo](https://mdn.github.io/dom-examples/view-transitions/mpa/ "External link (opens in new tab)") includes the above CSS, but takes the customization a step further, defining custom animations and applying them to the `::view-transition-old(root)` and `::view-transition-new(root)` pseudo-elements. The result is that the default cross-fade transition is swapped out for a "swipe up" transition when navigation occurs:

cssCopy

```
/* Create a custom animation */

@keyframes move-out {
  from {
    transform: translateY(0%);
  }

  to {
    transform: translateY(-100%);
  }
}

@keyframes move-in {
  from {
    transform: translateY(100%);
  }

  to {
    transform: translateY(0%);
  }
}

/* Apply the custom animation to the old and new page states */

::view-transition-old(root) {
  animation: 0.4s ease-in both move-out;
}

::view-transition-new(root) {
  animation: 0.4s ease-in both move-in;
}
```

## [Different animations for different elements](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#different_animations_for_different_elements)

By default, all of the different elements that change during the view update are transitioned using the same animation. If you want some elements to animate differently from the default `root` animation, you can separate them out using the [`view-transition-name`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/view-transition-name) property. For example, in our [View Transitions SPA demo](https://mdn.github.io/dom-examples/view-transitions/spa/ "External link (opens in new tab)") the [`<figcaption>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figcaption) elements are given a `view-transition-name` of `figure-caption` to separate them from the rest of the page in terms of view transitions:

cssCopy

```
figcaption {
  view-transition-name: figure-caption;
}
```

With this CSS applied, the generated pseudo-element tree will now look like this:

```
::view-transition
├─ ::view-transition-group(root)
│ └─ ::view-transition-image-pair(root)
│     ├─ ::view-transition-old(root)
│     └─ ::view-transition-new(root)
└─ ::view-transition-group(figure-caption)
  └─ ::view-transition-image-pair(figure-caption)
      ├─ ::view-transition-old(figure-caption)
      └─ ::view-transition-new(figure-caption)
```

The existence of the second set of pseudo-elements allows separate view transition styling to be applied just to the `<figcaption>`. The different old and new view captures are handled separately from one another.

The following code applies a custom animation just to the `<figcaption>`:

cssCopy

```
@keyframes grow-x {
  from {
    transform: scaleX(0);
  }
  to {
    transform: scaleX(1);
  }
}

@keyframes shrink-x {
  from {
    transform: scaleX(1);
  }
  to {
    transform: scaleX(0);
  }
}

::view-transition-group(figure-caption) {
  height: auto;
  right: 0;
  left: auto;
  transform-origin: right center;
}

::view-transition-old(figure-caption) {
  animation: 0.25s linear both shrink-x;
}

::view-transition-new(figure-caption) {
  animation: 0.25s 0.25s linear both grow-x;
}
```

Here we've created a custom CSS animation and applied it to the `::view-transition-old(figure-caption)` and `::view-transition-new(figure-caption)` pseudo-elements. We've also added a number of other styles to both to keep them in the same place and stop the default styling from interfering with our custom animations.

**Note:**
You can use `*` as the identifier in a pseudo-element to target all snapshot pseudo-elements, regardless of what name they have. For example:

cssCopy

```
::view-transition-group(*) {
  animation-duration: 2s;
}
```

### [Valid `view-transition-name` values](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#valid_view-transition-name_values)

The `view-transition-name` property can take a unique [`<custom-ident>`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/custom-ident) value, which can be any identifier that wouldn't be misinterpreted as a keyword. The value of `view-transition-name` for each rendered element must be unique. If two rendered elements have the same `view-transition-name` at the same time, [`ViewTransition.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready) will reject and the transition will be skipped.

It can also take keyword values of:

- `none`: Causes the element to not participate in a separate snapshot, unless it has a parent element with a `view-transition-name` set, in which case it will be snapshotted as part of that element.
- `match-element`: Automatically sets unique `view-transition-name` values on all selected elements.

### [Taking advantage of the default animation styles](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#taking_advantage_of_the_default_animation_styles)

Note that we also discovered another transition option that is simpler and produced a nicer result than the above. Our final `<figcaption>` view transition ended up looking like this:

cssCopy

```
figcaption {
  view-transition-name: figure-caption;
}

::view-transition-group(figure-caption) {
  height: 100%;
}
```

This works because, by default, `::view-transition-group()` transitions `width` and `height` between the old and new views with a smooth scale. We just needed to set a fixed `height` on both states to make it work.

**Note:** [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)") contains several other customization examples.

## [Controlling view transitions with JavaScript](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#controlling_view_transitions_with_javascript)

A view transition has an associated [`ViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition) object instance, which contains several promise members allowing you to run JavaScript in response to different states of the transition being reached. For example, [`ViewTransition.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready) fulfills once the pseudo-element tree is created and the animation is about to start, whereas [`ViewTransition.finished`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished) fulfills once the animation is finished, and the new page view is visible and interactive to the user.

The `ViewTransition` can be accessed like so:

1. Via the [`Document.activeViewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeViewTransition) property. This provides a consistent way to access the active view transition in any context, without having to worry about saving it for easy access later on.
2. In the case of same-document (SPA) transitions, the [`document.startViewTransition()`](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition "document.startViewTransition()") method returns the `ViewTransition` associated with the transition.
3. In the case of cross-document (MPA) transitions:
   - A [`pageswap`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageswap_event "pageswap") event is fired when a document is about to be unloaded due to a navigation. Its event object ( [`PageSwapEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent)) provides access to the `ViewTransition` via the [`PageSwapEvent.viewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent/viewTransition) property, as well as a [`NavigationActivation`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationActivation) via [`PageSwapEvent.activation`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent/activation) containing the navigation type and current and destination document history entries.

     **Note:**
     If the navigation has a cross-origin URL anywhere in the redirect chain, the `activation` property returns `null`.

   - A [`pagereveal`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pagereveal_event "pagereveal") event is fired when a document is first rendered, either when loading a fresh document from the network or activating a document (either from [back/forward cache](https://developer.mozilla.org/en-US/docs/Glossary/bfcache) (bfcache) or [prerender](https://developer.mozilla.org/en-US/docs/Glossary/Prerender)). Its event object ( [`PageRevealEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageRevealEvent)) provides access to the `ViewTransition` via the [`PageRevealEvent.viewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PageRevealEvent/viewTransition) property.

Let's have a look at some example code to show how these features could be used.

### [A JavaScript-powered custom same-document (SPA) transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#a_javascript-powered_custom_same-document_spa_transition)

The following JavaScript could be used to create a circular reveal view transition emanating from the position of the user's cursor on click, with animation provided by the [Web Animations API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API "Web Animations API").

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

### [A JavaScript-powered custom cross-document (MPA) transition](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#a_javascript-powered_custom_cross-document_mpa_transition)

The [List of Chrome DevRel team members](https://view-transitions.chrome.dev/profiles/mpa/ "External link (opens in new tab)") demo provides a basic set of team profile pages, and demonstrates how to use the [`pageswap`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageswap_event "pageswap") and [`pagereveal`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pagereveal_event "pagereveal") events to customize the outgoing and inbound animations of a cross-document view transition based on the "from" and "to" URLs.

The [`pageswap`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageswap_event "pageswap") event listener looks as follows. This sets view transition names on the elements on the outbound page that link to the profile pages. When navigating from the home page to a profile page, custom animations are provided _only_ for the linked element that is clicked in each case.

jsCopy

```
window.addEventListener("pageswap", async (e) => {
  // Only run this if an active view transition exists
  if (e.viewTransition) {
    const currentUrl = e.activation.from?.url
      ? new URL(e.activation.from.url)
      : null;
    const targetUrl = new URL(e.activation.entry.url);

    // Going from profile page to homepage
    // ~> The big img and title are the ones!
    if (isProfilePage(currentUrl) && isHomePage(targetUrl)) {
      // Set view-transition-name values on the elements to animate
      document.querySelector(`#detail main h1`).style.viewTransitionName =
        "name";
      document.querySelector(`#detail main img`).style.viewTransitionName =
        "avatar";

      // Remove view-transition-names after snapshots have been taken
      // Stops naming conflicts resulting from the page state persisting in BFCache
      await e.viewTransition.finished;
      document.querySelector(`#detail main h1`).style.viewTransitionName =
        "none";
      document.querySelector(`#detail main img`).style.viewTransitionName =
        "none";
    }

    // Going to profile page
    // ~> The clicked items are the ones!
    if (isProfilePage(targetUrl)) {
      const profile = extractProfileNameFromUrl(targetUrl);

      // Set view-transition-name values on the elements to animate
      document.querySelector(`#${profile} span`).style.viewTransitionName =
        "name";
      document.querySelector(`#${profile} img`).style.viewTransitionName =
        "avatar";

      // Remove view-transition-names after snapshots have been taken
      // Stops naming conflicts resulting from the page state persisting in BFCache
      await e.viewTransition.finished;
      document.querySelector(`#${profile} span`).style.viewTransitionName =
        "none";
      document.querySelector(`#${profile} img`).style.viewTransitionName =
        "none";
    }
  }
});
```

**Note:**
We remove the `view-transition-name` values after snapshots have been taken in each case. If we left them set, they would persist in the page state saved in the [bfcache](https://developer.mozilla.org/en-US/docs/Glossary/bfcache) upon navigation. If the back button was then pressed, the `pagereveal` event handler of the page being navigated back to would then attempt to set the same `view-transition-name` values on different elements. If multiple elements have the same `view-transition-name` set, the view transition is skipped.

The [`pagereveal`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pagereveal_event "pagereveal") event listener looks as follows. This works in a similar way to the `pageswap` event listener, although bear in mind that here we are customizing the "to" animation, for page elements on the new page.

jsCopy

```
window.addEventListener("pagereveal", async (e) => {
  // If the "from" history entry does not exist, return
  if (!navigation.activation.from) return;

  // Only run this if an active view transition exists
  if (e.viewTransition) {
    const fromUrl = new URL(navigation.activation.from.url);
    const currentUrl = new URL(navigation.activation.entry.url);

    // Went from profile page to homepage
    // ~> Set VT names on the relevant list item
    if (isProfilePage(fromUrl) && isHomePage(currentUrl)) {
      const profile = extractProfileNameFromUrl(fromUrl);

      // Set view-transition-name values on the elements to animate
      document.querySelector(`#${profile} span`).style.viewTransitionName =
        "name";
      document.querySelector(`#${profile} img`).style.viewTransitionName =
        "avatar";

      // Remove names after snapshots have been taken
      // so that we're ready for the next navigation
      await e.viewTransition.ready;
      document.querySelector(`#${profile} span`).style.viewTransitionName =
        "none";
      document.querySelector(`#${profile} img`).style.viewTransitionName =
        "none";
    }

    // Went to profile page
    // ~> Set VT names on the main title and image
    if (isProfilePage(currentUrl)) {
      // Set view-transition-name values on the elements to animate
      document.querySelector(`#detail main h1`).style.viewTransitionName =
        "name";
      document.querySelector(`#detail main img`).style.viewTransitionName =
        "avatar";

      // Remove names after snapshots have been taken
      // so that we're ready for the next navigation
      await e.viewTransition.ready;
      document.querySelector(`#detail main h1`).style.viewTransitionName =
        "none";
      document.querySelector(`#detail main img`).style.viewTransitionName =
        "none";
    }
  }
});
```

## [Stabilizing page state to make cross-document transitions consistent](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using#stabilizing_page_state_to_make_cross-document_transitions_consistent)

Before running a cross-document transition, you ideally want to wait until the state of the page stabilizes, relying on [render blocking](https://developer.mozilla.org/en-US/docs/Glossary/Render_blocking) to ensure that:

1. Critical styles are loaded and applied.
2. Critical scripts are loaded and run.
3. The HTML visible for the user's initial view of the page has been parsed, so it renders consistently.

Styles are render blocked by default unless they are added to the document dynamically, via script. Both scripts and dynamically-added styles can be render blocked using the [`blocking="render"`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script#blocking) attribute.

To ensure that your initial HTML has been parsed and will always render consistently before the transition animation runs, you can use [`<link rel="expect">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel#expect). In this element, you include the following attributes:

- `rel="expect"` to indicate that you want to use this `<link>` element to render block some HTML on the page.
- `href="#element-id"` to indicate the ID of the element you want to render block.
- `blocking="render"` to render block the specified HTML.

**Note:**
In order to block rendering, `script`, `link`, and `style` elements with `blocking="render"` must be in the `head` of the document.

Let's explore what this looks like with an example HTML document:

htmlCopy

```
<!doctype html>
<html lang="en">
  <head>
    <!-- This will be render-blocking by default -->
    <link rel="stylesheet" href="style.css" />

    <!-- Marking critical scripts as render blocking will
         ensure they're run before the view transition is activated -->
    <script async src="layout.js" blocking="render"></script>

    <!-- Use rel="expect" and blocking="render" to ensure the
         #lead-content element is visible and fully parsed before
         activating the transition -->
    <link rel="expect" href="#lead-content" blocking="render" />
  </head>
  <body>
    <h1>Page title</h1>
    <nav>...</nav>
    <div id="lead-content">
      <section id="first-section">The first section</section>
      <section>The second section</section>
    </div>
  </body>
</html>
```

The result is that document rendering is blocked until the lead content `<div>` has been parsed, ensuring a consistent view transition.

You can also specify a [`media`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#media) attribute on `<link rel="expect">` elements. For example, you might want to block rendering on a smaller amount of content when loading the page on a narrow-screen device, than on a wide-screen device. This makes sense — on a mobile, less content will be visible when the page first loads than in the case of a desktop.

This could be achieved with the following HTML:

htmlCopy

```
<link
  rel="expect"
  href="#lead-content"
  blocking="render"
  media="screen and (width > 640px)" />
<link
  rel="expect"
  href="#first-section"
  blocking="render"
  media="screen and (width <= 640px)" />
```
