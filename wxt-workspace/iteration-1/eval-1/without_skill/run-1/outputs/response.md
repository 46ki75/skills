# Setting Up a Popup in WXT with React

WXT uses a file-based entrypoint system. To add a popup that appears when the user clicks the toolbar icon, follow these steps:

## 1. Create the Popup Entrypoint

Create a directory and files for your popup under the `entrypoints/` folder:

```
entrypoints/
└── popup/
    ├── index.html
    └── main.tsx
```

### `entrypoints/popup/index.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Popup</title>
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

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

### `entrypoints/popup/App.tsx` (optional, but recommended)

```tsx
export default function App() {
  return (
    <div style={{ width: 300, padding: 16 }}>
      <h1>My Extension Popup</h1>
      <p>Hello from the popup!</p>
    </div>
  );
}
```

## 2. WXT Auto-Configures the Manifest

WXT automatically detects the `popup/` entrypoint and sets `action.default_popup` in the generated `manifest.json`. You do **not** need to manually edit the manifest.

## 3. Ensure React is Installed

If you haven't already, install React:

```bash
npm install react react-dom
npm install -D @types/react @types/react-dom
```

WXT projects using React typically also set up the JSX transform. Make sure your `wxt.config.ts` includes the Vite React plugin:

```ts
// wxt.config.ts
import { defineConfig } from 'wxt';
import react from '@vitejs/plugin-react';

export default defineConfig({
  vite: () => ({
    plugins: [react()],
  }),
});
```

And install the plugin if needed:

```bash
npm install -D @vitejs/plugin-react
```

## 4. Run and Test

```bash
npm run dev
```

WXT will launch a browser with your extension loaded. Click the extension icon in the toolbar — your popup will appear.

## Summary

| File | Purpose |
|------|---------|
| `entrypoints/popup/index.html` | HTML shell for the popup |
| `entrypoints/popup/main.tsx` | React root mount point |
| `entrypoints/popup/App.tsx` | Your popup React component |
| `wxt.config.ts` | Registers the React Vite plugin |

WXT handles all the manifest wiring automatically based on the `popup/` directory name — no manual manifest editing required.
