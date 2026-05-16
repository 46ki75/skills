//! Shared helpers for the `toasty-app` integration tests.
//!
//! Every test calls [`fresh_db`] to get a clean SQLite in-memory database
//! with the model set already pushed. Equivalent in spirit to the
//! `tokio::io::duplex` harness used by `crates/mcp-server/tests/`.

#![allow(dead_code)]

/// Open a fresh in-memory SQLite database with the schema applied.
pub async fn fresh_db() -> toasty::Db {
    toasty_app::db::connect()
        .await
        .expect("failed to open sqlite::memory: with model set")
}
