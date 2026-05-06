# Injecting an Isolated Floating Button with a WXT Content Script

The best approach for this is **Shadow Root UI** (`createShadowRootUi`). It wraps your UI in a Shadow DOM, which means:

- **Your CSS applies only inside the shadow root** — the page's styles cannot reach your button.
- **Your styles don't leak out** — you won't accidentally break the host page's layout.

---

## Step-by-Step Setup

### 1. Create a directory entrypoint for the content script

Use a directory-style entrypoint so you can co-locate your CSS:

```
entrypoints/
  floating-button.content/
    index.ts
    style.css
```

### 2. Write your CSS (`style.css`)

Because the styles live inside the Shadow DOM, you can use simple class names without worrying about conflicts:

```css
/* entrypoints/floating-button.content/style.css */

.floating-btn {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 2147483647; /* maximum z-index */

  width: 56px;
  height: 56px;
  border-radius: 50%;
  border: none;
  cursor: pointer;

  background-color: #6200ee;
  color: #ffffff;
  font-size: 24px;
  font-family: sans-serif;

  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  transition: transform 0.15s ease, box-shadow 0.15s ease;
}

.floating-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
}
```

### 3. Write the content script (`index.ts`)

```ts
// entrypoints/floating-button.content/index.ts
import './style.css'; // Step 1: import CSS so WXT bundles it into the shadow root

export default defineContentScript({
  matches: ['<all_urls>'],      // run on every page the user visits
  cssInjectionMode: 'ui',       // required: injects CSS into shadow root, not the page

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'floating-button',  // used as the custom element tag name in the DOM
      position: 'overlay',      // overlaid on top of the anchor element
      anchor: 'body',
      zIndex: 2147483647,       // ensures the host element sits above page content

      onMount(container) {
        const button = document.createElement('button');
        button.className = 'floating-btn';
        button.textContent = '★';
        button.addEventListener('click', () => {
          console.log('Floating button clicked!');
        });
        container.append(button);
      },
    });

    ui.mount();
  },
});
```

---

## Why Each Option Matters

| Option | Why |
|---|---|
| `cssInjectionMode: 'ui'` | **Required** for `createShadowRootUi`. Tells WXT to inject your CSS into the shadow root rather than the page's `<head>`. Without this, your styles won't appear. |
| `import './style.css'` at the top | WXT picks up this import and bundles the CSS into the shadow root when `cssInjectionMode: 'ui'` is set. |
| `matches: ['<all_urls>']` | Makes the content script run on every http/https page the user visits. |
| `position: 'overlay'` | Overlays the container on top of the anchor (`body`) using absolute positioning, which is correct for a floating fixed-position button. |
| `name: 'floating-button'` | WXT uses this to create a custom HTML element (`<floating-button>`) that hosts the shadow root. Choose a unique name to avoid collisions. |

---

## How Shadow DOM Isolation Works

When `createShadowRootUi` mounts, WXT:

1. Creates a `<floating-button>` custom element and appends it to `<body>`.
2. Attaches a **closed Shadow DOM** to that element.
3. Injects your bundled CSS into the shadow root — not the page.
4. Passes the shadow root's inner `container` div to your `onMount` callback.

Because the button lives inside the Shadow DOM:
- The page's CSS rules **cannot reach** it (no selector crosses a shadow boundary by default).
- Your CSS rules **cannot leak** out to the page.
- Your button is safe from `* { margin: 0; }`, CSS resets, or any other aggressive global styles the page uses.

---

## Using a Framework (Optional)

If you prefer React, Vue, or Svelte, the pattern is the same — just mount your framework app inside `onMount`:

### React Example

```tsx
// entrypoints/floating-button.content/index.tsx
import './style.css';
import ReactDOM from 'react-dom/client';
import FloatingButton from './FloatingButton';

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui',

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'floating-button',
      position: 'overlay',
      anchor: 'body',
      onMount(container) {
        const wrapper = document.createElement('div');
        container.append(wrapper);
        const root = ReactDOM.createRoot(wrapper);
        root.render(<FloatingButton />);
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

> **React note**: React warns when mounting directly to the container, so add a wrapper `div` first (as shown above).

### Vue Example

```ts
// entrypoints/floating-button.content/index.ts
import './style.css';
import { createApp } from 'vue';
import FloatingButton from './FloatingButton.vue';

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui',

  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'floating-button',
      position: 'overlay',
      anchor: 'body',
      onMount(container) {
        const app = createApp(FloatingButton);
        app.mount(container);
        return app;
      },
      onRemove(app) {
        app?.unmount();
      },
    });
    ui.mount();
  },
});
```

---

## Common Mistakes to Avoid

### ❌ Forgetting `cssInjectionMode: 'ui'`

```ts
// Without this, createShadowRootUi still works, but your CSS is injected
// into the page's <head> — not the shadow root — so it won't apply.
export default defineContentScript({
  matches: ['<all_urls>'],
  // cssInjectionMode: 'ui',  ← missing!
  async main(ctx) {
    const ui = await createShadowRootUi(ctx, { /* ... */ });
  },
});
```

### ❌ Using `createIntegratedUi` instead

```ts
// createIntegratedUi puts your element directly in the page DOM.
// The page's CSS WILL affect your button — it's not isolated.
const ui = createIntegratedUi(ctx, { /* ... */ });
```

### ❌ Placing browser API calls at the top level

```ts
// ❌ This runs during WXT's build-time import — it will crash
document.body.appendChild(btn);

// ✅ Always put DOM/browser code inside main()
export default defineContentScript({
  matches: ['<all_urls>'],
  async main(ctx) {
    // safe here
  },
});
```

---

## Final File Structure

```
entrypoints/
  floating-button.content/
    index.ts      ← content script entry
    style.css     ← button styles (scoped to shadow root)
```

That's all you need — no `manifest.json` edits required. WXT reads the `matches` from `defineContentScript` and generates the correct manifest entry automatically.
