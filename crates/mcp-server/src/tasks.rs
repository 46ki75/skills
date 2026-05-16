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
        let status = if result.result.is_ok() {
            TaskStatus::Completed
        } else {
            TaskStatus::Failed
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
