# skill-cli

CLI orchestrator that validates, archives, and publishes the skills under
`skills/`. Built on top of `skill-parser`, `skill-validator`, and
`skill-archiver`.

## Subcommands

- `check` — parse and validate every skill. Exits non-zero on any failure
  without writing files.
- `build` — validate every skill (collecting every error, not fail-fast); if
  all skills pass, clean `dist/` and write `<name>-v<version>.zip` per skill
  in parallel. Exits non-zero on any validation failure and does *not* touch
  `dist/` in that case.
- `upload` — runs `build`; aborts if any skill failed validation. Otherwise
  lists existing GitHub Releases on the target repo and:
  - **No release with that tag** → create the release and upload the ZIP.
  - **Release exists but the ZIP asset is missing** (e.g. a previous run
    crashed between create and upload) → upload the asset to the existing
    release.
  - **Release exists with the expected asset** → skip.

## Flags

- `--skills-dir <PATH>` — defaults to `./skills`.
- `--dist-dir <PATH>` — defaults to `./dist`.
- `--repo <OWNER/NAME>` (`upload` only) — overrides `$GITHUB_REPOSITORY` and
  the `origin` remote.
- `--dry-run` (`upload` only) — still calls the GitHub list endpoint (so it
  exercises auth + repo access), but logs the planned actions instead of
  creating releases or uploading assets.

## Environment

- `GITHUB_TOKEN` — required for `upload`. In GitHub Actions, `secrets.GITHUB_TOKEN`
  with `permissions: contents: write` is sufficient.
- `GITHUB_REPOSITORY` — set automatically by GitHub Actions. Falls back to
  parsing `git remote get-url origin` when running locally.
- `RUST_LOG` — standard `tracing_subscriber` env filter; defaults to `info`.

## Usage

```bash
# CI uses this in .github/workflows/release.yml
cargo run --release -p skill-cli -- upload

# Local validation
cargo run -p skill-cli -- check

# Local build + dry-run upload against a specific repo
GITHUB_TOKEN=ghp_... \
  cargo run -p skill-cli -- upload --repo 46ki75/skills --dry-run
```
