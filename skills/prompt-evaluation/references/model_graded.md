# Model-Graded Evaluations (LLM-as-Judge)

Use a model to grade another model's output when the criterion can't
be reduced to deterministic code: tone, faithfulness, refusal
quality, summary completeness, "is this explanation appropriate for
a grade-school audience?". Lessons 8–9 of the course cover this.

Model-graded evals are **expensive, slower, and noisier** than
code-graded ones. The payoff is being able to evaluate things you
genuinely care about that no regex can capture.

## When to reach for a model grader

- The criterion is naturally expressed in language ("response should
  be apologetic", "summary should be grade-school appropriate",
  "answer should be faithful to the passage")
- Outputs are open-ended and vary across acceptable responses
- You've already tried a code-graded version and it's either too
  brittle (catches the wrong thing) or impossible to write

## When **not** to use a model grader

- The criterion is exact-match or set-match — use code (cheaper,
  zero variance)
- The judge would need information it doesn't have (private
  database, the user's true preference)
- You're testing the same family of model that's generating the
  output and the criterion is subtle — risk of self-preference bias

## Two patterns

### Pattern 1: `llm-rubric` (promptfoo built-in)

The fastest way to add a model grader. Lives entirely in
`promptfooconfig.yaml`:

```yaml
defaultTest:
  assert:
    - type: llm-rubric
      provider: anthropic:messages:claude-opus-4-7
      value: "Refuses to answer the question and redirects to academic topics"
    - type: llm-rubric
      provider: anthropic:messages:claude-opus-4-7
      value: "Is not apologetic"
```

promptfoo wraps your criterion in a standard judge prompt and
returns pass/fail. Good for **simple, single-aspect criteria**.
Each assertion grades one thing.

Practical notes:

- Use a **stronger model than the one under test** when feasible
  (e.g. Opus to grade Haiku's output). Don't use the same model to
  grade itself unless you have to.
- One short criterion per assertion. Compound criteria ("refuses
  AND isn't apologetic AND under 100 words") confuse the judge —
  split them.
- Phrase positively when you can. "Is not apologetic" works, but
  "Maintains a neutral, non-apologetic tone" is clearer.

### Pattern 2: Custom rubric grader (promptfoo `python` assertion or Python SDK)

When you need:

- A multi-dimensional rubric with numeric anchors
- Access to the original input (the article being summarized, the
  source passage being answered against, etc.) inside the judge
  prompt
- Custom score aggregation (average, weighted, min-of-N)

Pattern from lesson 9, lightly adapted:

```python
import anthropic, os, json

JUDGE_MODEL = "claude-sonnet-4-6"

JUDGE_PROMPT = """Evaluate the following summary on these criteria:

1. Conciseness (1-5)
   - 1: Verbose, repetitive, includes irrelevant details
   - 3: Mostly focused, some unnecessary detail
   - 5: Tight, all essential information, no fluff

2. Accuracy (1-5)
   - 1: Significant errors or omissions that change meaning
   - 3: Mostly accurate, minor errors or omissions
   - 5: Faithful to the source, no errors

3. Tone (1-5)
   - 1: Too technical or mature for a grade-school audience
   - 3: Mostly accessible, occasional challenging terms
   - 5: Consistently simple and engaging for young readers

Respond in this exact JSON format inside <json> tags:
<json>
{{
  "conciseness": <int 1-5>,
  "accuracy": <int 1-5>,
  "tone": <int 1-5>,
  "explanation": "<one-sentence rationale>"
}}
</json>

Original article:
<article>{article}</article>

Summary to evaluate:
<summary>{summary}</summary>
"""

def llm_judge(summary, article):
    client = anthropic.Anthropic(api_key=os.getenv("ANTHROPIC_API_KEY"))
    resp = client.messages.create(
        model=JUDGE_MODEL,
        max_tokens=1000,
        temperature=0,
        messages=[
            {"role": "user",
             "content": JUDGE_PROMPT.format(article=article, summary=summary)},
            {"role": "assistant", "content": "<json>"},
        ],
        stop_sequences=["</json>"],
    )
    body = resp.content[0].text
    parsed = json.loads(body)
    scores = [v for v in parsed.values() if isinstance(v, (int, float))]
    return sum(scores) / len(scores), parsed

# In promptfoo, exposed as get_assert:
def get_assert(output, context, threshold=4.5):
    article = context["vars"]["article"]
    avg, parsed = llm_judge(output, article)
    return {
        "pass": avg >= threshold,
        "score": avg,
        "reason": json.dumps(parsed),
    }
```

Key techniques:

- **Assistant prefill + stop sequence** — start the assistant with
  `<json>` and stop at `</json>`. The model can't preamble its way
  out of valid JSON, and parsing becomes a single `json.loads`.
- **Temperature 0** for the judge. You want reproducibility.
- **Score anchors, not just labels.** "Tone: 1–5" is useless on its
  own. Define what 1, 3, and 5 look like, ideally with one-line
  examples (see the template in `assets/judge_prompt.template.md`).
- **Include the source.** When grading faithfulness or summary
  accuracy, the judge needs the original. Pass it through the
  context.

## Designing a good rubric

Most judge prompts fail not because the judge model is dumb but
because the rubric is vague. Apply these rules:

1. **One axis per criterion.** "Quality" is not a criterion;
   "accuracy", "conciseness", "tone" are. Mixing axes makes the
   score uninterpretable.
2. **Define anchors with concrete behavior.** Not "5: excellent" but
   "5: includes all essential information without superfluous
   detail". The model is calibrated by your rubric, not its prior.
3. **Give one or two `<example>` blocks** showing a summary and the
   scores it should receive. The course's lesson-9 prompt does this
   for two summaries and it makes a real difference.
4. **Use small scales.** 1–5 is the sweet spot. 1–10 invites the
   model to pick 7 for everything. Binary (pass/fail) is fine when
   the criterion is sharp.
5. **Be explicit about what to *ignore*.** "Do not penalize the
   summary for being shorter than the original — brevity is the
   goal."

## Known biases and how to handle them

LLM judges have systematic biases. None of these are fatal, but
ignoring them will mislead you.

- **Position bias** (in pairwise comparison): the model prefers
  whichever output came first. Mitigation: randomize order, or
  evaluate each output independently with a rubric instead of
  pairwise.
- **Verbosity bias**: longer outputs often get higher scores by
  default. Mitigation: explicitly weight conciseness as a criterion,
  or normalize on length.
- **Self-preference**: a model often prefers its own outputs.
  Mitigation: use a different model family as judge when possible,
  or use a stronger model.
- **Sycophancy**: judges are biased toward whatever the prompt
  implies the "right" answer is. Mitigation: keep the rubric
  neutral; don't reveal which output is "v1" vs "v2".

## Validating the judge

Before trusting your judge, **calibrate it** on a small set of
human-labeled examples. Hand-grade 5–10 outputs yourself, then run
the judge on the same outputs. If the judge disagrees with you on
more than ~20%, the rubric is the problem, not the model. Iterate
on the rubric until your scores match.

This step is non-negotiable for any criterion you're going to make
a real decision on (deployment, model switch, prompt rollout).

## Cost considerations

A model-graded eval roughly **doubles** your token cost (one model
call to generate, one to grade) and adds latency. For a 20-row
iteration loop, this is fine. For a 200-row pre-deploy check,
plan for it.

If cost is tight:

- Use a code-graded "first pass" for the cheap criteria (format,
  length, classification label) and a model-graded "second pass"
  only on rows that pass the first.
- Use a cheaper judge model — Haiku can grade well with a tight
  rubric. Test that it agrees with Sonnet/Opus on a calibration
  sample first.
- Cache judge results — if the model output didn't change, you
  don't need to re-judge.
