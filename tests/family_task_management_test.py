"""Browser-backed BDD scenarios for family task management.

Authenticated `User` actors are created by Reboot's development OAuth flow,
so these scenarios need the same Vite frontend and origin-aware OAuth setup
as Reboot's reference application.
"""

from collections.abc import Iterator

import pytest
from reboot.aio.applications import Application
from reboot.aio.auth.oauth import OAuth
from reboot.aio.auth.oauth_providers import Development, OAuthProviderByEnvironment
from reboot.bdd import scenarios
from reboot.bdd.frontend import Frontend
from reboot.bdd.vite import vite
from reboot.std.collections.ordered_map.v1.ordered_map import ordered_map_library
from servicers.family_tasks import HouseholdServicer, TaskServicer, UserServicer


@pytest.fixture
def frontend() -> Iterator[Frontend]:
    with vite(directory="frontend") as app:
        yield app


@pytest.fixture
def application(frontend: Frontend) -> Application:
    assert frontend.origin is not None
    development = Development()
    return Application(
        servicers=[UserServicer, HouseholdServicer, TaskServicer],
        libraries=[ordered_map_library()],
        oauth=OAuth(
            provider=OAuthProviderByEnvironment(dev=development, prod=development),
            allowed_origins=[frontend.origin],
        ),
    )


scenarios("family_task_management.feature")
