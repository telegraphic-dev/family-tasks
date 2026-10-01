#!/bin/sh
# Public Envoy owns :9991; Reboot's own Envoy stays private on :9992.
set -eu

reboot_pid=''
envoy_pid=''

stop_children() {
  trap - INT TERM
  [ -z "$envoy_pid" ] || kill -TERM "$envoy_pid" 2>/dev/null || true
  [ -z "$reboot_pid" ] || kill -TERM "$reboot_pid" 2>/dev/null || true
  [ -z "$envoy_pid" ] || wait "$envoy_pid" 2>/dev/null || true
  [ -z "$reboot_pid" ] || wait "$reboot_pid" 2>/dev/null || true
}

on_signal() {
  stop_children
  exit 0
}

trap on_signal INT TERM

# The outer Envoy is the image's public listener. Keep Reboot's generated
# Envoy and trusted application listeners entirely inside the container.
env -u PORT -u RBT_PORT rbt serve run --port=9992 &
reboot_pid=$!

envoy -c /app/docker/envoy-root-spa.yaml --disable-hot-restart &
envoy_pid=$!

while kill -0 "$reboot_pid" 2>/dev/null && kill -0 "$envoy_pid" 2>/dev/null; do
  sleep 1
done

status=0
if ! kill -0 "$reboot_pid" 2>/dev/null; then
  wait "$reboot_pid" || status=$?
fi
if ! kill -0 "$envoy_pid" 2>/dev/null; then
  wait "$envoy_pid" || status=$?
fi
stop_children
exit "$status"
