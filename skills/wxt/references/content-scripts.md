# Content Script UI Reference

WXT provides three ways to inject UI into web pages from content scripts.

## Choosing the Right Method

| Method | Isolated CSS | Isolated Events | HMR | Use page context |
| ------ | :---: | :---: | :---: | :---: |
| Integrated (`createIntegratedUi`) | ❌ | ❌ | ❌ | ✅ |
| Shadow Root (`createShadowRootUi`) | ✅ | opt-in | ❌ | ✅ |
| IFrame (`createIframeUi`) | ✅ | ✅ | ✅ | ❌ |

**Shadow Root** is the most common choice: CSS is isolated, the UI lives in the page's context (can access page variables), and setup is straightforward.

Use **IFrame** when you need full CSS/event isolation and HMR during development (the iframe loads a separate extension page).

Use **Integrated** when you intentionally want the page's styles to apply to your UI.

---

## Content Script Context (`ctx`)

The first argument to `main()` is a `ContentScriptContext` that tracks whether the content script is
still valid. When an extension updates, disables, or the user navigates away, the context becomes
"invalidated". Use `ctx` wrappers instead of raw browser APIs to avoid "Extension context invalidated"
errors:

```ts
ctx.addEventListener(window, 'scroll', handler);  // auto-removed on invalidation
ctx.setTimeout(() => { /* ... */ }, 1000);
ctx.setInterval(() => { /* ... */ }, 5000);
ctx.requestAnimationFrame(() => { /* ... */ });

// Manual check:
if (ctx.isValid) { /* ... */ }
if (ctx.isInvalid) { return; }
```

---

## Integrated UI

Injected directly into the page's DOM. Affected by the page's CSS — good when you want your UI to blend in.

### Vanilla

```ts
// entrypoints/example-ui.content.ts
export default defineContentScript({
  matches: ['<all_urls>'],
  main(ctx) {
    const ui = createIntegratedUi(ctx, {
      position: 'inline',   // 'inline' | 'overlay' | 'modal'
      anchor: 'body',       // CSS selector or element
      onMount(container) {
        const el = document.createElement('p');
        el.textContent = 'Hello!';
        container.append(el);
      },
    });
    ui.mount();
  },
});
```

### React

```tsx
import ReactDOM from 'react-dom/client';
import App from './App.tsx';

export default defineContentScript({
  matches: ['<all_urls>'],
  main(ctx) {
    const ui = createIntegratedUi(ctx, {
      position: 'inline',
      anchor: 'body',
      onMount(container) {
        const root = ReactDOM.createRoot(container);
        root.render(<App />);
        return root;
      },
      onRemove(root) {
        root.unmount();
      },
    });
    ui.mount();
  },
});
```

### Vue

```ts
// entrypoints/example-ui.content/index.ts
import { createApp } from 'vue';
import App from './App.vue';

export default defineContentScript({
  matches: ['<all_urls>'],
  main(ctx) {
    const ui = createIntegratedUi(ctx, {
      position: 'inline',
      anchor: 'body',
      onMount(container) {
        const app = createApp(App);
        app.mount(container);
        return app;
      },
      onRemove(app) {
        app.unmount();
      },
    });
    ui.mount();
  },
});
```

### Svelte

```ts
import App from './App.svelte';
import { mount, unmount } from 'svelte';

export default defineContentScript({
  matches: ['<all_urls>'],
  main(ctx) {
    const ui = createIntegratedUi(ctx, {
      position: 'inline',
      anchor: 'body',
      onMount(container) { return mount(App, { target: container }); },
      onRemove(app) { unmount(app); },
    });
    ui.mount();
  },
});
```

---

## Shadow Root UI

CSS is scoped inside a Shadow DOM, so the page's styles don't leak in and your styles don't leak out. This is the recommended approach for most extensions.

### Steps

1. Import your CSS file at the top of the content script
2. Set `cssInjectionMode: 'ui'` inside `defineContentScript`
3. Define the UI with `createShadowRootUi()` (it's async)
4. Call `ui.mount()`

### Vanilla

```ts
// entrypoints/overlay.content/index.ts
import './style.css';  // Step 1

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui',  // Step 2

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {  // Step 3 (async!)
      name: 'my-overlay',
      position: 'inline',
      anchor: 'body',
      onMount(container) {
        const el = document.createElement('p');
        el.textContent = 'Hello!';
        container.append(el);
      },
    });
    ui.mount();  // Step 4
  },
});
```

### React

```tsx
import './style.css';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui',

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'my-overlay',
      position: 'inline',
      anchor: 'body',
      onMount(container) {
        // React warns when mounting directly to body — use a wrapper div
        const wrapper = document.createElement('div');
        container.append(wrapper);
        const root = ReactDOM.createRoot(wrapper);
        root.render(<App />);
        return root;
      },
      onRemove(root) {
        root?.unmount();
      },
    });
    ui.mount();
  },
});
```

### Vue

```ts
import './style.css';
import { createApp } from 'vue';
import App from './App.vue';

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui',

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'my-overlay',
      position: 'inline',
      anchor: 'body',
      onMount(container) {
        const app = createApp(App);
        app.mount(container);
        return app;
      },
      onRemove(app) { app?.unmount(); },
    });
    ui.mount();
  },
});
```

---

## IFrame UI

Renders a separate extension HTML page inside an iframe. Full CSS and event isolation, and HMR works during development. The tradeoff: the iframe can't directly access the host page's DOM or JavaScript.

```ts
// entrypoints/sidebar.content.ts
export default defineContentScript({
  matches: ['<all_urls>'],

  main(ctx) {
    const ui = createIframeUi(ctx, {
      page: '/sidebar.html',     // path to an unlisted page entrypoint
      position: 'inline',
      anchor: 'body',
      onMount(wrapper, iframe) {
        // wrapper is the container div, iframe is the <iframe> element
      },
    });
    ui.mount();
  },
});
```

You need a corresponding unlisted page entrypoint (`entrypoints/sidebar.html`) that loads your app.

---

## Position Options

All three UI creators accept a `position` option:

- `'inline'` — inserted adjacent to the `anchor` element
- `'overlay'` — overlaid on top of the `anchor` using absolute positioning
- `'modal'` — covers the whole page

The `anchor` can be a CSS selector string, an `Element`, or a function returning an element.

---

## CSS Injection Modes (content scripts)

Configured via `cssInjectionMode` in `defineContentScript`:

- `'manifest'` (default) — CSS imported in JS is added to the content script's `css` array in the manifest. Injected by the browser automatically.
- `'manual'` — CSS is bundled but you inject it manually via `browser.tabs.insertCSS` or similar.
- `'ui'` — Required for `createShadowRootUi`. CSS is injected into the shadow root instead of the page.
