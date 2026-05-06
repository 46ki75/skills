# Injecting an Isolated Floating Button with WXT Content Scripts

To inject a floating button into every webpage while keeping its styles isolated from the host page, you should use a **Shadow DOM** inside a WXT content script. This prevents the page's CSS from leaking in and your CSS from leaking out.

---

## 1. Create the Content Script

Create `entrypoints/content.ts` (or `content/index.ts`):

```ts
import './style.css'; // WXT will handle injecting this into the shadow root

export default defineContentScript({
  matches: ['<all_urls>'],
  cssInjectionMode: 'ui', // tells WXT to scope CSS to the shadow DOM
  async main(ctx) {
    const ui = await createShadowRootUi(ctx, {
      name: 'my-floating-button',
      position: 'overlay',
      anchor: 'body',
      append: 'last',
      onMount(container) {
        const button = document.createElement('button');
        button.textContent = '🚀 Click Me';
        button.className = 'floating-btn';
        container.appendChild(button);

        button.addEventListener('click', () => {
          alert('Button clicked!');
        });

        return button;
      },
      onRemove(button) {
        button?.remove();
      },
    });

    ui.mount();
  },
});
```

---

## 2. Add Your CSS

Create `entrypoints/content/style.css` (co-located with your content script, or imported directly):

```css
.floating-btn {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 999999;
  padding: 12px 20px;
  background-color: #6366f1;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.floating-btn:hover {
  background-color: #4f46e5;
}
```

---

## Key Concepts

### Why Shadow DOM?
`createShadowRootUi` wraps your UI in a Shadow DOM, which provides **style encapsulation**:
- The host page's styles **cannot** affect your button.
- Your styles **cannot** accidentally affect the host page.

### `cssInjectionMode: 'ui'`
Setting this tells WXT to inject your CSS into the Shadow DOM container, not the main document `<head>`. Without this, your imported CSS would apply globally to the page.

### `position: 'overlay'`
This positions the shadow host element as an overlay on top of the page content. You can also use `'inline'` if you want to insert it relative to an anchor element.

---

## Alternative: Manual Shadow DOM (without `createShadowRootUi`)

If you need lower-level control:

```ts
export default defineContentScript({
  matches: ['<all_urls>'],
  main() {
    const host = document.createElement('div');
    const shadow = host.attachShadow({ mode: 'open' });

    const style = document.createElement('style');
    style.textContent = `
      button {
        position: fixed;
        bottom: 24px;
        right: 24px;
        /* ... */
      }
    `;

    const button = document.createElement('button');
    button.textContent = '🚀 Click Me';

    shadow.appendChild(style);
    shadow.appendChild(button);
    document.body.appendChild(host);
  },
});
```

This gives you full manual control but requires you to manage CSS as a string or use a CSS injection approach yourself.

---

## Summary

| Approach | Style Isolation | WXT Integration |
|---|---|---|
| `createShadowRootUi` + `cssInjectionMode: 'ui'` | ✅ Full isolation | ✅ Idiomatic WXT |
| Manual `attachShadow` | ✅ Full isolation | ⚠️ Manual wiring |
| Direct DOM injection (no Shadow DOM) | ❌ No isolation | ✅ Simple |

**Recommended:** Use `createShadowRootUi` with `cssInjectionMode: 'ui'` — it's the idiomatic WXT approach and handles shadow DOM, CSS scoping, and cleanup automatically.
