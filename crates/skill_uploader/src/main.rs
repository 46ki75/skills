//! CLI orchestrator that validates, archives, and publishes the skills under
//! `skills/`.
//!
//! Subcommands:
//!
//! - `check` — parse and validate every skill; exit non-zero on any failure.
//! - `build` — write a `<name>-v<version>.zip` per valid skill into `dist/`.
//! - `upload` — build, then push any artifact whose `<name>-v<version>` release
//!   tag does not yet exist on GitHub.
//!
//! Logging is controlled via `RUST_LOG` (defaults to `info`).

mod cli;
mod github;
mod pipeline;

use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = cli::Cli::parse();

    match cli::run(args).await {
        Ok(code) => code,
        Err(e) => {
            tracing::error!("{e:#}");
            ExitCode::from(2)
        }
    }
}
