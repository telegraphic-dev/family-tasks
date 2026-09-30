"""In-process BDD scenarios for family task management.

The Reboot test harness mints authenticated test identities directly. Browser
sign-in is exercised separately by the web application; this scenario verifies
the household domain flow through Reboot's in-process test harness.
"""

from collections.abc import Iterator
from typing import Any

import pytest
from reboot.aio.applications import Application
from reboot.aio.tests import Reboot
from reboot.bdd import scenarios
from reboot.bdd.loop import EventLoopThread
from reboot.std.collections.ordered_map.v1.ordered_map import ordered_map_library
from servicers.family_tasks import HouseholdServicer, TaskServicer, UserServicer


class FamilyTaskReboot(Reboot):
    """Avoid Docker's random-port collision on shared CI hosts."""

    async def up(self, application: Application, **kwargs: Any) -> Any:  # type: ignore[override]
        return await super().up(application, local_envoy_port=29990, **kwargs)


@pytest.fixture
def rbt(reboot_event_loop: EventLoopThread) -> Iterator[Reboot]:
    harness = FamilyTaskReboot()
    reboot_event_loop.run(harness.start())
    try:
        yield harness
    finally:
        reboot_event_loop.run(harness.stop())


@pytest.fixture
def application() -> Application:
    return Application(
        servicers=[UserServicer, HouseholdServicer, TaskServicer],
        libraries=[ordered_map_library()],
    )


scenarios("family_task_management.feature")
