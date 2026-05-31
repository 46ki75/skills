# promptfoo Reference

[`promptfoo`](https://github.com/promptfoo/promptfoo) is a YAML-driven
prompt evaluation framework. It runs your prompt(s) across one or
more models on a dataset, applies assertions (built-in or custom),
and produces a comparison dashboard. Lessons 5–9 of the course use
it as the eval tool.

Use promptfoo when the user wants:

- A visual diff dashboard (prompts × models × test cases)
- Declarative config (YAML) so non-engineers can edit test cases
- Easy multi-model comparison without writing client code

Stick with the Python SDK pattern instead when the eval needs to
live inside an existing Python codebase or CI script, or when the
user doesn't want a Node.js dependency.

## Installation and project layout

No global install required; use `npx`:

```bash
mkdir my-eval && cd my-eval
npx promptfoo@latest init
```

This creates `promptfooconfig.yaml`. A typical project ends up
looking like:

```text
my-eval/
├── promptfooconfig.yaml      # the config
├── prompts.py                # prompt-generating functions (optional)
├── dataset.csv               # test cases (optional)
├── transform.py              # output transform (optional)
├── grader.py                 # custom Python grader (optional)
└── .env                      # ANTHROPIC_API_KEY (don't commit)
```

Run with:

```bash
export ANTHROPIC_API_KEY=...
npx promptfoo@latest eval     # run the eval
npx promptfoo@latest view     # open the browser dashboard
```

## `promptfooconfig.yaml` anatomy

A full-featured config looks like this. See
`assets/promptfooconfig.template.yaml` for a copyable starter.

```yaml
description: "Customer complaint classifier"

prompts:
  - prompts.py:basic_prompt           # function from a .py file
  - prompts.py:improved_prompt
  # OR an inline prompt using Nunjucks variables:
  # - >-
  #   Classify the following complaint into one of: Bug, Outage, Feature.
  #   Complaint: {{complaint}}
  #   Classification:

providers:
  - id: anthropic:messages:claude-haiku-4-5-20251001
    label: "Haiku 4.5"
  - id: anthropic:messages:claude-sonnet-4-6
    label: "Sonnet 4.6"

tests: dataset.csv                    # or inline (see below)

defaultTest:
  options:
    transform: file://transform.py    # applied to every output
  assert:
    - type: python                    # custom code grader
      value: file://grader.py
    - type: llm-rubric                # built-in model grader
      provider: anthropic:messages:claude-opus-4-7
      value: "Response is polite and avoids blame"
```

### `description`

A human label for the run. Shows up in the dashboard.

### `prompts`

Multiple ways to specify prompts; the course shows three:

**1. Python function** (recommended for non-trivial prompts):

```yaml
prompts:
  - prompts.py:basic_prompt
  - prompts.py:improved_prompt
```

With `prompts.py`:

```python
def basic_prompt(animal_statement):
    return f"""How many legs does this animal have? <stmt>{animal_statement}</stmt>"""

def improved_prompt(animal_statement):
    return f"""How many legs? Respond with a single digit only.
<stmt>{animal_statement}</stmt>"""
```

Each function takes the test `vars` as keyword args and returns the
final prompt string.

**2. Inline with Nunjucks templating** (good for simple prompts):

```yaml
prompts:
  - >-
    Write a paragraph about {{topic}}. Mention {{topic}} exactly
    {{count}} times.
```

Double-brace variables are filled from `tests[].vars`.

**3. File reference**:

```yaml
prompts:
  - file://prompts/v1.txt
  - file://prompts/v2.txt
```

### `providers`

Anthropic provider strings follow the pattern
`anthropic:messages:<model-id>`. Examples:

```yaml
providers:
  - anthropic:messages:claude-opus-4-7
  - anthropic:messages:claude-sonnet-4-6
  - anthropic:messages:claude-haiku-4-5-20251001
```

Use the `id:` / `label:` form when you want a friendly label in the
dashboard. Adding a second provider line is all it takes to run the
same eval across two models — the dashboard will show side-by-side
columns.

### `tests`

The eval dataset. Three common forms:

**CSV file** (recommended for tabular inputs):

```yaml
tests: dataset.csv
```

`dataset.csv` columns become `vars` for the prompt template. A
special `__expected` column lets you embed simple assertions per
row:

```csv
animal_statement,__expected
"The animal is a dog.","equals:4"
"The animal is a snake.","equals:0"
```

**Inline** (recommended for short test sets in the same file):

```yaml
tests:
  - vars:
      complaint: "App crashes on photo upload"
    assert:
      - type: equals
        value: "Software Bug"
  - vars:
      complaint: "I can't find the login button"
    assert:
      - type: equals
        value: "User Error"
```

**File-loaded variables** for large inputs:

```yaml
tests:
  - vars:
      article: file://articles/article1.txt
  - vars:
      article: file://articles/article2.txt
```

The `article` variable is filled with the file's text contents at
runtime. Useful for long inputs (transcripts, source code, etc.).

### `assert` and `defaultTest`

Assertions run after the model output is produced and decide pass/
fail. Specify them per-test or globally under `defaultTest`.

**Built-in assertions** (most common):

| Type | What it checks |
| --- | --- |
| `equals` | Exact string match against `value` |
| `contains` | Output contains `value` as substring |
| `contains-all` | Output contains every string in a list |
| `contains-any` | Output contains at least one of a list |
| `regex` | Output matches a regular expression |
| `icontains` / `regex-i` | Case-insensitive versions |
| `is-json` | Output parses as JSON |
| `cost` | Output cost below threshold |
| `latency` | Output latency below threshold |
| `llm-rubric` | Model-graded; see `model_graded.md` |
| `python` | Custom Python function in a file |
| `javascript` | Custom JS function |

Example, per-row:

```yaml
tests:
  - vars: { complaint: "App crashes on upload" }
    assert:
      - type: equals
        value: "Software Bug"
```

Example, applied to every row via `defaultTest`:

```yaml
defaultTest:
  assert:
    - type: python
      value: file://grader.py
    - type: llm-rubric
      provider: anthropic:messages:claude-opus-4-7
      value: "Response is polite"
```

### `transform`

A function applied to every model output **before** assertions run.
Use this to strip wrapper tags, parse JSON, or normalize whitespace.

```yaml
defaultTest:
  options:
    transform: file://transform.py
```

`transform.py`:

```python
def get_transform(output, context):
    # Strip <thinking>...</thinking><answer>X</answer> down to X.
    if "<answer>" in output:
        try:
            return output.split("<answer>")[1].split("</answer>")[0].strip()
        except Exception:
            return output
    return output
```

The course uses this exact pattern to grade chain-of-thought
outputs against the same `__expected` column as non-CoT outputs.

## Custom code graders

Use a `python` assertion for anything beyond the built-ins:

```yaml
defaultTest:
  assert:
    - type: python
      value: file://grader.py
```

`grader.py`:

```python
import re

def get_assert(output, context):
    topic = context["vars"]["topic"]
    expected = int(context["vars"]["count"])
    actual = len(re.findall(rf"(^|\W){re.escape(topic)}(\W|$)", output.lower()))
    passed = actual == expected
    return {
        "pass": passed,
        "score": 1 if passed else 0,
        "reason": f"Expected {topic!r} ×{expected}, got ×{actual}",
    }
```

promptfoo calls `get_assert(output, context)` and accepts:

- `bool` — pass/fail
- `float` — a score
- `dict` with `pass`, `score`, `reason` — preferred; gives the
  dashboard a useful reason string

`context["vars"]` carries the row's input variables.

## Custom model graders

A `python` assertion that internally calls a model. This is how
lesson 9 implements the multi-dimensional summarization rubric.
See `model_graded.md` for the full pattern. The promptfoo
plumbing is identical to a code grader — only the body of
`get_assert` calls the Anthropic API and parses the JSON response.

## Comparing models and prompts

Adding a second `providers` entry runs the same prompt × test
matrix across both models, side by side in the dashboard. This is
the cheapest way to answer "can I switch to Haiku without
regressing?".

Adding a second `prompts` entry does the same for prompt versions.
You can combine: 3 prompts × 2 models × 20 tests = 120 cells in
one run.

The dashboard groups columns by prompt and within each prompt by
provider, which makes it easy to see whether the same prompt
behaves differently across models.

## The dashboard

```bash
npx promptfoo@latest view
```

Opens a local web dashboard. Click the magnifying glass on any
cell to see the full input prompt, the model's raw output, and
each assertion's grade with the reason. This is where failure-mode
analysis actually happens — read the failing cells, not just the
summary bar.

## Tips and gotchas

- **Set `ANTHROPIC_API_KEY` in the environment**, not in the YAML.
  Don't commit the key.
- **Pin model IDs** in the config (e.g. `claude-sonnet-4-6`, not
  an alias). Floating IDs make scores non-reproducible.
- **CSV `__expected` syntax** uses `equals:`, `contains:`,
  `regex:`, etc. — see the
  [docs](https://www.promptfoo.dev/docs/configuration/expected-outputs/).
- **One assertion = one criterion.** Splitting into multiple
  assertions gives you per-criterion pass rates in the dashboard,
  not just a single composite.
- **`temperature: 0` on judge providers.** Reproducibility matters
  for grading.
- **Save the eval output JSON** (`outputPath` in the config) when
  you want diffs across iterations.
