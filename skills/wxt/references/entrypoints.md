# WXT Entrypoints Reference

All entrypoints live in the `entrypoints/` directory. An entrypoint is either a single file or a folder with an `index` file (zero or one level deep — deeper nesting is not supported).

## Background

File: `background.ts` or `background/index.ts`

```ts
// Minimal
export default defineBackground(() => {
  // Runs when the background is loaded
});

// With options
export default defineBackground({
  persistent: false,     // MV2: false = event page, true = persistent
  type: 'module',        // service worker module type (MV3)
  include: ['chrome'],   // only include in these browser builds
  exclude: ['firefox'],

  main() {
    // CANNOT be async
  },
});
```

For MV2 the background runs as a script on a background page; for MV3 it becomes a service worker.

---

## Popup

File: `popup.html` or `popup/index.html`

```html
<!doctype html>
<html>
  <head>
    <title>Popup Title</title>
    <!-- Sets action.default_title in manifest -->

    <!-- Manifest options as <meta> tags: -->
    <meta name="manifest.type" content="page_action" />
    <!-- ^^ MV2 only: use page_action instead of browser_action -->
    <meta name="manifest.browser_style" content="true" />
    <meta name="manifest.include" content="['chrome']" />
    <meta name="manifest.exclude" content="['firefox']" />
  </head>
  <body>
    <script type="module" src="./main.ts"></script>
  </body>
</html>
```

---

## Options Page

File: `options.html` or `options/index.html`

```html
<!doctype html>
<html>
  <head>
    <title>Options</title>
    <meta name="manifest.open_in_tab" content="true" />
    <meta name="manifest.chrome_style" content="false" />
    <meta name="manifest.browser_style" content="false" />
    <meta name="manifest.include" content="['chrome', 'firefox']" />
  </head>
  <body>
    <script type="module" src="./main.ts"></script>
  </body>
</html>
```

---

## Side Panel

File: `sidepanel.html` or `sidepanel/index.html` (or `{name}.sidepanel.html` for named panels)

```html
<!doctype html>
<html>
  <head>
    <title>Side Panel</title>
    <meta name="manifest.open_at_install" content="false" />
    <meta name="manifest.browser_style" content="false" />
    <meta name="manifest.include" content="['chrome', 'firefox']" />
  </head>
  <body>
    <script type="module" src="./main.ts"></script>
  </body>
</html>
```

WXT automatically adds the `sidepanel` permission. Chrome uses `side_panel` API; Firefox uses `sidebar_action` API.

---

## Content Scripts

File: `content.ts`, `content/index.ts`, `{name}.content.ts`, or `{name}.content/index.ts`

```ts
export default defineContentScript({
  // Manifest options:
  matches: ['*://*.example.com/*', '<all_urls>'],
  excludeMatches: ['*://*.example.com/admin/*'],
  includeGlobs: ['*://*.example.com/app*'],
  excludeGlobs: [],
  allFrames: false,
  runAt: 'document_idle',  // 'document_start' | 'document_end' | 'document_idle'
  matchAboutBlank: false,
  matchOriginAsFallback: false,
  world: 'ISOLATED',       // 'ISOLATED' | 'MAIN'
  cssInjectionMode: 'manifest',  // 'manifest' | 'manual' | 'ui'
  registration: 'manifest',      // 'manifest' | 'runtime'

  // Build filtering:
  include: ['chrome', 'firefox'],
  exclude: ['safari'],

  main(ctx) {
    // ctx = ContentScriptContext, tracks invalidation
  },
});
```

---

## Newtab Override

File: `newtab.html` or `newtab/index.html`

Overrides the browser's new tab page.

```html
<!doctype html>
<html>
  <head>
    <title>New Tab</title>
    <meta name="manifest.include" content="['chrome']" />
  </head>
  <body>
    <script type="module" src="./main.ts"></script>
  </body>
</html>
```

---

## History Override

File: `history.html` or `history/index.html`

Overrides the browser's history page.

---

## Bookmarks Override

File: `bookmarks.html` or `bookmarks/index.html`

Overrides the browser's bookmarks page.

---

## Devtools

File: `devtools.html` or `devtools/index.html`

```html
<!doctype html>
<html>
  <head>
    <meta name="manifest.include" content="['chrome']" />
  </head>
  <body></body>
</html>
```

Use the [Devtools Example](https://github.com/wxt-dev/examples/tree/main/examples/devtools-extension) to add panels and panes.

---

## Sandbox (Chromium only)

File: `sandbox.html`, `sandbox/index.html`, `{name}.sandbox.html`, or `{name}.sandbox/index.html`

Firefox does not support sandboxed pages.

---

## Unlisted Pages

Any `.html` file not matching a known entrypoint name becomes an unlisted page (not in the manifest, but accessible at runtime):

```ts
const url = browser.runtime.getURL('/welcome.html');
window.open(url); // open in new tab
```

---

## Unlisted Scripts

Any `.ts/.js/.tsx/.jsx` file not matching a known entrypoint name becomes an unlisted script:

```ts
// entrypoints/injected.ts
export default defineUnlistedScript(() => {
  // Injected into pages via chrome.scripting or content script
});
```

```ts
// Getting the URL to inject it:
const url = browser.runtime.getURL('/injected.js');
```

Add to `web_accessible_resources` if the page needs to load it.

---

## Unlisted CSS

Any `.css/.scss/.sass/.less` file in `entrypoints/` becomes an unlisted CSS file. To add CSS to a content script, import it from the content script's JS/TS entrypoint file instead.
