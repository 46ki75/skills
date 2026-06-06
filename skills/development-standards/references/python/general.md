# Python Generic Development Standards

Org-wide convention for any Python project. The reference implementation is the
`46ki75/anthropic-course` repository — places where it has not caught up with this
standard yet (no `justfile`, flat `src` layout) are called out below.

## Use uv with workspaces, declare tooling once at the root

Every Python repo uses [uv](https://docs.astral.sh/uv/) as the package and
environment manager, organized as a [uv workspace](https://docs.astral.sh/uv/concepts/projects/workspaces/).
Shared dev tooling (linter, type checker, test runner) is declared **once** in the
root `pyproject.toml` via `[dependency-groups]`; member packages declare only their
own runtime dependencies. This mirrors the Cargo workspace-inheritance rule on the
Rust side: one place to bump tool versions, zero drift between packages.

### Root `pyproject.toml`

```toml
[tool.uv.workspace]
members = ["packages/*"]

[dependency-groups]
dev = [
    "pyright>=1.1.409",
    "pytest>=8.3.0",
    "ruff>=0.14.0",
]

[tool.pyright]
typeCheckingMode = "strict"

[tool.pytest.ini_options]
testpaths = ["packages"]
markers = [
    "live: hits real services, costs money, requires secrets",
]
addopts = '-m "not live"'
```

### Member `pyproject.toml`

```toml
[project]
name = "foo-service"
version = "0.1.0"
description = "..."
readme = "README.md"
requires-python = ">=3.13"
dependencies = [
    "pydantic>=2.13.4",
]

[build-system]
requires = ["uv_build>=0.7.0,<0.8.0"]
build-backend = "uv_build"
```

### Rules

- Commit `uv.lock`. CI installs with `uv sync --frozen` so the lockfile is the
  single source of truth — a stale lock is a CI failure, not a silent re-resolve.
- Dev tooling (`ruff`, `pyright`, `pytest`) lives **only** in the root
  `[dependency-groups]`. A member re-declaring its own `pytest` pin is drift —
  remove it.
- Runtime dependencies live in the member's `[project.dependencies]`, never at
  the root. The root `pyproject.toml` is workspace plumbing, not a package.
- Invoke every tool through `uv run` (`uv run pytest`, `uv run ruff check .`) so
  it executes inside the workspace venv. Never rely on globally installed tools.

## Pin the interpreter with `.python-version`

`requires-python` in `[project]` declares the supported floor, but does not
control which interpreter actually runs. Pair it with a `.python-version` file at
the repo root pinning the **same major.minor**, so local `uv` invocations and CI
resolve to the version the floor promises — the same pairing as `rust-version`
(declares) plus `rust-toolchain.toml` (enforces) on the Rust side.

```text
3.13
```

When both agree, `uv run pytest` in CI is a real floor test, not just a
declaration. Bump them together, in one commit.

## Package layout

Default to the packaged `src` layout — an importable package directory under
`src/`:

```text
packages/foo-service/
  pyproject.toml
  src/
    foo_service/
      __init__.py
      util.py
  tests/
    test_util.py
```

uv installs workspace members in editable mode, so `tests/test_util.py` does a
plain `import foo_service` with no path manipulation. This layout is required for
anything published, imported by another workspace member, or shipped as an
artifact — the package boundary is the import name.

### Flat variant (script-style repos only)

Course-work and experiment repos may use flat modules directly under `src/`, with
pytest's `pythonpath` shim wiring imports up (this is what `anthropic-course`
does):

```toml
# member pyproject.toml
[tool.pytest.ini_options]
testpaths = ["tests"]
pythonpath = ["src"]
```

Pyright needs the matching hint in the **root** `pyproject.toml`, since the shim
only exists inside pytest:

```toml
[tool.pyright]
typeCheckingMode = "strict"
executionEnvironments = [
    { root = "packages/foo-course/tests", extraPaths = ["packages/foo-course/src"] },
]
```

The flat variant trades a `[build-system]` table for two config shims and
per-package test invocation. The moment a module needs to be imported from
another package, switch to the packaged layout — do not deepen the shims.

## Lint and format with ruff

[ruff](https://docs.astral.sh/ruff/) is both linter and formatter. Run with the
**default rule set** — no `[tool.ruff]` section unless a project has a concrete
reason to override, and then the override carries a comment saying why. Default
rules keep every repo's lint behavior identical and make `ruff` version bumps the
only source of new findings.

```bash
uv run ruff check .           # lint
uv run ruff format --check .  # format check (CI)
uv run ruff format .          # format in place (local)
```

## Type check with pyright, strict mode

`[tool.pyright]` lives in the root `pyproject.toml` with
`typeCheckingMode = "strict"`. Strict is the default for new code, not an
aspiration — relaxing it per-file with `# pyright: basic` is acceptable for
vendored or generated code, relaxing it repo-wide is not.

```bash
uv run pyright
```

## Logging

Use the standard-library [`logging`](https://docs.python.org/3/library/logging.html)
module for all diagnostics. `print()` is reserved for a program's actual
output — the thing a user would pipe to the next command. Everything else
(progress, debug traces, warnings, errors) goes through a logger, so verbosity,
format, and destination are configuration rather than code edits.

```python
import logging

logger = logging.getLogger(__name__)


def load_config(path: str) -> Config:
    logger.debug("loading config from %s", path)
    ...
```

### Rules

- Every module that logs declares one module-level logger via
  `logging.getLogger(__name__)`. Never log on the root logger and never share
  loggers across modules — `__name__` gives the hierarchy (`foo_service.util`)
  for free, so verbosity can be tuned per subsystem without touching code.
- Only the application entry point configures logging (`logging.basicConfig`
  or `logging.config.dictConfig`). Library and module code never calls
  `basicConfig`, never adds handlers, never sets levels — a library that
  configures logging hijacks that decision from every application that imports
  it.
- Use lazy `%`-style placeholders (`logger.info("loaded %s", path)`), not
  f-strings. The message is only formatted when the level is actually emitted,
  and a broken `__repr__` in an argument can't crash the log call itself.
- Log exceptions with `logger.exception(...)` inside `except` blocks — it
  records the traceback automatically; `logger.error(str(e))` throws the stack
  trace away.

## Testing with pytest

Split tests into two tiers, mirroring the Rust hermetic-vs-live convention:

- **Hermetic** — no external dependencies, no secrets, deterministic. Runs on
  every PR.
- **Live / approval-required** — hits real services, costs money, needs secrets,
  may have side effects. Runs only behind a manual approval gate.

The `live` marker is registered in the root `pyproject.toml` (see above), and
`addopts = '-m "not live"'` makes the hermetic tier the default — a bare `pytest`
can never accidentally spend money. The live tier opts in by overriding the
marker expression, which works because the last `-m` on the command line wins:

```python
import pytest


@pytest.mark.live
def test_calls_real_api() -> None:  # live: hits real API, requires API_KEY
    ...
```

```bash
uv run pytest            # hermetic only (addopts filters out live)
uv run pytest -m live    # live tier only
```

Name live test files and functions with a `live` prefix or suffix so triage is
grep-able. Live-test failures MUST NOT block PR merges — they fail for reasons
unrelated to the diff.

## Task runner

Use [`just`](https://github.com/casey/just), same as every other language in the
org. The `justfile` is the single source of truth for what
`lint`/`typecheck`/`test`/`ci` mean in this repo; CI, contributors, and AI agents
invoke `just <recipe>`, never the underlying `uv run ...` command directly.

```text
fmt-check:
    uv run ruff format --check .

lint:
    uv run ruff check .

typecheck:
    uv run pyright

test:
    uv run pytest

test-live:
    uv run pytest -m live

ci: fmt-check lint typecheck test
```

(`anthropic-course` predates this rule and drives `uv` directly from CI — new
repos start with the `justfile`.)

## CI

GitHub Actions with [`astral-sh/setup-uv`](https://github.com/astral-sh/setup-uv)
and its built-in cache. Two jobs: lint + typecheck, and test. Both run from the
repo root over the whole workspace — do not hardcode `working-directory` to a
single package; that silently stops covering packages added later.

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  lint-and-typecheck:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v8.1.0
        with:
          enable-cache: true
      - run: uv sync --frozen
      - run: just fmt-check
      - run: just lint
      - run: just typecheck

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v8.1.0
        with:
          enable-cache: true
      - run: uv sync --frozen
      - run: just test
```

`just test-live` runs on manual dispatch or schedule, behind a GitHub Actions
environment with required reviewers and environment-scoped secrets — the same
gating as `ci-live` on the Rust side.

## `.gitignore` baseline

```gitignore
.env
!.env.example

dist/

# Python
.venv/
__pycache__/
```

Secrets follow the `.env` + committed `.env.example` pattern: the example file
documents which variables a contributor must provide, the real file never lands
in history.
