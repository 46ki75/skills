//! Eager-loading coverage: `Model::filter_by_id(...).include(...).get(...)`.
//!
//! Anchors `references/guide/preloading-associations.md`. Without `.include`,
//! traversing `user.todos()` issues a second query; the test below confirms
//! that pulling the same relation through `.include()` returns the children
//! preloaded into the returned record.

mod common;

use toasty_app::models::User;

#[tokio::test]
async fn include_eagerly_loads_has_many() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
        todos: [
            { title: "first", priority: 1 },
            { title: "second", priority: 2 },
        ],
    })
    .exec(&mut db)
    .await?;

    let preloaded = User::filter_by_id(user.id)
        .include(User::fields().todos())
        .get(&mut db)
        .await?;

    // `.include()` populates the relation on the parent record. The
    // collection lives behind the `Deferred` field's `.get()` after preload
    // so iterating it does not issue another query.
    let todos: Vec<_> = preloaded.todos.get().iter().collect();
    assert_eq!(todos.len(), 2);
    Ok(())
}
