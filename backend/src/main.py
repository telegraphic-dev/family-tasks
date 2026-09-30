import asyncio

from example_prompts import example_prompts
from reboot.aio.applications import Application
from reboot.aio.auth.oauth import OAuth
from reboot.aio.auth.oauth_providers import Development, OAuthProviderByEnvironment
from reboot.std.collections.ordered_map.v1.ordered_map import ordered_map_library
from servicers.family_tasks import HouseholdServicer, TaskServicer, UserServicer


async def main() -> None:
    await Application(
        title="Family Tasks",
        description="A shared task board for households, in the browser and in MCP hosts.",
        servicers=[UserServicer, HouseholdServicer, TaskServicer],
        libraries=[ordered_map_library()],
        oauth=OAuth(
            provider=OAuthProviderByEnvironment(dev=Development(), prod=None),
            allowed_origins=[],
        ),
        example_prompts=example_prompts,
    ).run()


if __name__ == "__main__":
    asyncio.run(main())
