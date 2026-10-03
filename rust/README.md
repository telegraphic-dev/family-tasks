# Rust Family Tasks slice

This is an executable, proto-first Rust consumer of the experimental Reboot Rust
SDK. It implements one durable **Task** actor lifecycle over Reboot's Database
gRPC sidecar:

- `CreateTask`, `UpdateTask`, `CompleteTask`, and `ReopenTask` writers, with the
  existing `Untitled task` default;
- `GetTaskDetails` reader;
- generated concrete unary Tonic adapters, durable state recovery, and
  idempotent writer replay.

The protobuf schema preserves the existing task-state and task-request field
tags. `Cargo.lock` pins the exact Reboot SDK Git revision used for generation
and runtime behavior.

## Run

The service requires a reachable Reboot Database sidecar endpoint (with a
scheme) and listens on loopback by default:

```sh
REBOOT_RUST_DATABASE_ENDPOINT=http://127.0.0.1:50053 \
  cargo run --manifest-path rust/Cargo.toml --locked -p family-tasks
```

Set `REBOOT_RUST_LISTEN_ADDR` to override `127.0.0.1:50051`.

## Verify

```sh
cargo test --manifest-path rust/Cargo.toml --locked
```

The integration test starts the SDK's generated Tonic FakeDatabase, serves the
Family Tasks adapters over loopback, verifies create replay under one UUID,
and verifies a separate completion persists and is readable.

## Deliberate boundary

This is **not** the Python application's full replacement. It does not yet
provide identity validation or household authorization, factory/internal calls,
cross-actor transactions, ordered task indexes, MCP/UI projection, OAuth, or
`rbt dev run --rust`. A bearer token is transport metadata only until a Rust
auth/runtime layer validates it.
