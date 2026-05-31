# Shared methodology with `prompt-evaluation`

This skill (subagent-based) and the SDK-based `prompt-evaluation`
skill share most of their methodology. They differ only in the
**execution venue** — Claude Code Agent vs. Anthropic SDK / promptfoo.
The dataset design, judge design, calibration discipline, and
iteration patterns are identical.

When you need the methodology in depth, read from the SDK-based
skill — these files are more complete on the underlying
techniques:

| Topic | Read in `prompt-evaluation/` |
| --- | --- |
| Real-failure-first dataset design | `references/dataset_design.md` |
| Stratification (feature × scenario × persona) | `references/dataset_design.md` |
| Capability vs. regression suites | `references/dataset_design.md` |
| Binary judges, why > Likert | `references/model_graded.md` |
| Bias taxonomy + Claude-specific effect sizes | `references/model_graded.md` |
| Calibration with Cohen's κ | `references/model_graded.md` |
| Reference-guided judging | `references/model_graded.md` |
| Position swap (canonical pattern) | `references/model_graded.md` and `assets/judge_prompt.template.md` |
| RAG metrics (faithfulness, answer-relevance, context-precision) | `references/rag_evals.md` |
| Tool-use / agent eval patterns | `references/tool_use_evals.md` |
| Production deployment patterns | `references/production_patterns.md` |
| Cost levers (prompt caching, batch, judge cascade) | `references/code_graded.md`, `references/production_patterns.md` |

## When to port from this skill to the SDK-based one

Use this skill for **exploratory iteration**. Port when the
prompt is approaching ship-readiness and you need:

- **Reproducibility outside Claude Code.** CI runs, scheduled
  regression sweeps, non-Claude-Code teammates running the eval.
- **Exact API semantics.** `system` vs `user` role distinction,
  precise model ID pinning, `output_config.format` (Structured
  Outputs), `temperature: 0`, prompt caching.
- **Larger eval sets.** Above ~50 samples per iteration, the
  SDK's async-with-semaphore pattern is more efficient than
  batched subagent fan-out, and the Batch API is ~50 % cheaper
  for offline sweeps.
- **Production telemetry.** Online eval, shadow scoring,
  eval-on-PR with junit.xml. None of these are practical from
  inside Claude Code.

The port is largely mechanical: the eval set already has the
shape needed; the judge prompts already follow the canonical
templates. You're moving from `Agent` calls to
`messages.create()` calls and from file-based output to
structured returns.

## Methodology that's identical in both skills

These are the principles that apply regardless of venue:

- **Look at the data first.** Read 20–50 real failures before
  designing the eval. Don't practice "eval-driven development"
  in the abstract.
- **Binary judges by default.** Anchored numeric only when you
  genuinely need granularity. Never 1–10 or 0–100.
- **Reason before verdict.** CoT-before-scoring reliably improves
  judge accuracy.
- **One isolated judge per criterion.** Compound rubrics produce
  halo effects.
- **Reference-guided when a reference exists.** Cheapest accuracy
  win available.
- **Position swap + agreement gate for pairwise.** Always.
- **Calibrate against your own labels.** Target Cohen's κ ≥ 0.6.
  Re-calibrate quarterly or whenever the judge model / rubric
  changes.
- **Lock the eval set and criteria between iterations.** Criteria
  drift is the most common form of self-deception.
- **One targeted edit per iteration.** Bundled edits make
  attribution impossible.

If you remember nothing else from the eval literature, remember
those nine.

## Methodology that's unique to this skill

The subagent venue adds a few concerns that don't appear in the
SDK-based skill:

- **File-based output contract** (Write + DONE).
- **Tool-block constraints** to prevent the subagent from
  consulting external resources.
- **Verification before grading** — confirm output files exist
  and parse before computing pass rate.
- **Re-spawn for partial failures** instead of retrying the
  whole batch.
- **Awareness that Claude Code's system prompt is always in
  effect** — this skill is an approximation of bare-API
  semantics, not a perfect simulation.

These are documented in `pitfalls.md` and inline in the candidate
and judge subagent reference files.
