//! Integration tests for the tool router in the `mcp-server` skeleton.
//!
//! Wires a [`Server`] to an in-memory `tokio::io::duplex` transport and a
//! minimal `ClientHandler`, then exercises:
//!
//! - `tools/list` — locks in the set of advertised tools.
//! - `tools/call ping` — covers a no-arg tool returning a text content
//!   block.
//! - `tools/call slow_count` with no `task` metadata — covers a typed
//!   `Parameters<T>` tool invoked on the synchronous path (a
//!   `task_support = "optional"` tool must still respond directly when
//!   the client does not opt in to tasks).
//!
//! Sampling, elicitation, and roots tools require client capabilities
//! and are covered in `tests/elicitation.rs` (and would need dedicated
//! mock handlers for the others) — not exercised here.
//!
//! Run with: `cargo test -p mcp-server --test tools`.

use mcp_server::Server;
use rmcp::{
    ClientHandler, ServiceExt,
    model::{
        CallToolRequestParams, ClientRequest, Content, ListToolsRequest, RawContent, Request,
        ServerResult,
    },
};

#[derive(Default, Clone)]
struct TestClient;

impl ClientHandler for TestClient {}

#[tokio::test]
async fn list_tools_returns_the_advertised_set() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = TestClient.serve(client_transport).await?;

    let response = client_service
        .send_request(ClientRequest::ListToolsRequest(ListToolsRequest::default()))
        .await?;
    let ServerResult::ListToolsResult(listed) = response else {
        panic!("expected ListToolsResult, got {response:?}");
    };

    let mut names: Vec<&str> = listed.tools.iter().map(|t| t.name.as_ref()).collect();
    names.sort();
    assert_eq!(
        names,
        vec![
            "ask_llm",
            "greet_user",
            "list_workspace_roots",
            "ping",
            "slow_count",
        ],
    );

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}

#[tokio::test]
async fn call_ping_returns_pong() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = TestClient.serve(client_transport).await?;

    let response = client_service
        .send_request(ClientRequest::CallToolRequest(Request::new(
            CallToolRequestParams::new("ping"),
        )))
        .await?;
    let ServerResult::CallToolResult(result) = response else {
        panic!("expected CallToolResult, got {response:?}");
    };
    assert!(!result.is_error.unwrap_or(false));
    let text = first_text(&result.content).expect("ping should return a text content block");
    assert_eq!(text, "pong");

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}

#[tokio::test]
async fn call_slow_count_synchronously_returns_the_target() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = TestClient.serve(client_transport).await?;

    // No `task` metadata — exercises the synchronous path for a
    // `task_support = "optional"` tool.
    let mut arguments = serde_json::Map::new();
    arguments.insert("target".into(), serde_json::Value::from(1u8));

    let response = client_service
        .send_request(ClientRequest::CallToolRequest(Request::new(
            CallToolRequestParams::new("slow_count").with_arguments(arguments),
        )))
        .await?;
    let ServerResult::CallToolResult(result) = response else {
        panic!("expected CallToolResult, got {response:?}");
    };
    assert!(!result.is_error.unwrap_or(false));
    let text = first_text(&result.content).expect("slow_count should return a text content block");
    assert_eq!(text, "1");

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}

fn first_text(content: &[Content]) -> Option<&str> {
    content.iter().find_map(|c| match &c.raw {
        RawContent::Text(t) => Some(t.text.as_str()),
        _ => None,
    })
}
