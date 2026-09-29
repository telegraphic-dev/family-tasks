import pytest
from reboot.aio.applications import Application
from reboot.bdd import scenarios
from reboot.std.collections.ordered_map.v1.ordered_map import ordered_map_library
from servicers.family_tasks import HouseholdServicer, TaskServicer, UserServicer


@pytest.fixture
def application() -> Application:
    return Application(
        servicers=[UserServicer, HouseholdServicer, TaskServicer],
        libraries=[ordered_map_library()],
    )


scenarios("family_task_management.feature")
