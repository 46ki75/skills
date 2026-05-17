# skills

My personal collection of [Agent Skills](https://agentskills.io/home). Each
skill lives under `skills/<name>/` with a `SKILL.md` (YAML frontmatter + body)
and any supporting files.

Not intended for external contributors — feel free to fork or copy ideas, but
issues/PRs aren't actively triaged.

## Layout

- `skills/` — my published skills.
- `crates/` — Rust workspace that validates, archives, and publishes them.
- `.agents/skills/` — skills authored by other providers (reference only).
- `submodules/` — upstream repositories tracked as git submodules.

## Releasing

1. Edit a skill under `skills/<name>/` and bump `metadata.version` in its
   `SKILL.md` frontmatter.
2. Push (or merge a PR) to `main`.

`.github/workflows/release.yml` runs `skill-cli upload`, which validates
every skill, builds `<name>-v<version>.zip` into `dist/`, then for each
artifact:

- Creates the release and uploads the ZIP if no release with that tag exists.
- Uploads the ZIP to the existing release if the tag exists but the asset is
  missing (orphan-asset recovery).
- Skips if both are already present.

### No manual `git tag`

The GitHub Releases API creates the tag when it creates the release, pointing
at the default-branch HEAD at API-call time (the commit that triggered the
workflow). Manual `git tag <name>-v<version>` is **not** needed and would
race with the workflow.

A `<name>-v<version>` release is immutable once published — bump
`metadata.version` to ship a fix.

## Local commands

```bash
# Validate every skill without writing files
cargo run -p skill-cli -- check

# Build ZIPs into ./dist (does not upload)
cargo run -p skill-cli -- build

# Dry-run upload (calls GitHub list API, logs planned actions)
GITHUB_TOKEN=ghp_... \
  cargo run -p skill-cli -- upload --repo 46ki75/skills --dry-run

# Markdown lint
pnpm run lint

# Init submodules (run at repo root)
git submodule update --init --recursive
```
