# Skill Repository

This repository contains [Agent Skills](https://agentskills.io/home). It was formerly called Claude Skills.

## Directory Structure

- `.agents/skills`: Contains Agent Skills which are created by other providers.
- `skills/`: Contains Agent Skills. When you create new skills, you should create them in this directory.
- `submodules/`: Contains official repositories as git submodules for reference.
- `.markdownlint-cli2.yaml`: Configuration file of `markdownlint-cli2`.

## Rules

When you create a skill, use `skill-creator` skill.

Regarding the frontmatter of the `SKILL.md` file, `$.metadata.author` and `$.metadata.version` are not mandatory
in the [specification](https://agentskills.io/specification.md), but they are mandatory in this repository.

```yaml
name: my-skill
description: placeholder
license: MIT # Always "MIT" in this repository.
metadata:
  author: "Ikuma Yamashita" # Always "Ikuma Yamashita" in this repository.
  version: "1.0"
```

### Linting

You should run `markdownlint-cli2` after creating markdown files.

Use following commands, it triggers npm scripts and runs lint.

```bash
# Install dependencies
❯ pnpm i

# Run lint
❯ pnpm run lint

> @ lint /home/ikuma/org/46ki75/skills
> markdownlint-cli2 ./**/*.md

markdownlint-cli2 v0.22.1 (markdownlint v0.40.0)
Linting: 75 file(s)
Summary: 0 error(s)
```

## Git Submodules

Run the commands at repository root, not at `skills/`.

```bash
git submodule update --init --recursive
```

To keep submodules up to date after pulling changes:

```bash
git submodule update --recursive
```

You can add git submodules if you need.
