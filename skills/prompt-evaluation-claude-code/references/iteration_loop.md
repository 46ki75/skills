# Iteration loop

After the first run you have: raw outputs, judge verdicts, and a
pass rate. The synthesis pass turns those into a targeted prompt
revision. This file covers failure-mode clustering, edit
discipline, and how to read run-over-run comparisons.

## Read the data first

Before proposing any edit, read **every failed** `candidate-v{V}.txt`
in full. Skim is not enough. The point of a small eval set is that
the iteration cost is low precisely because you can afford to read
every failure.

For each failure, write a one-line note in `iteration-N/notes.md`:

```text
eval-3 (input: "Your service is down and I urgently need a CSV
export feature") → output classified only as "Service Outage";
missed the embedded "Feature Request". Mode: undercounts
multi-label inputs.

eval-7 (input: "I forgot how to log in") → output classified as
"Software Bug"; should be "User Error". Mode: schema collapse —
default to most populated category.

eval-11 (input: "App crashed AGAIN") → refused to classify,
asked for clarification. Mode: over-asks instead of using the
context.
```

These notes are the raw material for clustering.

## Cluster by failure mode

Group the per-sample notes into 2–4 themes. The themes are usually
recognizable on inspection — you don't need a formal taxonomy,
just enough granularity to write distinct edits.

Common cluster shapes:

- **Schema confusion** — prompt allows multiple categories but
  output picks one; or schema mentioned in instructions doesn't
  match what the prompt asks the model to output.
- **Over-refusal / over-questioning** — prompt is too cautious,
  asks for clarification when the input is sufficient.
- **Under-following format** — prompt asks for JSON but model
  emits prose with JSON embedded.
- **Misses a feature the prompt mentions** — instruction is
  there but it's buried; model treats it as optional.
- **Hallucinated categories / facts** — model invents an answer
  outside the schema or unsupported by the input.

(Source: Hamel Husain's "open coding" — let the failure modes
emerge from the data, don't impose a pre-built taxonomy.)

## Propose one targeted edit per dominant mode

The disciplined move: ship one prompt change per iteration, where
the change targets the **most common** failure mode (or, if two
are tied for most common, whichever is easier to fix without
risking a regression on the other).

Do **not** bundle five edits into one revision. If you do, and
the pass rate changes:

- You won't know which edit caused the change.
- You won't be able to roll back the regression-causing edit
  without losing the others.
- The next iteration becomes a multi-variable mess.

A good single edit usually looks like:

- Adding one explicit instruction ("If the complaint contains
  more than one category from the schema, return all that
  apply, comma-separated, ordered by salience.")
- Adding one negative instruction ("Do not ask clarifying
  questions; classify based on what is provided.")
- Adding one example to a few-shot block.
- Sharpening one ambiguous word in the prompt.

Save the new version as `prompts/candidate-v(N+1).md`. The old
version stays around — you'll re-run both in iteration N+1 if
you want to confirm the improvement is real.

## Re-run on the same eval set

Lock the eval set between iterations. Don't add or remove
samples; don't sharpen the criterion. The pass rate comparison
is only meaningful if the measurement instrument hasn't changed.

Spawn N candidate subagents for v(N+1) (and optionally re-spawn
v(N) for stability), then judges. Compute the per-sample diff:

```text
iteration-1 → iteration-2:
- 18/20 passes (90 %) ← v1
- 19/20 passes (95 %) ← v2
- v2 fixed: eval-3, eval-7
- v2 broke: eval-12 (previously passed)
- Net: +1 sample
```

The headline number is +1. The per-sample diff tells the real
story: two fixes, one regression.

## Per-mode tracking, not just per-pass-rate

The pass rate compresses many failure modes into one number. A
revision that fixes 3 instances of mode A but breaks 2 instances
of mode B nets +1 in the headline — and may be the wrong direction
to go.

Track per-mode pass rate across iterations:

| Mode | v1 fail count | v2 fail count |
| --- | --- | --- |
| Multi-label undercounted | 5 | 2 |
| Schema collapse | 2 | 1 |
| Over-asks | 3 | 0 |
| Format-following | 2 | 4 ← regression |

This makes it obvious that v2 traded one mode for another. The
right next move is targeted: keep v2's multi-label / over-asks
improvements, recover v1's format-following discipline.

## When the pass rate moves but you don't know why

Two things can cause an unexplained delta:

- **Judge variance.** Even a calibrated judge has noise — Cohen's
  κ of 0.7 still means 15 % of verdicts disagree with humans. On
  a 20-sample set, that's 3 samples of expected disagreement.
  Re-run the judge on the same outputs in a fresh subagent —
  if the verdict flips, the sample was on the κ-noise margin.
- **Subagent variance.** Same prompt under test, same input,
  different sampled token at temperature > 0 → different output.
  For exploratory iteration, accept this; for ship-grade
  measurement, set temperature = 0 in the prompt under test (and
  state that constraint in `prompts/candidate-v{V}.md`).

If the delta is within the noise band of these two sources,
don't celebrate or panic. Run a third iteration and look at the
trend.

## When to stop iterating

Stop when one of these is true:

- **Pass rate plateaued for 2 iterations** at a level the user
  considers shippable.
- **You ran out of failure modes you know how to fix.** The
  remaining failures are genuinely hard cases that need new
  capabilities, not better prompting.
- **The user is happy.** This is the actual stopping criterion;
  pass rate is a proxy.

Once you stop iterating, **codify the eval as a regression suite**.
For Claude Code-internal use, the current setup is enough. For
CI / shared / cross-team use, port to the SDK-based
`prompt-evaluation` skill (`promptfoo` or Python).

## Source citations

- Hamel Husain, *Field Guide to Rapidly Improving AI Products*:
  open coding of failures; one targeted edit per cycle.
- Anthropic Engineering, *Demystifying Evals for AI Agents*:
  per-mode tracking; capability vs. regression suites.
- Shreya Shankar et al., *Who Validates the Validators?*:
  why iteration without locking the eval set produces
  self-deception.
