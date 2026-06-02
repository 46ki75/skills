//! Generic Toasty ORM skeleton.
//!
//! The crate exists to anchor the `rust-toasty` skill's claims to compiling,
//! tested code. The model set in [`models`] exercises every primitive the
//! skill talks about most: keys + auto-generation, `#[unique]`, `#[index]`,
//! `#[belongs_to]` / `#[has_many]` / `#[has_one]` relations (lazy via
//! `Deferred<T>`), embedded types, and `Vec<scalar>`
//! fields. The companion [`tests`] directory hosts one integration test per
//! topic, all driven by an in-memory SQLite [`Db`](toasty::Db) from
//! [`db::connect`].
//!
//! Modules:
//!
//! - [`models`] — `User`, `Todo`, `Profile`, all in one model set.
//! - [`embedded`] — the `Address` embed used inside `User`.
//! - [`db`] — `connect()` helper used by binary and tests.
//! - [`demo`] — end-to-end script: create, query, update, delete, batch,
//!   nested create, traversal. Mirrors `examples/hello-toasty` from the
//!   Toasty submodule but factored so the binary and a smoke-test can both
//!   call it.

pub mod db;
pub mod demo;
pub mod embedded;
pub mod models;
