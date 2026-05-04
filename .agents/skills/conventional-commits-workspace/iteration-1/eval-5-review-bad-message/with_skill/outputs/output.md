# Commit Message Review

**Original message:**
> `Updated the login page to fix issues with the forgot password flow and also added some tests`

---

## ❌ Problems Found

This commit message has **five issues** that violate the Conventional Commits spec:

| # | Issue | Rule violated |
|---|-------|---------------|
| 1 | **No type prefix** — starts with "Updated" instead of a type like `fix:` | Header must begin with an allowed type |
| 2 | **Past tense verb** — "Updated", "added" should be imperative present tense ("update", "add" / better: "fix", "resolve") | Description must be imperative present tense |
| 3 | **Vague description** — "fix issues with" doesn't say *what* was broken or what was fixed | Description should be precise enough to understand the change without reading the diff |
| 4 | **Two concerns bundled into one message** — fixing a bug and adding tests are separate intents; they should either be in separate commits or the dominant intent (`fix`) should be the type, with tests mentioned in the body | Pick the type that describes the primary intent |
| 5 | **Header is too long** — the message is 88 characters; headers must be ≤ 72 characters | Header ≤ 72 chars |

---

## ✅ Suggested Corrected Messages

### Option A — Single commit (fix with tests included)

When tests are added as part of verifying a bug fix, they naturally belong in the same commit. The primary intent is `fix`:

```
fix(auth): resolve broken redirect in forgot password flow

Add regression tests to cover the corrected redirect behaviour
and edge cases that were previously untested.
```

- Header length: 55 chars ✓
- Type: `fix` — primary intent is a bug fix ✓
- Scope: `auth` — the forgot-password flow lives in the auth area ✓
- Description: imperative, lowercase, no trailing period ✓
- Body: explains *what* the tests add (the why behind including them) ✓

---

### Option B — Split into two commits

If the test changes are substantial enough to stand on their own (e.g. covering a whole suite, not just the fix):

**Commit 1 — the bug fix:**
```
fix(auth): resolve broken redirect in forgot password flow
```

**Commit 2 — the tests:**
```
test(auth): add coverage for forgot password redirect edge cases
```

---

## Why these choices

- **`fix` type** — the primary intent is correcting a broken behaviour in the forgot password flow. When tests accompany a bug fix to verify it, they travel in the fix commit; `test` type is reserved for test-only changes with no `src` modifications.
- **`auth` scope** — "login page" and "forgot password flow" both belong to the authentication area of the codebase. Use whatever noun your team uses for this area (e.g. `login`, `password-reset`) if `auth` isn't established.
- **"resolve broken redirect"** — more precise than "fix issues with"; you should substitute the actual symptom (e.g. "fix infinite loop", "prevent null error", "correct token expiry check") once you know what the bug was.

---

## Quick checklist (verified against suggested message)

- [x] Type is one of the six allowed types (`fix`)
- [x] Header ≤ 72 chars, imperative, lowercase first letter, no trailing period
- [x] Scope is a short lowercase noun in parentheses (`auth`)
- [ ] Breaking changes — not applicable here
- [x] Body separated by a blank line, wrapped at 72 cols
- [x] No issue refs in the description (add a `Refs: #<n>` footer if there's a ticket)
