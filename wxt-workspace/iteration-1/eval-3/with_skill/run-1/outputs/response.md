# Using WXT's Storage API for Persistent User Preferences

WXT ships a built-in storage utility that wraps the browser's extension storage APIs, giving you typed, reactive access to persistent data. Here's exactly how to set up dark mode and a custom URL preference, plus watch for changes from your background script.

---

## 1. Add the `storage` Permission

First, declare the `storage` permission in `wxt.config.ts`:

```ts
// wxt.config.ts
import { defineConfig } from 'wxt';

export default defineConfig({
  manifest: {
    name: 'My Extension',
    permissions: ['storage'],
  },
});
```

---

## 2. Define Typed Storage Items

The recommended pattern is to define each preference as a typed item using `storage.defineItem` in a `utils/` file. WXT auto-imports everything from `utils/`, so you won't need explicit imports elsewhere.

```ts
// utils/settings.ts

export const darkMode = storage.defineItem<boolean>('local:darkMode', {
  fallback: false, // returned by getValue() when the key doesn't exist yet
});

export const customUrl = storage.defineItem<string>('local:customUrl', {
  fallback: 'https://example.com',
});
```

**Key points:**
- Keys must be prefixed with the storage area: `local:`, `sync:`, `session:`, or `managed:`.
- `local:` persists across browser restarts and is the right choice for user preferences.
- `fallback` is returned when the key hasn't been set yet — no need to handle `null` manually.
- Since these are in `utils/`, they are auto-imported anywhere in your extension.

---

## 3. Read and Write the Preferences

Use the typed items anywhere in your extension (popup, options page, content scripts):

```ts
// Read the current values
const isDarkMode = await darkMode.getValue();   // false if not set yet
const url = await customUrl.getValue();          // 'https://example.com' if not set yet

// Update the values
await darkMode.setValue(true);
await customUrl.setValue('https://my-site.com');

// Reset to the fallback (removes the key from storage)
await darkMode.removeValue();
```

---

## 4. Watch for Changes in Your Background Script

To react to changes in your background script, use the `.watch()` method on each defined item. Always register watchers **inside** the `main()` function — never at the top level, because WXT imports entrypoints in a Node.js environment during build.

```ts
// entrypoints/background.ts
export default defineBackground(() => {
  // Watch for dark mode preference changes
  const unwatchDarkMode = darkMode.watch((newVal, oldVal) => {
    console.log(`Dark mode changed from ${oldVal} to ${newVal}`);
    // React to the change, e.g. update icon, notify content scripts, etc.
    if (newVal) {
      // user switched to dark mode
    }
  });

  // Watch for custom URL changes
  const unwatchCustomUrl = customUrl.watch((newVal, oldVal) => {
    console.log(`Custom URL changed from ${oldVal} to ${newVal}`);
    // e.g. update declarative net request rules, clear cached data, etc.
  });

  // If you ever need to stop watching (e.g. during cleanup):
  // unwatchDarkMode();
  // unwatchCustomUrl();
});
```

The watcher fires every time the value changes in storage — whether the change came from the popup, options page, content script, or background itself.

---

## 5. Example: Saving Preferences from the Popup

Here's how a popup might let the user toggle dark mode and set a custom URL:

```ts
// entrypoints/popup/main.ts

// Read current values on load
const currentDark = await darkMode.getValue();
const currentUrl = await customUrl.getValue();

// Render them in your UI, then on user action:
async function onToggleDarkMode(checked: boolean) {
  await darkMode.setValue(checked);
  // The background watcher fires automatically
}

async function onSaveUrl(url: string) {
  await customUrl.setValue(url);
  // The background watcher fires automatically
}
```

---

## 6. Bulk Reading on Startup (Optional Optimization)

If you need both values at once, use `storage.getItems` to reduce storage calls:

```ts
const [isDark, url] = await storage.getItems([
  { key: 'local:darkMode', fallback: false },
  { key: 'local:customUrl', fallback: 'https://example.com' },
]);
```

Or with defined items:

```ts
const [isDark, url] = await Promise.all([
  darkMode.getValue(),
  customUrl.getValue(),
]);
```

---

## 7. If Settings Should Sync Across Devices

If you want preferences to follow the user across all their logged-in browsers, use the `sync:` prefix instead of `local:`. Note the sync quota is ~100 KB total, so it's appropriate only for small settings like these.

```ts
// utils/settings.ts
export const darkMode = storage.defineItem<boolean>('sync:darkMode', {
  fallback: false,
});

export const customUrl = storage.defineItem<string>('sync:customUrl', {
  fallback: 'https://example.com',
});
```

The rest of the API is identical — `getValue`, `setValue`, and `watch` all work the same way regardless of area.

---

## Summary

| Task | Code |
|---|---|
| Define a persistent preference | `storage.defineItem<T>('local:key', { fallback })` in `utils/` |
| Read a value | `await myItem.getValue()` |
| Save a value | `await myItem.setValue(newValue)` |
| Watch for changes (background) | `myItem.watch((newVal, oldVal) => { ... })` inside `defineBackground(() => { ... })` |
| Stop watching | Call the unwatch function returned by `.watch()` |
| Sync across devices | Use `sync:` prefix instead of `local:` |

The full project structure for this feature:

```
entrypoints/
  background.ts       ← registers .watch() listeners
  popup/
    index.html
    main.ts           ← calls getValue/setValue
utils/
  settings.ts         ← exports darkMode and customUrl defined items
wxt.config.ts         ← declares 'storage' permission
```
