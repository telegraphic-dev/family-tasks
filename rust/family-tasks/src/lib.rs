//! Family Tasks expressed as the intended public API of a future Rust Reboot SDK.
//!
//! The `reboot` crate and its derives are deliberately hypothetical. This is a
//! migration draft, not a claim that Rust Reboot support exists.

use reboot::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_HOUSEHOLD_NAME: &str = "Our household";
const DEFAULT_TASK_TITLE: &str = "Untitled task";
const OPEN: &str = "open";
const COMPLETED: &str = "completed";

// Wire schema ---------------------------------------------------------------

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct UserState {
    #[reboot(tag = 1)]
    pub household_ids: Vec<ActorId<Household>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct HouseholdSummary {
    #[reboot(tag = 1)]
    pub household_id: ActorId<Household>,
    #[reboot(tag = 2)]
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct ListHouseholdsResponse {
    #[reboot(tag = 1)]
    pub households: Vec<HouseholdSummary>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct CreateHouseholdRequest {
    #[reboot(tag = 1)]
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct CreateHouseholdResponse {
    #[reboot(tag = 1)]
    pub household_id: ActorId<Household>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct AddHouseholdRequest {
    #[reboot(tag = 1)]
    pub household_id: ActorId<Household>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct HouseholdState {
    #[reboot(tag = 1)]
    pub name: String,
    #[reboot(tag = 2)]
    pub owner_id: UserId,
    #[reboot(tag = 3)]
    pub member_ids: Vec<UserId>,
    #[reboot(tag = 4)]
    pub task_index: OrderedMap<String, ActorId<Task>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct CreateHouseholdActorRequest {
    #[reboot(tag = 1)]
    pub name: String,
    #[reboot(tag = 2)]
    pub owner_id: UserId,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct InviteMemberRequest {
    #[reboot(tag = 1)]
    pub user_id: UserId,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct AddTaskRequest {
    #[reboot(tag = 1)]
    pub title: String,
    #[reboot(tag = 2)]
    pub notes: String,
    #[reboot(tag = 3)]
    pub due_date: String,
    #[reboot(tag = 4)]
    pub assignee_id: Option<UserId>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct AddTaskResponse {
    #[reboot(tag = 1)]
    pub task_id: ActorId<Task>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct TaskSummary {
    #[reboot(tag = 1)]
    pub task_id: ActorId<Task>,
    #[reboot(tag = 2)]
    pub title: String,
    #[reboot(tag = 3)]
    pub status: String,
    #[reboot(tag = 4)]
    pub assignee_id: Option<UserId>,
    #[reboot(tag = 5)]
    pub due_date: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct BoardResponse {
    #[reboot(tag = 1)]
    pub name: String,
    #[reboot(tag = 2)]
    pub member_ids: Vec<UserId>,
    #[reboot(tag = 3)]
    pub tasks: Vec<TaskSummary>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct IsMemberResponse {
    #[reboot(tag = 1)]
    pub member: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct TaskState {
    #[reboot(tag = 1)]
    pub household_id: ActorId<Household>,
    #[reboot(tag = 2)]
    pub creator_id: UserId,
    #[reboot(tag = 3)]
    pub title: String,
    #[reboot(tag = 4)]
    pub notes: String,
    #[reboot(tag = 5)]
    pub due_date: String,
    #[reboot(tag = 6)]
    pub assignee_id: Option<UserId>,
    #[reboot(tag = 7)]
    pub status: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct CreateTaskRequest {
    #[reboot(tag = 1)]
    pub household_id: ActorId<Household>,
    #[reboot(tag = 2)]
    pub creator_id: UserId,
    #[reboot(tag = 3)]
    pub title: String,
    #[reboot(tag = 4)]
    pub notes: String,
    #[reboot(tag = 5)]
    pub due_date: String,
    #[reboot(tag = 6)]
    pub assignee_id: Option<UserId>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct UpdateTaskRequest {
    #[reboot(tag = 1)]
    pub title: Option<String>,
    #[reboot(tag = 2)]
    pub notes: Option<String>,
    #[reboot(tag = 3)]
    pub due_date: Option<String>,
    #[reboot(tag = 4)]
    pub assignee_id: Option<Option<UserId>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, reboot::Model)]
pub struct TaskDetailsResponse {
    #[reboot(tag = 1)]
    pub household_id: ActorId<Household>,
    #[reboot(tag = 2)]
    pub title: String,
    #[reboot(tag = 3)]
    pub notes: String,
    #[reboot(tag = 4)]
    pub due_date: String,
    #[reboot(tag = 5)]
    pub assignee_id: Option<UserId>,
    #[reboot(tag = 6)]
    pub status: String,
}

// Durable actor declarations ------------------------------------------------

#[reboot::service(state = UserState, description = "One signed-in person and their household memberships.")]
pub trait User {
    #[reboot::reader(mcp, response = ListHouseholdsResponse)]
    async fn list_households(&self, context: Reader<'_>) -> Result<ListHouseholdsResponse>;

    #[reboot::transaction(exclusive, mcp, response = CreateHouseholdResponse)]
    async fn create_household(
        &mut self,
        context: Transaction<'_>,
        request: CreateHouseholdRequest,
    ) -> Result<CreateHouseholdResponse>;

    #[reboot::writer(internal)]
    async fn add_household(
        &mut self,
        context: Writer<'_>,
        request: AddHouseholdRequest,
    ) -> Result<()>;

    #[reboot::ui(path = "frontend/mcp/dashboard", title = "Family task board")]
    async fn show_dashboard(&self, context: Reader<'_>) -> Result<()>;
}

#[reboot::service(state = HouseholdState, description = "One family's shared task board and membership boundary.")]
pub trait Household {
    #[reboot::transaction(exclusive, factory, internal)]
    async fn create(
        &mut self,
        context: Transaction<'_>,
        request: CreateHouseholdActorRequest,
    ) -> Result<()>;

    #[reboot::transaction(exclusive, mcp)]
    async fn invite_member(
        &mut self,
        context: Transaction<'_>,
        request: InviteMemberRequest,
    ) -> Result<()>;

    #[reboot::transaction(shared, mcp, response = AddTaskResponse)]
    async fn add_task(
        &mut self,
        context: Transaction<'_>,
        request: AddTaskRequest,
    ) -> Result<AddTaskResponse>;

    #[reboot::reader(mcp, response = BoardResponse)]
    async fn board(&self, context: Reader<'_>) -> Result<BoardResponse>;

    #[reboot::reader(internal, response = IsMemberResponse)]
    async fn is_member(&self, context: Reader<'_>) -> Result<IsMemberResponse>;

    #[reboot::ui(path = "frontend/mcp/household-board", title = "Household board")]
    async fn show_board(&self, context: Reader<'_>) -> Result<()>;
}

#[reboot::service(state = TaskState, description = "One durable household task with its own lifecycle.")]
pub trait Task {
    #[reboot::writer(factory, internal)]
    async fn create(&mut self, context: Writer<'_>, request: CreateTaskRequest) -> Result<()>;

    #[reboot::reader(mcp, response = TaskDetailsResponse)]
    async fn details(&self, context: Reader<'_>) -> Result<TaskDetailsResponse>;

    #[reboot::writer(mcp)]
    async fn update(&mut self, context: Writer<'_>, request: UpdateTaskRequest) -> Result<()>;

    #[reboot::writer(mcp)]
    async fn complete(&mut self, context: Writer<'_>) -> Result<()>;

    #[reboot::writer(mcp)]
    async fn reopen(&mut self, context: Writer<'_>) -> Result<()>;

    #[reboot::ui(path = "frontend/mcp/task", title = "Task")]
    async fn show(&self, context: Reader<'_>) -> Result<()>;
}

// Implementations -----------------------------------------------------------

fn caller(context: &impl AuthContext) -> Result<UserId> {
    context.user_id().ok_or(Error::Unauthenticated)
}

fn ensure_member(context: &impl AuthContext, household: &HouseholdState) -> Result<UserId> {
    let user_id = caller(context)?;
    household
        .member_ids
        .contains(&user_id)
        .then_some(user_id)
        .ok_or(Error::PermissionDenied)
}

pub struct UserServicer;
#[reboot::servicer(User)]
impl UserServicer {
    async fn list_households(&self, context: Reader<'_>) -> Result<ListHouseholdsResponse> {
        let mut households = Vec::new();
        for household_id in &self.state.household_ids {
            let board = HouseholdRef::from_id(household_id.clone())
                .board(&context)
                .await?;
            households.push(HouseholdSummary {
                household_id: household_id.clone(),
                name: board.name,
            });
        }
        Ok(ListHouseholdsResponse { households })
    }

    async fn create_household(
        &mut self,
        mut tx: Transaction<'_>,
        request: CreateHouseholdRequest,
    ) -> Result<CreateHouseholdResponse> {
        let owner_id = caller(&tx)?;
        let household = HouseholdRef::create(
            &mut tx,
            CreateHouseholdActorRequest {
                name: non_empty(request.name, DEFAULT_HOUSEHOLD_NAME),
                owner_id,
            },
        )
        .await?;
        self.state.household_ids.push(household.id());
        Ok(CreateHouseholdResponse {
            household_id: household.id(),
        })
    }

    async fn add_household(
        &mut self,
        _context: Writer<'_>,
        request: AddHouseholdRequest,
    ) -> Result<()> {
        if !self.state.household_ids.contains(&request.household_id) {
            self.state.household_ids.push(request.household_id);
        }
        Ok(())
    }
}

pub struct HouseholdServicer;
#[reboot::servicer(Household, authorizer = household_member_or_internal)]
impl HouseholdServicer {
    async fn create(
        &mut self,
        mut tx: Transaction<'_>,
        request: CreateHouseholdActorRequest,
    ) -> Result<()> {
        tx.require_internal()?;
        self.state.name = request.name;
        self.state.owner_id = request.owner_id.clone();
        self.state.member_ids = vec![request.owner_id];
        self.state.task_index = OrderedMap::create(&mut tx).await?;
        Ok(())
    }

    async fn invite_member(
        &mut self,
        mut tx: Transaction<'_>,
        request: InviteMemberRequest,
    ) -> Result<()> {
        if caller(&tx)? != self.state.owner_id {
            return Err(Error::permission(
                "Only the household parent may invite members.",
            ));
        }
        if !self.state.member_ids.contains(&request.user_id) {
            self.state.member_ids.push(request.user_id.clone());
            UserRef::from_user_id(request.user_id)
                .add_household(
                    &mut tx,
                    AddHouseholdRequest {
                        household_id: self.ref_.id(),
                    },
                )
                .await?;
        }
        Ok(())
    }

    async fn add_task(
        &mut self,
        mut tx: Transaction<'_>,
        request: AddTaskRequest,
    ) -> Result<AddTaskResponse> {
        let creator_id = ensure_member(&tx, &self.state)?;
        if let Some(assignee) = &request.assignee_id {
            self.state
                .member_ids
                .contains(assignee)
                .then_some(())
                .ok_or_else(|| Error::validation("Assignee must belong to this household."))?;
        }
        let task = TaskRef::create(
            &mut tx,
            CreateTaskRequest {
                household_id: self.ref_.id(),
                creator_id,
                title: non_empty(request.title, DEFAULT_TASK_TITLE),
                notes: request.notes,
                due_date: request.due_date,
                assignee_id: request.assignee_id,
            },
        )
        .await?;
        self.state
            .task_index
            .insert(&mut tx, Uuid::new_v4().to_string(), task.id())
            .await?;
        Ok(AddTaskResponse { task_id: task.id() })
    }

    async fn board(&self, context: Reader<'_>) -> Result<BoardResponse> {
        ensure_member(&context, &self.state)?;
        let mut tasks = Vec::new();
        for (_, task_id) in self.state.task_index.reverse_range(&context, 100).await? {
            let details = TaskRef::from_id(task_id.clone()).details(&context).await?;
            tasks.push(TaskSummary {
                task_id,
                title: details.title,
                status: details.status,
                assignee_id: details.assignee_id,
                due_date: details.due_date,
            });
        }
        Ok(BoardResponse {
            name: self.state.name.clone(),
            member_ids: self.state.member_ids.clone(),
            tasks,
        })
    }

    async fn is_member(&self, context: Reader<'_>) -> Result<IsMemberResponse> {
        Ok(IsMemberResponse {
            member: context
                .user_id()
                .is_some_and(|user| self.state.member_ids.contains(&user)),
        })
    }
}

pub struct TaskServicer;
#[reboot::servicer(Task, authorizer = task_member_or_internal)]
impl TaskServicer {
    async fn create(&mut self, context: Writer<'_>, request: CreateTaskRequest) -> Result<()> {
        context.require_internal()?;
        self.state = TaskState {
            household_id: request.household_id,
            creator_id: request.creator_id,
            title: request.title,
            notes: request.notes,
            due_date: request.due_date,
            assignee_id: request.assignee_id,
            status: OPEN.into(),
        };
        Ok(())
    }

    async fn details(&self, context: Reader<'_>) -> Result<TaskDetailsResponse> {
        self.ensure_member(&context).await?;
        Ok(TaskDetailsResponse {
            household_id: self.state.household_id.clone(),
            title: self.state.title.clone(),
            notes: self.state.notes.clone(),
            due_date: self.state.due_date.clone(),
            assignee_id: self.state.assignee_id.clone(),
            status: self.state.status.clone(),
        })
    }

    async fn update(&mut self, context: Writer<'_>, request: UpdateTaskRequest) -> Result<()> {
        self.ensure_member(&context).await?;
        if let Some(value) = request.title {
            self.state.title = non_empty(value, DEFAULT_TASK_TITLE);
        }
        if let Some(value) = request.notes {
            self.state.notes = value;
        }
        if let Some(value) = request.due_date {
            self.state.due_date = value;
        }
        if let Some(value) = request.assignee_id {
            self.state.assignee_id = value;
        }
        Ok(())
    }

    async fn complete(&mut self, context: Writer<'_>) -> Result<()> {
        self.ensure_member(&context).await?;
        self.state.status = COMPLETED.into();
        Ok(())
    }
    async fn reopen(&mut self, context: Writer<'_>) -> Result<()> {
        self.ensure_member(&context).await?;
        self.state.status = OPEN.into();
        Ok(())
    }
}

impl TaskServicer {
    async fn ensure_member(&self, context: &impl AuthContext) -> Result<()> {
        HouseholdRef::from_id(self.state.household_id.clone())
            .require_member(context)
            .await
    }
}

fn non_empty(value: String, fallback: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        fallback.into()
    } else {
        value.into()
    }
}
