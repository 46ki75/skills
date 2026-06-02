# toasty-app

Generic [Toasty](https://github.com/tokio-rs/toasty) ORM skeleton with one
tested example per primitive. The companion crate for the `rust-toasty`
skill — every reference page under `skills/rust-toasty/references/guide/`
points at a file in this crate so the documented claims stay anchored to
compiling, tested code.

The crate ships four models — `User`, `Todo`, `Profile`, `Article` — plus
an embedded `Address` struct. Together they exercise the primitives the
skill talks about most:

| Primitive                        | Example                                    | File              |
| -------------------------------- | ------------------------------------------ | ----------------- |
| `#[key] #[auto]`                 | `User::id: uuid::Uuid`                     | `src/models.rs`   |
| `#[unique]`                      | `User::email`                              | `src/models.rs`   |
| `Option<T>` field                | `User::motto`                              | `src/models.rs`   |
| `Vec<scalar>` field              | `User::tags: Vec<String>`                  | `src/models.rs`   |
| `#[has_many]` (lazy)             | `User::todos: Deferred<Vec<Todo>>`         | `src/models.rs`   |
| `#[has_one]` (optional, lazy)    | `User::profile: Deferred<Option<Profile>>` | `src/models.rs`   |
| `#[belongs_to(key, references)]` | `Todo::user: Deferred<User>`               | `src/models.rs`   |
| `#[index]`                       | `Todo::user_id`                            | `src/models.rs`   |
| `#[derive(toasty::Embed)]`       | `Address`                                  | `src/embedded.rs` |

It also ships a binary that walks every primitive end-to-end:

| Binary             | Default connection      |
| ------------------ | ----------------------- |
| `toasty-app-demo`  | `sqlite::memory:`       |

Override with `TOASTY_CONNECTION_URL=...` to point at a real database.

## Run

```bash
# Run the demo against an ephemeral in-memory SQLite database.
cargo run -p toasty-app --bin toasty-app-demo

# Run against a file-backed SQLite database.
TOASTY_CONNECTION_URL="sqlite:///tmp/toasty.db" \
    cargo run -p toasty-app --bin toasty-app-demo
```

## Test

```bash
cargo test -p toasty-app
```

All 28 integration tests run against `sqlite::memory:` — no external
services needed.

## Test → reference map

Each test file is cited from the matching reference page in the skill, so
"can I actually do X?" always has a working answer in source:

| Test file                | Skill reference                                                                |
| ------------------------ | ------------------------------------------------------------------------------ |
| `tests/crud.rs`          | `creating-records`, `querying-records`, `updating-records`, `deleting-records` |
| `tests/relationships.rs` | `relationships`, `belongs-to`, `has-many`, `has-one`                           |
| `tests/filtering.rs`     | `filtering-with-expressions`                                                   |
| `tests/preloading.rs`    | `preloading-associations`                                                      |
| `tests/batch.rs`         | `batch-operations`                                                             |
| `tests/embedded.rs`      | `embedded-types`                                                               |
| `tests/vec_fields.rs`    | `vec-scalar-fields`                                                            |
| `tests/transactions.rs`  | `transactions`                                                                 |
| `tests/pagination.rs`    | `sorting-limits-and-pagination`                                                |

## Driver features

Defaults to SQLite. To run against another SQL backend, enable the matching
feature and set `TOASTY_CONNECTION_URL` to a driver-specific URL:

```bash
cargo test -p toasty-app --no-default-features --features postgresql
cargo test -p toasty-app --no-default-features --features mysql
```

DynamoDB is intentionally not wired up here. Toasty's DynamoDB tests
require `--test-threads=1` and a running local DynamoDB instance; see the
upstream `submodules/toasty/CLAUDE.md` for the canonical command.

## Known limitations

- `toasty-sql` does not yet implement batch-insert lowering for
  embedded fields, so `create!(User::[...])` cannot be combined with a
  model that has a `#[derive(toasty::Embed)]` field. The batch tests in
  this crate use the embed-less `User` model; the embed test uses
  `Article` and creates one row at a time.

## Extending

Add a new model? Add the struct to `src/models.rs`, register it in
`src/db.rs::connect()`, and add a test file under `tests/`. Then cite the
new test file from the matching reference page in
`skills/rust-toasty/references/guide/` so the skill stays in sync.

For end-to-end runnable examples beyond what this skeleton covers
(composite keys, todo-with-cli, OAuth-style flows), browse
`submodules/toasty/examples/`.
