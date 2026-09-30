Feature: Family task management in the browser
  The deployed browser app signs a parent in through Reboot Development OAuth
  before it creates or changes their household board.

  Background:
    Given the application is up

  Scenario: A parent signs in, creates a household, and adds a task
    Given "alice" is an unauthenticated user
    When "alice" opens the web app
    And "alice" clicks the "Sign in" button in the web app
    And "alice" clicks the "Alice" link in the web app
    Then "alice" is signed in to the web app with their user id saved as "alice user id"
    When "alice" fills "Household name" in the web app with `"Home Team"`
    And "alice" clicks the "Create household" button in the web app
    Then "alice" eventually sees "Home Team" in the web app within 10 seconds
    When "alice" fills "New task" in the web app with `"Empty the dishwasher"`
    And "alice" clicks the "Add task" button in the web app
    Then "alice" eventually sees "Empty the dishwasher" in the web app within 10 seconds
