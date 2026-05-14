---
name: type-b-skill-creator
description: >
  Create a Type B Proprietary Skill — a project-specific agent skill backed by a
  KEDB (Known Error Database) that accumulates failure knowledge without ever mutating
  its core instructions. Use this skill whenever the user wants to build a project-specific
  skill, capture domain-specific workflows, or create a skill that learns from past mistakes.
  Trigger even when the user doesn't say "Type B" explicitly — phrases like "make a skill
  for my project", "I want Claude to remember failures on this codebase", "create a domain
  skill with memory", "skill that improves over time", "project-specific skill", or
  "proprietary skill" all count.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0"
---

# Type B Proprietary Skill Creator

Guide the user through creating a **Type B Proprietary Skill** — a project-specific agent
skill built on a generic problem-solver baseline that accumulates knowledge through a
**KEDB (Known Error Database)** without ever modifying its own core instructions.

---

## What a Type B skill produces

```text
<skill-name>/
├── SKILL.md              ← immutable core instructions (written once)
├── kedb-template.md      ← immutable entry template (written once)
├── kedb-index.md         ← mutable index, starts empty
└── kedb/                 ← empty directory for future KEDB entries
```

`SKILL.md` and `kedb-template.md` are never patched by the skill itself. Only
`kedb-index.md` and files under `kedb/` grow over time as the skill encounters
and records failures.

---

## Step 1 — Interview the user

Before writing any files, collect these five pieces of information:

1. **Skill name** — what should this skill be called?
2. **Domain** — what project or problem space does it operate in?
3. **Primary task** — what should this skill enable Claude to do?
4. **Trigger contexts** — what user phrases or situations should activate this skill?
5. **Known constraints or pitfalls** — anything already known that should seed the KEDB?

If the user says "generic for now" or "start blank", proceed with a domain-neutral
template and note that the KEDB will start empty.

---

## Step 2 — Write `SKILL.md`

Create `<skill-name>/SKILL.md`. The frontmatter must include `name`, `description`,
`license`, and `metadata` (with `author` and `version`). Fill every `<placeholder>`
from the interview.

The file must contain these sections:

### Frontmatter

```yaml
name: <skill-name>
description: >
  <What this skill does and when to use it. Be specific about trigger contexts.
  Mention that it consults a KEDB of past failures before acting.>
license: MIT
metadata:
  author: "<author-name>"
  version: "1.0"
```

### Body

**Heading** — one `# <Skill Name>` heading followed by a paragraph describing the domain.

**Execution Protocol** — three subsections in a `## Execution Protocol` section:

`### Before acting — always`

1. Read `kedb-index.md`.
2. If any entry title matches the current task or context, read that entry from `kedb/`
   before proceeding. Apply its guidance.
3. If multiple entries match, read all of them.

`### During execution`

Describe the skill's primary workflow as a numbered sequence of concrete steps. Tailor
this to the domain collected in Step 1. A good generic baseline:

1. Understand the user's goal. Ask clarifying questions if the input is ambiguous.
2. Identify the relevant tools, files, or services needed.
3. Execute the task step by step, verifying each step before continuing.
4. If an unexpected failure occurs, diagnose before retrying.
5. Summarize what was done and the outcome.

`### After execution — on failure or unexpected behavior`

If the task failed or produced incorrect output:

1. Diagnose the root cause.
2. Check whether an existing KEDB entry covers this case.
   - Yes → Note that the entry did not prevent the failure; flag it for update.
   - No → Proceed to create a new entry.
3. Read `kedb-template.md`.
4. Create a new file in `kedb/` following the template. Use a short kebab-case filename
   (e.g., `kedb/wrong-api-param.md`).
5. Append a new row to `kedb-index.md` referencing the new entry.
6. Inform the user that a KEDB entry was created and what it records.

**KEDB Reference table** — close `SKILL.md` with this table:

| File | Mutable | Purpose |
| --- | --- | --- |
| `kedb-index.md` | Yes | One-line summary per entry; read first to decide which entries to load |
| `kedb/<entry>.md` | Yes | Full entry detail; read only when index matches current context |
| `kedb-template.md` | No | Template for new entries; read before creating any entry |
| `SKILL.md` | No | This file; never modify |

---

## Step 3 — Write `kedb-template.md`

Create `<skill-name>/kedb-template.md` with exactly the content below. This file is
immutable — never modify it after creation. It defines the shape of every future KEDB entry.

```markdown
# KEDB Entry: <title>

## ID

KEDB-<NNN>   ← zero-padded sequential number (e.g., KEDB-001); check kedb-index.md for next available

## Date

<YYYY-MM-DD>

## Context

<What was being attempted when this failure occurred? 1–3 sentences.>

## Symptom

<What went wrong? What did Claude do incorrectly or unexpectedly?>

## Root Cause

<Why did it happen? Be precise — wrong parameter, missing step, bad assumption, etc.>

## Resolution

<What was done to recover in this session?>

## Prevention

<What should Claude do differently next time to avoid this? Write as an actionable rule.>

## Affected Skill Step

<Which step in SKILL.md's Execution Protocol is implicated? e.g., "Step 2 — identify tools">
```

---

## Step 4 — Create `kedb-index.md`

Create `<skill-name>/kedb-index.md` with exactly this content:

```markdown
# KEDB Index

| ID | Title | File | Date |
|---|---|---|---|

<!-- Add one row per entry when created. Format:
| KEDB-001 | Short description of the issue | kedb/filename.md | YYYY-MM-DD |
-->
```

---

## Step 5 — Create the `kedb/` directory

If using Git, create `<skill-name>/kedb/.gitkeep` as a placeholder so the empty directory
is tracked. For other version-control systems, use their equivalent mechanism to preserve
the empty directory (e.g., a `.keep` file). If no VCS is in use, simply create the directory.

---

## Step 6 — Seed the KEDB (optional)

If the user provided known pitfalls during Step 1, create initial KEDB entries for them
now using `kedb-template.md` as the shape. Assign IDs starting at `KEDB-001` and update
`kedb-index.md` with one row per entry.

If no pitfalls were provided, leave the KEDB empty.

---

## Step 7 — Confirm with the user

Present the generated file tree and the contents of `SKILL.md`. Then ask:

1. Does the primary workflow (Execution Protocol) match what you expect?
2. Are there any known failure modes to seed into the KEDB now?
3. Any changes to the trigger description?

Revise based on feedback before finalizing.

---

## Design principles to preserve

- **SKILL.md is immutable** — the skill learns through KEDB entries, not by patching its
  own instructions. This keeps it safe to replace with an upstream Generic (Type A) improvement.
- **KEDB is consulted proactively** — before acting, not only after failure.
- **Index-first loading** — `kedb-index.md` is always read first; individual entries are
  loaded only when relevant, keeping context overhead low.
- **Entries are append-only** — existing entries should only be updated to improve clarity
  or add new resolution steps; the original symptom/root-cause record is preserved.
