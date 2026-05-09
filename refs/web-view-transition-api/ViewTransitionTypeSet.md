---
url: https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet
---

# ViewTransitionTypeSet

Baseline

2026

Newly available

Since January 2026, this feature works across the latest devices and browser versions. This feature might not work in older devices or browsers.

- [Learn more](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility)
- [See full compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#browser_compatibility)
- [Report feedback](https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FAPI%2FViewTransitionTypeSet&level=low)

The **`ViewTransitionTypeSet`** interface of the [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API "View Transition API") is a [set-like object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set#set-like_browser_apis) representing the types of an active view transition. This enables the types to be queried or modified on-the-fly during a transition.

The `ViewTransitionTypeSet` object can be accessed via the [`ViewTransition.types`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types) property.

The property and method links below link to the JavaScript [`Set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set) object documentation.

## In this article

- [Instance properties](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#instance_properties)
- [Instance methods](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#instance_methods)
- [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#examples)
- [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#specifications)
- [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#browser_compatibility)
- [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#see_also)

## [Instance properties](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#instance_properties)

[`Set.prototype.size`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/size)

Returns the number of values in the set.

## [Instance methods](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#instance_methods)

[`Set.prototype.add`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/add)

Inserts the specified value into this set, if it is not already present.

[`Set.prototype.clear()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/clear)

Removes all values form the set.

[`Set.prototype.delete()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/delete)

Removes the specified value from this set, if it is in the set.

[`Set.prototype.entries()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/entries)

Returns a new iterator object that contains **an array of `[value, value]`** for each element in the set, in insertion order.

[`Set.prototype.forEach()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/forEach)

Calls a callback function once for each value present in the set, in insertion order.

[`Set.prototype.has()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/has)

Returns a boolean indicating whether the specified value exists in the set.

[`Set.prototype.keys()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/keys)

An alias for [`Set.prototype.values()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values).

[`Set.prototype.values()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values)

Returns a new iterator object that yields the **values** for each element in the set, in insertion order.

[`Set.prototype[Symbol.iterator]()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/Symbol.iterator)

Returns a new iterator object that yields the **values** for each element in the set, in insertion order.

## [Examples](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#examples)

jsCopy

```
// Start a view transition
const vt = document.startViewTransition({
  update: updateTheDOMSomehow,
  types: ["slideLeft", "fadeOut", "flipVertical"],
});

// Add another type
vt.types.add("flipHorizontal");

// Delete a type
vt.types.delete("flipVertical");

// Return the number of types in the set
console.log(vt.types.size);

// Log each type to the console
vt.types.forEach((type) => console.log(type));
```

## [Specifications](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#specifications)

| Specification                                                                                                                              |
| ------------------------------------------------------------------------------------------------------------------------------------------ |
| [CSS View Transitions Module Level 2\<br>\# viewtransitiontypeset](https://drafts.csswg.org/css-view-transitions-2/#viewtransitiontypeset) |

## [Browser compatibility](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#browser_compatibility)

[Report problems with this compatibility data](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet# "Report an issue with this compatibility data") •
[View data on GitHub](https://github.com/mdn/browser-compat-data/tree/main/api/ViewTransitionTypeSet.json "File: api/ViewTransitionTypeSet.json")

|                         | desktop                            | mobile                         |
| ----------------------- | ---------------------------------- | ------------------------------ | ------------------------------------ | -------------------------------- | ----------------------------------- | -------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- | ------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------- |
|                         | Chrome                             | Edge                           | Firefox                              | Opera                            | Safari                              | Chrome Android                                     | Firefox for Android                                          | Opera Android                                   | Safari on iOS                                     | Samsung Internet                                      | WebView Android                                      | WebView on iOS                                      |
| ---                     | ---                                | ---                            | ---                                  | ---                              | ---                                 | ---                                                | ---                                                          | ---                                             | ---                                               | ---                                                   | ---                                                  | ---                                                 |
| `ViewTransitionTypeSet` | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| \[Symbol.iterator\]     | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `add`                   | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `clear`                 | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `delete`                | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `entries`               | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `forEach`               | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `has`                   | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `keys`                  | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `size`                  | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |
| `values`                | Chrome – Full support<br>Chrome125 | Edge – Full support<br>Edge125 | Firefox – Full support<br>Firefox147 | Opera – Full support<br>Opera111 | Safari – Full support<br>Safari18.2 | Chrome Android – Full support<br>Chrome Android125 | Firefox for Android – Full support<br>Firefox for Android147 | Opera Android – Full support<br>Opera Android83 | Safari on iOS – Full support<br>Safari on iOS18.2 | Samsung Internet – Full support<br>Samsung Internet27 | WebView Android – Full support<br>WebView Android125 | WebView on iOS – Full support<br>WebView on iOS18.2 |

### Legend

Tip: you can click/tap on a cell for more information.

Full supportFull support

## [See also](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet#see_also)

- [`ViewTransition.types`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/types)
- [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API)
- [Using the View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using)
- [Using view transition types](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API/Using_types)
- [Smooth transitions with the View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/ "External link (opens in new tab)")
