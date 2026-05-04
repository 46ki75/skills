# Conventional Commit Message

```
fix(auth): use milliseconds for password reset link expiry

The expiry was calculated in seconds instead of milliseconds,
causing reset links to expire ~1,000× sooner than intended.
Users were effectively getting a link that expired almost
immediately rather than after the configured TTL.
```

---

## Explanation

**Type — `fix`:** This is a clear bug fix; a reproducing test (e.g. a
reset link that should still be valid is rejected) would now pass.

**Scope — `auth`:** Password reset is part of the authentication flow.
`auth` is a stable, well-understood area-of-codebase noun. If your
project uses a more specific scope (e.g. `password-reset` or
`tokens`), substitute that instead.

**Header** (`fix(auth): use milliseconds for password reset link expiry`):
- Imperative present tense ✓
- Lowercase first letter ✓
- No trailing period ✓
- 55 characters — well within the 72-char limit ✓

**Body:** Added to explain *what* was wrong and *why* it mattered
(users got effectively-instant expiry), since the header alone
doesn't convey the severity or root cause.

**No breaking change:** This is a pure bug-fix restoring intended
behavior. Consumers don't need to change any code or config.

**No footer:** No issue number was provided. If you have one, add:
```
Closes: #<issue-number>
```

---

## Quick checklist

- [x] Type is `fix` (one of the six allowed types)
- [x] Header ≤ 72 chars, imperative, lowercase first letter, no trailing period
- [x] Scope `auth` is a short lowercase noun in parentheses
- [x] No breaking change (none needed)
- [x] Body separated by a blank line, wrapped at 72 cols
- [x] No issue refs provided — placeholder noted above
