from uuid import uuid4

from family_tasks.v1.family_tasks import HouseholdSummary, TaskSummary
from family_tasks.v1.family_tasks_rbt import Household, Task, User
from reboot.aio.contexts import ReaderContext, TransactionContext, WriterContext
from reboot.std.collections.ordered_map.v1.ordered_map import OrderedMap


class UserServicer(User.Servicer):
    async def create(self, context: TransactionContext) -> None:
        if context.constructor:
            self.state.household_ids = []

    async def list_households(
        self, context: ReaderContext
    ) -> User.ListHouseholdsResponse:
        households = []
        for household_id in self.state.household_ids:
            board = await Household.ref(household_id).board(context)
            households.append(
                HouseholdSummary(household_id=household_id, name=board.name)
            )
        return User.ListHouseholdsResponse(households=households)

    async def create_household(
        self, context: TransactionContext, request: User.CreateHouseholdRequest
    ) -> User.CreateHouseholdResponse:
        owner_id = (
            context.auth.user_id
            if context.auth is not None and context.auth.user_id is not None
            else context.state_id
        )
        household, _ = await Household.create(
            context,
            str(uuid4()),
            name=request.name.strip() or "Our household",
            owner_id=owner_id,
        )
        self.state.household_ids.append(household.state_id)
        return User.CreateHouseholdResponse(household_id=household.state_id)

    async def add_household(
        self, context: WriterContext, request: User.AddHouseholdRequest
    ) -> None:
        if request.household_id not in self.state.household_ids:
            self.state.household_ids.append(request.household_id)


class HouseholdServicer(Household.Servicer):
    async def create(
        self, context: WriterContext, request: Household.CreateRequest
    ) -> None:
        if context.constructor:
            self.state.name = request.name
            self.state.owner_id = request.owner_id
            self.state.member_ids = [request.owner_id]
            self.state.task_index_id = str(uuid4())

    async def invite_member(
        self, context: TransactionContext, request: Household.InviteMemberRequest
    ) -> None:
        if context.auth is None or context.auth.user_id != self.state.owner_id:
            raise PermissionError("Only the household parent may invite members.")
        if request.user_id and request.user_id not in self.state.member_ids:
            self.state.member_ids.append(request.user_id)
            await User.ref(request.user_id).add_household(
                context, household_id=self.ref().state_id
            )

    async def add_task(
        self, context: TransactionContext, request: Household.AddTaskRequest
    ) -> Household.AddTaskResponse:
        caller_id = context.auth.user_id if context.auth else ""
        if caller_id not in self.state.member_ids:
            raise PermissionError("Only household members may add tasks.")
        task, _ = await Task.create(
            context,
            str(uuid4()),
            household_id=self.ref().state_id,
            creator_id=caller_id or "",
            title=request.title.strip() or "Untitled task",
            notes=request.notes,
            due_date=request.due_date,
            assignee_id=request.assignee_id,
        )
        await OrderedMap.ref(self.state.task_index_id).insert(
            context, key=str(uuid4()), bytes=task.state_id.encode()
        )
        return Household.AddTaskResponse(task_id=task.state_id)

    async def board(
        self, context: ReaderContext
    ) -> Household.BoardResponse:
        caller_id = context.auth.user_id if context.auth else ""
        if caller_id not in self.state.member_ids:
            raise PermissionError("Only household members may view this board.")
        page = await OrderedMap.ref(self.state.task_index_id).reverse_range(
            context, limit=100
        )
        tasks = []
        for entry in page.entries:
            task_id = entry.bytes.decode()
            details = await Task.ref(task_id).details(context)
            tasks.append(
                TaskSummary(
                    task_id=task_id,
                    title=details.title,
                    status=details.status,
                    assignee_id=details.assignee_id,
                    due_date=details.due_date,
                )
            )
        return Household.BoardResponse(
            name=self.state.name,
            member_ids=self.state.member_ids,
            tasks=tasks,
        )

    async def is_member(
        self, context: ReaderContext
    ) -> Household.IsMemberResponse:
        return Household.IsMemberResponse(
            member=bool(context.auth and context.auth.user_id in self.state.member_ids)
        )


class TaskServicer(Task.Servicer):
    async def _ensure_member(self, context: ReaderContext | WriterContext) -> None:
        membership = await Household.ref(self.state.household_id).is_member(context)
        if not membership.member:
            raise PermissionError("Only household members may access this task.")

    async def create(
        self, context: WriterContext, request: Task.CreateRequest
    ) -> None:
        if context.constructor:
            self.state.household_id = request.household_id
            self.state.creator_id = request.creator_id
            self.state.title = request.title
            self.state.notes = request.notes
            self.state.due_date = request.due_date
            self.state.assignee_id = request.assignee_id
            self.state.status = "open"

    async def details(self, context: ReaderContext) -> Task.DetailsResponse:
        await self._ensure_member(context)
        return Task.DetailsResponse(
            household_id=self.state.household_id,
            title=self.state.title,
            notes=self.state.notes,
            due_date=self.state.due_date,
            assignee_id=self.state.assignee_id,
            status=self.state.status,
        )

    async def update(
        self, context: WriterContext, request: Task.UpdateRequest
    ) -> None:
        await self._ensure_member(context)
        if request.title:
            self.state.title = request.title
        if request.notes:
            self.state.notes = request.notes
        if request.due_date:
            self.state.due_date = request.due_date
        self.state.assignee_id = request.assignee_id

    async def complete(self, context: WriterContext) -> None:
        await self._ensure_member(context)
        self.state.status = "completed"

    async def reopen(self, context: WriterContext) -> None:
        await self._ensure_member(context)
        self.state.status = "open"
