# Parallel execution

The Agent tool runs subagents concurrently when multiple `Agent`
calls appear in the **same assistant message**. This file covers
the batching pattern, rate-limit boundaries, and how to handle
partial failures.

## The single-turn fan-out

To run N candidate subagents in parallel, emit N `Agent` calls in
one assistant message:

```text
<assistant turn>
  Agent({description: "Run candidate v1 on eval-1", prompt: ...})
  Agent({description: "Run candidate v1 on eval-2", prompt: ...})
  Agent({description: "Run candidate v1 on eval-3", prompt: ...})
  ...
</assistant turn>
```

All N start concurrently. The assistant turn doesn't complete
until all N return. Wall time is roughly `max(durations)`, not
`sum(durations)`.

Same pattern for judges, but in the next turn — judges need the
candidate output files on disk before they can run.

## Two-phase pattern

```text
Turn 1 (main session): spawn N candidate subagents in parallel
Turn 2 (main session): verify all N output files exist
Turn 3 (main session): spawn N judge subagents in parallel
Turn 4 (main session): read N verdict JSONs, compute pass rate
```

If a candidate subagent failed to write its file in turn 1, you'll
see the gap in turn 2. Re-spawn that specific sample before
proceeding to judges.

## Rate-limit boundaries

Subagents share the user's rate-limit budget with the main
session. There is no published per-message cap on parallel
`Agent` calls. Observed behavior:

| Fan-out | Behavior |
| --- | --- |
| 1–5 | Reliable; no rate-limit hits |
| 5–15 | Reliable in practice; 15 candidates + 15 judges in two consecutive messages is field-confirmed |
| 15–25 | Usually works on Pro / higher tiers; watch for HTTP 429 inside subagents |
| > 25 | Batch into multiple messages |

The conservative ceiling used to be 10. Field experience pushed
that up: a 15-parallel candidate fan-out followed by a 15-parallel
judge fan-out (one assistant message per phase) completes reliably
on standard subscriptions without 429s. Bump cautiously and
verify on your tier before committing to larger fan-outs.

For an eval set of 30 samples, the cheapest pattern is now two
batches of 15, not three batches of 10. The main session waits
for each batch to fully complete before sending the next.

## Cost shape

Each subagent invocation consumes tokens at the same rate as the
main session model. Rough budget:

- A small candidate run (50-token prompt under test, 100-token
  output) = ~500 tokens of subagent context, billed at the
  current rate.
- A judge run with full input + output included = ~1–2k tokens.
- 30 samples × (candidate + judge) ≈ 30 × 2.5k = 75k tokens for
  one iteration pass.

Iteration cost scales linearly with eval-set size. The 20–30
sample range is the practical sweet spot — large enough to
distinguish real improvement from noise, small enough that 5
iteration cycles fit a typical session budget.

## Partial-failure handling

Subagents can fail for reasons unrelated to the prompt under
test:

- Rate limit (HTTP 429) inside the subagent — retry that sample.
- Subagent returned without writing the file — re-spawn with a
  reminder ("the file at {path} does not exist; please write it
  before returning").
- Subagent wrote malformed JSON in the judge step — re-spawn the
  judge for that sample with a stricter "JSON only, no fence,
  no commentary" instruction.

The main session should always verify before grading:

```bash
# Count candidate outputs
ls iteration-{N}/eval-*/candidate-v{V}.txt | wc -l

# Count judge verdicts
ls iteration-{N}/eval-*/judge-v{V}.json | wc -l

# Validate each judge JSON parses
for f in iteration-{N}/eval-*/judge-v{V}.json; do
  jq -e '.verdict | IN("correct", "incorrect")' "$f" > /dev/null || echo "BAD: $f"
done
```

If the count is short or any file is bad, re-spawn the offenders
**individually** — not the whole batch — to avoid burning tokens
on already-good samples.

## When NOT to parallelize

- **First 2–3 samples of a new candidate prompt.** Run them in
  series so you can inspect the output and catch a structurally
  broken prompt before fanning out across 30 samples.
- **When you're debugging the eval harness itself.** Sequential
  runs are easier to trace through.
- **When subagents are expected to use tools that compete for
  rate-limited external resources** (the same web API, the same
  database connection). Concurrency may produce flakey results
  unrelated to the prompt's quality.

## Idempotency

Subagents writing to a file via Write are idempotent — re-spawning
a sample overwrites the previous file. Take advantage of this:
when in doubt, just re-run the suspect sample. There's nothing
to clean up.

The exception is the pairwise judge, which has two calls per
sample with different position assignments. If you re-spawn
without changing the `call_index`, you'll overwrite one of the
two position results. Use distinct file paths
(`pairwise-1.json`, `pairwise-2.json`) to keep them separate.
