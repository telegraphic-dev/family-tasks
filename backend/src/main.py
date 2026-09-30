import asyncio
import os

from example_prompts import example_prompts
from reboot.aio.applications import Application
from reboot.aio.auth.oauth import OAuth
from reboot.aio.auth.oauth_providers import Development, OAuthProviderByEnvironment
from reboot.std.collections.ordered_map.v1.ordered_map import ordered_map_library
from servicers.family_tasks import HouseholdServicer, TaskServicer, UserServicer


async def main() -> None:
    # A pulled image can use Reboot's fake account picker for a local demo,
    # but production remains fail-closed until a real OAuth provider is set.
    local_demo_oauth = (
        Development() if os.environ.get("REBOOT_DEVELOPMENT_OAUTH") == "1" else None
    )
    await Application(
        title="Family Tasks",
        description="A shared task board for households, in the browser and in MCP hosts.",
        servicers=[UserServicer, HouseholdServicer, TaskServicer],
        libraries=[ordered_map_library()],
        oauth=OAuth(
            provider=OAuthProviderByEnvironment(
                dev=Development(), prod=local_demo_oauth
            ),
            allowed_origins=[],
        ),
        example_prompts=example_prompts,
    ).run()


if __name__ == "__main__":
    asyncio.run(main())
