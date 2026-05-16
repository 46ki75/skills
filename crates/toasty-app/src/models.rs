//! Toasty model set.
//!
//! Four models drive every test:
//!
//! - [`User`] — keyed by `uuid::Uuid` (`#[key] #[auto]`), with `#[unique]`,
//!   `Option<String>`, a `Vec<String>` scalar collection, a `HasMany<Todo>`,
//!   and a `HasOne<Option<Profile>>`. Covers the keys / unique / vec-fields
//!   / has-many / has-one slices of the skill.
//! - [`Todo`] — the `BelongsTo<User>` partner of `User`, plus a `priority`
//!   field used to demonstrate filtering, ordering, and pagination. Carries
//!   an `#[index]` on the foreign-key column so traversal queries hit the
//!   index.
//! - [`Profile`] — the optional `HasOne` partner of `User`. The
//!   `Option<User>` / `Option<Uuid>` shapes mirror the upstream
//!   `user-has-one-profile` example.
//! - [`Article`] — a stand-alone model that carries the embedded
//!   [`Address`](crate::embedded::Address). Kept separate from `User` so
//!   the batch-create tests can exercise `User::[...]` without tripping
//!   `toasty-sql 0.6`'s yet-to-be-implemented batch lowering for embedded
//!   fields.

// `#[derive(toasty::Model)]` generates a fair amount of public scaffolding
// (associated functions like `get_by_id`, `create`, the `fields()` helper,
// query builder structs, …) that we cannot annotate with doc comments.
// Suppress `missing_docs` on the whole module rather than peppering allow
// attributes on each derive site.
#![allow(missing_docs)]

use crate::embedded::Address;

/// Application user. Owns many [`Todo`]s and optionally has one
/// [`Profile`].
#[derive(Debug, Clone, toasty::Model)]
pub struct User {
    /// Primary key. `#[auto]` lets Toasty generate the UUID on insert.
    #[key]
    #[auto]
    pub id: uuid::Uuid,

    /// Display name. Plain `String`, no uniqueness guarantee.
    pub name: String,

    /// Login email. `#[unique]` puts a unique index on the column, so a
    /// second `create!` with the same value returns an error.
    #[unique]
    pub email: String,

    /// Optional personal motto. `Option<String>` becomes a nullable
    /// column.
    pub motto: Option<String>,

    /// Free-form tags. Plain `Vec<String>` — Toasty picks the right
    /// per-driver representation (text array on Postgres, JSON elsewhere).
    /// No `#[serialize(...)]` attribute needed.
    pub tags: Vec<String>,

    /// One-to-many to [`Todo`]. Foreign key lives on the `Todo` side
    /// (`#[belongs_to]`), not here.
    #[has_many]
    pub todos: toasty::HasMany<Todo>,

    /// One-to-one to [`Profile`]. `Option<Profile>` because a user may
    /// not have a profile yet.
    #[has_one]
    pub profile: toasty::HasOne<Option<Profile>>,
}

/// Todo belonging to one [`User`]. Carries a `priority` used by the
/// filtering / ordering / pagination tests.
#[derive(Debug, Clone, toasty::Model)]
pub struct Todo {
    /// Primary key.
    #[key]
    #[auto]
    pub id: uuid::Uuid,

    /// Foreign-key column referencing [`User::id`]. `#[index]` makes
    /// `user.todos()` traversal hit an index instead of scanning.
    #[index]
    pub user_id: uuid::Uuid,

    /// The owning user. The foreign-key column / referenced column are
    /// declared on the `BelongsTo` side, **not** the `HasMany` side.
    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::BelongsTo<User>,

    /// Short description of the work.
    pub title: String,

    /// Numeric priority. Lower numbers come first when sorting `asc`.
    pub priority: i64,
}

/// Optional one-to-one partner of [`User`]. The
/// `BelongsTo<Option<User>>` shape mirrors the upstream
/// `user-has-one-profile` example — Toasty's one-to-one is implemented as a
/// `HasOne` on the parent and a `BelongsTo` on the child, with both sides
/// nullable when the relationship is optional.
#[derive(Debug, Clone, toasty::Model)]
pub struct Profile {
    /// Primary key.
    #[key]
    #[auto]
    pub id: uuid::Uuid,

    /// Nullable foreign-key column. `#[unique]` enforces the one-to-one
    /// invariant: at most one profile per user.
    #[unique]
    pub user_id: Option<uuid::Uuid>,

    /// The owning user, if any.
    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::BelongsTo<Option<User>>,

    /// Public biography string.
    pub bio: String,
}

/// Stand-alone model that carries the embedded [`Address`]. Used by
/// `tests/embedded.rs` — kept off [`User`] so the batch tests can stay
/// simple.
#[derive(Debug, Clone, toasty::Model)]
pub struct Article {
    /// Primary key.
    #[key]
    #[auto]
    pub id: uuid::Uuid,

    /// Article title.
    pub title: String,

    /// Embedded postal address. Flattened to `address_street`,
    /// `address_city`, `address_country` columns on the `articles` table.
    pub address: Address,
}
