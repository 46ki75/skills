//! Run the MCP server over stdio.
//!
//! ```bash
//! cargo run -p mcp-server --bin mcp-server-stdio
//! ```

use anyhow::Result;
use mcp_server::Server;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("starting MCP server over stdio");

    let service = Server::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!(error = ?e, "failed to start MCP server");
    })?;

    service.waiting().await?;
    Ok(())
}
