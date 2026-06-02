//! Relationship coverage: `#[has_many]`, `#[belongs_to]`, `#[has_one]`,
//! plus relation `.remove()` and nested `create!`.
//!
//! Anchors `references/guide/relationships.md`, `belongs-to.md`,
//! `has-many.md`, and `has-one.md`.

mod common;

use toasty_app::models::{Profile, Todo, User};

#[tokio::test]
async fn has_many_traversal_returns_all_children() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
        todos: [
            { title: "first", priority: 1 },
            { title: "second", priority: 2 },
            { title: "third", priority: 3 },
        ],
    })
    .exec(&mut db)
    .await?;

    let todos = user.todos().exec(&mut db).await?;
    assert_eq!(todos.len(), 3);
    Ok(())
}

#[tokio::test]
async fn belongs_to_backref_resolves_to_parent() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Grace",
        email: "grace@example.com",
        tags: Vec::<String>::new(),
        todos: [{ title: "one", priority: 1 }],
    })
    .exec(&mut db)
    .await?;

    let todos = user.todos().exec(&mut db).await?;
    let todo = &todos[0];
    let parent = todo.user().exec(&mut db).await?;
    assert_eq!(parent.id, user.id);
    Ok(())
}

#[tokio::test]
async fn has_many_remove_unlinks_a_child() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
        todos: [
            { title: "keep", priority: 1 },
            { title: "drop", priority: 2 },
        ],
    })
    .exec(&mut db)
    .await?;

    let todos = user.todos().exec(&mut db).await?;
    assert_eq!(todos.len(), 2);

    let doomed = todos.iter().find(|t| t.title == "drop").unwrap();
    user.todos().remove(&mut db, doomed).await?;

    let remaining = user.todos().exec(&mut db).await?;
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].title, "keep");
    Ok(())
}

#[tokio::test]
async fn has_one_returns_none_when_unset() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Lonely",
        email: "lonely@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    let profile = user.profile().exec(&mut db).await?;
    assert!(profile.is_none());
    Ok(())
}

#[tokio::test]
async fn has_one_returns_the_linked_profile() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Linked",
        email: "linked@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    toasty::create!(Profile {
        user_id: Some(user.id),
        bio: "hello",
    })
    .exec(&mut db)
    .await?;

    let profile = user
        .profile()
        .exec(&mut db)
        .await?
        .expect("profile was just linked");
    assert_eq!(profile.bio, "hello");
    Ok(())
}

#[tokio::test]
async fn todo_can_be_attached_to_an_existing_user_via_user_id() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(&mut db)
    .await?;

    let todo = toasty::create!(Todo {
        user_id: user.id,
        title: "attached",
        priority: 1,
    })
    .exec(&mut db)
    .await?;

    assert_eq!(todo.user_id, user.id);
    let todos = user.todos().exec(&mut db).await?;
    assert_eq!(todos.len(), 1);
    Ok(())
}
