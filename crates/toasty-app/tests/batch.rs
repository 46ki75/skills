//! Batch-create coverage: `create!(User::[ ... ])` and the resulting
//! multi-row insert.
//!
//! Anchors `references/guide/batch-operations.md`. The model used here has
//! no embedded fields because `toasty-sql` does not yet implement the
//! lowering needed to combine batch inserts with embeds (see the note in
//! `src/demo.rs`).

mod common;

use toasty_app::models::User;

#[tokio::test]
async fn batch_create_inserts_every_row() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    toasty::create!(User::[
        { name: "Alice", email: "alice@example.com", tags: ["a"] },
        { name: "Bob",   email: "bob@example.com",   tags: ["b"] },
        { name: "Carol", email: "carol@example.com", tags: Vec::<String>::new() },
    ])
    .exec(&mut db)
    .await?;

    for email in ["alice@example.com", "bob@example.com", "carol@example.com"] {
        let user = User::get_by_email(&mut db, email).await?;
        assert_eq!(user.email, email);
    }
    Ok(())
}
