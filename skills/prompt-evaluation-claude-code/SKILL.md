---
name: prompt-evaluation-claude-code
description: >
  Eval-driven prompt refinement that runs entirely inside Claude
  Code via the Agent/Task tool — no Python, no SDK, no API key.
  Each candidate run and each judge call executes in an isolated
  subagent with a fresh context window, so samples are
  independently graded and the main session stays focused on
  synthesis and iteration. Trivially parallel: spawn N candidate +
  M judge subagents in one assistant message. Invoke when the user
  wants to evaluate, A/B test, regress-test, or iterate on a prompt
  directly inside Claude Code, especially when they reference
  subagents, the Task/Agent tool, or "test this prompt without
  writing code". Phrases like "use Claude Code to evaluate this
  prompt", "spawn subagents to test", "parallel-test these variants",
  "A/B these prompts in Claude Code", "grade this rubric with
  subagents", or "iterate this prompt with fresh contexts" qualify.
  Pairs with the broader `prompt-evaluation` skill for shared
  dataset-design and binary-judge methodology.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.2.0"
---

# Prompt Evaluation — Claude Code

This skill is a **router and workflow**. It teaches how to run a
prompt-evaluation loop using Claude Code's own subagent capability
— no external API calls, no Python, no `promptfoo`. Read the
references on demand.

## The core idea

Claude Code's `Agent`/`Task` tool spawns a subagent with a **fresh
context window**. The subagent sees only the prompt you pass it; it
inherits nothing from the main conversation. When the subagent
finishes, it returns a single message to the caller and its working
context is discarded.

For prompt evaluation, this gives you three properties for free
that are otherwise hard to engineer:

1. **Isolation.** Each test sample runs in its own context, so the
   main session's accumulated context cannot leak into the run.
   The same holds for judges: each verdict is independent of the
   others.
2. **Parallelism.** Multiple `Agent` calls in one assistant message
   run concurrently. 20 evals finish in roughly the wall time of 1.
3. **Synthesis stays in the main session.** Subagent outputs land
   in files; the main session reads only verdicts and failure
   patterns. The main session's context is spent on iteration
   ("what should v3 of this prompt change?"), not on executing
   the eval.

This is what the existing `prompt-evaluation` skill achieves with
async `messages.create()` calls. This skill achieves it with `Agent`
calls instead — same shape, no SDK.

## When this skill applies (vs. the SDK-based one)

| Situation | Use this skill | Use `prompt-evaluation` |
| --- | --- | --- |
| Quick exploration inside a Claude Code session | ✓ | |
| The user can't or won't write Python / install promptfoo | ✓ | |
| The prompt under test is destined for a Claude Code agent | ✓ | |
| You want a CI-runnable, reproducible eval in the repo | | ✓ |
| You need exact `messages.create()` semantics (system vs user roles, exact model ID, exact `output_config`) | | ✓ |
| You want the eval to survive outside Claude Code (CI, scheduled jobs, shared with non-Claude-Code teammates) | | ✓ |
| You need RAG-specific metrics (faithfulness, context-precision) at scale | | ✓ |
| You want a one-off, exploratory pass and then a portable codified version | start here, port later | |

The two skills share methodology — binary judges, position-swap
for pairwise, calibration against human labels, dataset stratified
by feature×scenario×persona, "look at data first". This skill
inherits all of that; see `references/shared_methodology.md` for
the cross-skill pointers.

## What you (Claude Code) actually do

For each invocation you produce four kinds of artifact in the
user's working directory:

```text
<workspace>/
├── eval-set-v1.jsonl                   ← the test inputs (versioned)
├── eval-set-v2.jsonl                   ← bumped when cases change
├── prompt-candidates/
│   ├── candidate-v1.md                 ← the prompt under test
│   └── candidate-v2.md                 ← the proposed revision
├── iteration-1/
│   ├── eval-1/
│   │   ├── candidate-v1.txt            ← raw output from subagent
│   │   ├── candidate-v2.txt
│   │   ├── judge-v1.json               ← {verdict, reasoning}
│   │   └── judge-v2.json
│   ├── ...
│   └── results.md                      ← stamps eval-set version,
│                                         pass rate, failure modes,
│                                         cumulative combined record
└── iteration-2/...                     ← after one refinement loop
```

Two orthogonal version axes — **prompt** (`candidate-vX.md`) and
**eval set** (`eval-set-vY.jsonl`) — are tracked separately so
that runs are unambiguous: every iteration is "candidate-vX ran
against eval-set-vY". See `references/eval_set.md` → Versioning
for the bump-don't-mutate rule and the cumulative-record pattern
in `references/iteration_loop.md`.

You orchestrate the loop. Subagents do the per-sample work. Files
are the interface.

## The loop

1. **Capture the prompt under test and its job.** Exact prompt
   string, what an input looks like, what a correct output looks
   like. If vague, ask for one example input + the user's ideal
   output — that becomes the seed golden pair.

2. **Build (or accept) the eval set.** Hand-curate 10–30 entries
   from real failures if possible, generate synthetically only as
   a complement. Store as JSONL. See `references/eval_set.md`.

3. **Pick a grading approach.**
   - **Reference match** when the criterion is "answer equals X"
     (multi-label set match, regex, etc.) → no judge subagent
     needed; the main session compares.
   - **Binary judge** for open-ended outputs → spawn one judge
     subagent per sample. Default to binary; use anchored numeric
     only when you genuinely need granularity. See
     `references/judge_subagents.md`.
   - **Pairwise with position swap** when comparing two variants
     head-to-head → spawn two judge subagents per sample (positions
     reversed), agree-or-tie. See
     `references/judge_subagents.md`.

4. **Spawn the run.** One assistant message, N+M parallel `Agent`
   calls (N candidate subagents, M judge subagents — or N
   candidates first, then M judges in a second turn if the judges
   need outputs to be on disk first). See
   `references/candidate_subagents.md` and
   `references/parallel_execution.md`.

5. **Aggregate, surface failures, propose a revision.** Read
   `judge-*.json` from each `eval-*/`. Compute pass rate. List
   failed inputs with one-line reasoning. Cluster the failures
   into 2–4 themes. Propose a single targeted edit to the prompt.
   Write `iteration-N/results.md` stamped with the eval-set
   version used and (after iteration 2) a cumulative combined
   record showing which cases the current shipping candidate has
   passed across iterations. Ask the user to greenlight
   `candidate-v(N+1).md`, then re-run on the same eval-set
   version.

That is the whole loop. Subsequent sections drill into the
mechanics.

## Spawning candidate subagents

A candidate subagent simulates one execution of the prompt under
test on one eval input. The pattern:

```text
Agent({
  description: "Run candidate v1 on eval-3",
  subagent_type: "general-purpose",
  prompt: """
    You are a subject under test. The instructions below are the
    only instructions you should follow. Do not consult any tools
    other than Write. Do not search the web. Do not call other
    agents. Do not 'help' beyond what the instructions say.

    <prompt_under_test>
    {paste candidate prompt verbatim}
    </prompt_under_test>

    The user message you are responding to is:

    <user_message>
    {paste eval input}
    </user_message>

    Write your final answer — and nothing else — to:
    {absolute path}/iteration-1/eval-3/candidate-v1.txt
    using the Write tool. Then return the single word: DONE
  """
})
```

Three load-bearing pieces:

1. **"Write to file X, then return DONE."** Subagents summarize
   their work by default. Without an explicit file-output
   contract, the response message contains a paraphrase of the
   answer, not the answer itself. File-based output is lossless;
   message-based output is not.
2. **"Do not consult any tools other than Write."** General-purpose
   subagents have Bash, Read, Web, etc. If you let them roam, the
   eval is no longer testing the prompt — it's testing the prompt
   plus whatever capabilities the subagent decided to use.
3. **Verbatim `<prompt_under_test>` and `<user_message>` tags.**
   This is the closest analog to a `system`+`user` API call you can
   get without leaving Claude Code.

The full template is at `assets/candidate_subagent.template.md`.
Critical pitfalls live at `references/pitfalls.md`.

## Spawning judge subagents

A judge subagent grades one candidate output against one criterion.
The pattern is the same shape as a candidate run — file-based output,
no tools except Write, fresh context.

Defaults (cross-referenced to `prompt-evaluation`):

- **Binary verdict** (`correct` / `incorrect`). Anthropic, Hamel,
  Yan, Arize, Databricks all converge on this. Likert scales aren't
  actionable.
- **Reasoning before verdict.** Always. "Reason then collapse to a
  label" is the canonical shape. Anthropic Cookbook uses
  `<thinking>` then `<correctness>`.
- **One isolated judge per criterion** when you have ≥2 criteria.
  Compound rubrics produce halo effects (Anthropic guidance).
- **For pairwise: always run with positions reversed and gate on
  agreement.** Without the swap, Claude has a 75 % first-position
  bias.
- **Calibrate against your own labels** before trusting the judge.
  Target Cohen's κ ≥ 0.6. See
  `references/judge_subagents.md`.

The judge subagent writes a JSON file:

```json
{
  "verdict": "correct",
  "reasoning": "The output identifies both Service Outage and Feature Request, matching the golden set."
}
```

Template at `assets/judge_binary_subagent.template.md`; pairwise
variant at `assets/judge_pairwise_subagent.template.md`.

## Parallel execution

If you have N eval inputs, spawn N candidate subagents **in the
same assistant message**. They run concurrently. Then, in the next
turn, spawn N judge subagents (also in one message). Wall time is
roughly 2× the slowest sample, not N× the average.

Two practical caveats:

- **Don't exceed model rate limits.** Fan-out of ~15 in a single
  message is field-confirmed reliable on standard subscriptions;
  bump above that cautiously and watch for 429s inside subagents.
  If your eval set is 30 inputs, two batches of 15 is the typical
  shape. See `references/parallel_execution.md` for the full table.
- **Don't fan out beyond your patience for failures.** If a
  candidate prompt has a fundamental issue, all 30 subagents will
  hit it. Run 3–5 first, eyeball the outputs, then fan out.

See `references/parallel_execution.md` for the batching pattern.

## Iteration

The synthesis pass is where this skill earns its keep. You have
the failures, the rubric, the pass rates, and (after the second
iteration) two passes' worth of comparison. The main session
should:

1. **Read failed `candidate-*.txt` outputs in full.** Don't skim.
2. **Cluster failures by mode** (Hamel's "open coding"). Examples:
   - "Misclassifies multi-label inputs as single-label"
   - "Refuses on ambiguous inputs the prompt doesn't authorize it
     to refuse on"
   - "Hallucinates a category not in the schema"
3. **Propose one targeted edit per dominant mode.** Don't bundle
   five edits in one revision — you won't know which one caused the
   change. Save as `prompt-candidates/candidate-v2.md`.
4. **Re-run on the same eval set.** Compare pass rates per failure
   mode, not just overall. Sometimes you fix one mode and regress
   another.

When the user reports "v2 doesn't look better, just different",
you almost certainly traded one failure mode for another. Look at
the per-mode breakdown, not the headline number. See
`references/iteration_loop.md`.

## Pitfalls (read these before your first run)

The subagent-as-eval-runner pattern has failure modes that
`messages.create()` does not. The most common:

1. **Subagent paraphrases instead of pasting.** Mitigation:
   "Write to file X, return DONE." Always.
2. **Subagent uses tools that distort the eval.** Mitigation: "Do
   not consult any tools other than Write."
3. **Subagent applies Claude Code's safety/helpfulness on top of
   the prompt under test.** Mitigation: explicit "subject under
   test" framing. Be aware that this is an approximation of a bare
   API call, not a perfect simulation. If exact API semantics
   matter, port to the SDK-based `prompt-evaluation` skill.
4. **Judge subagent leaks reasoning into the verdict.** Mitigation:
   require structured output (JSON file) with `verdict` as an
   enum.
5. **Position bias in pairwise.** Mitigation: always swap, always
   gate on agreement.
6. **Self-enhancement when the judge model is the same family as
   the candidate.** Mitigation: if budget allows, use a different
   model for the judge — e.g. Haiku as judge for an Opus candidate,
   or vice versa.

Full list with mitigations at `references/pitfalls.md`.

## What this skill is not

- **Not a replacement for a production-grade regression suite.**
  When the prompt ships, codify the eval in the SDK or `promptfoo`
  so it runs in CI without a human at the keyboard. Use this
  skill for the exploratory loop and then port.
- **Not a substitute for looking at the data.** Pass rate on a
  small synthetic eval set is a vibes-with-extra-steps. The number
  is only meaningful if the eval set is sourced from real failures
  (Hamel; Anthropic).
- **Not a free pass on calibration.** A judge subagent that
  agrees with itself on every sample but disagrees with the user
  on 40 % of them isn't a judge — it's noise. Spend a few minutes
  on a κ check before trusting the verdicts.

## References

| File | When to read |
| --- | --- |
| `references/eval_set.md` | Designing the eval set; what counts as a good sample |
| `references/candidate_subagents.md` | Full anatomy of a candidate-subagent prompt + variants |
| `references/judge_subagents.md` | Binary / numeric / pairwise judges + calibration |
| `references/parallel_execution.md` | Batching, rate limits, partial-failure handling |
| `references/iteration_loop.md` | Cluster failures → propose edit → compare runs |
| `references/pitfalls.md` | Subagent-specific failure modes + mitigations |
| `references/shared_methodology.md` | Cross-pointers into the `prompt-evaluation` skill |

| Asset | Purpose |
| --- | --- |
| `assets/candidate_subagent.template.md` | Drop-in prompt for a candidate run |
| `assets/judge_binary_subagent.template.md` | Drop-in prompt for a binary judge |
| `assets/judge_pairwise_subagent.template.md` | Drop-in prompt for a pairwise judge |
| `assets/eval_set.template.jsonl` | Shape of the eval-set file |
