//! A concrete, proto-first Rust Family Tasks lifecycle slice.
//!
//! This crate intentionally proves one durable actor boundary only: create,
//! complete, and read a task through generated unary Tonic adapters backed by
//! Reboot's Database sidecar. Household membership, OAuth, MCP/UI projection,
//! ordered indexes, and cross-actor transactions remain outside this slice.

pub mod proto {
    tonic::include_proto!("family_tasks.v1");
}

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/family_tasks/v1/task.reboot.rs"));
}

use reboot::runtime::DatabaseActorStore;

const DEFAULT_TASK_TITLE: &str = "Untitled task";
const OPEN: &str = "open";
const COMPLETED: &str = "completed";

#[derive(Clone, Copy, Default)]
pub struct TaskHandler;

#[tonic::async_trait]
impl generated::TaskWritesDatabaseHandler for TaskHandler {
    async fn create_task(
        &self,
        state: &mut proto::TaskState,
        request: proto::CreateTaskRequest,
    ) -> Result<proto::CreateTaskResponse, tonic::Status> {
        state.household_id = request.household_id;
        state.creator_id = request.creator_id;
        state.title = if request.title.trim().is_empty() {
            DEFAULT_TASK_TITLE.to_owned()
        } else {
            request.title
        };
        state.notes = request.notes;
        state.due_date = request.due_date;
        state.assignee_id = request.assignee_id;
        state.status = OPEN.to_owned();
        Ok(proto::CreateTaskResponse {})
    }

    async fn complete_task(
        &self,
        state: &mut proto::TaskState,
        _: proto::CompleteTaskRequest,
    ) -> Result<proto::CompleteTaskResponse, tonic::Status> {
        state.status = COMPLETED.to_owned();
        Ok(proto::CompleteTaskResponse {})
    }
}

#[tonic::async_trait]
impl generated::TaskReadsDatabaseHandler for TaskHandler {
    async fn get_task_details(
        &self,
        state: &proto::TaskState,
        _: proto::GetTaskDetailsRequest,
    ) -> Result<proto::TaskDetailsResponse, tonic::Status> {
        Ok(proto::TaskDetailsResponse {
            household_id: state.household_id.clone(),
            title: state.title.clone(),
            notes: state.notes.clone(),
            due_date: state.due_date.clone(),
            assignee_id: state.assignee_id.clone(),
            status: state.status.clone(),
        })
    }
}

pub async fn task_adapters(
    database_endpoint: &str,
) -> Result<
    (
        generated::TaskWritesDatabaseAdapter<TaskHandler>,
        generated::TaskReadsDatabaseAdapter<TaskHandler>,
    ),
    tonic::transport::Error,
> {
    let store = DatabaseActorStore::connect(database_endpoint).await?;
    Ok((
        generated::TaskWritesDatabaseAdapter::new(store.clone(), TaskHandler),
        generated::TaskReadsDatabaseAdapter::new(store, TaskHandler),
    ))
}
