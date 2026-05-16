//! Regression test for the `greet_user` tool's elicitation handling.
//!
//! `Peer::elicit` in rmcp 1.7 returns
//! `Err(ElicitationError::UserDeclined)` / `UserCancelled` (see
//! `submodules/mcp-rust-sdk/crates/rmcp/src/service/server.rs:768-769`)
//! when the user explicitly declines or cancels — those are user actions,
//! not service failures, and must not surface to the client as
//! `internal_error`. `src/tools.rs::greet_user` pattern-matches on those
//! variants and returns a graceful `CallToolResult::success`; this test
//! locks that contract in.
//!
//! The default `ClientHandler::create_elicitation` implementation already
//! returns `ElicitationAction::Decline`, so this test only needs to
//! advertise the form-elicitation capability — the decline is automatic.
//!
//! Run with: `cargo test -p mcp-server --test elicitation`.

use mcp_server::Server;
use rmcp::{
    ClientHandler, ServiceExt,
    model::{
        CallToolRequestParams, ClientCapabilities, ClientInfo, ClientRequest, Content,
        Implementation, RawContent, Request, ServerResult,
    },
};

#[derive(Default, Clone)]
struct DecliningClient;

impl ClientHandler for DecliningClient {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::new(
            ClientCapabilities::builder().enable_elicitation().build(),
            Implementation::from_build_env(),
        )
    }
}

#[tokio::test]
async fn declined_elicitation_returns_a_graceful_success() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = DecliningClient.serve(client_transport).await?;

    let params = CallToolRequestParams::new("greet_user");
    let response = client_service
        .send_request(ClientRequest::CallToolRequest(Request::new(params)))
        .await?;

    let ServerResult::CallToolResult(result) = response else {
        panic!("expected CallToolResult, got {response:?}");
    };
    assert!(
        !result.is_error.unwrap_or(false),
        "greet_user must treat a user decline as a graceful outcome, \
         not is_error=true. Content: {:?}",
        result.content,
    );
    let text =
        first_text(&result.content).expect("decline should be surfaced as a text content block");
    assert!(
        text.contains("declined"),
        "expected the success message to mention 'declined', got: {text}",
    );

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
