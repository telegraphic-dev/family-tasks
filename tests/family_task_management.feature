@wip
Feature: A family can coordinate household tasks
  A parent creates a household and adds a shared task. The same person
  can then read the household board and see that task.

  Rule: Household tasks are visible to household members
    A task belongs to one household, and a member sees it on that household's board.

    Scenario: A parent creates a household and adds a task
      Given the application is up
      And "alice" is an authenticated user
      When "alice" does a `create_household` with `name="Home Team"` on `User` of "alice"
      And the resulting `household_id` is saved as "household id"
      And "alice" does an `add_task` with `title="Empty the dishwasher"` on `Household` of "<household id>"
      Then as "alice", `board` on the `Household` for "<household id>" has `name="Home Team"` and `tasks` of length `1` and `tasks[0].title="Empty the dishwasher"`
