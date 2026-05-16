//! `Vec<scalar>` field coverage: round-trip through SQLite (which stores
//! the value as JSON-encoded text).
//!
//! Anchors `references/guide/vec-scalar-fields.md`.

mod common;

use toasty_app::models::User;

#[tokio::test]
async fn vec_string_roundtrips() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Tagged",
        email: "tagged@example.com",
        tags: ["rust", "toasty", "orm"],
    })
    .exec(&mut db)
    .await?;

    let loaded = User::get_by_id(&mut db, &user.id).await?;
    assert_eq!(loaded.tags, vec!["rust", "toasty", "orm"]);
    Ok(())
}

#[tokio::test]
async fn empty_vec_persists_as_empty_not_null() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Empty",
        email: "empty@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    let loaded = User::get_by_id(&mut db, &user.id).await?;
    assert!(loaded.tags.is_empty());
    Ok(())
}

#[tokio::test]
async fn update_replaces_the_collection() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let mut user = toasty::create!(User {
        name: "Tagged",
        email: "tagged2@example.com",
        tags: ["old"],
    })
    .exec(&mut db)
    .await?;

    user.update()
        .tags(vec!["new".to_string(), "shiny".to_string()])
        .exec(&mut db)
        .await?;

    let loaded = User::get_by_id(&mut db, &user.id).await?;
    assert_eq!(loaded.tags, vec!["new", "shiny"]);
    Ok(())
}
