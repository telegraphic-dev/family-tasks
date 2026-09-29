from reboot.api import (
    API,
    Exclusive,
    Field,
    Methods,
    Model,
    Reader,
    Shared,
    Tool,
    Transaction,
    Type,
    UI,
    Writer,
)


class UserState(Model):
    household_ids: list[str] = Field(
        tag=1,
        default_factory=list,
        description="IDs of households this signed-in person belongs to.",
    )


class HouseholdSummary(Model):
    household_id: str = Field(tag=1, default="", description="ID of one household.")
    name: str = Field(tag=2, default="", description="The household's display name.")


class ListHouseholdsResponse(Model):
    households: list[HouseholdSummary] = Field(
        tag=1, default_factory=list, description="Every household the person belongs to."
    )


class CreateHouseholdRequest(Model):
    name: str = Field(tag=1, default="", description="The name shown to household members.")


class CreateHouseholdResponse(Model):
    household_id: str = Field(
        tag=1, default="", description="ID of the new household for later calls."
    )


class AddHouseholdRequest(Model):
    household_id: str = Field(
        tag=1, default="", description="ID of a household the user was invited to."
    )


class HouseholdState(Model):
    name: str = Field(tag=1, default="", description="The household's display name.")
    owner_id: str = Field(tag=2, default="", description="The parent who created this household.")
    member_ids: list[str] = Field(
        tag=3, default_factory=list, description="Signed-in people allowed to coordinate this household."
    )
    task_index_id: str = Field(
        tag=4, default="", description="ID of the ordered index containing this household's task IDs."
    )


class CreateHouseholdActorRequest(Model):
    name: str = Field(tag=1, default="", description="The household's display name.")
    owner_id: str = Field(tag=2, default="", description="The parent creating the household.")


class InviteMemberRequest(Model):
    user_id: str = Field(tag=1, default="", description="Signed-in user ID of the family member to invite.")


class AddTaskRequest(Model):
    title: str = Field(tag=1, default="", description="The concrete household job to do.")
    notes: str = Field(tag=2, default="", description="Optional context needed to complete the task.")
    due_date: str = Field(tag=3, default="", description="Optional ISO-8601 due date, or empty when unscheduled.")
    assignee_id: str = Field(tag=4, default="", description="Optional member ID assigned to the task.")


class AddTaskResponse(Model):
    task_id: str = Field(tag=1, default="", description="ID of the task that was created.")


class TaskSummary(Model):
    task_id: str = Field(tag=1, default="", description="ID of one task.")
    title: str = Field(tag=2, default="", description="The task's short name.")
    status: str = Field(tag=3, default="", description="Whether the task is open or completed.")
    assignee_id: str = Field(tag=4, default="", description="Member currently responsible, if any.")
    due_date: str = Field(tag=5, default="", description="Optional ISO-8601 due date.")


class BoardResponse(Model):
    name: str = Field(tag=1, default="", description="The household's display name.")
    member_ids: list[str] = Field(tag=2, default_factory=list, description="People sharing this board.")
    tasks: list[TaskSummary] = Field(tag=3, default_factory=list, description="Most recent household tasks.")


class IsMemberResponse(Model):
    member: bool = Field(tag=1, default=False, description="Whether the caller belongs to this household.")


class TaskState(Model):
    household_id: str = Field(tag=1, default="", description="Household that owns this task.")
    creator_id: str = Field(tag=2, default="", description="Member who created the task.")
    title: str = Field(tag=3, default="", description="The concrete household job to do.")
    notes: str = Field(tag=4, default="", description="Optional context needed to complete the task.")
    due_date: str = Field(tag=5, default="", description="Optional ISO-8601 due date, or empty when unscheduled.")
    assignee_id: str = Field(tag=6, default="", description="Member currently responsible, if any.")
    status: str = Field(tag=7, default="", description="Open until completed, then completed until reopened.")


class CreateTaskRequest(Model):
    household_id: str = Field(tag=1, default="", description="Household that owns the new task.")
    creator_id: str = Field(tag=2, default="", description="Member creating the task.")
    title: str = Field(tag=3, default="", description="The concrete household job to do.")
    notes: str = Field(tag=4, default="", description="Optional context needed to complete the task.")
    due_date: str = Field(tag=5, default="", description="Optional ISO-8601 due date, or empty when unscheduled.")
    assignee_id: str = Field(tag=6, default="", description="Optional member ID assigned to the task.")


class UpdateTaskRequest(Model):
    title: str = Field(tag=1, default="", description="Replacement short task name, or empty to leave unchanged.")
    notes: str = Field(tag=2, default="", description="Replacement task notes, or empty to leave unchanged.")
    due_date: str = Field(tag=3, default="", description="Replacement due date, or empty to leave unchanged.")
    assignee_id: str = Field(tag=4, default="", description="Replacement assignee, or empty to clear the assignee.")


class TaskDetailsResponse(Model):
    household_id: str = Field(tag=1, default="", description="Household that owns this task.")
    title: str = Field(tag=2, default="", description="The task's short name.")
    notes: str = Field(tag=3, default="", description="The task's longer context.")
    due_date: str = Field(tag=4, default="", description="Optional ISO-8601 due date.")
    assignee_id: str = Field(tag=5, default="", description="Member currently responsible, if any.")
    status: str = Field(tag=6, default="", description="Whether the task is open or completed.")


api = API(
    User=Type(
        state=UserState,
        description="One signed-in person and their household memberships.",
        methods=Methods(
            list_households=Reader(request=None, response=ListHouseholdsResponse, description="List households this person belongs to.", mcp=Tool()),
            create_household=Transaction(mode=Exclusive(), request=CreateHouseholdRequest, response=CreateHouseholdResponse, description="Create a household and make the caller its parent.", mcp=Tool()),
            add_household=Writer(request=AddHouseholdRequest, response=None, description="Record a household membership after an invitation.", mcp=None),
            show_dashboard=UI(request=None, path="frontend/mcp/dashboard", title="Family task board", description="Open the signed-in family's task dashboard."),
        ),
    ),
    Household=Type(
        state=HouseholdState,
        description="One family's shared task board and its membership boundary.",
        methods=Methods(
            create=Writer(request=CreateHouseholdActorRequest, response=None, factory=True, description="Create a household with its parent as the first member.", mcp=None),
            invite_member=Transaction(mode=Exclusive(), request=InviteMemberRequest, response=None, description="Invite a family member; only the parent may do this.", mcp=Tool()),
            add_task=Transaction(mode=Shared(), request=AddTaskRequest, response=AddTaskResponse, description="Create and index a household task.", mcp=Tool()),
            board=Reader(request=None, response=BoardResponse, description="Read the household board and its recent tasks.", mcp=Tool()),
            is_member=Reader(request=None, response=IsMemberResponse, description="Check whether the caller belongs to this household.", mcp=None),
            show_board=UI(request=None, path="frontend/mcp/household-board", title="Household board", description="Open a live board for this household."),
        ),
    ),
    Task=Type(
        state=TaskState,
        description="One durable household task with its own lifecycle and status.",
        methods=Methods(
            create=Writer(request=CreateTaskRequest, response=None, factory=True, description="Create a new open household task.", mcp=None),
            details=Reader(request=None, response=TaskDetailsResponse, description="Read this task's full details.", mcp=Tool()),
            update=Writer(request=UpdateTaskRequest, response=None, description="Edit task details or assignment.", mcp=Tool()),
            complete=Writer(request=None, response=None, description="Mark this task complete.", mcp=Tool()),
            reopen=Writer(request=None, response=None, description="Return a completed task to the open board.", mcp=Tool()),
            show=UI(request=None, path="frontend/mcp/task", title="Task", description="Open a live detail view for this task."),
        ),
    ),
)
