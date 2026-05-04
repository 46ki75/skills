# Commit Message

```
refactor!: rename getUser() to fetchUser() in public API

BREAKING CHANGE: The `getUser()` function has been renamed to `fetchUser()`
across the entire codebase. SDK consumers must update all call sites from
`getUser()` to `fetchUser()`.
```

---

## Explanation

- **`refactor!`** – The `!` suffix signals a breaking change. The `refactor` type is appropriate because this is a rename with no behavior change.
- **Short subject line** – Concise summary of what changed and why it matters (`in public API` signals the scope of impact).
- **`BREAKING CHANGE:` footer** – Required by Conventional Commits to formally declare a breaking change, triggering a major version bump in semantic versioning tools. It clearly states what changed and what action consumers must take.
