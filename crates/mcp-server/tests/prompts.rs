//! Integration test for the prompt handlers in the `mcp-server` skeleton.
//!
//! Wires a [`Server`] to an in-memory `tokio::io::duplex` transport and a
//! minimal `ClientHandler`, then exercises `prompts/get` against the `echo`
//! prompt to cover the single-argument case end-to-end.
//!
//! Run with: `cargo test -p mcp-server --test prompts`.

use mcp_server::Server;
use rmcp::{
    ClientHandler, ServiceExt,
    model::{
        ClientRequest, GetPromptRequestParams, PromptMessageContent, PromptMessageRole, Request,
        ServerResult,
    },
};

#[derive(Default, Clone)]
struct TestClient;

impl ClientHandler for TestClient {}

#[tokio::test]
async fn get_echo_prompt_returns_the_supplied_message() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = TestClient.serve(client_transport).await?;

    let mut arguments = serde_json::Map::new();
    arguments.insert("message".into(), serde_json::Value::String("hi".into()));

    let response = client_service
        .send_request(ClientRequest::GetPromptRequest(Request::new(
            GetPromptRequestParams::new("echo").with_arguments(arguments),
        )))
        .await?;
    let ServerResult::GetPromptResult(result) = response else {
        panic!("expected GetPromptResult, got {response:?}");
    };
    assert_eq!(result.messages.len(), 1);
    let message = &result.messages[0];
    assert_eq!(message.role, PromptMessageRole::User);
    let PromptMessageContent::Text { text } = &message.content else {
        panic!("expected text content, got {:?}", message.content);
    };
    assert_eq!(text, "hi");

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}
