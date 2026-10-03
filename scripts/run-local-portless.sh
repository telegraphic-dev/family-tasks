#!/usr/bin/env bash
# Start Family Tasks in Docker and expose it through Portless HTTPS.
set -euo pipefail

app_name="${FAMILY_TASKS_PORTLESS_NAME:-family-tasks}"
image="${FAMILY_TASKS_IMAGE:-ghcr.io/telegraphic-dev/family-tasks:latest}"
container_name="${FAMILY_TASKS_CONTAINER:-family-tasks-local}"
host_port="${FAMILY_TASKS_PORT:-9991}"
state_volume="${FAMILY_TASKS_STATE_VOLUME:-family-tasks-data}"
state_home="${XDG_STATE_HOME:-$HOME/.local/state}/family-tasks"
env_file="$state_home/local.env"
project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

mise_exec() {
  # Keep unrelated global tool declarations out of this locked project run.
  MISE_GLOBAL_CONFIG_FILE="$project_root/.mise-launcher-global.toml" \
    mise -C "$project_root" -E portless exec --locked -- "$@"
}

require() {
  command -v "$1" >/dev/null 2>&1 || {
    printf 'Missing required command: %s\n' "$1" >&2
    exit 1
  }
}

open_url() {
  if command -v open >/dev/null 2>&1; then
    open "$1" >/dev/null 2>&1 &
  elif command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$1" >/dev/null 2>&1 &
  else
    printf 'Could not open a browser automatically; open %s\n' "$1" >&2
  fi
}

require docker
require mise
require openssl
require curl

node_version="$(mise_exec node --version)"
node_major="${node_version#v}"
node_major="${node_major%%.*}"
if [[ ! "$node_major" =~ ^[0-9]+$ ]] || (( node_major < 24 )); then
  printf 'Portless requires Node.js 24 or newer (found %s).\n' "$node_version" >&2
  exit 1
fi

docker info >/dev/null

mkdir -p "$state_home"
if [[ ! -f "$env_file" ]]; then
  umask 077
  printf 'REBOOT_CRYPTO_ROOT_KEYS=v1:%s\n' "$(openssl rand -base64 48 | tr -d '\n')" > "$env_file"
  printf 'Created persistent local Reboot root key: %s\n' "$env_file"
fi

printf 'Pulling %s\n' "$image"
docker pull "$image"

docker rm --force "$container_name" >/dev/null 2>&1 || true
docker run --detach \
  --name "$container_name" \
  --publish "127.0.0.1:${host_port}:9991" \
  --volume "${state_volume}:/data" \
  --env-file "$env_file" \
  --env RBT_DEV=true \
  "$image" >/dev/null

for ((attempt = 0; attempt < 45; attempt++)); do
  if curl --fail --silent --max-time 2 \
    --header 'Accept: text/html' "http://127.0.0.1:${host_port}/" >/dev/null; then
    break
  fi
  sleep 1
done

curl --fail --silent --show-error --max-time 2 \
  --header 'Accept: text/html' "http://127.0.0.1:${host_port}/" >/dev/null

# Portless creates/trusts its local CA on first use and binds loopback HTTPS.
# Pin the proxy to the standard HTTPS port so the browser URL has no port suffix.
mise_exec portless proxy start --https --port 443
mise_exec portless alias "$app_name" "$host_port" --force
proxy_url="$(mise_exec portless get "$app_name" --no-worktree)"
if [[ ! "$proxy_url" =~ ^https://[^/:]+$ ]]; then
  printf 'Portless did not return an HTTPS hostname for %s: %s\n' \
    "$app_name" "$proxy_url" >&2
  exit 1
fi

app_url="$proxy_url"
printf '\nFamily Tasks is running at:\n  %s\n' "$app_url"
printf 'Container: %s  |  State volume: %s\n' "$container_name" "$state_volume"
open_url "$app_url"
