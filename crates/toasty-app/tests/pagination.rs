//! Sorting and pagination coverage: `order_by`, `paginate`, `.after()`.
//!
//! Anchors `references/guide/sorting-limits-and-pagination.md`. Runs against
//! SQLite — `order_by` is only valid for SQL drivers and indexed scans on
//! DynamoDB.

mod common;

use toasty::stmt::Page;
use toasty_app::models::{Todo, User};

async fn seed(db: &mut toasty::Db) -> toasty::Result<()> {
    let user = toasty::create!(User {
        name: "Ada",
        email: "ada@example.com",
        tags: Vec::<String>::new(),
    })
    .exec(db)
    .await?;

    for i in 0..20i64 {
        toasty::create!(Todo {
            user_id: user.id,
            title: format!("todo-{i:02}"),
            priority: i,
        })
        .exec(db)
        .await?;
    }
    Ok(())
}

#[tokio::test]
async fn order_by_asc_returns_records_in_priority_order() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;
    seed(&mut db).await?;

    let items: Vec<Todo> = Todo::all()
        .order_by(Todo::fields().priority().asc())
        .exec(&mut db)
        .await?;

    assert_eq!(items.len(), 20);
    for w in items.windows(2) {
        assert!(w[0].priority < w[1].priority);
    }
    Ok(())
}

#[tokio::test]
async fn order_by_desc_returns_records_in_reverse_priority_order() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;
    seed(&mut db).await?;

    let items: Vec<Todo> = Todo::all()
        .order_by(Todo::fields().priority().desc())
        .exec(&mut db)
        .await?;

    assert_eq!(items.len(), 20);
    for w in items.windows(2) {
        assert!(w[0].priority > w[1].priority);
    }
    Ok(())
}

#[tokio::test]
async fn paginate_returns_one_page_at_a_time() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;
    seed(&mut db).await?;

    let first: Page<Todo> = Todo::all()
        .order_by(Todo::fields().priority().asc())
        .paginate(5)
        .exec(&mut db)
        .await?;
    assert_eq!(first.len(), 5);
    for (i, todo) in first.iter().enumerate() {
        assert_eq!(todo.priority, i as i64);
    }
    Ok(())
}
