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

fn task_exists(state: &proto::TaskState) -> bool {
    !state.household_id.is_empty()
}

#[derive(Clone, Copy, Default)]
pub struct TaskHandler;

#[tonic::async_trait]
impl generated::TaskWritesDatabaseHandler for TaskHandler {
    async fn create_task(
        &self,
        state: &mut proto::TaskState,
        request: proto::CreateTaskRequest,
    ) -> Result<proto::CreateTaskResponse, tonic::Status> {
        if state.household_id.is_empty() {
            if request.household_id.trim().is_empty() || request.creator_id.trim().is_empty() {
                return Err(tonic::Status::invalid_argument(
                    "household_id and creator_id are required",
                ));
            }
        } else {
            return Err(tonic::Status::already_exists("task already exists"));
        }

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

    async fn update_task(
        &self,
        state: &mut proto::TaskState,
        request: proto::UpdateTaskRequest,
    ) -> Result<proto::UpdateTaskResponse, tonic::Status> {
        if !task_exists(state) {
            return Err(tonic::Status::not_found("task does not exist"));
        }
        if !request.title.is_empty() {
            state.title = request.title;
        }
        if !request.notes.is_empty() {
            state.notes = request.notes;
        }
        if !request.due_date.is_empty() {
            state.due_date = request.due_date;
        }
        state.assignee_id = request.assignee_id;
        Ok(proto::UpdateTaskResponse {})
    }

    async fn complete_task(
        &self,
        state: &mut proto::TaskState,
        _: proto::CompleteTaskRequest,
    ) -> Result<proto::CompleteTaskResponse, tonic::Status> {
        if !task_exists(state) {
            return Err(tonic::Status::not_found("task does not exist"));
        }
        state.status = COMPLETED.to_owned();
        Ok(proto::CompleteTaskResponse {})
    }

    async fn reopen_task(
        &self,
        state: &mut proto::TaskState,
        _: proto::ReopenTaskRequest,
    ) -> Result<proto::ReopenTaskResponse, tonic::Status> {
        if !task_exists(state) {
            return Err(tonic::Status::not_found("task does not exist"));
        }
        state.status = OPEN.to_owned();
        Ok(proto::ReopenTaskResponse {})
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
