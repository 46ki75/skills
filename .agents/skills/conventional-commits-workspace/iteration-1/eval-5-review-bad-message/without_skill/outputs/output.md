# Commit Message Review

**Original message:** `Updated the login page to fix issues with the forgot password flow and also added some tests`

## Issues

1. **Too vague** – "fix issues" and "added some tests" don't explain *what* was broken or *what* the tests cover.
2. **Multiple concerns in one commit** – Fixing a bug and adding tests are two separate things; ideally split into separate commits (or at least clearly labeled).
3. **No conventional commit type** – Missing a type prefix (e.g., `fix:`, `test:`), making it hard to parse in changelogs or CI tooling.
4. **Filler words** – "Updated the login page to" adds noise without adding meaning. Prefer imperative mood.
5. **Long and rambling** – Uses "and also" to chain unrelated changes.

## Suggested Messages

If keeping as a single commit:

```
fix(auth): resolve forgot-password flow on login page and add tests
```

Or split into two commits:

```
fix(auth): resolve forgot-password redirect on login page
```
```
test(auth): add tests for forgot-password flow
```
