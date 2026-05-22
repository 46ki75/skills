# Git Repository Standards

## Create `.editorconfig`

```ini
# editorconfig.org
root = true

[*]
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
```

## Markdown linting with `markdownlint-cli2`

Every repository that contains Markdown should lint it with
[`markdownlint-cli2`](https://github.com/DavidAnson/markdownlint-cli2).
Use the config and `package.json` shape below — do not improvise rule
sets per repo.

### `.markdownlint-cli2.yaml`

Place at the repository root.

```yaml
config:
  MD013:
    line_length: 200
    tables: false
    code_blocks: false
  MD024:
    siblings_only: true
  MD029:
    style: ordered
ignores:
  - "**/node_modules/**"
```

Rule choices, and why:

- **MD013 (line length) → 200, tables/code blocks exempt.** Long URLs,
  command examples, and tables routinely exceed the default 80. A hard
  wrap there hurts readability more than it helps. 200 is generous
  enough that real prose still gets flagged.
- **MD024 (no duplicate headings) → `siblings_only: true`.** Repeated
  headings under different parents (e.g. `## Install` appearing under
  multiple OS sections) are legitimate. Only flag duplicates at the
  same level.
- **MD029 (ordered list prefix) → `ordered`.** Use `1.`, `2.`, `3.`
  literally — never the `1.`, `1.`, `1.` lazy style. The rendered
  output is the same, but the source stays diff-friendly and human-
  readable.

### `ignores`

Add globs for anything the linter should not touch. Common entries:

| Path                  | Reason                                                                      |
| --------------------- | --------------------------------------------------------------------------- |
| `**/node_modules/**`  | Third-party packages.                                                       |
| `./submodules/**`     | Git submodules — upstream owns their lint rules.                            |
| `./target/**`         | Rust build output.                                                          |
| `./.claude/**`        | Agent-generated transcripts and worktrees.                                  |
| `./refs/`, `./notes/` | Local-only scratch / reference material, if the repo uses such conventions. |

Vendored or generated Markdown (third-party docs copied in, generated
API docs) should also be ignored — fix it upstream, not here.

### `package.json`

Pin `markdownlint-cli2` as a dev dependency and expose a `lint` script:

```json
{
  "scripts": {
    "lint": "markdownlint-cli2 \"**/*.md\""
  },
  "devDependencies": {
    "markdownlint-cli2": "^0.22.1"
  }
}
```

The glob in the script is the lint **target**; `ignores` in the YAML
is the exclusion list. Keep both — narrowing the glob to skip a
directory hides the file from `--fix` runs too.

### Running

```bash
npm run lint           # or: bun run lint
npx markdownlint-cli2 --fix "**/*.md"   # auto-fix where possible
```

Wire `npm run lint` into CI so Markdown regressions fail the build the
same way code-lint regressions do.

### Editor integration

Recommend the VS Code extension
[`DavidAnson.vscode-markdownlint`](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint).
It reads the same `.markdownlint-cli2.yaml`, so editor diagnostics
match CI output exactly. Add it to `.vscode/extensions.json` under
`recommendations` so contributors get prompted on first open:

```json
{
  "recommendations": ["DavidAnson.vscode-markdownlint"]
}
```
