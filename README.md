# skills

A collection of [Agent Skills](https://agentskills.io/home) maintained by
[@46ki75](https://github.com/46ki75). Each skill lives under `skills/<name>/`
with a `SKILL.md` (YAML frontmatter + body) and any supporting files.

## Layout

- `skills/` — published skills.
- `crates/` — Rust workspace that validates, archives, and publishes the
  skills. See each crate's README for details.
- `.agents/skills/` — skills authored by other providers (reference only).
- `submodules/` — upstream repositories tracked as git submodules.

## Releasing a skill

The release pipeline is fully automated. To ship a new (or bumped) skill:

1. Edit the skill under `skills/<name>/` and bump `metadata.version` in its
   `SKILL.md` frontmatter.
2. Commit and push (or merge) to `main`.

That's it. `.github/workflows/release.yml` then runs
`cargo run --release -p skill_uploader -- upload`, which:

- Validates every skill (see `crates/skill_validator` for the rules).
- Builds a `<name>-v<version>.zip` per skill into `dist/`.
- For each artifact, queries GitHub Releases and:
  - creates a release **and** uploads the ZIP if no release with that tag
    exists,
  - uploads the ZIP to the existing release if the tag exists but the asset
    is missing (orphan-asset recovery),
  - skips if both are already present.

### No manual `git tag` required

The GitHub Releases API creates the tag for you when the release is created;
the tag points at the default branch's HEAD at API-call time (i.e. the commit
that triggered the workflow). Manually running `git tag <name>-v<version>`
before push is **not** needed and would race with the workflow.

A release/tag pair is treated as immutable: once `<name>-v<version>` exists
with the expected ZIP attached, the workflow will never overwrite it. To
publish a fix, bump `metadata.version` and let a new release be cut.

## Local development

```bash
# Install Node tooling (markdownlint-cli2 etc.)
pnpm i

# Lint markdown
pnpm run lint

# Validate every skill without writing files
cargo run -p skill_uploader -- check

# Build ZIPs into ./dist (does not upload)
cargo run -p skill_uploader -- build

# Dry-run upload against a specific repo
GITHUB_TOKEN=ghp_... \
  cargo run -p skill_uploader -- upload --repo 46ki75/skills --dry-run
```

## Git submodules

The `submodules/` directory contains upstream reference repositories. Run at
the repo root, not at `skills/`:

```bash
git submodule update --init --recursive
```

## License

MIT.
