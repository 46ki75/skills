# Eval set design

The dataset is the eval. Everything downstream — judges, pass
rates, iteration decisions — is only as good as the inputs you
chose to grade against.

## Format

A JSONL file. One sample per line. Minimum fields:

```json
{"id": "eval-1", "input": "...", "criterion": "..."}
```

Optional fields:

```json
{
  "id": "eval-1",
  "input": "...",
  "criterion": "the answer is a single category from the schema, lowercase",
  "golden_answer": "software bug",
  "tags": ["multi-label", "ambiguous"],
  "notes": "from production failure 2026-04-02"
}
```

- `id` — used as the directory name (`eval-1/`). Keep it stable
  across iterations so per-sample comparisons line up.
- `input` — the user message the candidate prompt will receive.
  Paste it verbatim; don't paraphrase.
- `criterion` — what makes an output correct. Goes into the judge
  prompt. One sentence. Avoid "good" or "high quality" — those
  produce noisy judges.
- `golden_answer` — only when the task has a single correct
  reference. Reference-guided judging drops failure rates
  dramatically on tasks like math or factual QA (MT-Bench:
  70 % → 15 %). When there's no single correct reference (open-
  ended writing, summarization), leave it out.
- `tags` — free-form labels you'll cluster failures by ("multi-
  label", "ambiguous", "long-input", "tool-use-required").
  Important for failure-mode analysis later.
- `notes` — for human reading only; ignored by the runner.

## Sizing

| Phase | Target size | Notes |
| --- | --- | --- |
| First-pass exploration | 5–10 | Just enough to spot the obvious failure modes |
| Iteration loop | 20–30 | Stable enough to distinguish "real improvement" from noise |
| Pre-ship regression | 50–200 | Codify before shipping; port to the SDK-based skill |

Don't start with 200. The marginal value of sample 51 in
exploration is near zero, and judge-subagent cost scales linearly.

## Sourcing

In priority order:

1. **Real production failures** if the user has them. Inspect them
   first — failure clusters tell you what the eval set should
   stratify on.
2. **Hand-crafted "I know this should work" cases** representing
   the happy paths. ~30–50 % of the set.
3. **Hand-crafted hard cases** that you suspect the prompt will
   fail on. Boundary conditions, ambiguous inputs, multi-label,
   refusals.
4. **Synthetic generation as a complement only.** Generated inputs
   tend to be too uniform and miss the long tail. Use them to
   pad coverage of features you've identified, not to discover
   features.

## Stratification

A good eval set covers the **cross-product** of:

- **Feature**: which capability of the prompt this exercises
  (classification, extraction, refusal, tool-call).
- **Scenario**: which input shape (short, long, noisy, multi-
  lingual, ambiguous).
- **Persona**: which user (technical user, novice, adversarial).

You don't need every cell of the cross-product. You do need to
notice gaps — if all 20 samples are short and English, you can't
claim anything about the prompt's behavior on long Japanese
inputs.

## Capability vs. regression suites

Two suites, two purposes:

- **Capability suite** — "does this prompt do the thing we want?"
  This is what you iterate on during development. Pass rate is a
  signal about the prompt's design.
- **Regression suite** — "did this change break anything that used
  to work?" Frozen. Touched only when a deliberate behavior
  change is intended. Pass rate is a signal about the safety of a
  prompt edit.

In the exploration loop this skill targets, you're working on the
capability suite. Once the prompt is shipping, promote the cases
that the prompt now passes into a regression suite (and port the
runner to the SDK-based skill so it can run in CI).

## Versioning

The eval set is the measurement instrument. Pass rates are only
comparable across iterations when they were measured against the
**same** set. The cheapest way to enforce that is a version-numbered
filename, bumped any time you add, remove, or modify a case.

```text
<workspace>/
├── eval-set-v1.jsonl           ← initial set
├── eval-set-v2.jsonl           ← after adding adversarial cases
├── eval-set-v3.jsonl           ← after fixing a criterion
├── eval_set.jsonl.notes.md     ← optional: provenance per sample
└── iteration-N/
    └── results.md              ← records: "ran candidate-vX against
                                  eval-set-v2.jsonl, pass rate = …"
```

The discipline:

- **Bump the file (don't mutate)** when you change the eval set —
  even by one sample, even just sharpening a criterion. The old
  file stays on disk; the new file is what subsequent iterations
  run against.
- **Stamp every iteration's `results.md` with the eval-set version
  it used.** This is the single most important thing for honest
  prompt-version comparisons: a v3 prompt scoring 24/26 on
  `eval-set-v3` is not "better than" a v2 prompt scoring 22/22
  on `eval-set-v2` — they were measured against different
  instruments.
- **Reset baselines when the eval set bumps.** When you go from
  `eval-set-vN` to `eval-set-v(N+1)`, re-run the current
  candidate prompt against `eval-set-v(N+1)` so the next
  iteration has a fair baseline to compare against.
- **Cases keep stable IDs across versions.** `eval-7` in v2 and
  `eval-7` in v3 must refer to the same case. If a case is
  retired, mark it (`"retired": true`) but keep the slot; don't
  reuse the ID.

A separate per-iteration snapshot (`eval-set-snapshot.jsonl` copied
into the iteration directory) is optional belt-and-braces. The
versioned filename is the primary record; the snapshot is insurance
against accidental mid-iteration edits.

### Concrete example

```text
iteration-3/results.md:
  candidate-v2 on eval-set-v1.jsonl (12 cases) — 11/12 pass
  → failure mode: "stale-knowledge trap"
  → action: added 4 adversarial cases as eval-set-v2.jsonl

iteration-4/results.md:
  candidate-v2 on eval-set-v2.jsonl (16 cases) — 11/16 pass
  candidate-v3 on eval-set-v2.jsonl (16 cases) — 15/16 pass
  → v3 is the new shipping baseline

iteration-5/results.md:
  candidate-v3 on eval-set-v2.jsonl — 22/22 pass after regression
  sweep (combined record across cases first introduced in v1)
```

The shape that's *not* allowed: comparing "v2 at 11/12 on v1"
against "v3 at 15/16 on v2" and declaring a +33 % improvement.
The denominators are different sets.

## Common mistakes

- **Vague criteria.** "The answer is good" produces near-random
  judge verdicts. Specify the observable property.
- **All-happy-path samples.** Pass rate looks great, prompt
  regresses on the long tail in production.
- **Criteria drift mid-iteration.** Don't sharpen the criterion
  between v1 and v2 — you'll think v2 improved when really the
  rubric got easier. Lock the criterion before measuring.
- **Bundling multiple criteria into one.** "The answer is
  faithful AND concise AND relevant" produces halo-effect verdicts.
  One criterion per judge. See `judge_subagents.md`.
- **Mixing exploration and regression samples.** Exploration
  samples change as you learn; regression samples must not.
  Keep them in separate files.

## Source citations

- Hamel Husain, *LLM Evals FAQ*: "Source realistic tasks from the
  failures you see."
- Anthropic Engineering, *Demystifying Evals for AI Agents*:
  capability vs. regression suites; "Start early and don't wait
  for the perfect suite."
- Shreya Shankar et al., *Who Validates the Validators?*:
  criteria drift; the rubric you ship with is rarely the rubric
  you started with.
