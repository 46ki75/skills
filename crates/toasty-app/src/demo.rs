//! End-to-end script that walks every primitive once.
//!
//! Factored out of `main` so the binary (`src/bin/demo.rs`) and a smoke test
//! can both invoke the same code path. Mirrors the flow of the upstream
//! `examples/hello-toasty/src/main.rs` example so anyone familiar with the
//! Toasty README finds the same shape here.

use crate::{embedded::Address, models::*};

/// Run the full demo against an already-opened [`toasty::Db`]. Prints
/// progress to stdout so a human running the binary can see each step.
pub async fn run(db: &mut toasty::Db) -> toasty::Result<()> {
    println!("==> create User (with tags + nested todos)");
    let ada = toasty::create!(User {
        name: "Ada Lovelace",
        email: "ada@example.com",
        tags: ["math", "engines"],
        todos: [
            { title: "Write Toasty docs", priority: 1 },
            { title: "Ship release", priority: 2 },
        ],
    })
    .exec(db)
    .await?;
    println!("    id = {}", ada.id);

    println!("==> get_by_id, get_by_email");
    let by_id = User::get_by_id(db, &ada.id).await?;
    assert_eq!(by_id.email, ada.email);
    let by_email = User::get_by_email(db, &ada.email).await?;
    assert_eq!(by_email.id, ada.id);

    println!("==> update name");
    let mut ada = User::get_by_id(db, &ada.id).await?;
    ada.update().name("Augusta Ada King").exec(db).await?;
    assert_eq!(ada.name, "Augusta Ada King");

    println!("==> traverse has_many relation");
    let todos = ada.todos().exec(db).await?;
    assert_eq!(todos.len(), 2);
    for todo in &todos {
        println!("    todo: {} (priority {})", todo.title, todo.priority);
    }

    println!("==> batch create more users");
    toasty::create!(User::[
        { name: "Grace Hopper", email: "grace@example.com", tags: ["compilers"] },
        { name: "Linus Torvalds", email: "linus@example.com", tags: ["kernel", "git"] },
    ])
    .exec(db)
    .await?;

    println!("==> create Article with embedded Address");
    let article = toasty::create!(Article {
        title: "On Engines",
        address: Address {
            street: "1 Analytical Way".into(),
            city: "London".into(),
            country: "GB".into(),
        },
    })
    .exec(db)
    .await?;
    let reloaded = Article::get_by_id(db, &article.id).await?;
    assert_eq!(reloaded.address.city, "London");

    println!("==> delete the first user");
    let ada_id = ada.id;
    let ada = User::get_by_id(db, &ada_id).await?;
    ada.delete().exec(db).await?;
    assert!(User::get_by_id(db, &ada_id).await.is_err());

    println!(">>> demo complete <<<");
    Ok(())
}
