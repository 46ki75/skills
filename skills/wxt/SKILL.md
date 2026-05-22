---
name: wxt
description: >
  Expert guidance for building browser extensions with WXT — covering project
  setup, file-based entrypoints (background, content scripts, popup, options,
  side panel), the unified browser API, auto-imports, manifest configuration,
  storage, content script UIs (integrated, shadow root, iframe), frontend
  frameworks (React, Vue, Svelte, Solid), multi-browser targeting (Chrome,
  Firefox, Safari, Edge), MV2/MV3 compatibility, and publishing. Use this
  skill whenever someone is building, debugging, or reviewing a WXT extension,
  asking how entrypoints work, trying to add a popup or content script, using
  `defineBackground`, `defineContentScript`, `createShadowRootUi`, `storage`,
  `wxt.config.ts`, or wondering why their extension code breaks at build time.
  Always invoke this skill for any question that mentions WXT, wxt.config.ts,
  defineBackground, defineContentScript, or refers to building a Chrome/Firefox
  extension using WXT, even if the question seems simple.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.1.0"
---

# WXT Browser Extension Skill

You are an expert in WXT, the modern framework for building cross-browser web extensions. Your goal is to help users write correct, idiomatic WXT code that works across Chrome, Firefox, Edge, and Safari.

## What WXT is

WXT (inspired by Nuxt) is a build framework for web extensions that provides:

- **File-based entrypoints** — your folder structure drives the manifest
- **Auto-imports** — WXT APIs and project utils are available without imports
- **Cross-browser builds** — one codebase for Chrome, Firefox, Safari, Edge
- **MV2 + MV3 support** — build for both manifest versions from the same source
- **Fast dev mode** — HMR for UI, fast reloads for scripts
- **TypeScript by default**

## Project Setup

Bootstrap a new project:

```sh
pnpm dlx wxt@latest init   # also works with npx/bunx
```

Templates: Vanilla, Vue, React, Svelte, Solid (all TypeScript by default).

Recommended `package.json` scripts:

```json
{
  "scripts": {
    "dev": "wxt",
    "dev:firefox": "wxt -b firefox",
    "build": "wxt build",
    "build:firefox": "wxt build -b firefox",
    "zip": "wxt zip",
    "zip:firefox": "wxt zip -b firefox",
    "postinstall": "wxt prepare"
  }
}
```

## Project Structure

```text
📂 project-root/
   📁 .output/          ← build artifacts (gitignore this)
   📁 .wxt/             ← generated TS config (gitignore this)
   📁 assets/           ← CSS, images processed by Vite
   📁 components/       ← auto-imported UI components
   📁 composables/      ← auto-imported Vue composables
   📁 entrypoints/      ← ⭐ all extension entrypoints go here
   📁 hooks/            ← auto-imported React/Solid hooks
   📁 modules/          ← local WXT modules
   📁 public/           ← static files copied as-is (icons, etc.)
   📁 utils/            ← auto-imported utilities
   📄 wxt.config.ts     ← main config
   📄 web-ext.config.ts ← browser startup config
```

To use a `src/` directory, set `srcDir: 'src'` in `wxt.config.ts`.

## Entrypoints

The `entrypoints/` directory is the heart of WXT. File names determine entrypoint type. Each entrypoint is either a single file or a directory with an `index` file.

**Critical rule**: Never put code that uses browser APIs (`browser.*`, `chrome.*`, DOM APIs) outside the
`main()` function. WXT imports entrypoint files in a Node.js environment during build, so top-level
extension API calls will fail with errors like `Browser.action.onClicked.addListener not implemented`.

### Background Script

```ts
// entrypoints/background.ts
export default defineBackground(() => {
  browser.action.onClicked.addListener(() => {
    // ✅ browser.* is safe here inside main()
  });
});

// With manifest options:
export default defineBackground({
  persistent: false, // MV2 only
  type: "module",
  main() {
    /* ... */
  },
});
```

### Content Script

```ts
// entrypoints/content.ts  OR  entrypoints/my-feature.content.ts
export default defineContentScript({
  matches: ["*://*.example.com/*"],
  runAt: "document_idle", // 'document_start' | 'document_end' | 'document_idle'
  world: "ISOLATED", // or 'MAIN' for main world access
  cssInjectionMode: "manifest", // 'manifest' | 'manual' | 'ui'

  main(ctx) {
    // ctx tracks context invalidation
    ctx.addEventListener(window, "resize", handler);
    ctx.setInterval(() => {
      /* ... */
    }, 1000);
  },
});
```

Multiple content scripts: name them `foo.content.ts`, `bar.content.ts`.

### Popup

```html
<!-- entrypoints/popup.html or entrypoints/popup/index.html -->
<!doctype html>
<html>
  <head>
    <title>Extension Popup</title>
    <!-- For MV2 page_action instead of browser_action: -->
    <!-- <meta name="manifest.type" content="page_action" /> -->
  </head>
  <body>
    <script type="module" src="./main.ts"></script>
  </body>
</html>
```

### Options Page

```html
<!-- entrypoints/options.html -->
<!doctype html>
<html>
  <head>
    <meta name="manifest.open_in_tab" content="true" />
  </head>
  <body>
    ...
  </body>
</html>
```

### Side Panel

```html
<!-- entrypoints/sidepanel.html -->
<!-- Chrome uses side_panel API; Firefox uses sidebar_action -->
<!-- WXT adds the sidepanel permission automatically -->
```

### Other Entrypoints

See `references/entrypoints.md` for full details on: Newtab, History, Bookmarks, Devtools, Sandbox, Unlisted Pages, Unlisted Scripts, Unlisted CSS.

## Auto-imports

WXT sets up auto-imports (like Nuxt) for:

- All WXT APIs: `defineBackground`, `defineContentScript`, `browser`, `storage`, `createShadowRootUi`, etc.
- Files in `components/`, `composables/`, `hooks/`, `utils/`

You can use these without importing them. When auto-imports are disabled or you prefer explicit imports, use:

```ts
import { storage, createShadowRootUi } from "#imports";
import { browser } from "wxt/browser";
```

Run `wxt prepare` (or `pnpm postinstall`) to regenerate the `.wxt/types/imports-module.d.ts` type declarations after adding files.

## Extension APIs

WXT provides a unified `browser` variable that works across all browsers:

```ts
// Works in Chrome (uses chrome.*) and Firefox (uses browser.*)
browser.storage.local.set({ key: "value" });
browser.runtime.onMessage.addListener((msg, sender) => {
  /* ... */
});
```

For feature detection (don't rely on types — they assume all APIs exist):

```ts
browser.runtime.onSuspend?.addListener(() => {
  /* ... */
});
```

## Manifest Configuration

No `manifest.json` in source — WXT generates it from `wxt.config.ts` and entrypoint options:

```ts
// wxt.config.ts
import { defineConfig } from "wxt";

export default defineConfig({
  manifest: {
    name: "My Extension",
    permissions: ["storage", "tabs"],
    host_permissions: ["https://example.com/*"],
    action: { default_title: "My Extension" },
  },
});
```

Dynamic manifest based on target:

```ts
export default defineConfig({
  manifest: ({ browser, manifestVersion }) => ({
    permissions:
      browser === "firefox"
        ? ["storage", "webRequest"]
        : ["storage", "declarativeNetRequest"],
  }),
});
```

**MV2/MV3**: Always write manifest in MV3 format — WXT auto-converts to MV2 when targeting Firefox/Safari. For example, define `action` (not `browser_action`); WXT handles the conversion.

Icons: place `icon-16.png`, `icon-48.png`, `icon-128.png` in `public/` and WXT discovers them automatically.

## Storage

WXT ships a built-in storage wrapper. All keys must be prefixed with the storage area:

```ts
// Quick access (needs 'storage' permission in manifest)
await storage.getItem<string>("local:username");
await storage.setItem("local:username", "alice");
await storage.removeItem("local:username");

// Reactive watcher
const unwatch = storage.watch<string>("local:username", (newVal, oldVal) => {});
unwatch(); // stop watching
```

**Recommended: define typed storage items** in `utils/`:

```ts
// utils/settings.ts
export const darkMode = storage.defineItem<boolean>("local:darkMode", {
  fallback: false,
});

// Usage anywhere (auto-imported):
const isDark = await darkMode.getValue();
await darkMode.setValue(true);
darkMode.watch((val) => console.log("theme changed:", val));
```

Versioned storage for schema migrations:

```ts
export const prefs = storage.defineItem<PrefsV2>("local:prefs", {
  version: 2,
  fallback: defaultPrefs,
  migrations: {
    2: (oldPrefs: PrefsV1): PrefsV2 => ({ ...oldPrefs, newField: "default" }),
  },
});
```

Add `'storage'` to `manifest.permissions` in `wxt.config.ts`.

## Content Script UIs

For rendering UI components onto a page, WXT provides three strategies. See `references/content-scripts.md` for full code examples with each framework.

| Method                             | Isolated CSS | Isolated Events | HMR | Use page context |
| ---------------------------------- | :----------: | :-------------: | :-: | :--------------: |
| Integrated (`createIntegratedUi`)  |      ❌      |       ❌        | ❌  |        ✅        |
| Shadow Root (`createShadowRootUi`) |      ✅      |   ✅ (opt-in)   | ❌  |        ✅        |
| IFrame (`createIframeUi`)          |      ✅      |       ✅        | ✅  |        ❌        |

**Shadow Root** is the most commonly used — it isolates your extension's styles from the page:

```ts
// entrypoints/overlay.content/index.ts
import "./style.css";

export default defineContentScript({
  matches: ["<all_urls>"],
  cssInjectionMode: "ui", // required for shadow root

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: "my-overlay",
      position: "inline",
      anchor: "body",
      onMount(container) {
        // mount your framework app here
      },
    });
    ui.mount();
  },
});
```

## Frontend Frameworks

Install a module and add it to `wxt.config.ts`:

```ts
// React
import { defineConfig } from "wxt";
export default defineConfig({ modules: ["@wxt-dev/module-react"] });

// Vue
export default defineConfig({ modules: ["@wxt-dev/module-vue"] });

// Svelte
export default defineConfig({ modules: ["@wxt-dev/module-svelte"] });

// Solid
export default defineConfig({ modules: ["@wxt-dev/module-solid"] });
```

Each popup/options/sidepanel entrypoint needs its own app instance. Use a directory entrypoint with an `index.html` and framework-specific `main.tsx/ts`.

**Router note**: Web extension pages can't use path-based routing. Configure your router to use **hash mode** (e.g., `createHashRouter` for React Router, `createWebHashHistory()` for Vue Router).

**Qwik in CSR-only mode**: a `render(root, <App/>)` setup (no SSR) does *not* auto-inject qwikloader, and without it every `onClick$` / `onChange$`
listener is silently dead — only `useVisibleTask$` fires. Copy `node_modules/@builder.io/qwik/dist/qwikloader.js` into `public/` and add
`<script src="/qwikloader.js"></script>` before the module script in each Qwik entrypoint's `index.html`. You can't `import` the loader
(Qwik's `"sideEffects": false` lets Vite tree-shake it) and you can't `new Function(source)()` it (MV3 CSP forbids `unsafe-eval`). The Qwik
skill's "CSR-only deployment" section has the full diagnosis.

## MV3 Content Security Policy gotchas

MV3's default extension CSP is `script-src 'self'; object-src 'self'` — strictly **no `unsafe-eval`**. This silently breaks any code that does runtime code generation:

- `new Function(source)()` — throws `EvalError: ... unsafe-eval`
- `eval(...)` — same
- Libraries that compile templates at runtime (older Vue runtime-compiler builds, some chart engines)
- "Late-loading" tricks where a string is fetched then evaluated

When you need a side-effect-only script that you'd normally import from `node_modules`, ship it as a real file under `public/` and pull it in via a
non-module `<script src=>` from the entrypoint HTML. Vite copies `public/` into the output unchanged. Don't rely on `<script>` tag injection from
JS either — MV3 also blocks `eval`-style script element creation for inline content.

## E2E Testing with Playwright

Driving an unpacked extension over CDP has several non-obvious gotchas. The working pattern, which any reliable `pnpm test:e2e`-style harness converges on:

### Use Playwright's bundled chromium, not Google Chrome stable

Recent Google Chrome stable releases (≳ v148) silently ignore `--load-extension` for unpacked extensions in headless mode — the flag is accepted,
the extension never loads, `chrome://extensions` shows an empty list, and your test sees `ERR_BLOCKED_BY_CLIENT` or
`chrome-error://chromewebdata/`. Playwright's bundled chromium loads them fine:

```ts
const chromePath =
  process.env.E2E_CHROMIUM_PATH ||
  "/home/<you>/.cache/ms-playwright/chromium-1200/chrome-linux64/chrome";
```

`chromium.executablePath()` is brittle here too — it hardcodes the path for whichever browser version Playwright thinks it should have,
which may not be the one actually downloaded. Allow override via env var.

### Compute the extension ID locally — don't wait for the SW

Chrome derives an unpacked extension's ID **deterministically from the absolute path** of the load directory. No need to wait for the MV3
service worker to register as a CDP target (it's lazy and doesn't appear until something wakes it):

```ts
import crypto from "node:crypto";

const computeExtensionId = (absPath: string): string => {
  const hex = crypto
    .createHash("sha256")
    .update(absPath)
    .digest("hex")
    .slice(0, 32);
  const a = "a".charCodeAt(0);
  return Array.from(hex, (c) =>
    String.fromCharCode(a + parseInt(c, 16)),
  ).join("");
};
```

This is far more reliable than scraping the SW URL or the `chrome://extensions` DOM.

### Launch shape: spawn + connectOverCDP, not launchPersistentContext

`chromium.launchPersistentContext()` is finicky with extensions + headless mode (the SW often fails to register and event subscriptions to it
never fire, even with `--headless=new`). Spawn Chrome directly and attach over CDP:

```ts
const proc = spawn(chromePath, [
  "--headless=new",
  "--disable-gpu",
  "--no-sandbox",
  "--no-first-run",
  "--no-default-browser-check",
  `--user-data-dir=${userDataDir}`,
  `--remote-debugging-port=${port}`,
  `--disable-extensions-except=${EXT_PATH}`,
  `--load-extension=${EXT_PATH}`,
]);
// poll http://127.0.0.1:${port}/json/version until it answers, then:
const browser = await chromium.connectOverCDP(`http://127.0.0.1:${port}`);
const ctx = browser.contexts()[0]!;
```

Always pass `--disable-extensions-except` together with `--load-extension`. Without it, Chrome also loads its bundled extensions (Hangouts,
Cast, etc.) and they pollute `ctx.serviceWorkers()` and the target list, making "find our SW" heuristics unreliable.

### Open `chrome-extension://` URLs via raw CDP, not `page.goto`

After `chromium.connectOverCDP()`, calling `page.goto("chrome-extension://<id>/popup.html")` on an existing tab fails with
`ERR_BLOCKED_BY_CLIENT` — Playwright's CDP attach refuses the scheme. Spawn the popup as a brand-new CDP target via the JSON HTTP endpoint
instead, and attach via the `page` event:

```ts
const evt = ctx.waitForEvent("page", { timeout: 10_000 });
await fetch(`${cdpUrl}/json/new?${encodeURI(popupUrl)}`, { method: "PUT" });
const page = await evt;
```

The `PUT` verb is required — Chrome rejects `GET` on `/json/new` with "Using unsafe HTTP verb GET to invoke /json/new. This action supports only PUT verb."

### Debugging persistence: read storage directly

When persistence is suspect (state appears to update but doesn't survive reload), read `chrome.storage.local` from inside the page to confirm
what was actually written, rather than just asserting the DOM:

```ts
const stored = await page.evaluate(
  () =>
    new Promise<Record<string, unknown>>((r) =>
      chrome.storage.local.get(null, r),
    ),
);
```

An empty object here when you expected a value usually means the framework's change handler didn't fire — and in CSR-mode Qwik that's the qwikloader symptom described above.

## Multi-Browser Targeting

```sh
wxt -b firefox        # dev mode for Firefox
wxt build -b safari   # build for Safari
wxt -b chrome --mv2   # Chrome MV2
```

Runtime browser detection:

```ts
if (import.meta.env.BROWSER === "firefox") {
  /* ... */
}
if (import.meta.env.FIREFOX) {
  /* shorthand */
}
if (import.meta.env.MANIFEST_VERSION === 2) {
  /* ... */
}
```

Per-entrypoint filtering:

```ts
export default defineContentScript({
  include: ["firefox"], // only built for Firefox
  matches: ["*://*/*"],
  main(ctx) {
    /* ... */
  },
});
```

## Common Pitfalls

**Extension API calls at top level** — the most common mistake:

```ts
// ❌ Breaks at build time — WXT imports this file in Node.js
browser.action.onClicked.addListener(() => {});

// ✅ Always wrap in the main() function
export default defineBackground(() => {
  browser.action.onClicked.addListener(() => {});
});
```

**Deeply nested entrypoints** — WXT only discovers 0–1 levels deep:

```text
entrypoints/
  youtube/content/index.ts   ❌ not discovered
  youtube.content/index.ts   ✅ correct
```

**Related files inside entrypoints/** — put them in a directory entrypoint:

```text
entrypoints/
  popup.ts    ❌ also don't put popup.css next to it
  popup/      ✅ use a folder instead
    index.html
    main.ts
    style.css
```

## Reference Files

Read these when you need deeper details:

- **`references/entrypoints.md`** — complete list of all entrypoint types with code templates (Newtab, History, Bookmarks, Devtools, Sandbox, Unlisted Pages/Scripts/CSS, Popup, Options, Side Panel)
- **`references/content-scripts.md`** — content script context API, CSS injection, and all three UI strategies (Integrated, Shadow Root, IFrame) with per-framework code examples
- **`references/storage.md`** — full storage API: bulk operations, metadata, versioned migrations, `defineItem` patterns
