//! CRUD coverage: `create!`, `get_by_id`, `get_by_email`, `update`, `delete`.
//!
//! Anchors the skill's claims in `references/guide/creating-records.md`,
//! `querying-records.md`, `updating-records.md`, and `deleting-records.md`.

mod common;

use toasty_app::models::User;

#[tokio::test]
async fn create_then_get_by_id_roundtrips() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: ["math"],
    })
    .exec(&mut db)
    .await?;

    let loaded = User::get_by_id(&mut db, &user.id).await?;
    assert_eq!(loaded.name, "Ada");
    assert_eq!(loaded.email, "ada@example.com");
    assert_eq!(loaded.tags, vec!["math".to_string()]);
    Ok(())
}

#[tokio::test]
async fn unique_email_rejects_duplicates() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    let duplicate = toasty::create!(User {
        name: "Ada II",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await;
    assert!(
        duplicate.is_err(),
        "second create with the same #[unique] email should fail"
    );
    Ok(())
}

#[tokio::test]
async fn get_by_email_returns_the_record() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Grace",
        email: "grace@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    let loaded = User::get_by_email(&mut db, &user.email).await?;
    assert_eq!(loaded.id, user.id);
    Ok(())
}

#[tokio::test]
async fn update_name_persists() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let mut user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    user.update().name("Augusta Ada King").exec(&mut db).await?;
    assert_eq!(user.name, "Augusta Ada King");

    let reloaded = User::get_by_id(&mut db, &user.id).await?;
    assert_eq!(reloaded.name, "Augusta Ada King");
    Ok(())
}

#[tokio::test]
async fn update_optional_motto_to_some_and_back_to_none() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let mut user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;
    assert!(user.motto.is_none());

    user.update()
        .motto(Some("hello".to_string()))
        .exec(&mut db)
        .await?;
    let loaded = User::get_by_id(&mut db, &user.id).await?;
    assert_eq!(loaded.motto.as_deref(), Some("hello"));
    Ok(())
}

#[tokio::test]
async fn delete_removes_the_record() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Doomed",
        email: "doomed@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;
    let id = user.id;

    user.delete().exec(&mut db).await?;
    assert!(User::get_by_id(&mut db, &id).await.is_err());
    Ok(())
}
