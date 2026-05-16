//! Filter-expression coverage: `eq`, `gt`, `or` combinators via the
//! `Model::filter()` entry point.
//!
//! Anchors `references/guide/filtering-with-expressions.md`.

mod common;

use toasty_app::models::{Todo, User};

async fn seed_user_with_todos(db: &mut toasty::Db) -> toasty::Result<User> {
    toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
        todos: [
            { title: "alpha", priority: 1 },
            { title: "beta", priority: 5 },
            { title: "gamma", priority: 9 },
        ],
    })
    .exec(db)
    .await
}

#[tokio::test]
async fn filter_eq_returns_matching_records() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;
    seed_user_with_todos(&mut db).await?;

    let results: Vec<Todo> = Todo::filter(Todo::fields().title().eq("beta"))
        .exec(&mut db)
        .await?;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "beta");
    Ok(())
}

#[tokio::test]
async fn filter_gt_returns_records_above_threshold() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;
    seed_user_with_todos(&mut db).await?;

    let mut results: Vec<Todo> = Todo::filter(Todo::fields().priority().gt(2_i64))
        .exec(&mut db)
        .await?;
    results.sort_by_key(|t| t.priority);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].priority, 5);
    assert_eq!(results[1].priority, 9);
    Ok(())
}

#[tokio::test]
async fn filter_or_combinator() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;
    seed_user_with_todos(&mut db).await?;

    let mut results: Vec<Todo> = Todo::filter(
        Todo::fields()
            .title()
            .eq("alpha")
            .or(Todo::fields().title().eq("gamma")),
    )
    .exec(&mut db)
    .await?;
    results.sort_by(|a, b| a.title.cmp(&b.title));
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].title, "alpha");
    assert_eq!(results[1].title, "gamma");
    Ok(())
}
