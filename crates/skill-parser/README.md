# skill-parser

Parses an Agent Skill's `SKILL.md` file: splits the YAML frontmatter from the
markdown body using [`gray_matter`](https://crates.io/crates/gray_matter) and
deserializes the frontmatter into typed Rust structs.

This crate only parses. Rule enforcement (kebab-case names, mandatory
`metadata.author`/`metadata.version`, etc.) lives in `skill-validator`.

## API

- `parse_skill(dir: &Path).await -> Result<ParsedSkill, ParseError>` — reads
  `dir/SKILL.md` (async, via `tokio::fs`) and returns the parsed result.
- `SkillFrontmatter` — fields from the
  [Agent Skills spec](https://agentskills.io/specification.md) plus the
  repo-local `metadata` block.
- `Metadata { author, version }` — both `Option<String>`; required-ness is
  enforced by `skill-validator`, not here.
- `ParsedSkill { dir_name, dir_path, frontmatter, body }`.
- `ParseError` — `MissingSkillMd`, `NoFrontmatter`, `InvalidFrontmatter`,
  `InvalidDirName`, `Io`.

## Example

```rust,no_run
use skill_parser::parse_skill;
use std::path::Path;

# async fn run() -> Result<(), skill_parser::ParseError> {
let skill = parse_skill(Path::new("skills/markdown")).await?;
println!("{} v{:?}", skill.frontmatter.name, skill.frontmatter.metadata);
# Ok(()) }
```
