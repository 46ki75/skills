---
name: development-standards
description: >
  Org-internal engineering standards for this organization's projects.
  Invoke whenever scaffolding a new repo, auditing an existing one,
  setting up CI, or configuring any of these tooling files: `Cargo.toml`,
  `rust-toolchain.toml`, `justfile`, `.editorconfig`, `tsconfig.json`,
  `package.json`, `bunfig.toml`, or `*.tf`. Also invoke for any work
  involving `axum`, `utoipa`, `utoipa-axum`, or `utoipa-swagger-ui`.
  Rust is currently the only fully documented language and covers Cargo
  workspace inheritance, MSRV pinning via `rust-toolchain.toml`, `just`
  as the task runner, `cargo-llvm-cov` coverage, the hermetic-vs-live
  integration test split, and the Axum + utoipa code-first OpenAPI
  layout (`OpenApiRouter`, Controller/UseCase/Repository layering,
  `ToSchema` DTOs). TypeScript, Node.js, Bun, Terraform, Rust libraries,
  and Rust GraphQL sections are stubs — invoke anyway so the user is
  offered the chance to define the convention rather than receiving an
  improvised one.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "0.2.0"
---

# Development Standards

Org-internal engineering standards. This file is a **router** — load the
reference that matches the task, not the whole tree.

## Routing

### Cross-cutting — `references/general/`

| File                | When to read                                                  |
| ------------------- | ------------------------------------------------------------- |
| `git-repository.md` | New repo setup, configuring `.editorconfig`, baseline layout. |

Commit-message conventions live in the separate `conventional-commits`
skill — defer there, not here.

### Rust — `references/rust/`

| File             | When to read                                                                                                                                |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `general.md`     | Any Rust project: workspace inheritance, `rust-toolchain.toml`, `just` recipes, `cargo-llvm-cov`, integration test tiers.                   |
| `web-openapi.md` | HTTP API with `axum` + `utoipa`: `OpenApiRouter`, Controller/UseCase/Repository layering, `ToSchema` DTOs, error mapping, Swagger UI.       |
| `web-graphql.md` | _Stub — not yet documented._                                                                                                                |
| `library.md`     | _Stub — not yet documented._                                                                                                                |

### Planned but unwritten

| Section                  | Status                       |
| ------------------------ | ---------------------------- |
| `references/typescript/` | _Stub — not yet documented._ |
| `references/nodejs/`     | _Stub — not yet documented._ |
| `references/bun/`        | _Stub — not yet documented._ |
| `references/terraform/`  | _Stub — not yet documented._ |

## Handling stubbed sections

The user invoked this skill expecting org conventions. If the matching
reference is a stub, **do not improvise an org standard** — that risks
laundering a one-off decision into apparent policy. Instead:

1. Tell the user the section is not yet documented.
2. Look for a de facto convention in the current repo or in sibling
   projects the user has open. If found, propose it and ask whether to
   adopt it as the standard.
3. If no convention exists, offer a recommendation based on general
   engineering judgment, label it clearly as a suggestion (not policy),
   and offer to write it up into the stub once the user decides.

## When NOT to invoke

- General programming or library tutorials — use language- or library-
  specific skills (`mcp-knowledge`, `ag-ui-knowledge`, `rust-toasty`,
  `conventional-commits`) or upstream docs.
- Debugging business logic.
- Reviewing changes that do not touch tooling, project layout, or the
  architectural seams covered in `references/`.
