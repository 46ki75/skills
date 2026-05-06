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
  author: "Copilot"
  version: "1.0"
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
  persistent: false,  // MV2 only
  type: 'module',
  main() { /* ... */ },
});
```

### Content Script

```ts
// entrypoints/content.ts  OR  entrypoints/my-feature.content.ts
export default defineContentScript({
  matches: ['*://*.example.com/*'],
  runAt: 'document_idle',       // 'document_start' | 'document_end' | 'document_idle'
  world: 'ISOLATED',            // or 'MAIN' for main world access
  cssInjectionMode: 'manifest', // 'manifest' | 'manual' | 'ui'

  main(ctx) {
    // ctx tracks context invalidation
    ctx.addEventListener(window, 'resize', handler);
    ctx.setInterval(() => { /* ... */ }, 1000);
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
  <body>...</body>
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
import { storage, createShadowRootUi } from '#imports';
import { browser } from 'wxt/browser';
```

Run `wxt prepare` (or `pnpm postinstall`) to regenerate the `.wxt/types/imports-module.d.ts` type declarations after adding files.

## Extension APIs

WXT provides a unified `browser` variable that works across all browsers:

```ts
// Works in Chrome (uses chrome.*) and Firefox (uses browser.*)
browser.storage.local.set({ key: 'value' });
browser.runtime.onMessage.addListener((msg, sender) => { /* ... */ });
```

For feature detection (don't rely on types — they assume all APIs exist):

```ts
browser.runtime.onSuspend?.addListener(() => { /* ... */ });
```

## Manifest Configuration

No `manifest.json` in source — WXT generates it from `wxt.config.ts` and entrypoint options:

```ts
// wxt.config.ts
import { defineConfig } from 'wxt';

export default defineConfig({
  manifest: {
    name: 'My Extension',
    permissions: ['storage', 'tabs'],
    host_permissions: ['https://example.com/*'],
    action: { default_title: 'My Extension' },
  },
});
```

Dynamic manifest based on target:

```ts
export default defineConfig({
  manifest: ({ browser, manifestVersion }) => ({
    permissions: browser === 'firefox'
      ? ['storage', 'webRequest']
      : ['storage', 'declarativeNetRequest'],
  }),
});
```

**MV2/MV3**: Always write manifest in MV3 format — WXT auto-converts to MV2 when targeting Firefox/Safari. For example, define `action` (not `browser_action`); WXT handles the conversion.

Icons: place `icon-16.png`, `icon-48.png`, `icon-128.png` in `public/` and WXT discovers them automatically.

## Storage

WXT ships a built-in storage wrapper. All keys must be prefixed with the storage area:

```ts
// Quick access (needs 'storage' permission in manifest)
await storage.getItem<string>('local:username');
await storage.setItem('local:username', 'alice');
await storage.removeItem('local:username');

// Reactive watcher
const unwatch = storage.watch<string>('local:username', (newVal, oldVal) => {});
unwatch(); // stop watching
```

**Recommended: define typed storage items** in `utils/`:

```ts
// utils/settings.ts
export const darkMode = storage.defineItem<boolean>('local:darkMode', {
  fallback: false,
});

// Usage anywhere (auto-imported):
const isDark = await darkMode.getValue();
await darkMode.setValue(true);
darkMode.watch((val) => console.log('theme changed:', val));
```

Versioned storage for schema migrations:

```ts
export const prefs = storage.defineItem<PrefsV2>('local:prefs', {
  version: 2,
  fallback: defaultPrefs,
  migrations: {
    2: (oldPrefs: PrefsV1): PrefsV2 => ({ ...oldPrefs, newField: 'default' }),
  },
});
```

Add `'storage'` to `manifest.permissions` in `wxt.config.ts`.

## Content Script UIs

For rendering UI components onto a page, WXT provides three strategies. See `references/content-scripts.md` for full code examples with each framework.

| Method | Isolated CSS | Isolated Events | HMR | Use page context |
| ------ | :---: | :---: | :---: | :---: |
| Integrated (`createIntegratedUi`) | ❌ | ❌ | ❌ | ✅ |
| Shadow Root (`createShadowRootUi`) | ✅ | ✅ (opt-in) | ❌ | ✅ |
| IFrame (`createIframeUi`) | ✅ | ✅ | ✅ | ❌ |

**Shadow Root** is the most commonly used — it isolates your extension's styles from the page:

```ts
// entrypoints/overlay.content/index.ts
import './style.css';

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui',  // required for shadow root

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'my-overlay',
      position: 'inline',
      anchor: 'body',
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
import { defineConfig } from 'wxt';
export default defineConfig({ modules: ['@wxt-dev/module-react'] });

// Vue
export default defineConfig({ modules: ['@wxt-dev/module-vue'] });

// Svelte
export default defineConfig({ modules: ['@wxt-dev/module-svelte'] });

// Solid
export default defineConfig({ modules: ['@wxt-dev/module-solid'] });
```

Each popup/options/sidepanel entrypoint needs its own app instance. Use a directory entrypoint with an `index.html` and framework-specific `main.tsx/ts`.

**Router note**: Web extension pages can't use path-based routing. Configure your router to use **hash mode** (e.g., `createHashRouter` for React Router, `createWebHashHistory()` for Vue Router).

## Multi-Browser Targeting

```sh
wxt -b firefox        # dev mode for Firefox
wxt build -b safari   # build for Safari
wxt -b chrome --mv2   # Chrome MV2
```

Runtime browser detection:

```ts
if (import.meta.env.BROWSER === 'firefox') { /* ... */ }
if (import.meta.env.FIREFOX) { /* shorthand */ }
if (import.meta.env.MANIFEST_VERSION === 2) { /* ... */ }
```

Per-entrypoint filtering:

```ts
export default defineContentScript({
  include: ['firefox'],  // only built for Firefox
  matches: ['*://*/*'],
  main(ctx) { /* ... */ },
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
