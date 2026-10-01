#!/usr/bin/env bash
# Exercise the Docker-only Envoy ingress against a real Reboot runtime image.
set -euo pipefail

image="${1:-family-tasks:root-spa-ingress-test}"
container_name="family-tasks-root-spa-ingress-${RANDOM}${RANDOM}"
container_id=""

cleanup() {
  local status="${1:-$?}"
  if [[ -n "$container_id" ]] && (( status != 0 )); then
    docker logs "$container_name" >&2 || true
  fi
  docker rm --force "$container_name" >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

docker run --detach \
  --name "$container_name" \
  --publish 127.0.0.1::9991 \
  --env "REBOOT_CRYPTO_ROOT_KEYS=v1:$(openssl rand -base64 48 | tr -d '\n')" \
  --env RBT_DEV=true \
  "$image" >/dev/null
container_id="$container_name"

host_port="$(docker port "$container_name" 9991/tcp | sed 's/.*://')"
origin="http://127.0.0.1:${host_port}"
root_body="$(mktemp)"
root_headers="$(mktemp)"
deep_body="$(mktemp)"
deep_headers="$(mktemp)"
trap 'status=$?; rm -f "$root_body" "$root_headers" "$deep_body" "$deep_headers"; cleanup "$status"' EXIT

for ((attempt = 0; attempt < 120; attempt++)); do
  if curl --fail --silent --max-time 2 \
    --header 'Accept: text/html' "$origin/" --output "$root_body"; then
    break
  fi
  sleep 1
done

curl --fail --silent --show-error --max-time 2 \
  --header 'Accept: text/html' --dump-header "$root_headers" \
  "$origin/" --output "$root_body"
grep --quiet '<title>Family Tasks</title>' "$root_body"
! grep --ignore-case --quiet '^location:' "$root_headers"

curl --fail --silent --show-error --max-time 2 \
  --header 'Accept: text/html' --dump-header "$deep_headers" \
  "$origin/households/demo?tab=open" --output "$deep_body"
grep --quiet '<title>Family Tasks</title>' "$deep_body"
! grep --ignore-case --quiet '^location:' "$deep_headers"

asset_path="$(python3 -c 'import re, sys; print(re.search(r"src=\"([^\"]+\.js)\"", open(sys.argv[1]).read()).group(1))' "$root_body")"
curl --fail --silent --show-error --max-time 2 \
  --header 'Accept: application/javascript' "$origin$asset_path" --output /dev/null

missing_status="$(curl --silent --show-error --max-time 2 \
  --header 'Accept: application/javascript' --output /dev/null --write-out '%{http_code}' \
  "$origin/__/frontend/web/assets/missing.js")"
test "$missing_status" = 404

post_body="$(mktemp)"
post_status="$(curl --silent --show-error --max-time 2 --request POST \
  --header 'Content-Type: application/json' --data '{}' --output "$post_body" \
  --write-out '%{http_code}' "$origin/")"
test "$post_status" != 200
! grep --quiet '<title>Family Tasks</title>' "$post_body"
rm -f "$post_body"

docker exec "$container_name" sh -lc \
  'test "$(ps -eo args | grep -c "envoy ")" -ge 2'

printf 'Root SPA ingress smoke test passed.\n'
