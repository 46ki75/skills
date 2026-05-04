# Conventional Commit Message

## Commit Message

```
feat(users): add pagination support to GET /users endpoint

Accept `page` and `limit` query params to control result windows.
Return a `meta` object in the response body containing the total
record count, enabling clients to calculate page ranges.
```

---

## Explanation

**Type — `feat`:** Pagination is a new user-facing capability; consumers can now request a specific page of results and know the total count. This warrants a `feat` (MINOR SemVer bump).

**Scope — `users`:** The change is contained to the `/users` API route, so `users` is a clear, stable area-of-the-codebase noun that adds real clarity.

**Header check:**
- ✅ Type: `feat` ✓
- ✅ Header length: 48 chars (≤ 72) ✓
- ✅ Imperative, lowercase first letter, no trailing period ✓

**Body:** Explains *what* the new params do and *why* the `meta` object matters for clients — the diff will show *how*, so this focuses on intent.

**Breaking change?** Not marked as breaking because:
- Existing calls to `GET /users` without `page`/`limit` still work (additive change).
- If your API returns *all* records by default today and pagination would now silently cap responses, that **would** be breaking — add `!` and a `BREAKING CHANGE:` footer in that case.

**Scope note:** If your codebase uses a different convention (e.g., `api` instead of `users`), swap the scope accordingly.
