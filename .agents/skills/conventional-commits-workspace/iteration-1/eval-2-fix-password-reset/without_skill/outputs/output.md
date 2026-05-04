# Commit Message

```
fix: use milliseconds for password reset link expiry calculation

The expiry time was incorrectly calculated using seconds instead of
milliseconds, causing the password reset link to expire too early.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
```

## Explanation

This is a `fix` commit since it corrects a bug in existing behavior. The subject line is concise and describes what changed. The body provides context on what was wrong (seconds vs milliseconds) and the impact (link expiring too early).
