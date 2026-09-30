"""Browser-backed Development OAuth BDD for the family task SPA.

This runs the app from Vite's own origin, drives the Development OAuth picker,
and then creates real household/task state through the browser. It covers the
same cross-origin session, CORS, and OAuth callback path used by the deployed
web app rather than minting an in-process test identity.
"""

from collections.abc import Iterator
from typing import Any

import pytest
from reboot.aio.applications import Application
from reboot.aio.tests import Reboot
from reboot.aio.auth.oauth import OAuth
from reboot.aio.auth.oauth_providers import Development, OAuthProviderByEnvironment
from reboot.bdd import scenarios
from reboot.bdd.loop import EventLoopThread
from reboot.bdd.frontend import Frontend
from reboot.bdd.vite import vite
from reboot.std.collections.ordered_map.v1.ordered_map import ordered_map_library
from servicers.family_tasks import HouseholdServicer, TaskServicer, UserServicer


class FamilyTaskReboot(Reboot):
    """Use a deterministic public Envoy port on shared CI runners."""

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
def frontend(monkeypatch: pytest.MonkeyPatch) -> Iterator[Frontend]:
    # The test Envoy deliberately uses 29990, not Reboot's default 9991.
    # Vite gives an explicit environment value precedence over `.env`.
    monkeypatch.setenv("VITE_REBOOT_URL", "http://127.0.0.1:29990")
    with vite(directory="frontend") as running_frontend:
        yield running_frontend


@pytest.fixture
def application(frontend: Frontend) -> Application:
    assert frontend.origin is not None
    development = Development()
    return Application(
        servicers=[UserServicer, HouseholdServicer, TaskServicer],
        libraries=[ordered_map_library()],
        oauth=OAuth(
            provider=OAuthProviderByEnvironment(
                dev=development,
                prod=development,
            ),
            allowed_origins=[frontend.origin],
        ),
    )


scenarios("family_task_management_web.feature")
