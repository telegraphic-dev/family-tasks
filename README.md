# Family Tasks

A shared household task manager built with [Reboot](https://reboot.dev/). It exposes one durable task model through a browser SPA and MCP tools/UIs.

## Local development

```sh
uv sync
uv run rbt generate
cd frontend && npm install && npm run dev
# separate terminal
uv run rbt dev run
```

Open `http://localhost:4444` for the browser app. The Reboot development provider offers a local account picker. The MCP setup wizard is served by the backend at `http://localhost:9991`.

## Current scope

- Parent-created households and member invitations
- Shared tasks with title, notes, due date, assignee, and open/completed state
- Browser dashboard and MCP task tools/UIs sharing the same Reboot state

Production identity deliberately remains unconfigured (`prod=None`); choose and configure the real OAuth provider before deployment.
