//! Run the MCP server over streamable HTTP.
//!
//! ```bash
//! cargo run -p mcp-server --bin mcp-server-http
//! # MCP endpoint: http://127.0.0.1:8000/mcp
//! ```
//!
//! Override the bind address with the `MCP_BIND_ADDRESS` environment variable.

use anyhow::Result;
use mcp_server::Server;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use tracing_subscriber::EnvFilter;

const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let bind_address =
        std::env::var("MCP_BIND_ADDRESS").unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string());

    let cancellation = tokio_util::sync::CancellationToken::new();

    let service = StreamableHttpService::new(
        || Ok(Server::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_cancellation_token(cancellation.child_token()),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!(%bind_address, "MCP server listening at /mcp");

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let _ = tokio::signal::ctrl_c().await;
            cancellation.cancel();
        })
        .await?;

    Ok(())
}
