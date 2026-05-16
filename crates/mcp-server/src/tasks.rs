//! `tasks/list` override for [`Server`].
//!
//! The default `#[task_handler]`-generated `list_tasks` returns only
//! *running* tasks. This module's [`list_tasks`] merges running and
//! recently-completed tasks so callers can observe the full lifecycle.

use rmcp::{
    ErrorData as McpError,
    model::{ListTasksResult, Task, TaskStatus},
    task_manager::current_timestamp,
};

use crate::Server;

/// Body of the `tasks/list` override. Merges currently-running tasks and
/// recently-completed task results from the [`OperationProcessor`].
///
/// # Caveats
///
/// `OperationProcessor` does not expose per-task `created_at` /
/// `last_updated_at` timestamps in rmcp 1.7, so both fields are set to the
/// time `list_tasks` was called. A task that has been running for several
/// minutes will appear to have just been created. Switch to the upstream
/// fields once they become public.
///
/// [`OperationProcessor`]: rmcp::task_manager::OperationProcessor
pub async fn list_tasks(server: &Server) -> Result<ListTasksResult, McpError> {
    let mut processor = server.processor.lock().await;
    let now = current_timestamp();

    let mut tasks: Vec<Task> = processor
        .list_running()
        .into_iter()
        .map(|task_id| Task::new(task_id, TaskStatus::Working, now.clone(), now.clone()))
        .collect();

    for result in processor.peek_completed() {
        // `OperationProcessor::cancel_task` records cancellations as
        // `Err(TaskError("Operation cancelled"))` and timeouts as
        // `Err(TaskError("Operation timed out"))`. `TaskResult` exposes no
        // structured discriminator in rmcp 1.7, so we string-match on the
        // rendered error to distinguish cancellation from other failures.
        // Timeouts intentionally fall through to `Failed`.
        let status = match &result.result {
            Ok(_) => TaskStatus::Completed,
            Err(e) if e.to_string().to_lowercase().contains("cancelled") => TaskStatus::Cancelled,
            Err(_) => TaskStatus::Failed,
        };
        tasks.push(Task::new(
            result.descriptor.operation_id.clone(),
            status,
            now.clone(),
            now.clone(),
        ));
    }

    Ok(ListTasksResult::new(tasks))
}
