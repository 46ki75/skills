---
name: qwik
description: >
  Expert guidance for Qwik (v1 and v2) and its meta-framework — Qwik City in
  v1, Qwik Router in v2 — covering resumability, components, signals, stores,
  tasks, context, QRLs, file-based routing, route loaders/actions, middleware,
  endpoints, layouts, async data (useResource$/useAsync$/Suspense), v1→v2
  migration, and deployment. Use whenever someone writes, debugs, or reviews
  Qwik code, asks about useSignal, useStore, useTask$, useAsync$, routeLoader$,
  routeAction$, component$, the $ suffix, resumability vs hydration, migrating
  v1 to v2, server-side data fetching, REST/JSON endpoints, middleware, or
  deployment. Always invoke for any question mentioning Qwik, Qwik City, Qwik
  Router, QRL, routeLoader$, routeAction$, useVisibleTask$, useAsync$,
  noSerialize, or the resumable architecture.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.6.0"
---

# Qwik Skill (v1 & v2)

You are an expert in the Qwik framework and its meta-framework (Qwik City in
v1, Qwik Router in v2). Your goal is to help users write correct, idiomatic,
and performant Qwik code.

## Version detection — do this first

Qwik v2 is in **beta** (stable as of 2026-05 is still v1). Check the user's
`package.json` before answering anything API-specific:

| Sees in `package.json`                       | Version |
| -------------------------------------------- | ------- |
| `@builder.io/qwik` + `@builder.io/qwik-city` | **v1**  |
| `@qwik.dev/core` + `@qwik.dev/router`        | **v2**  |

If unclear, ask. Several APIs differ in non-obvious ways (`useResource$` →
`useAsync$`, `<Resource>` → `<Suspense>`, `<QwikCityProvider>` →
`useQwikRouter()`, sync-only `useComputed$`). **For any v2-specific question,
or whenever the user mentions migration, read `references/qwik-v2.md`.**

## Quick orientation

| Package (v1)            | Package (v2)       | Key exports                                                                                                                                                                     |
| ----------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@builder.io/qwik`      | `@qwik.dev/core`   | `component$`, `useSignal`, `useStore`, `useTask$`, `useVisibleTask$`, `useComputed$`, `useContext`, `useContextProvider`, `createContextId`, `$`, `noSerialize`                 |
| ↑ same                  | ↑ same             | v1 only: `useResource$`, `<Resource>` &nbsp;·&nbsp; v2 only: `useAsync$`, `<Suspense>`, `<Reveal>`, `useSerializer$`, `createSerializer$`                                       |
| `@builder.io/qwik-city` | `@qwik.dev/router` | `routeLoader$`, `routeAction$`, `Form`, `Link`, `useLocation`, `useNavigate`, `RequestHandler` (v1: `QwikCityProvider` &nbsp;·&nbsp; v2: `useQwikRouter`, `QwikRouterProvider`) |

## Core concepts you must always apply

### 1 – Resumability, not hydration

Qwik **serializes** the entire application state (listeners, component tree,
store values) into the HTML at SSR time. On the client, it **resumes** exactly
where the server left off — no code needs to be re-downloaded or re-executed to
make the page interactive.

Practical implications for code you write:

- State that crosses the server-to-client boundary **must be serializable**.
  Primitives, plain objects/arrays, `Date`, `URL`, `Map`, `Set`, DOM refs, and
  QRL-wrapped functions all work. Class instances (custom `instanceof`) and
  streams do not.
- Use `noSerialize()` to wrap non-serializable values (e.g., third-party editor
  instances). They will be `undefined` on resume and must be re-initialised
  inside `useVisibleTask$`.
- Don't think of components as "running on mount" — the lifecycle is
  **server -> pause -> resume on client**, not "mount everything in the browser".

### 2 – The `$` suffix and QRLs

Any function ending in `$` (e.g., `component$`, `useTask$`, `onClick$`, `$()`)
creates a **QRL boundary**. The Qwik optimiser extracts the closure into a
separate lazy-loadable chunk. Rules:

- Variables captured inside a `$`-closure must themselves be serializable
  (signals, stores, primitives, other QRLs) — they are serialized along with
  the listener.
- Don't import non-serializable values into a `$`-closure and expect them to
  survive resumption.
- The optimiser runs at build time; you don't call it manually.

### 3 – State management at a glance

```tsx
// Fine-grained reactive value
const count = useSignal(0);
count.value++; // triggers only components that read count.value

// Deep reactive object — tracks nested mutations by default
const state = useStore({ user: { name: "Alice" }, items: [] });
state.user.name = "Bob"; // triggers re-render of consumers

// Derived/computed — synchronous, memoised
const upper = useComputed$(() => name.value.toUpperCase());

// Async data — v1: useResource$ + <Resource>
const data = useResource$<T>(async ({ track, cleanup }) => {
  track(() => id.value); // re-run when id changes
  const ctrl = new AbortController();
  cleanup(() => ctrl.abort());
  return fetch(`/api/item/${id.value}`, { signal: ctrl.signal }).then((r) =>
    r.json(),
  );
});
// Render with <Resource value={data} onPending=... onResolved=... />

// Async data — v2: useAsync$ + <Suspense>
const data = useAsync$<T>(async ({ track, abortSignal }) => {
  track(id); // signal shorthand (track(() => id.value) also works)
  const r = await fetch(`/api/item/${id.value}`, { signal: abortSignal });
  return r.json();
});
// Render: <Suspense fallback={<p>…</p>}><p>{data.value.name}</p></Suspense>
// data.value is T directly (not Promise<T>); .loading and .error are also exposed.
```

Prefer `useSignal` for single values, `useStore` for related multi-field
objects. Use `useComputed$` over `useTask$` for pure derived values — it is
simpler and auto-tracks. **In v2, `useComputed$` is sync-only — passing an
async function throws runtime error Q29; use `useAsync$` instead.**

### 4 – Tasks and the lifecycle

```text
useTask$ -> RENDER -> useVisibleTask$
           |                |
     SERVER or BROWSER   BROWSER only
```

| Hook                | When it runs                                                                | Use it for                                                    |
| ------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `useTask$`          | Before first render (server or browser); re-runs when tracked state changes | Async init, side effects that should run on server AND client |
| `useVisibleTask$`   | After render, browser only, when element becomes visible                    | DOM manipulation, third-party libs, subscriptions             |
| `useResource$` (v1) | Before render, async, non-blocking                                          | Async data that should not block rendering                    |
| `useAsync$` (v2)    | Before render, async, non-blocking                                          | v2 replacement for `useResource$` — pair with `<Suspense>`    |

`useTask$` blocks rendering until its promise resolves. Use it for critical
data. `useResource$`/`useAsync$` do not block — the component renders
immediately with the pending state.

> **v2 note:** `useVisibleTask$` no longer accepts `eagerness: 'load' | 'idle'`
> — remove it when migrating. A `strategy: 'document-ready'` option exists for
> cases that need eager client execution.

**`intersection-observer` (the default) silently fails in two common cases —
reach for `{ strategy: "document-ready" }` when:**

- **The host component renders another component, not a direct DOM element.**
  Qwik can't attach the intersection observer and logs
  `"You are trying to add the event 'q-e:qvisible' using the useVisibleTask$
  hook with the 'intersection-observer' strategy, but this only works when the
  component outputs a DOM element. Falling back to 'document-ready' instead."`
  Be explicit to silence the warning.
- **The task lives inside a conditionally-rendered or collapsed subtree** (a
  closed `ElmCollapse`, a hidden tab panel, a modal that hasn't opened yet).
  The element is `height: 0` / `display: none`, so the observer never fires —
  signal writes that would have triggered the task slip past unobserved. This
  bites focus-on-open patterns: a `useVisibleTask$` that should focus the first
  field when a modal opens will never subscribe.

Pair with `requestAnimationFrame()` (not `queueMicrotask`) when focusing a
freshly-mounted ref — gives Qwik a frame to wire the `ref={}` attribute onto
the just-rendered element.

Important: `useTask$` that tracks no state runs **exactly once**, either on the
server or the browser, not both. Use `isServer`/`isBrowser` guards only when
you genuinely need to branch.

### 5 – Context

```tsx
// 1. Declare a typed context ID (module scope — not inside a component)
export const ThemeCtx = createContextId<Signal<string>>("app.theme");

// 2. Provide it in an ancestor
useContextProvider(ThemeCtx, useSignal("dark"));

// 3. Consume in any descendant
const theme = useContext(ThemeCtx);
```

Context is the idiomatic way to share state across a subtree without prop
drilling. The provided value can be a signal, store, or any serializable value.

---

## Qwik City (v1) / Qwik Router (v2) — routing and server integration

The same APIs (`routeLoader$`, `routeAction$`, `Form`, middleware, endpoints,
layouts) exist in both versions — only the package name and a few wrapper
identifiers differ:

- v1: `import { routeLoader$ } from '@builder.io/qwik-city'`, root wraps in
  `<QwikCityProvider>`.
- v2: `import { routeLoader$ } from '@qwik.dev/router'`, root calls
  `useQwikRouter()` (no provider wrapper).

Read `references/qwik-city.md` for detailed API docs including: routing,
`routeLoader$`, `routeAction$`, middleware, endpoints, `server$`, caching,
error handling, re-exporting loaders, `validator$`, complex forms, advanced
routing (404 pages, grouped/named layouts, plugin files), request handling /
cookie API, redirects, SSG, speculative module fetching, and HTML attributes.

See `references/doc-index.md` for a complete index of all documentation pages
and their coverage status.

Key points:

### File-based routing

```text
src/routes/
├── layout.tsx          ← wraps all child routes
├── index.tsx           ← page: /
├── about/
│   └── index.tsx       ← page: /about
├── blog/
│   ├── layout.tsx      ← wraps /blog/**
│   └── [slug]/
│       └── index.tsx   ← page: /blog/:slug  (params.slug)
└── api/
    └── items/
        └── index.ts    ← JSON endpoint: GET/POST /api/items
```

Catch-all: `[...all]/index.tsx` matches any depth.

### Data loading — `routeLoader$`

The right way to load server data that is needed before the page renders:

```tsx
// src/routes/product/[id]/index.tsx
export const useProduct = routeLoader$(async ({ params, fail }) => {
  const product = await db.products.find(params.id);
  if (!product) return fail(404, { message: "Not found" });
  return product;
});

export default component$(() => {
  const product = useProduct(); // Signal<Product>
  return <h1>{product.value.name}</h1>;
});
```

- Must be exported from `layout.tsx` or `index.tsx` (or re-exported from those
  files if defined elsewhere).
- Runs **on the server on every navigation** before the component renders.
- Access request context via `params`, `cookie`, `headers`, `url`, `method`,
  `env`, `sharedMap`.
- Call `requestEvent.resolveValue(useOtherLoader)` to depend on another loader.

### Form actions — `routeAction$`

Handle mutations (POST forms, API calls):

```tsx
import { routeAction$, zod$, z } from "@builder.io/qwik-city";

export const useCreateItem = routeAction$(
  async (data, { redirect }) => {
    await db.items.create(data);
    throw redirect(302, "/items");
  },
  zod$({ name: z.string().min(1), qty: z.number() }),
);

export default component$(() => {
  const action = useCreateItem();
  return (
    <Form action={action}>
      <input name="name" />
      <input name="qty" type="number" />
      <button type="submit">Create</button>
    </Form>
  );
});
```

### Middleware

Export `onRequest` / `onGet` / `onPost` (etc.) from any `layout.tsx` or
`index.tsx`. They run in order from outermost layout to innermost `index.ts`.

```tsx
export const onRequest: RequestHandler = async ({ next, cookie, redirect }) => {
  const token = cookie.get("session")?.value;
  if (!token) throw redirect(302, "/login");
  await next();
};
```

### JSON / REST endpoints

Export only `onGet`/`onPost`/… (no default component export) from an
`index.ts` file:

```ts
// src/routes/api/users/index.ts
export const onGet: RequestHandler = async ({ json }) => {
  const users = await db.users.findAll();
  json(200, users);
};
```

---

## React-to-Qwik migration quick reference

If the user is coming from React, this table is often the most useful thing to show first:

| React pattern                                  | Qwik equivalent                                                     | Notes                               |
| ---------------------------------------------- | ------------------------------------------------------------------- | ----------------------------------- |
| `useEffect(() => {}, [])`                      | `useVisibleTask$(() => {})`                                         | Browser-only, after render          |
| `useEffect(() => {}, [dep])`                   | `useVisibleTask$(({ track }) => { track(() => sig.value); ... })`   | Re-runs when tracked signal changes |
| `useEffect(() => { return () => cleanup(); })` | `useVisibleTask$(({ cleanup }) => { cleanup(() => ...); })`         |                                     |
| `useRef<T>(null)`                              | `useSignal<T>()` + `ref={myRef}`                                    |                                     |
| `useState(v)`                                  | `useSignal(v)` — read/write via `.value`                            |                                     |
| `useState({ a, b, c })`                        | `useStore({ a, b, c })` — deep reactive proxy                       |                                     |
| `useMemo(() => expr, deps)`                    | `useComputed$(() => expr)` — auto-tracks                            |                                     |
| Storing a class instance in state              | `noSerialize(instance)` stored in `useStore<{ x: NoSerialize<T> }>` |                                     |
| Context (`createContext` + `Provider`)         | `createContextId` + `useContextProvider` + `useContext`             |                                     |

### Non-serializable values — the critical pattern React developers miss

This is the most common stumbling block when migrating. Class instances (chart
libraries, editors, WebSocket connections, etc.) cannot be serialized by Qwik.
**Always** use `noSerialize`:

```tsx
import { component$, useStore, useSignal, useVisibleTask$, noSerialize, type NoSerialize } from '@builder.io/qwik';
import type { Chart } from 'chart.js';

export const ChartComponent = component$(() => {
  const canvasRef = useSignal<HTMLCanvasElement>();
  // NoSerialize<T>: will be undefined on the server and on resume
  const store = useStore<{ chart: NoSerialize<Chart> }>({ chart: undefined });

  useVisibleTask$(({ cleanup }) => {
    // This replaces React's useEffect(() => {...}, [])
    import('chart.js/auto').then(({ Chart }) => {
      store.chart = noSerialize(new Chart(canvasRef.value!, { type: 'bar', data: {...} }));
    });
    cleanup(() => store.chart?.destroy());
  });

  return <canvas ref={canvasRef} />;
});
```

**Why**: Qwik serializes all component state to HTML for resumability. Without
`noSerialize`, Qwik will try (and fail) to serialize the class instance. With
`noSerialize`, Qwik sets the value to `undefined` during SSR and resume — you
re-create it in `useVisibleTask$`.

---

## Common patterns and pitfalls

### Passing signals vs values as props

```tsx
// ✅ Pass value when child only reads
<Child isOpen={modal.value} />

// ✅ Pass signal when child needs to write (or when prop is used as a bind:)
<Child modal={modal} />

// ⛔ Don't pass the whole signal when only its value is needed
<Child isOpen={modal} />   // child receives Signal object, not boolean
```

### Keying single-child renderers on a dynamic id

Qwik reconciles children by JSX position when no `key` is present. The
list iteration case (`{items.map((x) => <Row key={x.id} />)}`) makes the
keyed pattern obvious. The non-obvious case is a **single-child renderer
whose prop id swaps** based on parent state — a `Card` whose `child`
rebinds from `"a"` to `"b"`, a `Modal` whose body component changes, a
recursive renderer that descends into a different node:

```tsx
const renderChild = (childId: string) => (
  <ComponentHost id={childId} basePath={path} />
);
```

There is still exactly one `<ComponentHost>` at the same JSX position,
so Qwik reuses the **same `component$` instance** with `props.id`
swapped from `"a"` to `"b"`. Internal state carries over: `useSignal`
latches keep their old values, `useTask$` does not re-run, and any
subscription opened inside that task is still bound to the previous id.

The fix is to put the prop id into the `key`:

```tsx
<ComponentHost key={childId} id={childId} basePath={path} />
```

This forces Qwik to unmount the previous instance (running its
cleanups) and mount a fresh one. The signal slots reset, the task
re-runs, the stale subscription is torn down by its own cleanup.

Three things to internalise:

- **`useSignal` is component-local but NOT id-local.** A "fresh" prop id
  on the same `component$` instance still sees the previous signal
  values.
- **Subscriptions opened inside `useTask$` leak across the swap unless
  the cleanup runs.** Cleanup only fires on unmount, or on the next
  `track()` re-run. Re-tracking `props.id` inside the task is not a fix
  — the closure capture of the previous id stays in flight until the
  task body completes its current invocation.
- **Per-row keys on list iteration cover this for `.map()` cases but not
  for single-child slots.** `Card.child`, `Modal.body`, `Button.child`
  style slots need the key applied at the recursion site too.

Rule of thumb: if a component's identity is determined by a prop, that
prop must be in the `key`.

### `bind:value` two-way binding

```tsx
const name = useSignal("");
<input bind:value={name} />; // equivalent to value={name.value} + onInput$
```

### `useVisibleTask$` for DOM/browser-only work

```tsx
useVisibleTask$(({ cleanup }) => {
  const timer = setInterval(() => tick(), 1000);
  cleanup(() => clearInterval(timer));
});
```

### `noSerialize` for non-serializable third-party instances

```tsx
const store = useStore<{ editor: NoSerialize<Monaco> }>({ editor: undefined });

useVisibleTask$(() => {
  store.editor = noSerialize(monaco.editor.create(ref.value!, {}));
});
```

### `useStore` gotchas when deep-cloning or seeding two stores

`useStore` returns a `Proxy`. Two consequences that bite often:

- `structuredClone(store)` throws `DataCloneError` — the proxy's internal
  traps are incompatible. Use `cloneDeep` from `es-toolkit` (or `lodash`) when
  you need a deep snapshot for comparison, debouncing, throttling, or
  history.
- `useStore({ ...initialValue })` only shallow-copies. If you create two
  stores from the same seed, their nested objects are aliased — mutating one
  mutates the other. Pass `cloneDeep(initialValue)` to each.

```ts
import { cloneDeep, isEqual } from "es-toolkit";

const live = useStore<T>(cloneDeep(initialValue));
const snapshotted = useStore<T>(cloneDeep(initialValue)); // independent

useTask$(({ track }) => {
  const snap = track(() => cloneDeep(live)); // reactive deep snapshot
  if (!isEqual(snap, snapshotted)) Object.assign(snapshotted, snap);
});
```

See `references/qwik-core.md` ("Testing helper") for more, including the
related `useTask$` cleanup pattern.

### `useTask$` cleanup fires on re-run, not only on unmount

A `cleanup()` registered inside a tracking `useTask$` fires **before every
re-run** as well as on unmount. That's the right behaviour for things that
should reset on the next write (e.g. a debounce timer). It's the wrong
behaviour for things that must survive across writes (e.g. a throttle
cooldown timer).

When you need an unmount-only cleanup, register it from a **separate
`useTask$` with no `track()`** — a no-track task runs once on construction
and its cleanup fires only on unmount. Pattern and rationale documented in
`references/qwik-core.md` ("Testing helper" / cleanup).

### Testing async signal writes with `createDOM`

`createDOM` only flushes pending renders inside `userEvent` / `render`. A
signal write from a raw `setTimeout` (or any callback outside Qwik's invoke
context) queues a render that `await new Promise(r => setTimeout(r, ms))`
will not pump. In production this Just Works; in tests, click a no-op
`#btn-flush` button after waiting to flush the scheduler. Details in
`references/qwik-core.md` ("Testing helper").

### `isServer` is `true` inside `createDOM` tests

`createDOM`'s test environment reports `isServer === true` from
`@builder.io/qwik/build` (or `@qwik.dev/core/build` in v2), even though the
test DOM is fully present. Any `useTask$` / `useVisibleTask$` body guarded
with `if (isServer) return;` therefore takes the early exit and the entire
setup block is skipped. Symptoms: subscriptions never wire up, signals
never refresh, the component renders its initial state and never updates.

```ts
useTask$(({ cleanup }) => {
  // ❌ Fires in createDOM tests; the body never runs
  if (isServer) return;
  // ✅ NoSerialize values are undefined on the server and populated on
  //    both the client AND in createDOM tests — branch on the value
  //    the task is actually waiting for
  if (!surfaceModel) return;
  // ... subscribe etc.
});
```

Production Qwik happens to gate the same way and works, so the regression
only appears once unit tests get added. Cleanest discipline: never branch
on `isServer` inside a task that depends on a `NoSerialize` value — branch
on the value itself.

### Programmatic value writes on form controls don't fire `onInput$`

Setting `textarea.value = "..."` (or `input.value = "..."`) from JS does
**not** trigger Qwik's `onInput$` listener. Any parent component that mirrors
form text into a Signal via that listener will be stale, and the submit path
will see the old value.

Dispatch a bubbling input event after the write:

```ts
ta.value = newText;
ta.focus();
ta.setSelectionRange(caret, caret);
ta.dispatchEvent(new Event("input", { bubbles: true }));
```

The dispatched object is technically `Event`, not `InputEvent` (no `data` /
`inputType`), but Qwik's listener still fires. Common scenario: splicing
resolved prompt-template text into a chat input from a side panel; the
template's content has to reach the parent's input mirror through this synthetic
event.

### Cross-component imperative actions via Signal + `useTask$`

Qwik has no React-style imperative refs for invoking a child component's
methods. The clean pattern for "component A asks component B to do something"
(e.g., a keyboard handler in the parent input asks a sibling picker to open
its modal):

1. Parent creates `const trigger = useSignal<Payload | null>(null);`
2. Passes it down as a prop to the child.
3. Child watches via `useTask$`:

   ```tsx
   useTask$(async ({ track }) => {
     if (!trigger) return; // optional prop guard
     const payload = track(() => trigger.value);
     if (payload === null) return;
     trigger.value = null; // reset BEFORE the work so a re-trigger on the
                           // same value still fires (track only sees changes)
     await doTheThing(payload);
   });
   ```

4. Parent invokes by writing: `trigger.value = somePayload;`

Constraints:

- Payload must be **serializable** — plain object / string / number, not a
  QRL, class instance, or function reference.
- **Reset to `null` *before* the async work**, not after — `track()` only
  re-runs on value changes, so resetting first lets the same payload
  re-trigger without writing a different intermediate value.
- For one-shot signals that should re-fire even on writing the same value,
  use a nonce / counter field in a store instead.

### ESLint plugin quirks

Two `eslint-plugin-qwik` rules surprise first-time users:

- **`qwik/valid-lexical-scope`** rejects booleans built from a `QRL<...>`
  operand (`const hasPicker = prompts && resolvePrompt$`) when captured into
  a child `$()` closure, with the misleading message `"Symbol, which is not
  serializable."` The composite IS fine in JSX render; only nested child
  QRLs trigger it.
- **`qwik/no-async-prevent-default`** fires for **any** `preventDefault()`
  inside a `$()` closure regardless of whether the body is actually `async`
  — the name is misleading. The proper fix is `sync$()`, not just removing
  `async`.

Read `references/lint-quirks.md` when you hit either rule — it covers the
underlying cause for each, the recommended pattern (`sync$()` split for
prevent-default, inlined per-signal gates for lexical-scope), and when an
`eslint-disable` block is the pragmatic last resort.

---

## Reference files

### Core references (read for most Qwik questions)

- **`references/qwik-core.md`** — complete API reference for `@builder.io/qwik`
  (all hooks, component lifecycle, QRL details, slots, rendering). Read when
  you need exact signatures or advanced topics (containers, optimizer details).
- **`references/qwik-city.md`** — complete API reference for `@builder.io/qwik-city`
  (routing, loaders, actions, middleware, layouts, endpoints, deployment). Read
  when working on anything server-side or routing-related.
- **`references/qwik-deprecated.md`** — migration table for all deprecated APIs
  (`useWatch$`, `useClientEffect$`, `loader$`, `action$`, etc.). Read when the
  user mentions old APIs or is migrating from an older version.
- **`references/qwik-v2.md`** — complete v1 → v2 migration reference: package
  renames (`@builder.io/*` → `@qwik.dev/*`), `useResource$` → `useAsync$`,
  `<Resource>` → `<Suspense>`, sync-only `useComputed$`, `useQwikRouter()`
  hook, `useVisibleTask$` eagerness removal, qwik-labs removal, new
  serialization APIs, Vite 8 / Rolldown support, `pnpm qwik migrate-v2`
  codemod, and v1-lib interop. **Read for any v2-specific question or
  migration request.**
- **`references/lint-quirks.md`** — `eslint-plugin-qwik` rules that surprise
  users: `qwik/valid-lexical-scope` rejecting QRL-mixed composites, and
  `qwik/no-async-prevent-default` firing for any `preventDefault()` inside
  `$()` (not just async). Covers the `sync$()` split pattern, inlined per-signal
  gates, and when `eslint-disable` is the right call. Read when either rule
  fires unexpectedly.

### Cookbook recipes (read for specific "how do I" patterns)

When the user asks *how to implement* a specific common pattern, read the
matching file from `references/cookbook/`:

| Pattern                      | File                                     |
| ---------------------------- | ---------------------------------------- |
| Algolia / search             | `cookbook/algolia-search.md`             |
| Composing middleware         | `cookbook/combine-request-handlers.md`   |
| Debounce input               | `cookbook/debouncer.md`                  |
| Throttle input               | `cookbook/throttle.md`                   |
| Image load detection         | `cookbook/detect-img-onload.md`          |
| Drag and drop                | `cookbook/drag-and-drop.md`              |
| Fonts / FOIT / CLS           | `cookbook/fonts.md`                      |
| `import.meta.glob`           | `cookbook/glob-import.md`                |
| iOS media / audio playback   | `cookbook/media-controller.md`           |
| Active nav link              | `cookbook/nav-link.md`                   |
| Docker deployment            | `cookbook/node-docker-deploy.md`         |
| Modals / tooltips / portals  | `cookbook/portals.md`                    |
| Streaming / deferred loaders | `cookbook/streaming-deferred-loaders.md` |
| `sync$` / `preventDefault`   | `cookbook/sync-events.md`                |
| Dark/light theme toggle      | `cookbook/theme-management.md`           |
| View Transitions API         | `cookbook/view-transition.md`            |

### Integrations (read when user asks about a specific tool or library)

When the user is setting up or asking about a specific third-party tool, read
the matching file from `references/integrations/`:

| Tool / Library         | File                                     |
| ---------------------- | ---------------------------------------- |
| React (`qwikify$`)     | `integrations/react.md`                  |
| Auth.js / NextAuth     | `integrations/authjs.md`                 |
| Tailwind CSS v4        | `integrations/tailwind.md`               |
| Tailwind CSS v3        | `integrations/tailwind-v3.md`            |
| Vitest (unit tests)    | `integrations/vitest.md`                 |
| Playwright (E2E)       | `integrations/playwright.md`             |
| Cypress                | `integrations/cypress.md`                |
| i18n / translations    | `integrations/i18n.md`                   |
| Drizzle ORM            | `integrations/drizzle.md`                |
| Prisma ORM             | `integrations/prisma.md`                 |
| Supabase               | `integrations/supabase.md`               |
| Turso / libSQL         | `integrations/turso.md`                  |
| Modular Forms          | `integrations/modular-forms.md`          |
| Image optimization     | `integrations/image-optimization.md`     |
| Icons                  | `integrations/icons.md`                  |
| Partytown              | `integrations/partytown.md`              |
| Panda CSS              | `integrations/panda-css.md`              |
| PostCSS                | `integrations/postcss.md`                |
| styled-vanilla-extract | `integrations/styled-vanilla-extract.md` |
| Bootstrap              | `integrations/bootstrap.md`              |
| Storybook              | `integrations/storybook.md`              |
| Nx monorepo            | `integrations/nx.md`                     |
| OG image generation    | `integrations/og-img.md`                 |
| Orama search           | `integrations/orama.md`                  |
| Leaflet maps           | `integrations/leaflet-map.md`            |
| Builder.io CMS         | `integrations/builderio.md`              |
| Astro                  | `integrations/astro.md`                  |
| Tauri desktop app      | `integrations/tauri.md`                  |

### Labs / experimental (read for cutting-edge or experimental APIs)

When the user asks about experimental Qwik features, read the matching file
from `references/labs/`:

| Feature                             | File                           |
| ----------------------------------- | ------------------------------ |
| Qwik Insights (real-user analytics) | `labs/insights.md`             |
| Qwik Devtools / `qwik/json` parsing | `labs/devtools.md`             |
| Typed routes                        | `labs/typed-routes.md`         |
| `usePreventNavigate$`               | `labs/use-prevent-navigate.md` |

### Full documentation index

**`references/doc-index.md`** — complete index of all 132 pages under
`packages/docs/src/routes/docs/` with coverage status for each. Read when you
need to verify what's covered or to navigate to a specific topic area.
