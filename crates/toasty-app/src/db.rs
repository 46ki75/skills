//! [`toasty::Db`] connection helper.
//!
//! Centralizing the model set + schema push in one place keeps every test
//! and the binary in lockstep: when a new model is added, only this
//! function needs updating.

use crate::{embedded::Address, models::*};

/// Default connection string used when `TOASTY_CONNECTION_URL` is unset.
/// Picks a fresh in-memory SQLite database per call.
pub const DEFAULT_CONNECTION_URL: &str = "sqlite::memory:";

/// Open a [`toasty::Db`] and run [`Db::push_schema`](toasty::Db::push_schema)
/// so the tables exist before the caller does any work.
///
/// Reads `TOASTY_CONNECTION_URL` and falls back to [`DEFAULT_CONNECTION_URL`].
/// `sqlite::memory:` gives every caller a clean database, which is exactly
/// what the integration tests want.
pub async fn connect() -> toasty::Result<toasty::Db> {
    let url =
        std::env::var("TOASTY_CONNECTION_URL").unwrap_or_else(|_| DEFAULT_CONNECTION_URL.into());

    let db = toasty::Db::builder()
        .models(toasty::models!(User, Todo, Profile, Article, Address))
        .connect(&url)
        .await?;

    db.push_schema().await?;
    Ok(db)
}
