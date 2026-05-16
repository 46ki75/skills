//! Embedded-struct coverage: `#[derive(toasty::Embed)]` flattening and
//! round-trip through SQLite.
//!
//! Anchors `references/guide/embedded-types.md`.

mod common;

use toasty_app::{embedded::Address, models::Article};

fn sample_address() -> Address {
    Address {
        street: "1 Analytical Way".into(),
        city: "London".into(),
        country: "GB".into(),
    }
}

#[tokio::test]
async fn create_and_read_back_an_embedded_struct() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let article = toasty::create!(Article {
        title: "On Engines",
        address: sample_address(),
    })
    .exec(&mut db)
    .await?;

    let loaded = Article::get_by_id(&mut db, &article.id).await?;
    assert_eq!(loaded.address, sample_address());
    Ok(())
}

#[tokio::test]
async fn update_replaces_the_entire_embed() -> toasty::Result<()> {
    let mut db = common::fresh_db().await;

    let mut article = toasty::create!(Article {
        title: "Before",
        address: sample_address(),
    })
    .exec(&mut db)
    .await?;

    let new_address = Address {
        street: "456 Oak Ave".into(),
        city: "Shelbyville".into(),
        country: "US".into(),
    };
    article
        .update()
        .address(new_address.clone())
        .exec(&mut db)
        .await?;

    let loaded = Article::get_by_id(&mut db, &article.id).await?;
    assert_eq!(loaded.address, new_address);
    Ok(())
}
