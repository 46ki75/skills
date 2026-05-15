# skill_uploader

CLI orchestrator that validates, archives, and publishes the skills under
`skills/`. Built on top of `skill_parser`, `skill_validator`, and
`skill_archiver`.

## Subcommands

- `check` — parse and validate every skill. Exits non-zero on any failure
  without writing files.
- `build` — clean `dist/`, validate every skill, then write
  `<name>-v<version>.zip` per valid skill into `dist/`. Exits non-zero if any
  skill failed validation, but always processes the rest first.
- `upload` — runs `build`, then lists existing GitHub Releases on the target
  repo and creates a new release plus uploads the ZIP for any artifact whose
  `<name>-v<version>` tag does not yet exist. A release is treated as an
  immutable version: existing tags are skipped.

## Flags

- `--skills-dir <PATH>` — defaults to `./skills`.
- `--dist-dir <PATH>` — defaults to `./dist`.
- `--repo <OWNER/NAME>` (`upload` only) — overrides `$GITHUB_REPOSITORY` and
  the `origin` remote.
- `--dry-run` (`upload` only) — log what would be uploaded without creating
  releases.

## Environment

- `GITHUB_TOKEN` — required for `upload`. In GitHub Actions, `secrets.GITHUB_TOKEN`
  with `permissions: contents: write` is sufficient.
- `GITHUB_REPOSITORY` — set automatically by GitHub Actions. Falls back to
  parsing `git remote get-url origin` when running locally.
- `RUST_LOG` — standard `tracing_subscriber` env filter; defaults to `info`.

## Usage

```bash
# CI uses this in .github/workflows/release.yml
cargo run --release -p skill_uploader -- upload

# Local validation
cargo run -p skill_uploader -- check

# Local build + dry-run upload against a specific repo
GITHUB_TOKEN=ghp_... \
  cargo run -p skill_uploader -- upload --repo 46ki75/skills --dry-run
```
