# Toasty Documentation Index

One-line summary of every reference file shipped with the `rust-toasty`
skill. All paths are relative to `skills/rust-toasty/references/`. Source of
truth lives in `submodules/toasty/docs/` — these are curated copies, kept in
sync with the submodule revision pinned at the repo root.

## User guide (`guide/`)

### Foundations

| Path | Topic |
| ---- | ----- |
| `guide/introduction.md` | What Toasty is, design goals, SQL + NoSQL story |
| `guide/getting-started.md` | First project: `Cargo.toml`, `Db::builder`, hello-toasty |
| `guide/defining-models.md` | `#[derive(toasty::Model)]`, supported field types, `Option<T>` |
| `guide/keys-and-auto-generation.md` | `#[key]`, `#[auto]`, composite keys, UUID vs auto-increment |

### CRUD

| Path | Topic |
| ---- | ----- |
| `guide/creating-records.md` | `toasty::create!`, nested creates, default values |
| `guide/querying-records.md` | `find_by_*`, `filter_*`, returning `Option` vs error |
| `guide/updating-records.md` | `toasty::update!`, mutable references, partial updates |
| `guide/deleting-records.md` | `delete()`, cascading semantics per driver |

### Schema features

| Path | Topic |
| ---- | ----- |
| `guide/indexes-and-unique-constraints.md` | `#[index]`, `#[unique]`, composite indexes |
| `guide/field-options.md` | All attributes, defaults, nullability, attribute reference |
| `guide/vec-scalar-fields.md` | `Vec<T>` for scalars — array on Postgres, JSON elsewhere, list on DynamoDB |

### Relationships

| Path | Topic |
| ---- | ----- |
| `guide/relationships.md` | Overview: relation kinds, ownership, key direction |
| `guide/belongs-to.md` | `#[belongs_to(key=…, references=…)]`, FK on the child side |
| `guide/has-many.md` | `#[has_many]`, `model.relation().exec()`, filtering relations |
| `guide/has-one.md` | `#[has_one]`, when to use vs `BelongsTo` |
| `guide/preloading-associations.md` | Eager loading / `include`, avoiding N+1 |

### Advanced queries

| Path | Topic |
| ---- | ----- |
| `guide/filtering-with-expressions.md` | `eq`, `gt`, `in`, boolean combinators, expression DSL |
| `guide/sorting-limits-and-pagination.md` | `order_by`, `limit`, cursor-style pagination |

### Advanced features

| Path | Topic |
| ---- | ----- |
| `guide/embedded-types.md` | `#[derive(toasty::Embed)]`, flattening into parent columns |
| `guide/deferred-fields.md` | Lazy column loading for large fields |
| `guide/batch-operations.md` | Bulk inserts, batched lookups |
| `guide/transactions.md` | `db.transaction()`, isolation, driver differences |
| `guide/concurrency-control.md` | Optimistic concurrency, version columns |

### Database admin

| Path | Topic |
| ---- | ----- |
| `guide/database-setup.md` | Connecting `Db` to a backend, connection strings |
| `guide/schema-management.md` | Creating tables, migrations, schema diffs |

### Database backends

| Path | Topic |
| ---- | ----- |
| `guide/postgresql.md` | PostgreSQL driver setup, type mappings, quirks |
| `guide/mysql.md` | MySQL driver setup, type mappings, quirks |
| `guide/sqlite.md` | SQLite driver setup, type mappings, quirks |
| `guide/dynamodb.md` | DynamoDB driver setup, primary key model, query vs scan |

## Developer docs (`dev/`)

### Architecture

| Path | Topic |
| ---- | ----- |
| `dev/README.md` | Contributor onboarding, where things live |
| `dev/architecture/README.md` | High-level architecture overview, crate map |
| `dev/architecture/query-engine.md` | Full pipeline: AST → simplify → lower → plan → exec |
| `dev/architecture/type-system.md` | Type system design, app/db schema mapping |

### Design proposals

| Path | Topic |
| ---- | ----- |
| `dev/design/README.md` | Index of active design proposals |
| `dev/design/_template.md` | Template for new proposals |
| `dev/design/column-projection.md` | Projecting a subset of columns |
| `dev/design/ddb-scan.md` | When DynamoDB scans are acceptable, opt-in design |
| `dev/design/deferred-fields.md` | Implementation plan for deferred fields |
| `dev/design/document-fields.md` | JSON / document column support |
| `dev/design/enums-and-embedded-structs.md` | Enum and embed lowering rules |
| `dev/design/field-version.md` | Versioning model fields for migration |
| `dev/design/query-macro.md` | `toasty::query!` macro design |
| `dev/design/static-assertions-create-macro.md` | Compile-time checks in `create!` |

### Roadmap

| Path | Topic |
| ---- | ----- |
| `dev/roadmap.md` | Prioritized planned features (composite keys, FK, migrations, JSON) |
