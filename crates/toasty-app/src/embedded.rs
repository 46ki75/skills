//! Embedded struct used inside [`crate::models::User`].
//!
//! An embedded struct is **not** a separate table — its fields are flattened
//! into the parent model's table as `<parent_field>_<embed_field>` columns.
//! `Address` here becomes `address_street`, `address_city`, `address_country`
//! columns on the `users` table.

// `#[derive(toasty::Embed)]` generates a `fields()` accessor and other
// helpers we cannot doc-comment. Suppress `missing_docs` at module scope.
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Postal address embedded into [`crate::models::User`].
///
/// `Clone`, `Debug`, `PartialEq` make the embed convenient to assert on in
/// tests. `Serialize` / `Deserialize` aren't required by Toasty but are
/// useful when ferrying the value through JSON-shaped tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, toasty::Embed)]
pub struct Address {
    /// Street line.
    pub street: String,
    /// City name.
    pub city: String,
    /// Country code (ISO-3166 alpha-2 in tests, but Toasty doesn't enforce
    /// the shape).
    pub country: String,
}
