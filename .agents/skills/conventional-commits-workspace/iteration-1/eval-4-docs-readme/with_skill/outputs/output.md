# Commit Message

## Recommended commit message

```
docs: add env vars section to README and fix typos in contributing guide
```

---

## Explanation

**Type: `docs`**
Both changes — adding a section to the README and correcting typos in the contributing guide — are documentation-only. No source code, tests, or build configuration changed, so `docs` is the correct type.

**Scope: omitted**
The changes touch two different documentation files (`README` and the contributing guide). A single scope would misrepresent one of the two files, so it's cleaner to leave it out. If you prefer scopes, split this into two commits (see alternative below).

**Description:**
Imperative present tense ("add … and fix …"), lowercase first letter, no trailing period. At exactly 72 characters it sits right at the limit — perfectly valid.

---

## Checklist verification

- [x] Type is `docs` (documentation only)
- [x] Header ≤ 72 chars (`docs: add env vars section to README and fix typos in contributing guide` = 72 chars)
- [x] Imperative present tense, lowercase first letter, no trailing period
- [x] No scope (intentionally omitted — changes span two files)
- [x] No breaking changes
- [x] No body needed (the header is fully self-explanatory)
- [x] No issue references to add

---

## Alternative: two focused commits

If you prefer each commit to touch exactly one concern, split it:

```
docs(readme): add environment variables section
```

```
docs(contributing): fix typos
```

Both use the optional `scope` to pinpoint which document changed, making `git log` and blame easier to read. This is the cleaner approach if your project values atomic commits.

---

## Quick reference

| Question | Answer |
|---|---|
| Why not `chore`? | `chore` is for tooling, config, and build changes — not prose docs. |
| Why not `fix`? | Fixing a typo in a doc is still a `docs` change; `fix` is reserved for code bugs. |
| Is a body needed? | No — the header describes both changes clearly on its own. |
