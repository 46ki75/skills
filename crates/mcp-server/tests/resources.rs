//! Integration tests for the resource handlers in the `mcp-server` skeleton.
//!
//! Wires a [`Server`] to an in-memory `tokio::io::duplex` transport and a
//! minimal `ClientHandler`, then exercises `resources/list` and
//! `resources/read` end-to-end. Also covers the empty-payload rejection on
//! `echo://`.
//!
//! Run with: `cargo test -p mcp-server --test resources`.

use mcp_server::Server;
use rmcp::{
    ClientHandler, ServiceExt,
    model::{
        ClientRequest, ListResourcesRequest, ReadResourceRequestParams, Request, ResourceContents,
        ServerResult,
    },
};

#[derive(Default, Clone)]
struct TestClient;

impl ClientHandler for TestClient {}

#[tokio::test]
async fn list_resources_returns_the_static_example() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = TestClient.serve(client_transport).await?;

    let response = client_service
        .send_request(ClientRequest::ListResourcesRequest(
            ListResourcesRequest::default(),
        ))
        .await?;
    let ServerResult::ListResourcesResult(listed) = response else {
        panic!("expected ListResourcesResult, got {response:?}");
    };
    assert_eq!(listed.resources.len(), 1);
    assert_eq!(listed.resources[0].raw.uri, "mem://example");

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}

#[tokio::test]
async fn read_resource_serves_static_and_templated_uris() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = Server::new().serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = TestClient.serve(client_transport).await?;

    // Static `mem://example` returns the canned body.
    let response = client_service
        .send_request(ClientRequest::ReadResourceRequest(Request::new(
            ReadResourceRequestParams::new("mem://example"),
        )))
        .await?;
    let ServerResult::ReadResourceResult(result) = response else {
        panic!("expected ReadResourceResult, got {response:?}");
    };
    let ResourceContents::TextResourceContents { text, .. } = &result.contents[0] else {
        panic!("expected text contents, got {:?}", result.contents[0]);
    };
    assert!(text.contains("Example in-memory resource"));

    // `echo://hello` echoes the suffix verbatim.
    let response = client_service
        .send_request(ClientRequest::ReadResourceRequest(Request::new(
            ReadResourceRequestParams::new("echo://hello"),
        )))
        .await?;
    let ServerResult::ReadResourceResult(result) = response else {
        panic!("expected ReadResourceResult, got {response:?}");
    };
    let ResourceContents::TextResourceContents { text, .. } = &result.contents[0] else {
        panic!("expected text contents, got {:?}", result.contents[0]);
    };
    assert_eq!(text, "hello");

    // Empty `echo://` payload must be rejected with `invalid_params`.
    let err = client_service
        .send_request(ClientRequest::ReadResourceRequest(Request::new(
            ReadResourceRequestParams::new("echo://"),
        )))
        .await
        .expect_err("empty echo:// URI should be rejected");
    let rendered = err.to_string();
    assert!(
        rendered.contains("non-empty"),
        "expected invalid_params error mentioning 'non-empty', got: {rendered}",
    );

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}
