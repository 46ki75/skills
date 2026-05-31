# Pitfalls

Failure modes specific to running prompt evaluations through
Claude Code subagents. Read this once before your first run,
re-read when you're debugging a weird result.

## Output fidelity

### Subagent paraphrases the answer in its return message

**Symptom.** The candidate output file is missing or contains a
short summary rather than the full answer.

**Cause.** Subagents are trained to summarize their work in the
returned message. Without an explicit file-output contract, the
"useful" thing to do from the subagent's perspective is to put
the answer in the response.

**Mitigation.** End the candidate prompt with: *"Write your final
answer — and nothing else — to {file}. Then return the single
word DONE. Do not summarize your answer in the return message;
the answer must be in the file."*

After spawning, verify file existence before grading:

```bash
ls iteration-{N}/eval-*/candidate-v{V}.txt | wc -l
```

### Judge writes verdict in prose instead of JSON

**Symptom.** The judge file exists but `jq` fails to parse it.

**Cause.** Default behavior under ambiguous instructions; judge
explains its reasoning in markdown and includes a JSON block as
an afterthought, or wraps the JSON in a markdown fence.

**Mitigation.** Explicit shape in the judge prompt: *"Write your
verdict to {file} as a JSON object with two fields. No markdown
fence, no surrounding text, just the JSON object."*

If a parse fails, re-spawn the judge for that one sample with a
stricter instruction.

## Contamination

### Subagent uses tools that distort the eval

**Symptom.** The candidate output cites a fact the candidate
prompt didn't authorize it to know; or the answer's quality is
suspiciously high for the prompt's instructions.

**Cause.** General-purpose subagents have access to Web, Bash,
Read, etc. They use them helpfully — but the eval is no longer
testing the prompt; it's testing the prompt plus whatever
external capability the subagent decided to invoke.

**Mitigation.** Tool block in the constraints section: *"Do not
use Bash, Read, Web, Glob, Grep, or any tool except Write."*

Spot-check the first 2–3 runs by reading the subagent's tool-use
trace. If it deviates, the prompt under test should probably get
sharper "do not look up anything external" guidance, or you should
use a more restrictive `subagent_type`.

### Claude Code's system prompt biases the subagent

**Symptom.** The candidate output exhibits safety behaviors,
helpfulness patterns, or refusal triggers that the prompt under
test doesn't ask for.

**Cause.** Subagents inherit Claude Code's system prompt — the
prompt under test sits inside the subagent's `user` message, not
in `system`. Whatever Claude Code's system prompt asks for is
always in effect.

**Mitigation.** Be aware that this is an **approximation** of a
bare API call, not a perfect simulation. The "subject under test"
framing reduces the effect but doesn't eliminate it. If exact
`messages.create()` semantics matter — e.g., the prompt under
test is a system prompt for a deployed product that must behave
differently than Claude Code's defaults — port to the SDK-based
`prompt-evaluation` skill.

### Eval set leaks into the main session's context

**Symptom.** When you propose a prompt revision, your suggestions
look suspiciously well-tuned to the exact failures in the eval
set. Iteration 2 pass rate is great; samples added in iteration 3
fall off a cliff.

**Cause.** You overfit to the eval set because you read every
output in the main session.

**Mitigation.** Two strategies:

1. **Hold out test samples.** Split the eval set into
   `eval_set.train.jsonl` (the samples you iterate against) and
   `eval_set.test.jsonl` (the samples you grade against only
   at the end). Don't read the test outputs during iteration.
2. **Track per-mode improvement, not per-sample improvement.**
   "v3 fixes the schema-collapse mode" generalizes; "v3 passes
   eval-7" does not.

## Judge reliability

### Judge agrees with itself but not with the user

**Symptom.** Pass rate looks reasonable, you eyeball some
"correct" verdicts, and find a few you'd have called incorrect.
You re-run the judge → same verdicts. The judge is consistent
internally but disagrees with the human.

**Cause.** The rubric is underspecified, the criterion has
ambiguity the judge resolved one way and you'd resolve another.

**Mitigation.** Calibrate. Hand-grade 25–50 samples, compute
Cohen's κ. If κ < 0.6, **iterate on the rubric, not the judge
model**. Sharper criterion, sharper anchors, explicit "what to
ignore" guidance.

### Position bias in pairwise

**Symptom.** Pairwise verdicts swing wildly between runs; v1 wins
sometimes, v2 wins sometimes, on the same samples.

**Cause.** Claude has a 75 % first-position preference in
pairwise comparison (MT-Bench measurement on Claude). Without a
swap, you're measuring position, not quality.

**Mitigation.** Always run pairwise with positions reversed, gate
on agreement. See `judge_subagents.md` for the swap protocol.

### Self-enhancement

**Symptom.** When the judge is the same model family as the
candidate, the candidate wins suspiciously often.

**Cause.** Measured Claude self-enhancement: +25 % own-family win
rate (MT-Bench).

**Mitigation.** Use a different family as judge when possible.
For internal Claude Code use this often isn't actionable — you're
stuck with the model the session is on — but at minimum, note the
caveat when reporting results, and prefer reference-guided judging
when a reference exists (which doesn't have a self-enhancement
failure mode).

### Verbosity bias

**Symptom.** Longer outputs win pairwise comparisons even when
the shorter output is more correct.

**Cause.** Measured Claude verbosity bias: 91 % prefer longer
answers (MT-Bench).

**Mitigation.** Add to the judge prompt: *"Ignore length unless
concision is itself a scored criterion. A shorter correct answer
is preferable to a longer correct answer."*

## Process

### Eval-set drift between iterations

**Symptom.** Pass rate went from 70 % to 85 % between v1 and v2,
but you also "improved" the eval-set criteria mid-way.

**Cause.** You sharpened the rubric because v1 outputs revealed
ambiguity in the criterion. The new rubric is easier to pass.

**Mitigation.** Lock the eval set and the criteria between
iterations. If you must change either, **bump the eval-set
version** (`eval-set-v1.jsonl` → `eval-set-v2.jsonl`) and re-run
the current candidate against the new version to re-establish a
fair baseline. Stamp every iteration's `results.md` with the
eval-set version it used. See `eval_set.md` → Versioning.

(Source: Shankar et al., *Who Validates the Validators?* —
criteria drift is the most common form of self-deception in
prompt eval.)

### Bundled edits between iterations

**Symptom.** v2 differs from v1 in five places. Pass rate went up
but you have no idea why.

**Cause.** You proposed multiple edits in one revision.

**Mitigation.** One targeted edit per iteration. See
`iteration_loop.md`.

### Over-iterating

**Symptom.** Pass rate keeps wandering between 88 % and 92 %
across iterations. You can't tell if v5 is better than v3.

**Cause.** You've reached the noise floor of the judge + eval set.
Further iteration is moving samples around within the noise band,
not actually improving the prompt.

**Mitigation.** Stop. Either expand the eval set (more samples
reduce noise) or accept the current version. "Pass rate
plateaued" is a legitimate stopping condition.

### Treating iteration count as progress

**Symptom.** v8, v12, v15 of the prompt. None is shipping.

**Cause.** Iteration is a means, not the end. After 3–5 iterations
without convergence, the bottleneck is usually somewhere else —
the eval set is wrong, the judge is wrong, the underlying task is
harder than prompt engineering can solve.

**Mitigation.** After 5 unconvincing iterations, step back. Ask:
is this a prompt problem, a model-capability problem, a dataset
problem, or a judge problem? Each calls for a different fix.

## When in doubt: re-spawn

Subagents are idempotent — re-spawning a sample overwrites the
previous output. There's no cleanup required. If a result looks
suspicious:

- Re-spawn the candidate to check if the output was a temperature
  artifact.
- Re-spawn the judge to check if the verdict was on the noise
  margin.
- Compare. If they're stable, the result is real. If they flip,
  the sample is noisy and you should not draw conclusions from it.
