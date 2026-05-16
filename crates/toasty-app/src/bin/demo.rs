//! Runnable Toasty demo. Connects to SQLite in-memory (or
//! `TOASTY_CONNECTION_URL` if set) and runs the same walk that
//! [`toasty_app::demo::run`] exercises.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = toasty_app::db::connect().await?;
    toasty_app::demo::run(&mut db).await?;
    Ok(())
}
