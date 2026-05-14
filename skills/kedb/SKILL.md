---
name: kedb
description: >
  Maintains a Known Error Database (KEDB) of documented errors, symptoms, workarounds, root causes, and solutions. READ mode (autonomous): on any error, stack trace, test/build
  failure, or unexpected behavior, consult `kedb-index.md` before investigating from scratch. Triggers: "this error", "is this a known issue", "check KEDB", "known error database",
  "have we seen this before". WRITE mode (human-invoked only): create or update entries via `kedb-template.md` when asked to "add to KEDB", "document this issue", "record this fix",
  "log this error", "save this workaround", or "create a KEDB entry". Always update `kedb-index.md` after writing. Never auto-create entries.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# KEDB Skill

You are an expert custodian of a Known Error Database (KEDB) — a structured corpus
of documented errors, symptoms, workarounds, root causes, and permanent solutions.
The KEDB exists so that future sessions do not have to re-solve problems that have
already been investigated. Past pain becomes future leverage.

This skill governs two distinct workflows:

- **Read** — autonomous lookup when troubleshooting.
- **Write** — human-invoked creation or update of entries.

Treat the two workflows as separate concerns. Reading happens silently and
constantly; writing happens only on explicit human request.

## File Map

| File                | Mutability | When to read                              | When to write                        |
| ------------------- | ---------- | ----------------------------------------- | ------------------------------------ |
| `SKILL.md`          | Immutable  | Loaded automatically when skill triggers  | Never                                |
| `kedb-template.md`  | Immutable  | Before creating a new entry               | Never                                |
| `kedb-index.md`     | Mutable    | Start of every read; start of every write | After every entry create/update      |
| `entries/<slug>.md` | Mutable    | When the index points to a matching entry | When creating or updating that entry |

The `entries/` subdirectory holds one markdown file per entry, named `<slug>.md`
where `<slug>` matches the `Slug` field in the entry's frontmatter.

## Workflow A — Reading the KEDB (autonomous)

Invoked whenever you encounter an error, stack trace, failing test, unexpected
behavior, or any debugging context. Run this before guessing or investigating
from scratch.

1. Read `kedb-index.md`.
2. Scan the `Description`, `Title`, and `Slug` columns of the index table for
   keywords that match the current symptom. Be liberal — partial matches are
   worth opening.
3. For each candidate, read `entries/<slug>.md` and compare the entry's `Description`,
   `Service Affected`, and `Context` sections against the current situation.
4. If a match is found:
   - Apply the documented `Workaround` or `Solution` if applicable.
   - If the recorded fix does NOT apply to the current case (e.g., versions have
     diverged, the environment differs, the workaround no longer works), do NOT
     silently move on. Tell the user: "KEDB entry `<slug>` looked relevant but
     does not apply because `<reason>`. The entry may need updating."
5. If no match is found, proceed with normal investigation. Do not create an
   entry yet — entry creation is a separate, human-invoked workflow (see B).

## Workflow B — Writing or Updating the KEDB (human-invoked)

Invoked ONLY when the user explicitly asks to record something in the KEDB.
Never run this workflow autonomously, even after solving a hard problem. The
user — or reviewer — decides what merits an entry.

1. Read `kedb-index.md` to enumerate existing entries.
2. Decide: **update an existing entry**, or **create a new one**:
   - **Update existing** if the symptom is similar to an entry already in the
     index, OR if parts of an existing entry are wrong, outdated, or incomplete.
   - **Create new** if the symptom is sufficiently different from every existing
     entry that grouping them would obscure the distinction.
3. If updating: read `entries/<existing-slug>.md`, modify the relevant sections,
   bump `Status` if appropriate, preserve `Event Date` (it records the original
   occurrence), and save.
4. If creating:
   1. Read `kedb-template.md`.
   2. Choose a kebab-case `<slug>` that is unique across `entries/`. The slug
      should be a concise hint at the symptom (e.g., `pg-connection-pool-exhaust`,
      `vite-hmr-stuck-after-rename`).
   3. Fill every section of the template. Sections that genuinely do not apply
      may say "N/A" with a brief justification — do not delete them.
   4. Save as `entries/<slug>.md`. The filename stem MUST equal the `Slug` field.
5. **Required final step — update `kedb-index.md`.** Add a new row (or modify
   the existing row) with `Slug`, `Title`, `Status`, `Severity`, `Event Date`,
   and a one-line `Description` (≤200 chars). An entry that is not in the index
   is invisible to future sessions and effectively does not exist.

If step 5 is skipped, the entry has been lost. Always finish what you started.

## Status Lifecycle

The `Status` field has exactly five permitted values:

- `Not Started` — entry created, no investigation has happened yet.
- `In Progress` — investigation is active; root cause not yet confirmed.
- `Resolved` — permanent fix applied and verified; the problem cannot recur in
  the documented context.
- `Resolved (Workaround)` — a workaround is in place; the root cause is known
  but a permanent fix is pending. The entry remains active.
- `Resolved (Cause Unknown)` — the symptom no longer occurs but the root cause
  was never identified. Keep the entry — the symptom may return.

Transitions are not enforced mechanically; use judgment. A `Resolved` entry can
be reopened (e.g., reverted to `In Progress`) if the problem recurs.

## Severity

Severity is a stable judgment about impact, not urgency:

- `High` — blocks production, causes data loss, or halts critical workflows.
  Affects many users or core revenue paths.
- `Medium` — significant friction for some users or a workflow; visible
  degradation. Workaround exists but is awkward.
- `Low` — cosmetic, minor inconvenience, edge case affecting few users, or
  internal-only impact with negligible business consequence.

When in doubt, choose the lower severity — `High` should mean something.

## Third-Party Issues

Entries documenting bugs in third-party software (dependencies, runtimes,
managed services, browsers, OS) follow stricter rules:

- The affected version range MUST be recorded in the `Context` section
  (e.g., "axios 1.7.0 – 1.7.4").
- The entry MUST remain in the index even after the upstream issue is fixed.
  Future sessions on older versions still need to find it.
- When the issue is fixed upstream, update `Solution` with the fixed version
  and set `Status` to `Resolved`. Do NOT delete the entry.

## Staleness

KEDB entries can become misleading as the codebase and environment evolve:

- During Workflow A, if an entry's `Context` no longer matches current reality
  (e.g., the runtime version has moved past the affected range), surface this
  to the user rather than silently ignoring the entry.
- Prefer updating a stale entry to abandoning it. A corrected entry continues
  to help; a stale entry quietly misleads.
- Do not delete entries except in narrow cases: duplicates that should be
  merged, or entries that were created in error. Resolved entries stay.

## Index Row Format

`kedb-index.md` is a single markdown table. Each row corresponds to one entry
file in `entries/`. Columns, in order:

| Slug | Title | Status | Severity | Event Date | Description |

- `Slug` — the kebab-case slug; also the filename stem (`entries/<slug>.md`).
- `Title` — human-readable; copied verbatim from the entry's frontmatter.
- `Status` — one of the five permitted values.
- `Severity` — `Low`, `Medium`, or `High`.
- `Event Date` — RFC 3339 timestamp from the entry's frontmatter.
- `Description` — one-line summary (≤200 chars). Strong enough that an agent
  scanning the index can decide whether to open the full entry without opening
  every candidate. Avoid raw `|` characters — escape with `\|` or rephrase,
  as unescaped pipes break the table's column count.

Note: individual entry files (`entries/<slug>.md`) use **YAML frontmatter** for
their metadata. The index file uses a **markdown table** for fast scanning.
Keep the two in sync — every entry file must have a matching index row, and
the row values must match the entry's frontmatter values.

Sort the table by `Event Date` descending (most recent first) so that the
freshest entries are at the top. Re-sort when adding new rows.

## Anti-patterns

- **Auto-creating entries.** Never. Even after solving a hard problem, wait
  for explicit human instruction. The reviewer decides what's worth recording.
- **Skipping the index update.** An entry not in the index is dead weight on
  disk. The index update is the act of publishing.
- **Vague descriptions.** A `Description` of "build broken" or "weird bug" is
  not searchable. Name the symptom: "pnpm install hangs on workspace protocol
  resolution behind corporate proxy".
- **Deleting resolved entries.** Resolved entries are how future sessions on
  old branches or old versions find the fix.
- **Inventing new status or severity values.** The enums are fixed.
