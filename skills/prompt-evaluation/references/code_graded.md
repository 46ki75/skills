# Code-Graded Evaluations

Code-graded evals use a deterministic function (`output == expected`,
a regex, a set check, a JSON-schema validator) to decide pass/fail.
They are cheap, fast, and reproducible. Reach for them whenever the
correctness criterion can be expressed as code.

This reference covers the Python + Anthropic SDK pattern (lessons
3–4 of the course). For the promptfoo equivalent, see
`promptfoo.md`. The two are interchangeable — pick whichever fits
the user's stack.

## When code-graded works

- **Exact string match** — fixed short answer (a number, a label, a
  yes/no). E.g. "how many legs does this animal have?"
- **Set match** — multi-label classification where order doesn't
  matter. E.g. "classify this ticket into one or more of
  {Bug, Outage, Feature Request}".
- **Substring / keyword presence** — output must mention specific
  required words. Brittle but acceptable for some compliance checks.
- **Regex** — output must match a specific pattern, e.g.
  `r"Your score of \d{3} (qualifies|does not qualify)"`.
- **Structured-output validation** — output parses as JSON and
  conforms to a schema. Use `json.loads` + `jsonschema` or a
  Pydantic model.
- **Programmatic checks** — output is code that compiles, SQL that
  parses, etc.

## When code-graded breaks down

- **Open-ended generation** — summaries, explanations, rewrites.
  No fixed correct string; substring checks measure presence, not
  quality.
- **Tone, refusal, faithfulness** — language-level criteria that
  can't be reduced to a regex without missing the point.
- **Numeric answers with formatting noise** — model outputs "The
  answer is 5." instead of "5". Two fixes:
  1. **Tighten the prompt** to require bare output ("Respond only
     with a digit, nothing else"). This is usually the right move.
  2. **Extract before grading** via a transform — e.g. instruct the
     model to wrap its answer in `<answer>...</answer>` tags and
     strip them out in code. Especially useful with chain-of-thought.

If the deterministic check is fundamentally the wrong measurement,
do not torture a regex into shape — switch to model-graded (see
`model_graded.md`).

## Minimal Python eval loop

The Anthropic-SDK pattern from lessons 3–4 of the course, stripped
to essentials. See `assets/python_eval.template.py` for a runnable
template.

```python
from anthropic import Anthropic

client = Anthropic()
MODEL = "claude-haiku-4-5-20251001"

def build_prompt(item):
    return f"""You will be provided a customer complaint. Classify it
into one or more of: Software Bug, Hardware Malfunction, User Error,
Feature Request, Service Outage. Respond with only the category
name(s), comma-separated, and nothing else.

Complaint: {item['complaint']}
Classification:"""

def get_completion(prompt):
    resp = client.messages.create(
        model=MODEL,
        max_tokens=200,
        messages=[{"role": "user", "content": prompt}],
    )
    return resp.content[0].text

def grade(output, golden_answer):
    """Set-match grader for multi-label classification."""
    predicted = {c.strip().lower() for c in output.split(",")}
    expected = {c.lower() for c in golden_answer}
    return predicted == expected

eval_data = [
    {"complaint": "The app crashes when I upload a photo",
     "golden_answer": ["Software Bug"]},
    # ...
]

results = []
for item in eval_data:
    output = get_completion(build_prompt(item))
    passed = grade(output, item["golden_answer"])
    results.append({"item": item, "output": output, "passed": passed})

passes = sum(r["passed"] for r in results)
print(f"Score: {passes}/{len(results)} = {passes/len(results):.1%}")

for r in results:
    if not r["passed"]:
        print(f"FAIL: {r['item']['complaint']!r}")
        print(f"  expected={r['item']['golden_answer']}  got={r['output']!r}")
```

Important details:

- **Print failures with input + expected + got**, not just the count.
  Eyeballing failure rows is where prompt edits actually come from.
- **Lowercase / strip / split** consistently in both the model output
  and the golden answer before comparing. Whitespace is the #1 cause
  of false-fail.
- **Pin the model name** in the script, and make the user aware of it.
  Switching from Haiku to Sonnet without re-running is a regression
  risk.

## Common grader patterns

### Exact match

```python
def grade(output, golden):
    return output.strip() == golden.strip()
```

Use only when the prompt is tight enough to produce bare answers.

### Set match (multi-label classification)

```python
def grade(output, golden):
    predicted = {c.strip().lower() for c in output.split(",")}
    expected = {c.lower() for c in golden}
    return predicted == expected
```

### Substring / keyword presence

```python
def grade(output, required_keywords):
    o = output.lower()
    return all(kw.lower() in o for kw in required_keywords)
```

Use sparingly. "Contains X" is a weak signal of quality.

### Regex pattern

```python
import re

def grade(output, pattern):
    return bool(re.search(pattern, output))
```

### Extract-then-compare (for chain-of-thought)

```python
import re

def extract_answer(text):
    m = re.search(r"<answer>(.*?)</answer>", text, re.DOTALL)
    return m.group(1).strip() if m else None

def grade(output, golden):
    extracted = extract_answer(output)
    return extracted == golden
```

This is the canonical pattern when you've asked the model to reason
in `<thinking>` tags first. If the extraction returns `None`, count
it as a fail and surface it in the report — "no `<answer>` tag" is
itself a useful failure mode to see.

### JSON structure check

```python
import json

def grade(output, expected):
    try:
        parsed = json.loads(output)
    except json.JSONDecodeError:
        return False
    return parsed == expected  # or schema-validate
```

For structured output, also consider `assistant`-prefill (`{`) to
force the model to start emitting JSON immediately.

## Reporting and aggregation

When you run the eval, produce:

1. **Headline score** — `passes / total`.
2. **Per-row breakdown** — at least the failures, with input,
   expected, and got. Ideally save to a CSV/JSON for diffing across
   iterations.
3. **Failure categorization** — after looking at the failures,
   group them by *kind* of failure (format, reasoning, off-topic,
   classification confusion). This is the input to your next
   prompt edit.

Compare runs by **same dataset, different prompt** (or same prompt,
different model). Don't change two things at once.

## Cost and latency

The Python pattern makes one model call per row, sequentially. For
20 rows this is fine. For 200+:

- Use `asyncio` + `anthropic.AsyncAnthropic` to parallelize.
- Cap concurrency to your rate limit.
- Cache results by `(prompt, input)` hash if you'll re-run with the
  same prompt for unrelated reasons.

For the iteration loop covered in this skill, sequential is the
right default — keep the code simple.
