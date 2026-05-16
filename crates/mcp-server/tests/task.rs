//! Integration test for the task (SEP-1319) feature of the `mcp-server`
//! skeleton.
//!
//! Wires a `Server` to an in-memory `tokio::io::duplex` transport, has a
//! `ClientHandler` invoke `slow_count` with task metadata, and checks that
//! the server returns a `CreateTaskResult` and lists the task as running.
//!
//! Run with: `cargo test -p mcp-server --test task`.

use std::time::Duration;

use mcp_server::Server;
use rmcp::{
    ClientHandler, ServiceExt,
    model::{
        CallToolRequestParams, ClientRequest, ListTasksRequest, Request, ServerResult, TaskStatus,
    },
};

#[derive(Default, Clone)]
struct TestClient;

impl ClientHandler for TestClient {}

#[tokio::test]
async fn slow_count_can_be_invoked_as_a_task() -> anyhow::Result<()> {
    let server = Server::new();
    let client = TestClient;

    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });

    let client_service = client.serve(client_transport).await?;

    let mut args = serde_json::Map::new();
    args.insert("target".into(), serde_json::Value::from(3u8));

    let mut task_meta = serde_json::Map::new();
    task_meta.insert(
        "source".into(),
        serde_json::Value::String("integration-test".into()),
    );

    let params = CallToolRequestParams::new("slow_count")
        .with_arguments(args)
        .with_task(task_meta);

    let response = client_service
        .send_request(ClientRequest::CallToolRequest(Request::new(params)))
        .await?;

    let ServerResult::CreateTaskResult(info) = response else {
        panic!("expected CreateTaskResult, got {response:?}");
    };
    assert_eq!(info.task.status, TaskStatus::Working);

    let tasks = client_service
        .send_request(ClientRequest::ListTasksRequest(ListTasksRequest::default()))
        .await?;
    let ServerResult::ListTasksResult(listed) = tasks else {
        panic!("expected ListTasksResult, got {tasks:?}");
    };
    assert!(
        listed.tasks.iter().any(|t| t.task_id == info.task.task_id),
        "expected the newly created task to appear in the task list",
    );

    // Give the task a short moment to start running, then cancel and clean up.
    tokio::time::sleep(Duration::from_millis(50)).await;
    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}

#[tokio::test]
async fn list_tasks_surfaces_completed_tasks() -> anyhow::Result<()> {
    let server = Server::new();
    let client = TestClient;

    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = client.serve(client_transport).await?;

    // Enqueue a task that finishes quickly (target=1 → ~100ms).
    let mut args = serde_json::Map::new();
    args.insert("target".into(), serde_json::Value::from(1u8));
    let mut task_meta = serde_json::Map::new();
    task_meta.insert("source".into(), serde_json::Value::String("test".into()));
    let params = CallToolRequestParams::new("slow_count")
        .with_arguments(args)
        .with_task(task_meta);
    let response = client_service
        .send_request(ClientRequest::CallToolRequest(Request::new(params)))
        .await?;
    let ServerResult::CreateTaskResult(info) = response else {
        panic!("expected CreateTaskResult, got {response:?}");
    };
    let task_id = info.task.task_id;

    // Wait for the task to finish.
    tokio::time::sleep(Duration::from_millis(400)).await;

    // The default macro-generated `list_tasks` would return an empty list
    // here. Our override merges completed tasks, so the task should be
    // visible with TaskStatus::Completed.
    let tasks = client_service
        .send_request(ClientRequest::ListTasksRequest(ListTasksRequest::default()))
        .await?;
    let ServerResult::ListTasksResult(listed) = tasks else {
        panic!("expected ListTasksResult, got {tasks:?}");
    };
    let entry = listed
        .tasks
        .iter()
        .find(|t| t.task_id == task_id)
        .expect("completed task should appear in tasks/list");
    assert_eq!(entry.status, TaskStatus::Completed);

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}
