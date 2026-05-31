# Candidate subagent prompt template

Drop-in prompt for running one candidate prompt on one eval input
via the `Agent` tool. Replace the `{placeholders}` and pass the
whole string as the `prompt` parameter.

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

## Agent call shape

```text
Agent({
  description: "Run candidate v{V} on eval-{ID}",
  subagent_type: "general-purpose",
  prompt: <the filled-in template above>
})
```

## Variants

### When the prompt under test expects multi-turn input

Replace the `<user_message>` block with:

```text
<user_message>
<turn role="user">{first user message}</turn>
<turn role="assistant">{first assistant message}</turn>
<turn role="user">{second user message — respond to this turn}</turn>
</user_message>

Respond only to the final user turn.
```

### When the prompt under test requires JSON output

Don't add JSON formatting instructions outside `<prompt_under_test>`.
If the prompt under test doesn't already specify the JSON shape,
that's a defect to surface, not a defect to paper over.

### When you need tool-call output (for tool-using prompts)

Add to the constraints section:

```text
- When the prompt under test would call a tool, instead emit a
  JSON object on a single line describing the call:
  {"tool": "<name>", "input": {<args>}}
- After emitting the tool-call JSON, stop. Do not continue past
  the first tool call.
```

The judge can then grade the trajectory deterministically. See
the SDK-based skill's `references/tool_use_evals.md` for the
trajectory-grading patterns.
