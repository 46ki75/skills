# WXT Storage Reference

WXT includes a built-in storage utility (`wxt/utils/storage`) that wraps the browser's extension storage APIs.

## Setup

Add the `storage` permission to your manifest:

```ts
// wxt.config.ts
export default defineConfig({
  manifest: {
    permissions: ['storage'],
  },
});
```

With auto-imports enabled, `storage` is available globally. Otherwise:

```ts
import { storage } from '#imports';
// or:
import { storage } from 'wxt/utils/storage';
```

---

## Basic Usage

All keys must be prefixed with the storage area: `local:`, `sync:`, `session:`, or `managed:`.

```ts
// Get / set / remove
await storage.getItem<string>('local:username');
await storage.getItem<number>('sync:counter', { fallback: 0 });
await storage.setItem('local:username', 'alice');
await storage.removeItem('local:username');

// Check if a key exists
await storage.getItem('local:key') !== null;
```

---

## Watchers

Listen for changes to a single key:

```ts
const unwatch = storage.watch<string>('local:username', (newVal, oldVal) => {
  console.log('username changed from', oldVal, 'to', newVal);
});

// Stop listening:
unwatch();
```

---

## Defining Typed Storage Items (recommended)

`storage.defineItem` creates a reusable, typed accessor for a single key. Define items in `utils/` so they are auto-imported:

```ts
// utils/storage.ts
export const darkMode = storage.defineItem<boolean>('local:darkMode', {
  fallback: false,  // returned by getValue() when key is missing
});

export const installDate = storage.defineItem<number>('local:installDate', {
  init: () => Date.now(),  // saved to storage on first call
});

export const userId = storage.defineItem<string>('local:userId', {
  init: () => crypto.randomUUID(),
});
```

Usage (auto-imported, no explicit import needed):

```ts
const isDark = await darkMode.getValue();
await darkMode.setValue(true);
await darkMode.removeValue();
const unwatch = darkMode.watch((newVal) => console.log('theme:', newVal));
```

---

## Versioned Storage Items (schema migrations)

When you expect a stored value's shape to change over time, add versioning:

```ts
// Starting with version 1:
type PrefsV1 = { theme: 'light' | 'dark' };
export const prefs = storage.defineItem<PrefsV1>('local:prefs', {
  version: 1,
  fallback: { theme: 'dark' },
});
```

When the schema changes, bump the version and add a migration function:

```ts
// After changing schema:
type PrefsV1 = { theme: 'light' | 'dark' };
type PrefsV2 = { theme: 'light' | 'dark'; fontSize: number };

export const prefs = storage.defineItem<PrefsV2>('local:prefs', {
  version: 2,
  fallback: { theme: 'dark', fontSize: 14 },
  migrations: {
    // Called when upgrading from v1 to v2
    2: (old: PrefsV1): PrefsV2 => ({ ...old, fontSize: 14 }),
  },
});
```

**Starting without versioning?** No problem — WXT treats unversioned items as version 1. Just add `version: 2` and a migration for `2` when the schema changes.

Migrations run automatically as soon as `defineItem` is called. All get/set calls wait for migrations to finish before proceeding.

---

## Metadata

Each key can have associated metadata (stored at `key + "$"`):

```ts
await storage.setMeta('local:username', { lastModified: Date.now(), v: 2 });
const meta = await storage.getMeta<{ lastModified: number }>('local:username');

// Remove specific properties:
await storage.removeMeta('local:username', 'lastModified');
// Remove all metadata:
await storage.removeMeta('local:username');
```

Multiple `setMeta` calls merge properties rather than overwriting.

---

## Bulk Operations

Reduce the number of storage calls when reading/writing multiple values:

```ts
// Get multiple values
const [username, theme] = await storage.getItems([
  'local:username',
  { key: 'local:theme', fallback: 'dark' },
]);

// Set multiple values
await storage.setItems([
  { key: 'local:username', value: 'alice' },
  { key: 'local:theme', value: 'dark' },
]);

// Works with defined items too:
await storage.setItems([
  { item: darkMode, value: true },
  { item: userId, value: 'abc-123' },
]);

// Remove multiple values
await storage.removeItems(['local:username', 'local:theme']);
// Optionally remove metadata too:
await storage.removeItems([{ key: 'local:username', options: { removeMeta: true } }]);
```

---

## Storage Areas

| Area | Scope | Quota | Notes |
| ---- | ----- | ----- | ----- |
| `local:` | Device-local | ~10 MB | Most commonly used |
| `sync:` | Synced across devices | ~100 KB | Requires login + sync enabled |
| `session:` | Current browser session only | ~1 MB | Cleared when browser closes |
| `managed:` | Read-only, set by enterprise policy | — | Rarely used |

Use `local:` by default. Use `sync:` only for small user settings that should follow them across devices.
