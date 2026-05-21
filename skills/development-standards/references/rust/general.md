# Rust Generic Development Standards

Org-wide convention for any Rust project. The HTTP-layer specifics live in [Rust Axum OpenAPI Web Server](https://www.notion.so/Rust-Axum-OpenAPI-Web-Server-35934608d5c9806c8eb8fa19a78efd7e?pvs=21).

## Use Cargo workspaces, inherit everything from the root

Every Rust repo is a Cargo workspace. Shared dependencies, package metadata, and lints are declared **once** in the root `Cargo.toml`; member crates inherit with `workspace = true`.

Stable since Rust 1.64 (`[workspace.dependencies]`, `[workspace.package]`) and 1.74 (`[workspace.lints]`).[[1]](https://doc.rust-lang.org/cargo/reference/workspaces.html)

### Root `Cargo.toml`

```toml
[workspace]
resolver = "3"
members = ["crates/*"]

[workspace.package]
edition = "2024"
rust-version = "1.85"
license = "Apache-2.0"
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/your-org/your-repo"

[workspace.dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"

[workspace.lints.clippy]
unwrap_used = "deny"
```

### Member crate `Cargo.toml`

```toml
[package]
name = "foo-core"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true

[lints]
workspace = true

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
thiserror = { workspace = true }
```

## Pin the toolchain with `rust-toolchain.toml`

`rust-version` in `[workspace.package]` declares the MSRV, but does not enforce it — the build still uses whatever toolchain is active. Pair it with a `rust-toolchain.toml` at the repo root set to the **same version**, so local builds and CI actually run against the declared minimum.

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.85"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

When both agree, `cargo build` and `cargo test` in CI are a real MSRV test, not just a declaration.

## Rules

- Any dependency used by **more than one** crate MUST be declared in `[workspace.dependencies]` and inherited with `workspace = true`. Single-use deps MAY stay local.
- Members never re-pin a version. `tokio = "1.40"` directly in a member crate is a CI failure.
- Member-level `features` are **additive** with the workspace entry. You cannot subtract features or override `default-features` from the member side — if you need that, the workspace entry is wrong.[[1]](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- `optional = true` MUST live at the member level. Cargo disallows `optional` in `[workspace.dependencies]`.[[2]](https://rust-lang.github.io/rfcs/2906-cargo-workspace-deduplicate.html)
- Publishing is unaffected: `cargo publish` rewrites `workspace = true` into concrete versions in the published manifest.[[3]](https://users.rust-lang.org/t/cargo-workspaces-inheriting-and-publishing/128298)

## Lints for documentation

`missing_docs` can be enforced across the workspace via `[workspace.lints]` — no need to add `#![warn(missing_docs)]` to each crate's `lib.rs`.

```toml
[workspace.lints.rust]
missing_docs = "warn"
```

Members inherit it automatically via `[lints] workspace = true`.

If you want it on only _some_ crates (e.g. public library crates but not internal binaries), keep it out of `[workspace.lints]` and put `#![warn(missing_docs)]` directly in those specific crates' `lib.rs` instead.

## Formatting

Run `cargo fmt --all` before every commit. CI rejects unformatted code with `cargo fmt --all -- --check`.

## Task runner

Use [`just`](https://github.com/casey/just) as the task runner. Test commands and their setup dependencies vary by project — the `justfile` is the single source of truth for what `test`/`lint`/`build`/`ci` mean here. Other tools (CI, contributors, AI agents, the rest of this guide) invoke `just <recipe>`, never the underlying `cargo` command directly.

```
fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace

ci: fmt-check lint test
```

## Test coverage

Use [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) for test coverage. It wraps Rust's built-in LLVM source-based instrumentation (`-C instrument-coverage`), works on stable, and reports at the region level rather than just lines.[[1]](https://rustprojectprimer.com/measure/coverage.html)[[2]](https://doc.rust-lang.org/rustc/instrument-coverage.html)

```bash
cargo install cargo-llvm-cov --locked
```

Per the task-runner principle, the coverage tool never decides what to test — the project's test recipe does. Use `cargo llvm-cov --no-report` to run instrumented tests without emitting a report, then `cargo llvm-cov report` to consume the resulting profdata. Keep the fast `test` recipe plain so the inner loop isn't slowed by instrumentation, and add a parallel `test-cov` recipe alongside it:

```jsx
# Instrumented unit / hermetic test run (no report yet)
test-cov:
    cargo llvm-cov --no-report --workspace

# AI-friendly: per-file table (drop 100% files) + uncovered line numbers
coverage: test-cov
    cargo llvm-cov report --show-missing-lines --color=always 2>&1 | grep -v " 100.00%"

# Local HTML drilldown
coverage-html: test-cov
    cargo llvm-cov report --html --open

# CI / Codecov upload
coverage-ci: test-cov
    cargo llvm-cov report --lcov --output-path lcov.info
```

`--show-missing-lines` is the recommended output for LLM consumption or PR triage: a short per-file table plus a flat list of uncovered `file:line` ranges. Pair it with `--fail-uncovered-lines <MAX>` in CI for self-explaining failure messages. Live-tier coverage uses the same pattern — see the Integration tests section below.

Caveats:

- Branch coverage requires nightly on `cargo-llvm-cov`; on stable, expect line/region coverage only.[[3]](https://mcpmarket.com/tools/skills/rust-code-coverage-cargo-llvm-cov)
- Coverage % is a floor signal, not a quality signal. Treat it as a guardrail (e.g. fail CI if coverage drops by more than X points), not a target to maximize.

## Integration tests

Split integration tests into two tiers:

- **Hermetic** — no external dependencies, no secrets, deterministic. Runs on every PR.
- **Live / approval-required** — hits real services, costs money, needs secrets, may have side effects. Runs only behind a manual approval gate.

Keep them in physically separate files so the split is obvious:

```
crates/foo/tests/
  integration.rs   # hermetic, runs on every PR
  live.rs          # #[ignore]'d, approval-required
```

Gate live tests with the built-in `#[ignore]` attribute and a reason string. Default `cargo test` skips them; CI opts in with `-- --ignored`.[[1]](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

```rust
#[test]
#[ignore = "live: hits real API, requires API_KEY"]
fn calls_live_api() { /* ... */ }
```

Use `#[cfg_attr(not(feature = "live-tests"), ignore)]` instead when the live tests pull in optional dependencies you don't want compiled by default.[[2]](https://stackoverflow.com/questions/48583049/run-additional-tests-by-using-a-feature-flag-to-cargo-test)

Extend the `justfile` with live recipes:

```jsx
test-live:
    cargo test --workspace -- --ignored

# Instrumented live test run (mirrors test-cov from `## Test coverage`)
test-live-cov:
    cargo llvm-cov --no-report --workspace -- --ignored

coverage-live: test-live-cov
    cargo llvm-cov report --show-missing-lines --color=always 2>&1 | grep -v " 100.00%"

ci-live: fmt-check lint test test-live
```

CI wiring:

- Default `just ci` runs hermetic tests only and gates PR merges.
- `just ci-live` runs on manual dispatch or schedule, behind a GitHub Actions environment with required reviewers and environment-scoped secrets. Live-test failures MUST NOT block PR merges — they fail for reasons unrelated to the diff.

Name any file or test that hits an external system with a `live_` prefix so triage is grep-able. See _Rust Project Primer_'s External Services chapter for the broader pattern.[[3]](https://rustprojectprimer.com/testing/external-services.html)

## Enforcement

Cargo has no built-in lint for "member specifies a version directly." Run `cargo-workspace-inheritance-check` in CI to catch drift and promotion candidates.[[4]](https://github.com/marketplace/actions/cargo-workspace-inheritance-check)
