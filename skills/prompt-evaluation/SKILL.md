---
name: prompt-evaluation
description: >
  Eval-driven prompt refinement for Anthropic / Claude API prompts.
  Turn vibes-based prompt tweaking into a measurable loop: dataset
  of inputs + golden answers, pick a grading approach (code-graded,
  model-graded, or both), scaffold a runnable eval (Python with the
  Anthropic SDK, or `promptfoo`), run it, analyze failures by
  category, propose targeted prompt edits, re-run and compare.
  Invoke whenever the user wants to evaluate, improve, compare,
  A/B test, regress-test, or systematically iterate on a prompt —
  even if they don't say "eval". Phrases like "is this prompt
  good?", "help me make this prompt better", "compare these two
  prompts", "my prompt fails on X", "set up tests for this prompt",
  "find the edge cases this classifier misses", "switch to Haiku
  without regressions" all qualify. Covers all three approaches
  from Anthropic's prompt-evaluations course: code-graded (exact
  match, set match, regex, custom Python), promptfoo built-in and
  custom assertions, and model-graded (LLM-as-judge) with rubric
  design.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# Prompt Evaluation

This skill is a **router and workflow**. It teaches the eval-driven
prompt refinement loop, then dispatches to one of the reference files
for the chosen grading approach or tool. Read the references on demand
— do not pre-load all of them.

## When this matters

Prompt engineering without evals is guesswork. Two prompts can both
"look good" on a handful of cherry-picked inputs and still differ by
20+ points on a real test set. The whole point of this skill is to
replace "v2 feels better than v1" with a number you can defend.

The user has come to you because they want one of these:

- **Improve a prompt** they already have, in a measurable way
- **Compare two or more prompts**, or the same prompt across models
- **Catch regressions** before they ship a prompt change
- **Find failure modes** on edge cases they haven't enumerated

In every case, the deliverable is the same shape: a runnable eval,
a baseline score, a proposed prompt change, and a new score.

## The loop you are running

1. **Capture the prompt under test and what it's supposed to do.**
   Get the exact prompt string (or template), the inputs it consumes,
   and a description of what a *correct* output looks like. If you
   only have a vague task description, ask for an example input and
   the user's ideal output — that's the first golden pair.

2. **Build a small but real dataset.**
   See `references/dataset_design.md`. Aim for ~20 cases to start
   (the course recommends 100+ for real production work, but small
   sets are fine for iteration). Include: typical cases, edge cases
   the user worries about, and a couple of adversarial inputs.

3. **Pick a grading approach.** Decision tree:

   - Output is a **fixed label, number, JSON shape, or extractable
     value** → **code-graded** (`references/code_graded.md`)
   - Output is **open-ended** (summary, explanation, refusal,
     rewrite, tone) → **model-graded** (`references/model_graded.md`)
   - **Both** kinds of criteria apply (e.g. "must contain a
     category label AND be polite") → use both assertions on the
     same test cases

4. **Pick a tool.** `references/promptfoo.md` covers the YAML-based
   `promptfoo` workflow (recommended when comparing prompts ×
   models, or when you want a browser dashboard).
   `references/code_graded.md` and `references/model_graded.md`
   show the equivalent Python + Anthropic SDK pattern (recommended
   when the eval is part of a larger Python codebase or CI script).

5. **Scaffold the eval and run it.** Produce a runnable artifact in
   the user's repo. Show them the command to run.

6. **Analyze failures, don't just report the score.** A pass rate
   is the headline. The interesting work is in the failure rows:
   group them, name the failure mode, and tie each mode to a
   specific prompt edit you'll propose. "We failed 4 of 20. Three
   of those are the model adding explanatory prose around the
   answer (fix: tighten the output-format instruction). One is a
   genuine reasoning error on the adversarial case (fix: try
   chain-of-thought)."

7. **Propose a v2 prompt with a clear hypothesis**, re-run on the
   *same* dataset, and report the delta. If the score went down,
   say so — don't paper over it.

## Decision points

### Code-graded vs model-graded — be honest about cost

Code-graded evals are **cheap, fast, deterministic, and
reproducible**. Use them whenever you can. The trap is forcing a
code-graded eval onto an open-ended task ("does this summary cover
the main points?") via brittle regex or keyword presence — that
measures something that isn't quite what you care about, and the
score will mislead you.

Model-graded evals are **expensive, slower, and have grader
variance**. Use them when the criterion genuinely needs language
understanding (tone, faithfulness, refusal quality, "is this
explanation grade-school appropriate?"). When you do, write a
rubric, not a single sentence — see `references/model_graded.md`.

When both apply, run both as separate assertions. promptfoo and
the Python pattern both support this naturally.

### promptfoo vs Python SDK — match the user's stack

Default to **promptfoo** if the user wants:

- a visual diff dashboard
- to compare multiple prompts × multiple models in one run
- to keep the eval declarative (YAML) and let non-engineers tweak
  test cases

Default to **Python + Anthropic SDK** if the user wants:

- the eval to live inside their existing Python codebase / CI
- programmatic control over inputs (e.g. sampling from a database)
- to keep the eval in a single language (no Node.js dependency)

If the user has no preference, ask. Don't pick for them silently.

### Dataset size

The course recommends ~100 input/golden-answer pairs for
production-quality evals. For an iteration loop with a human in
the loop, 10–30 is fine and is what you should start with — small
enough to scan by eye, large enough to expose real failure modes.
Grow it when you find a failure category not covered.

## Failure-mode analysis (the actual skill)

After the first run, do not just report `X/Y passed`. Read the
failing rows and classify them. Typical categories from the
course:

- **Format issues** — model adds prose around the answer, uses the
  wrong delimiter, misses required tags. Fix: tighten the
  output-format instruction; add a concrete example of the exact
  output shape; consider an `assistant`-prefilled message.

- **Reasoning errors on hard cases** — the model gets the easy
  cases but fails on tricky ones (the canonical example from the
  course: "the fox lost a leg and grew back two"). Fix: add
  chain-of-thought (`<thinking>...</thinking>` then
  `<answer>...</answer>`), then extract the answer via a transform.

- **Category confusion in classification** — the model picks the
  wrong label when two are close. Fix: expand category definitions
  in the prompt; add discriminating examples; consider letting the
  model emit multiple labels when applicable.

- **Subjective failures (tone, length, refusal)** — usually only
  visible to a model grader. Fix: add the missing constraint
  explicitly to the prompt with a positive example.

For each category, propose **one targeted prompt edit**, not a
rewrite. Then re-run. This is how you learn which edits actually
help vs. which just feel like they should.

## Output you produce

By the end of an interaction, leave the user with:

- A directory in their repo (often `evals/` or `prompt-evals/`)
  containing the dataset and eval config/script
- A documented command to run it (`npx promptfoo@latest eval` or
  `python evals/run.py`)
- A baseline score for the original prompt
- A revised prompt (often saved as `prompts.py` or alongside the
  original) with a one-line hypothesis about why it should help
- The new score and a short failure-mode summary

Templates you can copy from:

- `assets/promptfooconfig.template.yaml` — minimal promptfoo
  config covering prompts, providers, tests, transforms, and both
  code- and model-graded assertions
- `assets/python_eval.template.py` — minimal Python eval loop
  using the Anthropic SDK
- `assets/judge_prompt.template.md` — rubric-style judge prompt
  with score anchors and JSON output

## References

Load only what's relevant to the chosen approach:

- `references/dataset_design.md` — what makes a useful test set,
  how to source golden answers, edge-case checklist, when to grow
  the dataset
- `references/code_graded.md` — exact match, set match (for
  classification with multiple labels), regex, `<answer>`-tag
  extraction, custom Python assertions
- `references/model_graded.md` — `llm-rubric` built-in, writing a
  good judge prompt, score anchors, JSON-output parsing, bias
  pitfalls (position, verbosity, self-preference), choosing a
  judge model
- `references/promptfoo.md` — `promptfooconfig.yaml` anatomy,
  providers, prompts (inline / file / Python function), tests
  (CSV / inline / vars), assertions (built-in and custom),
  `defaultTest`, transforms, the `view` dashboard, comparing
  models

## What this skill is not

- **Not a benchmark like MMLU.** This evaluates *your* prompt on
  *your* task, not the model's general capability.
- **Not human-in-the-loop labeling.** The course mentions human
  grading (Workbench) but you, as Claude Code, are automating
  code- and model-graded approaches.
- **Not a substitute for production monitoring.** Evals catch
  regressions before deploy; logging catches them after.
