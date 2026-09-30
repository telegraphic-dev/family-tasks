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

## Container image

After a merge to `main`, GitHub publishes `ghcr.io/telegraphic-dev/family-tasks:latest` and immutable `sha-<commit>` tags. The image contains the generated Reboot bindings and compiled browser/MCP frontends; it needs no source checkout or separate frontend container.

For a local demo, run the image in Reboot development mode. `RBT_DEV=true` enables Reboot's local account picker; this is intentionally a local-only setting:

```sh
export REBOOT_CRYPTO_ROOT_KEYS="v1:$(openssl rand -base64 48 | tr -d '\n')"
docker run --rm -p 9991:9991 -v family-tasks-data:/data \
  -e REBOOT_CRYPTO_ROOT_KEYS \
  -e RBT_DEV=true \
  ghcr.io/telegraphic-dev/family-tasks:latest
```

Open `http://localhost:9991/__/frontend/web/` for the Family Tasks browser app. `http://localhost:9991/` is Reboot's MCP integration guide; it is the expected root endpoint for a Reboot application. Keep the same root key when reusing `family-tasks-data`; changing it makes encrypted state unreadable. `RBT_DEV=true` selects Reboot's development runtime as well as its fake OAuth provider, so never use it for an internet-facing deployment. Without it, production OAuth is deliberately unconfigured and the image fails closed until a real provider is configured.

## Current scope

- Parent-created households and member invitations
- Shared tasks with title, notes, due date, assignee, and open/completed state
- Browser dashboard and MCP task tools/UIs sharing the same Reboot state

Production identity deliberately remains unconfigured (`prod=None`); choose and configure the real OAuth provider before deployment.
