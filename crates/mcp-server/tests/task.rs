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
    ClientHandler, RoleClient, ServiceExt,
    model::{
        CallToolRequestParams, CancelTaskParams, ClientRequest, ListTasksRequest, Request,
        ServerResult, TaskStatus,
    },
    service::RunningService,
};

#[derive(Default, Clone)]
struct TestClient;

impl ClientHandler for TestClient {}

/// Poll `tasks/list` until the given task reaches `target` status, or fail
/// after `within` elapses. Replaces fixed `tokio::time::sleep` waits to keep
/// the tests robust on slow CI runners.
async fn wait_for_task_status(
    client: &RunningService<RoleClient, TestClient>,
    task_id: &str,
    target: TaskStatus,
    within: Duration,
) -> anyhow::Result<()> {
    tokio::time::timeout(within, async {
        loop {
            let tasks = client
                .send_request(ClientRequest::ListTasksRequest(ListTasksRequest::default()))
                .await?;
            let ServerResult::ListTasksResult(listed) = tasks else {
                anyhow::bail!("expected ListTasksResult, got {tasks:?}");
            };
            if listed
                .tasks
                .iter()
                .any(|t| t.task_id == task_id && t.status == target)
            {
                return anyhow::Ok(());
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .map_err(|_| anyhow::anyhow!("task {task_id} did not reach {target:?} within {within:?}"))?
}

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

    // The default macro-generated `list_tasks` would return an empty list
    // once the task finishes; our override merges completed tasks. Poll
    // until the task surfaces as `Completed` (bounded by a generous timeout
    // so the test fails fast on regressions rather than flaking on slow CI).
    wait_for_task_status(
        &client_service,
        &task_id,
        TaskStatus::Completed,
        Duration::from_secs(5),
    )
    .await?;

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}

/// Exercises the `Cancelled` branch of the `list_tasks` override in
/// `src/tasks.rs`. The override distinguishes cancellations from generic
/// failures by string-matching the underlying `Error::TaskError` message
/// (rmcp 1.7 has no structured discriminator). This test locks in that
/// behavior so a future upstream change to the error text — or an
/// accidental flip back to the always-`Failed` branch — fails loudly.
#[tokio::test]
async fn list_tasks_reports_cancelled_status_for_cancelled_task() -> anyhow::Result<()> {
    let server = Server::new();
    let client = TestClient;

    let (server_transport, client_transport) = tokio::io::duplex(4096);
    let server_handle = tokio::spawn(async move {
        let service = server.serve(server_transport).await?;
        service.waiting().await?;
        anyhow::Ok(())
    });
    let client_service = client.serve(client_transport).await?;

    // Enqueue a long-running task (target=20 → ~2s) so we have time to
    // observe `Working` and then cancel it before it can complete.
    let mut args = serde_json::Map::new();
    args.insert("target".into(), serde_json::Value::from(20u8));
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
    let task_id = info.task.task_id.clone();

    // Confirm the task is actually `Working` before we cancel — otherwise
    // a regression that never reports `Working` (e.g. dropping running
    // tasks from `list_tasks`) would slip past this test.
    wait_for_task_status(
        &client_service,
        &task_id,
        TaskStatus::Working,
        Duration::from_secs(2),
    )
    .await?;

    // `CancelTaskResult` and `GetTaskResult` share an identical wire shape
    // (`allOf[Result, Task]`), so rmcp's untagged `ServerResult` may
    // deserialize the cancel response as either variant. Accept both and
    // assert on the embedded task instead.
    let cancel = client_service
        .send_request(ClientRequest::CancelTaskRequest(Request::new(
            CancelTaskParams {
                meta: None,
                task_id: task_id.clone(),
            },
        )))
        .await?;
    let cancelled_task = match cancel {
        ServerResult::CancelTaskResult(r) => r.task,
        ServerResult::GetTaskResult(r) => r.task,
        other => panic!("expected Cancel/GetTaskResult after tasks/cancel, got {other:?}"),
    };
    assert_eq!(cancelled_task.task_id, task_id);
    assert_eq!(cancelled_task.status, TaskStatus::Cancelled);

    // The override should surface the cancelled task with
    // `TaskStatus::Cancelled` — not `Failed`.
    wait_for_task_status(
        &client_service,
        &task_id,
        TaskStatus::Cancelled,
        Duration::from_secs(2),
    )
    .await?;

    client_service.cancel().await?;
    let _ = server_handle.await;
    Ok(())
}
