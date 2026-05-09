# Cross-Fade Transitions with the View Transition API in React + Vite

The [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transitions_API) lets you animate between two DOM states with a simple `document.startViewTransition()` call. No extra libraries needed.

---

## How It Works

1. Call `document.startViewTransition(callback)`.
2. Inside the callback, update React state (which causes a re-render).
3. The browser snapshots the old state, renders the new state, and cross-fades between them automatically.

---

## Project Structure

```
src/
  App.jsx
  App.css
  main.jsx
```

---

## `src/main.jsx`

```jsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.jsx'

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)
```

---

## `src/App.jsx`

```jsx
import { useState, useCallback } from 'react'
import './App.css'

const ITEMS = [
  { id: 1, title: 'Card One',   body: 'Details for card one. Lorem ipsum dolor sit amet.' },
  { id: 2, title: 'Card Two',   body: 'Details for card two. Consectetur adipiscing elit.' },
  { id: 3, title: 'Card Three', body: 'Details for card three. Sed do eiusmod tempor.' },
]

// Helper: wraps a state update in a View Transition when supported.
function withViewTransition(updateFn) {
  if (document.startViewTransition) {
    document.startViewTransition(updateFn)
  } else {
    updateFn() // graceful fallback for unsupported browsers
  }
}

export default function App() {
  const [selected, setSelected] = useState(null)

  const openDetail = useCallback((item) => {
    withViewTransition(() => setSelected(item))
  }, [])

  const closeDetail = useCallback(() => {
    withViewTransition(() => setSelected(null))
  }, [])

  return (
    <div className="app">
      {selected ? (
        // ── Detail panel ──────────────────────────────────────────────
        <div className="detail" key="detail">
          <button className="back-btn" onClick={closeDetail}>← Back</button>
          <h1>{selected.title}</h1>
          <p>{selected.body}</p>
        </div>
      ) : (
        // ── Card list ─────────────────────────────────────────────────
        <div className="list" key="list">
          <h1>Items</h1>
          <ul>
            {ITEMS.map((item) => (
              <li key={item.id} className="card" onClick={() => openDetail(item)}>
                <span>{item.title}</span>
                <span className="arrow">›</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  )
}
```

### Key Points

| Line | Why |
|------|-----|
| `document.startViewTransition(updateFn)` | Tells the browser to snapshot the current page, run `updateFn` (which triggers React's re-render), then animate between the two snapshots. |
| `if (document.startViewTransition)` | Feature-detection for browsers that don't support it yet (Firefox < 126, Safari < 18). |
| No `key` prop trickery needed | React's normal conditional rendering is enough; the API works at the DOM level. |

---

## `src/App.css`

```css
/* ── Layout ── */
.app {
  max-width: 480px;
  margin: 2rem auto;
  font-family: system-ui, sans-serif;
}

.list ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  margin-bottom: 0.5rem;
  background: #f5f5f5;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
}
.card:hover { background: #e8e8e8; }
.arrow { font-size: 1.4rem; color: #888; }

.detail {
  padding: 1rem;
}
.back-btn {
  background: none;
  border: none;
  font-size: 1rem;
  cursor: pointer;
  color: #0070f3;
  padding: 0;
  margin-bottom: 1rem;
}

/* ── View Transition animation ── */

/*
  The browser automatically creates two pseudo-elements during a transition:
    ::view-transition-old(root)  — snapshot of the outgoing state
    ::view-transition-new(root)  — snapshot of the incoming state

  Override them here to customise the animation.
*/

/* Default cross-fade (this is actually what the browser does by default,
   shown here explicitly so you can tweak it): */
::view-transition-old(root) {
  animation: 250ms ease-out both fade-out;
}

::view-transition-new(root) {
  animation: 250ms ease-in both fade-in;
}

@keyframes fade-out {
  from { opacity: 1; }
  to   { opacity: 0; }
}

@keyframes fade-in {
  from { opacity: 0; }
  to   { opacity: 1; }
}
```

### Tweaking the Animation

```css
/* Slide in from the right when opening detail */
::view-transition-new(root) {
  animation: 300ms cubic-bezier(0.4, 0, 0.2, 1) both slide-in;
}
@keyframes slide-in {
  from { opacity: 0; transform: translateX(40px); }
  to   { opacity: 1; transform: translateX(0); }
}
```

---

## Per-Element Transitions (named view-transitions)

If you want only the card title to animate (e.g., the title morphs into the detail heading) rather than the whole page cross-fading, use **named** view transitions:

```css
/* In App.css */
.card-title {
  view-transition-name: active-card-title; /* unique name */
}
.detail h1 {
  view-transition-name: active-card-title; /* same name → browser morphs between them */
}
```

The browser will automatically animate between the two elements with matching `view-transition-name`. Make sure only **one** element at a time carries each name (use inline styles or a state-driven class to apply it only to the selected card).

---

## Browser Support (as of 2025)

| Browser | Support |
|---------|---------|
| Chrome / Edge 111+ | ✅ Full support |
| Safari 18+ | ✅ Full support |
| Firefox 126+ | ✅ Full support |
| Older browsers | ✅ Graceful fallback (no animation, instant switch) |

---

## Summary

1. **Detect** `document.startViewTransition` for safety.
2. **Wrap** your `setState` call inside the transition callback.
3. **Customise** the animation with `::view-transition-old(root)` and `::view-transition-new(root)` CSS pseudo-elements.
4. Use `view-transition-name` on specific elements for per-element morph animations.

No extra dependencies, no React animation library — pure platform API.
