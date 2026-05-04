# Commit Message

```
refactor(api)!: rename getUser to fetchUser

The name `getUser` implied a synchronous local lookup, but the
function actually performs an async network fetch. Renaming to
`fetchUser` aligns with the project convention for async
data-retrieval operations and removes that ambiguity.

BREAKING CHANGE: `getUser()` has been removed and replaced by
`fetchUser()`. All SDK consumers must update their call sites:

  Before: getUser(id)
  After:  fetchUser(id)
```

---

## Explanation

### Type — `refactor`
The behavior of the function is unchanged — it still fetches the
same data and returns the same result. Only the public name changes.
`refactor` is the right type for an internal-or-API restructuring
that does not add new behavior or fix a bug.

### Breaking-change markers — `!` + `BREAKING CHANGE:` footer
Both markers are used for maximum visibility:

- The `!` after `refactor(api)` makes the breaking nature visible
  at a glance in changelogs and tooling (e.g. `semantic-release`
  will cut a MAJOR version bump).
- The `BREAKING CHANGE:` footer provides the migration instructions
  that SDK consumers need — old name, new name, and what to do.

This maps to a **MAJOR** SemVer bump (e.g. `1.x.x` → `2.0.0`).

### Scope — `api`
`api` identifies the public SDK surface as the affected area, making
it easy for consumers scanning a changelog to spot that this entry
affects them directly. If your project uses a different canonical
noun for the public SDK layer (e.g. `sdk`, `client`), substitute
that scope instead.

### Header length check
`refactor(api)!: rename getUser to fetchUser` = 44 characters ✓ (≤ 72)

### Checklist
- [x] Type is `refactor` (one of the six allowed)
- [x] Header ≤ 72 chars, imperative ("rename"), lowercase first letter, no trailing period
- [x] Scope `api` is a short lowercase noun in parentheses
- [x] Breaking change marked with `!` in header **and** `BREAKING CHANGE:` footer
- [x] Body separated from header by one blank line, wrapped at 72 cols
- [x] No issue refs embedded in description or body (footer is the right place — add `Refs: #<issue>` if applicable)
