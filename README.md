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

For a local demo, use the included HTTPS proxy. Reboot's browser OAuth flow uses `Secure` cookies; Safari will not send those cookies over plain `http://localhost`. The proxy terminates locally trusted TLS, redirects `/` to the Family Tasks SPA, and keeps Reboot's MCP guide available behind its normal paths.

```sh
brew install mkcert
mkcert -install
mkdir -p .local-certs
mkcert -cert-file .local-certs/localhost.pem \
  -key-file .local-certs/localhost-key.pem localhost
printf 'REBOOT_CRYPTO_ROOT_KEYS=v1:%s\n' "$(openssl rand -base64 48 | tr -d '\n')" > .env.local
docker compose --env-file .env.local -f compose.local.yml up
```

Open `https://localhost:9991`: it redirects to the Family Tasks browser app, and the Development OAuth picker works in Safari as well as Chromium. Keep `.env.local` and the `family-tasks-data` volume when reusing the demo; changing the root key makes encrypted state unreadable. `RBT_DEV=true` selects Reboot's development runtime and fake OAuth provider, so this compose file is local-only. Production OAuth is deliberately unconfigured and fails closed until a real provider is configured.

## Current scope

- Parent-created households and member invitations
- Shared tasks with title, notes, due date, assignee, and open/completed state
- Browser dashboard and MCP task tools/UIs sharing the same Reboot state

Production identity deliberately remains unconfigured (`prod=None`); choose and configure the real OAuth provider before deployment.
