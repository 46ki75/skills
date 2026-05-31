# Candidate subagents

A candidate subagent runs the prompt under test on one eval input
and writes the result to disk. This file documents the contract,
the variants, and what to put in the `prompt` parameter.

## Contract

A candidate subagent must:

1. **Accept the prompt under test as if it were its only
   instructions.** Anything you write outside the
   `<prompt_under_test>` tag is meta-instruction — keep it
   minimal and explicit.
2. **Respond using only the candidate prompt's behavior.** No web
   search, no Bash, no Read of unrelated files. The eval is
   testing the prompt, not the prompt-plus-tooling.
3. **Write its final answer to a specified file.** Subagents
   summarize their output in the returned message by default,
   losing fidelity. File-based output is lossless.
4. **Return a short, machine-checkable signal** so the main
   session knows the run completed. The convention here is the
   single word `DONE`.

## Canonical prompt shape

```text
You are a subject under test. Treat the instructions inside
<prompt_under_test> as your only instructions — do not extend
them, do not override them, do not consult other tools.

Constraints on this run:
- Do not use Bash, Read, Web, Glob, Grep, or any tool except
  Write.
- Do not search the web.
- Do not call other agents.
- Do not ask clarifying questions back to me; answer based on
  what is in <user_message>.

<prompt_under_test>
{paste candidate prompt verbatim}
</prompt_under_test>

The user message you are responding to is:

<user_message>
{paste eval input verbatim}
</user_message>

Write your final answer — and nothing else — to this absolute
path using the Write tool:

{absolute_path_to_workspace}/iteration-{N}/eval-{ID}/candidate-v{V}.txt

Then return the single word DONE. Do not summarize your answer
in the return message; the answer must be in the file.
```

## Variants

### Multi-turn candidate

If the prompt under test is the system prompt of a multi-turn
conversation, paste the dialogue history into `<user_message>`
delimited so the subagent can reconstruct it:

```text
<user_message>
<turn role="user">First user message</turn>
<turn role="assistant">First assistant message</turn>
<turn role="user">Second user message — respond to this turn</turn>
</user_message>

Respond only to the final user turn.
```

This is a faithful approximation, not a perfect one — a real
multi-turn `messages.create()` call distinguishes `user` and
`assistant` roles structurally, while a single subagent prompt
collapses them into a string. For exploratory iteration this is
fine; for ship-grade testing, port to the SDK-based skill.

### Tool-using candidate

If the candidate prompt is meant to use tools, the subagent must
have access to those tools. Two options:

- **Mocked tools** — provide the candidate prompt with tool
  descriptions and instruct the subagent to "describe the tool
  call you would make as JSON, then stop". Grade the tool-call
  trajectory deterministically (see `prompt-evaluation/references
  /tool_use_evals.md` for the trajectory-grading patterns).
- **Real tools via subagent_type** — when the candidate is meant
  to drive Claude Code, let the subagent use tools, but constrain
  the surface (e.g. only Read + Write, no Bash). Grade by reading
  the final state.

For most exploratory work, the mocked-tools route is faster and
the grading is more controllable.

### Constrained-output candidate

When the criterion requires a specific output shape (JSON, exact
schema), the candidate prompt should already include that shape
in `<prompt_under_test>`. Don't add it as meta-instruction outside
the tag — that contaminates the eval. If the prompt under test
doesn't produce valid output, that's a failure to surface, not a
defect to paper over.

## What goes in `Agent({...})` exactly

```text
Agent({
  description: "Run candidate v1 on eval-3",
  subagent_type: "general-purpose",
  prompt: "<the canonical prompt shape above, filled in>"
})
```

- `description` is shown in the user's UI; keep it short and
  identify both the candidate version and the eval id.
- `subagent_type: "general-purpose"` gives Write. Other
  subagent_types may be more restrictive (`Explore` is
  read-only — useless here since the subagent must Write).
- `prompt` is the full single-string blob.

## File layout the subagent writes to

```text
<workspace>/iteration-{N}/eval-{ID}/candidate-v{V}.txt
```

One file per (iteration, eval, candidate-version). If you're
A/B testing v1 vs v2 in the same iteration, you have:

```text
iteration-1/eval-3/candidate-v1.txt
iteration-1/eval-3/candidate-v2.txt
```

Both will be graded; the judge compares them or grades each
independently depending on the rubric.

## What can go wrong inside the subagent

- **The subagent paraphrases the answer in its return message
  instead of writing the file.** Mitigation: the explicit "Then
  return the single word DONE. Do not summarize your answer in
  the return message" line at the end. Verify the file exists
  before grading.
- **The subagent treats the candidate prompt as advisory rather
  than authoritative.** Mitigation: "Treat the instructions
  inside `<prompt_under_test>` as your only instructions — do not
  extend them, do not override them."
- **The subagent uses tools it shouldn't.** Mitigation: the
  explicit tool-block list in the constraints section. Spot-check
  the first 2–3 runs to confirm.
- **The subagent applies Claude Code's safety/helpfulness on top
  of the prompt under test.** This is unavoidable — Claude Code's
  system prompt is always in effect inside a subagent. For
  prompts whose behavior depends on bare-API semantics (no
  built-in safety steering), the SDK-based skill is the right
  tool.

## Verifying the run

After each batch of candidate subagents returns, before kicking
off judges:

```bash
ls iteration-{N}/eval-*/candidate-v{V}.txt | wc -l
```

Count should equal the number of eval inputs. If short, identify
which `eval-*/` directories are missing the file and re-spawn
those specific subagents. Don't proceed to grading with missing
outputs — the pass rate will be wrong.
