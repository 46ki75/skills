# Setting Up a React Popup in a WXT Browser Extension

Here's a complete walkthrough for adding a popup to your WXT extension using React.

---

## 1. Bootstrap the Project

If you haven't already, scaffold a new WXT project with the React template:

```sh
pnpm dlx wxt@latest init
# Select "React" when prompted for a template
```

Or, if you're adding React to an existing WXT project, install the module:

```sh
pnpm add -D @wxt-dev/module-react
```

---

## 2. Configure WXT to Use React

Add the React module to your `wxt.config.ts`:

```ts
// wxt.config.ts
import { defineConfig } from 'wxt';

export default defineConfig({
  modules: ['@wxt-dev/module-react'],
  manifest: {
    name: 'My Extension',
    description: 'A browser extension with a popup',
    action: {
      default_title: 'Open Popup',
    },
  },
});
```

The `action` key in the manifest is what registers the toolbar icon behavior. WXT auto-converts this for MV2/MV3 as needed.

---

## 3. Create the Popup Entrypoint

Use a **directory entrypoint** (recommended for frameworks) so you can keep the HTML, TypeScript entry, and any CSS together:

```
entrypoints/
  popup/
    index.html    ← WXT discovers this as the popup
    main.tsx      ← React root mount
    App.tsx       ← your React component
    style.css     ← optional styles
```

### `entrypoints/popup/index.html`

```html
<!doctype html>
<html>
  <head>
    <title>My Extension</title>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="./main.tsx"></script>
  </body>
</html>
```

### `entrypoints/popup/main.tsx`

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './style.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

### `entrypoints/popup/App.tsx`

```tsx
import React from 'react';

export default function App() {
  return (
    <div style={{ width: 300, padding: 16 }}>
      <h1>Hello from the Popup!</h1>
      <p>Click the toolbar icon to open this popup.</p>
    </div>
  );
}
```

### `entrypoints/popup/style.css` (optional)

```css
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: sans-serif;
  min-width: 300px;
}
```

---

## 4. Add Toolbar Icons (Optional but Recommended)

Place PNG icons in the `public/` directory. WXT auto-discovers them:

```
public/
  icon-16.png
  icon-32.png
  icon-48.png
  icon-128.png
```

---

## 5. Run the Dev Server

```sh
pnpm dev          # Chrome (MV3 by default)
pnpm dev:firefox  # Firefox
```

WXT will:
- Build the popup entrypoint
- Generate the `manifest.json` with `action.default_popup` pointing to your popup
- Hot-reload the popup UI on file changes

---

## 6. Final Project Structure

```
📂 my-extension/
  📁 entrypoints/
    📁 popup/
      index.html
      main.tsx
      App.tsx
      style.css
  📁 public/
    icon-16.png
    icon-48.png
    icon-128.png
  📄 wxt.config.ts
  📄 package.json
```

---

## Key Rules to Remember

1. **Use a directory entrypoint** (`popup/index.html`) rather than a flat file (`popup.html`) when using React — it keeps related files together cleanly.

2. **No browser API calls at top level.** If you need background logic (e.g., listening for the toolbar icon click programmatically), put that in `entrypoints/background.ts` wrapped in `defineBackground()`.

3. **No path-based routing.** If your popup needs multiple "pages," use a hash-based router:
   ```tsx
   import { createHashRouter, RouterProvider } from 'react-router-dom';
   
   const router = createHashRouter([
     { path: '/', element: <Home /> },
     { path: '/settings', element: <Settings /> },
   ]);
   ```

4. **Auto-imports** — WXT APIs like `browser` and `storage` are available without imports in entrypoints. In React components, import them explicitly:
   ```ts
   import { browser } from 'wxt/browser';
   ```

---

## Generated Manifest (what WXT produces)

WXT automatically generates a `manifest.json` similar to:

```json
{
  "manifest_version": 3,
  "name": "My Extension",
  "action": {
    "default_title": "Open Popup",
    "default_popup": "popup.html",
    "default_icon": {
      "16": "icon-16.png",
      "48": "icon-48.png",
      "128": "icon-128.png"
    }
  }
}
```

You don't write this file — WXT derives it from your `wxt.config.ts` and entrypoint file names.
