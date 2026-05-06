# Using WXT's Storage API for Persistent User Preferences

WXT provides a built-in storage API (`wxt/storage`) that wraps the browser's `storage` API with a typed, reactive interface. Here's how to save dark mode and a custom URL, then watch for changes in your background script.

---

## 1. Define Storage Items

Create a shared file (e.g., `utils/storage.ts`) to define your storage items with types and defaults:

```ts
// utils/storage.ts
import { storage } from 'wxt/storage';

export const darkModeStorage = storage.defineItem<boolean>(
  'local:darkMode',
  {
    defaultValue: false,
  }
);

export const customUrlStorage = storage.defineItem<string>(
  'local:customUrl',
  {
    defaultValue: 'https://example.com',
  }
);
```

- The key prefix `local:` stores data in `browser.storage.local`, which persists across browser restarts.
- Use `sync:` instead if you want settings synced across devices (requires the `storage` permission with sync support).

---

## 2. Read and Write from a Popup or Options Page

```ts
// entrypoints/popup/main.ts
import { darkModeStorage, customUrlStorage } from '../../utils/storage';

// Read values
const isDarkMode = await darkModeStorage.getValue();
const url = await customUrlStorage.getValue();

// Write values
await darkModeStorage.setValue(true);
await customUrlStorage.setValue('https://my-custom-url.com');
```

---

## 3. Watch for Changes in the Background Script

```ts
// entrypoints/background.ts
import { darkModeStorage, customUrlStorage } from '../utils/storage';

export default defineBackground(() => {
  // Watch dark mode changes
  darkModeStorage.watch((newValue, oldValue) => {
    console.log('Dark mode changed:', oldValue, '->', newValue);
    // React to the change, e.g., apply theming logic
  });

  // Watch custom URL changes
  customUrlStorage.watch((newValue, oldValue) => {
    console.log('Custom URL changed:', oldValue, '->', newValue);
    // React to the change
  });
});
```

The `.watch()` method sets up a listener that fires whenever the stored value changes, from any part of the extension (popup, options page, content scripts, etc.).

---

## 4. Required Manifest Permission

Ensure `storage` is listed in your `wxt.config.ts` manifest permissions:

```ts
// wxt.config.ts
import { defineConfig } from 'wxt';

export default defineConfig({
  manifest: {
    permissions: ['storage'],
  },
});
```

---

## Summary

| Action | Method |
|---|---|
| Define a typed item | `storage.defineItem<T>('local:key', { defaultValue })` |
| Read a value | `await item.getValue()` |
| Write a value | `await item.setValue(newValue)` |
| Watch for changes | `item.watch((newVal, oldVal) => { ... })` |

Using `local:` as the key prefix guarantees persistence across browser restarts. The `.watch()` listener in the background script gives you reactive updates whenever preferences change from anywhere in the extension.
