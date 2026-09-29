from reboot.application import ExamplePrompt

example_prompts = [
    ExamplePrompt(
        title="Start our family board",
        prompts=[
            "Create a household called Home Team, add a task to take out the recycling tomorrow, and show me the household board.",
        ],
    ),
    ExamplePrompt(
        title="Plan today's chores",
        prompts=[
            "Show my family task dashboard, then add the chores I mention to the right household.",
            "Assign the laundry task to me and show me the board again.",
        ],
    ),
    ExamplePrompt(
        title="Finish a task",
        prompts=[
            "List my household tasks and mark the recycling task complete.",
            "Show me the completed task.",
        ],
    ),
]
