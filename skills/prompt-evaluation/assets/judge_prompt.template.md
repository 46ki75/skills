# Judge Prompt Template

Copy this into a Python string when building a custom model-grader.
Replace the criteria and anchors for your task. Use it with
`temperature=0`, assistant-prefill `<json>`, and stop-sequence
`</json>` for reliable JSON parsing (see
`references/model_graded.md`).

---

```text
Evaluate the following <OUTPUT_KIND> based on these criteria:

1. <CRITERION_1> (1-5)
   - 1: <what a 1 looks like — concrete behavior, not "bad">
   - 3: <what a 3 looks like — middle case>
   - 5: <what a 5 looks like — concrete behavior, not "good">

2. <CRITERION_2> (1-5)
   - 1: <...>
   - 3: <...>
   - 5: <...>

3. <CRITERION_3> (1-5)
   - 1: <...>
   - 3: <...>
   - 5: <...>

Notes on what to ignore:
- <e.g. "Do not penalize the summary for being shorter than the
  original — brevity is the goal.">
- <e.g. "Ignore minor stylistic preferences; only score the
  criteria above.">

<examples>
<example>
This <OUTPUT_KIND>:
<output>
<paste a concrete example output that should score well>
</output>
Should receive: <criterion_1>=5, <criterion_2>=5, <criterion_3>=5
Reason: <one-sentence explanation>
</example>

<example>
This <OUTPUT_KIND>:
<output>
<paste a concrete example output that should score poorly on at
least one criterion>
</output>
Should receive: <criterion_1>=1, <criterion_2>=5, <criterion_3>=3
Reason: <one-sentence explanation>
</example>
</examples>

Respond inside <json> tags using exactly this format:

<json>
{
  "<criterion_1>": <int 1-5>,
  "<criterion_2>": <int 1-5>,
  "<criterion_3>": <int 1-5>,
  "explanation": "<one-sentence rationale for the scores>"
}
</json>

<INPUT_CONTEXT_TAG>{input_context}</INPUT_CONTEXT_TAG>

<OUTPUT_TAG>{output_to_evaluate}</OUTPUT_TAG>
```

## Why each part is here

- **Numeric 1–5 scale with anchors** — forces the judge to commit
  to a position. "Excellent / good / fair" without behavioral
  anchors lets the model pick the middle.
- **Concrete anchor descriptions** — calibrate the model on your
  task, not its prior. "5: tight, all essential information, no
  fluff" is much more reliable than "5: excellent".
- **"What to ignore" section** — closes off the most common ways a
  judge over-penalizes (verbosity, style preferences, length vs.
  original).
- **Two examples with target scores** — turns the rubric into a
  few-shot calibration. Pick one example near 5 and one near 1 so
  the spread is clear.
- **`<json>` prefill + stop-sequence** — forces the model to start
  producing JSON immediately, and lets you call `json.loads()`
  without parsing free-form text.
- **Input context inside its own tag** — when grading faithfulness,
  the judge needs the source (article, passage, original prompt).
  Pass it in. When grading something that doesn't need the source
  (refusal, tone), omit the tag.
