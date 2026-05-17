# skill-archiver

Packages a validated `ParsedSkill` into a ZIP archive ready to attach to a
GitHub Release.

Each archive is written as `<dist>/<name>-v<version>.zip` and contains the
skill's directory at its top level — e.g. inside `markdown-v1.0.0.zip` the
entries are `markdown/SKILL.md`, `markdown/references/...`. Hidden entries
(any path component starting with `.`) are pruned entirely, including their
subtrees. The underlying `zip` crate is synchronous, so file writes happen
inside `tokio::task::spawn_blocking`.

## API

- `clean_dist(dist: &Path) -> Result<(), ArchiveError>` — removes `dist/` if
  present and recreates it empty.
- `build_archive(skill: &ParsedSkill, dist: &Path) -> Result<BuiltArtifact, ArchiveError>` —
  writes one ZIP and returns metadata about it. Returns `MissingVersion` if
  `metadata.version` is absent; run `skill_validator::validate` first.
- `BuiltArtifact { name, version, tag, file_name, zip_path }` — `tag` is
  always `"{name}-v{version}"` and matches what `skill-cli` looks for on
  GitHub.

## Example

```rust,no_run
use skill_archiver::{build_archive, clean_dist};
use skill_parser::parse_skill;
use std::path::Path;

# async fn run() -> anyhow::Result<()> {
let dist = Path::new("dist");
clean_dist(dist).await?;
let skill = parse_skill(Path::new("skills/markdown")).await?;
let artifact = build_archive(&skill, dist).await?;
println!("built {}", artifact.zip_path.display());
# Ok(()) }
```
