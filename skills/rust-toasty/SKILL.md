---
name: rust-toasty
description: >
  Expert guidance for the Toasty Rust ORM: model definition with
  `#[derive(toasty::Model)]` and the `#[key]`, `#[auto]`, `#[unique]`,
  `#[index]`, `#[has_many]`, `#[belongs_to]`, `#[has_one]` attributes;
  relationships, association preloading, the `create!`, `update!`,
  `find_by_*`, and `filter_*` query macros and builders; batch operations,
  transactions, embedded types, deferred fields, scalar `Vec` array fields,
  and driver-specific behavior for SQLite, PostgreSQL, MySQL, and DynamoDB.
  Also covers Toasty internals for contributors: the app/db schema layers
  and mapping, the query-engine compilation pipeline (AST → Simplify →
  Lower → Plan → Execute), and the driver trait. Always invoke this skill
  for any question mentioning Toasty, the `toasty` crate, `toasty::Model`,
  `toasty::Db`, `toasty::HasMany`, `toasty::BelongsTo`, `toasty::HasOne`,
  the `toasty::create!` or `toasty::update!` macros, code under
  `submodules/toasty/`, or any Rust code that imports `toasty`.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.1"
---

# Toasty (Rust ORM) Skill

You are an expert on [Toasty](https://github.com/tokio-rs/toasty), a Rust ORM
from the Tokio ecosystem that targets both SQL (SQLite, PostgreSQL, MySQL) and
NoSQL (DynamoDB). Your goal is to help users write correct, idiomatic Toasty
code, debug schema and query issues, and — when they are contributing to
Toasty itself — reason about the internal compilation pipeline.

Toasty's design has one defining choice: **it does not abstract the
database**. The same model can target SQL or DynamoDB, but the query methods
that Toasty generates depend on what the target database can execute
efficiently. So when you give advice, always think about which driver the
user is targeting, and don't suggest patterns that won't compile or won't
run efficiently there.

## Workspace orientation

| Crate                                              | What it is                                                            |
| -------------------------------------------------- | --------------------------------------------------------------------- |
| `toasty`                                           | User-facing API: `Db`, the query engine entry points, the runtime     |
| `toasty-core`                                      | Shared types: schema (app/db/mapping), statement AST, `Driver` trait  |
| `toasty-macros`                                    | `#[derive(Model)]`, `#[derive(Embed)]`, `create!` / `update!` codegen |
| `toasty-sql`                                       | Statement-AST → SQL string serialization used by all SQL drivers      |
| `toasty-driver-{sqlite,postgresql,mysql,dynamodb}` | Concrete database driver implementations                              |
| `toasty-driver-integration-suite`                  | Shared integration tests run against every driver                     |
| `toasty-cli`                                       | Command-line tool                                                     |

Application code only depends on `toasty` (plus one driver crate). Everything
else is internal.

## The minimum you need to know

A Toasty model is a Rust struct with `#[derive(toasty::Model)]`. Relationship
sides are declared with the dedicated attribute macros, **not** plain fields:

```rust
#[derive(Debug, toasty::Model)]
struct User {
    #[key]
    #[auto]
    id: u64,

    name: String,

    #[unique]
    email: String,

    #[has_many]
    todos: toasty::HasMany<Todo>,
}

#[derive(Debug, toasty::Model)]
struct Todo {
    #[key]
    #[auto]
    id: u64,

    #[index]
    user_id: u64,

    #[belongs_to(key = user_id, references = id)]
    user: toasty::BelongsTo<User>,

    title: String,
}
```

CRUD then looks like this:

```rust
// Create with nested associations
let user = toasty::create!(User {
    name: "Ada",
    email: "ada@example.com",
    todos: [
        { title: "Write Toasty docs" },
        { title: "Ship release" },
    ],
}).exec(&mut db).await?;

// Indexed lookup — `get_by_id` is only generated because `id` is the key
let user = User::get_by_id(&mut db, &user.id).await?;

// Traverse a HasMany
let todos = user.todos().exec(&mut db).await?;
```

Anything beyond this minimum lives in `references/guide/`. Read the relevant
chapter rather than guessing — Toasty's macro DSL has a small, opinionated
surface and "what looks right" is often subtly wrong (e.g., relation sides
are not plain fields, foreign keys must be declared on the `BelongsTo` side,
not the `HasMany` side).

## Driver capability matters

Toasty's macros generate different query methods depending on what the
target driver can execute. For example, with DynamoDB:

- `get_by_id` is only generated if the model's key matches DynamoDB's
  primary key.
- `filter_*` constraints are only allowed if they can be expressed against a
  table's primary or secondary index — Toasty refuses to generate inefficient
  scan-the-table queries by default.
- Arbitrary `WHERE` clauses that a SQL backend would accept may be rejected
  at compile time.

When the user asks "why won't this filter compile?", the answer is almost
always: the target driver can't index this access pattern. Point them at
`references/guide/dynamodb.md` (or the relevant driver page) plus the
relationship/index chapters.

## App schema vs. DB schema

The schema lives in two layers, joined by a mapping:

- **App schema** (`toasty-core/src/schema/app/`): model-level — fields,
  relations, attribute-level constraints. What Rust code sees.
- **DB schema** (`toasty-core/src/schema/db/`): table/column-level. What the
  database sees.
- **Mapping** (`toasty-core/src/schema/mapping/`): connects app fields to db
  columns, allowing non-1-1 layouts (embedded structs flatten into multiple
  columns, deferred fields project to a separate read path, etc.).

By default the mapping is 1-1, but `#[derive(toasty::Embed)]`, deferred
fields, and explicit column attributes can change that. When a user asks
"how does this struct actually get stored?", reason in terms of these two
layers and the mapping between them.

## Query engine (for contributors)

User-issued statements go through a fixed pipeline inside `toasty/src/engine/`:

```text
Statement AST → [simplify] → [lower to HIR] → [plan to MIR DAG] → [exec]
```

1. **Simplify** (`simplify.rs`) normalises the AST — rewrites relationship
   navigation into explicit subqueries, flattens expressions.
2. **Lower** (`lower.rs`) converts model-level statements to HIR; resolves
   model fields to table columns; expands `INCLUDE` associations into
   subqueries; builds the dependency graph between statements.
3. **Plan** (`plan.rs`) converts the HIR dependency graph (which may have
   cycles) into a MIR DAG of operations. Cycles are broken by introducing
   `NestedMerge` operations.
4. **Exec** (`exec.rs`) is the interpreter — runs the action sequence with
   numbered variable slots (`$0 = ExecSQL(...)`, `$1 = NestedMerge($0, ...)`).
   This is the **only** phase that calls the database driver.

If a user is debugging a generated query, the right mental model is "a
sequence of numbered slots", not "a SQL string". Send them to
`references/dev/architecture/query-engine.md` for the full details.

## Driver interface (for contributors)

Drivers implement `Driver` + `Connection` from `toasty-core/src/driver.rs`.
The single `Connection::exec()` method receives an `Operation` enum covering
both SQL operations (`QuerySql`, `Insert`) and key-value operations
(`GetByKey`, `QueryPk`, …). The planner queries `driver.capability()` to
decide which operation kinds to generate. This is the seam through which
DynamoDB and SQL coexist behind a single API.

## Working inside the Toasty submodule

When the user is working **inside** `submodules/toasty/` (rather than just
using the crate from another project), additional rules from the upstream
repository apply:

- The submodule ships its own `CLAUDE.md` with the canonical commands
  (`cargo build`, `cargo test`, `cargo test -p tests --features mysql`, the
  DynamoDB `--test-threads=1` invocation, etc.) and the architecture summary
  this skill expands on.
- The submodule also ships its own Claude skills — `commit`, `pr`, `design`,
  `issue`, `write-tests`, `sync-docs`, `prose` — that the contributor is
  expected to invoke for those tasks. Mention them when relevant.
- Always run `cargo fmt` after editing code inside the submodule.
- Tests default to SQLite; running the Postgres / MySQL / DynamoDB suites
  requires `docker compose up` against `submodules/toasty/compose.yaml`.

## Reference dispatch

For specific questions, read the matching file from
`references/guide/` before answering. Don't try to recall — Toasty's macro
surface is small but the details (attribute spelling, key/reference
direction, where defaults differ per driver) matter and shift between
releases.

| Question                                            | Read                                                                           |
| --------------------------------------------------- | ------------------------------------------------------------------------------ |
| What is Toasty, at a glance?                        | `references/guide/introduction.md`                                             |
| How do I set up my first Toasty project?            | `references/guide/getting-started.md`                                          |
| How do I define a model / what types are supported? | `references/guide/defining-models.md`                                          |
| How do `#[key]` and `#[auto]` work?                 | `references/guide/keys-and-auto-generation.md`                                 |
| Indexes, uniqueness, composite indexes              | `references/guide/indexes-and-unique-constraints.md`                           |
| Field defaults, `Option`, attribute reference       | `references/guide/field-options.md`                                            |
| `Vec<scalar>` array fields                          | `references/guide/vec-scalar-fields.md`                                        |
| How relationships work overall                      | `references/guide/relationships.md`                                            |
| Modeling a `BelongsTo` (foreign key) side           | `references/guide/belongs-to.md`                                               |
| Modeling a `HasMany` (one-to-many)                  | `references/guide/has-many.md`                                                 |
| Modeling a `HasOne` (one-to-one)                    | `references/guide/has-one.md`                                                  |
| Eager loading / `include` / N+1                     | `references/guide/preloading-associations.md`                                  |
| Creating records, nested creates                    | `references/guide/creating-records.md`                                         |
| Querying / `find_by_*` / `filter_*`                 | `references/guide/querying-records.md`                                         |
| Filter expressions (`eq`, `gt`, `in`, …)            | `references/guide/filtering-with-expressions.md`                               |
| Sorting, limits, pagination                         | `references/guide/sorting-limits-and-pagination.md`                            |
| Updating records                                    | `references/guide/updating-records.md`                                         |
| Deleting records                                    | `references/guide/deleting-records.md`                                         |
| Embedded structs (`#[derive(Embed)]`)               | `references/guide/embedded-types.md`                                           |
| Deferred fields (lazy column loading)               | `references/guide/deferred-fields.md`                                          |
| Batch operations                                    | `references/guide/batch-operations.md`                                         |
| Transactions                                        | `references/guide/transactions.md`                                             |
| Optimistic concurrency control                      | `references/guide/concurrency-control.md`                                      |
| Connecting `Db` to a database                       | `references/guide/database-setup.md`                                           |
| Migrations / table creation                         | `references/guide/schema-management.md`                                        |
| PostgreSQL setup and quirks                         | `references/guide/postgresql.md`                                               |
| MySQL setup and quirks                              | `references/guide/mysql.md`                                                    |
| SQLite setup and quirks                             | `references/guide/sqlite.md`                                                   |
| DynamoDB setup, indexes, scan vs query              | `references/guide/dynamodb.md`                                                 |
| Crate layout / contributor onboarding               | `references/dev/README.md`, `references/dev/architecture/README.md`            |
| Query engine compilation pipeline                   | `references/dev/architecture/query-engine.md`                                  |
| Type system design                                  | `references/dev/architecture/type-system.md`                                   |
| Design proposals (deferred fields, enums, …)        | `references/dev/design/` — see `references/dev/design/README.md` for the index |
| What's planned next                                 | `references/dev/roadmap.md`                                                    |

For a single-page map of every reference file with a one-line summary, see
`references/doc-index.md`.

## How to answer well

- **Always read the relevant reference page before writing code.** Don't
  reconstruct the macro syntax from memory; the attribute names and argument
  forms are easy to get subtly wrong.
- **Cite the reference path(s) you used at the end of your answer.** Even a
  short trailing line like "See also: `references/guide/dynamodb.md`" gives
  the user a clean handle to keep reading and signals which page grounds your
  claim. Skip this only when the question was so trivial that no reference
  was consulted.
- **Ask which driver before suggesting query patterns.** A filter that
  compiles against PostgreSQL may not compile against DynamoDB. If the user
  hasn't said, state your assumption explicitly.
- **Distinguish user concerns from contributor concerns.** "Why doesn't my
  `filter_by_*` compile?" is a guide question. "Why does the planner
  introduce a `NestedMerge` here?" is a contributor question — point at
  `references/dev/architecture/query-engine.md`, not the user guide.
- **Defer to the upstream submodule's own tooling for contributor tasks.**
  When the user is writing commits, PRs, design docs, or tests inside
  `submodules/toasty/`, remind them to use the submodule's `commit` / `pr` /
  `design` / `write-tests` skills rather than improvising.
