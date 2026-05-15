# skill_validator

Validates a `ParsedSkill` (from `skill_parser`) against the Agent Skills spec
plus this repository's house rules. Errors are collected, not short-circuited,
so a single pass over `skills/` surfaces every problem at once.

## Rules enforced

- `name` is present, non-empty, kebab-case (`[a-z0-9-]`, no leading, trailing,
  or consecutive `-`), and equals the on-disk directory name.
- `description` is present and non-empty.
- `metadata.author` is present and non-empty (repo rule; optional in the spec).
- `metadata.version` is present and shaped like `MAJOR.MINOR` or
  `MAJOR.MINOR.PATCH` with digits only (repo rule; optional in the spec).

## API

- `validate(skill: &ParsedSkill) -> ValidationReport` — always returns a
  report; check `is_ok()` to decide whether the skill is publishable.
- `ValidationReport { dir_name, errors }` — implements `Display` for human-
  readable output.
- `ValidationError` — one variant per rule (`NameMissing`, `NameNotKebabCase`,
  `NameMismatch`, `DescriptionMissing`, `AuthorMissing`, `VersionMissing`,
  `InvalidVersion`).

## Example

```rust,no_run
use skill_parser::parse_skill;
use skill_validator::validate;
use std::path::Path;

# async fn run() -> Result<(), skill_parser::ParseError> {
let skill = parse_skill(Path::new("skills/markdown")).await?;
let report = validate(&skill);
if !report.is_ok() {
    eprintln!("{report}");
}
# Ok(()) }
```
