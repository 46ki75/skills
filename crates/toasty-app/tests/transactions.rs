//! Interactive transaction coverage: commit, rollback, drop-without-commit.
//!
//! Anchors `references/guide/transactions.md`. SQLite is the test target,
//! which gives us `requires(sql)` behavior — DynamoDB has different
//! transaction semantics.

mod common;

use toasty_app::models::User;

#[tokio::test]
async fn commit_persists_data() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let mut tx = db.transaction().await?;
    toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut tx)
    .await?;
    tx.commit().await?;

    let all: Vec<User> = User::all().exec(&mut db).await?;
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].name, "Ada");
    Ok(())
}

#[tokio::test]
async fn rollback_discards_data() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let mut tx = db.transaction().await?;
    toasty::create!(User {
        name: "Ghost",
        email: "ghost@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut tx)
    .await?;
    tx.rollback().await?;

    let all: Vec<User> = User::all().exec(&mut db).await?;
    assert!(all.is_empty());
    Ok(())
}

#[tokio::test]
async fn drop_without_finalize_rolls_back() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    {
        let mut tx = db.transaction().await?;
        toasty::create!(User {
            name: "Ghost",
            email: "ghost@example.com",
            tags: Vec::<String>::new(),
        })
        .exec(&mut tx)
        .await?;
        // `tx` dropped here without commit/rollback.
    }

    let all: Vec<User> = User::all().exec(&mut db).await?;
    assert!(all.is_empty());
    Ok(())
}
