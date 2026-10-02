# What Reboot needs before this Rust port can be real

This draft is deliberately shaped around Family Tasks rather than a toy counter.
It shows the ecosystem gaps Rust must close to be a **third supported Reboot
language**, not merely a client library.

## 1. One semantic contract, generated bindings

The Python API is the contract today. Rust needs a versioned IDL/descriptor
pipeline that generates actor references, service dispatch, request/response
codecs, MCP schemas, stable field tags, and compatibility checks. Derives alone
are not a contract. Hand-maintaining service metadata across languages is how
wire compatibility dies quietly.

**Acceptance test:** identical descriptors and golden encoded payloads for
Python, TypeScript, and Rust; additive changes survive rolling upgrades.

## 2. Durable state and effects must feel native

The port relies on `Reader`, `Writer`, and `Transaction` capability values.
Those need real runtime enforcement: reads cannot write, a writer is serialized
correctly, and multi-actor work is atomic or explicitly rejected. The runtime,
not a macro, owns retries, idempotency, state snapshots, optimistic conflicts,
and version migrations.

**Acceptance test:** crash/retry, concurrent mutation, and upgrade tests run
unchanged against all three language implementations.

## 3. Actor references, identity, and ownership

`ActorId<T>`, `UserId`, factory creation, typed cross-actor calls, ordered
collections of actor IDs, and actor lifecycle need a single cross-language
meaning. `Clone`, `Send`, and async ownership rules must be designed rather
than leaked from implementation details. This is the part that makes Rust nice
or unbearable.

**Acceptance test:** an actor created in Python can be read and mutated by Rust
and TypeScript without adapter code or an identity translation layer.

## 4. Serialization and schema evolution

Serde is easy; durable compatibility is not. Rust needs protobuf-like numeric
tags, explicit missing/null/default semantics, maps and ordered maps, unknown
field preservation where needed, and generated JSON Schema/MCP schema output.
The Python model behaviour must be specified first; otherwise “compatible”
means three subtly different things.

**Acceptance test:** a shared corpus covers every field shape, malformed input,
old state, unknown field, and nullable/default case.

## 5. Async, streaming, and cancellation

A first-class SDK needs a defined Tokio integration, cancellation propagation,
deadline handling, bounded streaming/backpressure, and a `Send` story for actor
handlers. Blocking inside a durable transaction needs a crisp rule. Async
traits and proc macros are the easy bit; deterministic operational behaviour is
the product.

**Acceptance test:** cancellation, long-running streaming, and backpressure
behave identically under language-neutral integration tests.

## 6. Auth context and authorization

Family Tasks' security boundary is household membership. Rust handlers must
receive verified OAuth identity and claims through the same context as Python,
and authorization must be composable and auditable at actor/method level. No
thread-local identity, no raw JWT spelunking in business logic.

**Acceptance test:** unauthorized direct actor calls, forged client metadata,
and cross-household access fail consistently in all SDKs.

## 7. MCP and UI are runtime features, not decorators

The API needs generated tool descriptions, input schemas, annotations,
resource/stream support, errors, and UI metadata. The Rust port should expose
the same MCP surface from the same descriptor—not force every application to
re-describe tools manually. The current `#[reboot::ui]` is a marker for that
missing contract, not a proposed implementation detail.

**Acceptance test:** the MCP inspector sees equivalent tools and schemas for
all languages; the dashboard opens with the correct authenticated context.

## 8. Local developer loop, observability, and test harness

`cargo test`, local runtime, deterministic clock/IDs, actor-state inspection,
tracing, metrics, and BDD fixtures need to work without Docker archaeology.
Errors must preserve causal chains across the Rust/runtime boundary. A Rust SDK
without great local debugging is just a very efficient way to be confused.

**Acceptance test:** the existing Family Tasks BDD suite is a language-neutral
conformance suite runnable locally and in CI.

## 9. Packaging and supported-version policy

Publish the SDK, runtime bridge, macros, codegen, and test kit as a coherent
set. Define Rust MSRV, edition policy, feature flags, platform support,
semver/compatibility guarantees, and how SDK/runtime mismatches fail. Cargo
features cannot be an accidental ABI policy.

## Recommended build order

1. Specify descriptor, state, effect, identity, and wire semantics once.
2. Build language-neutral conformance fixtures from Family Tasks behaviours.
3. Implement codec + generated client/server bindings in Rust.
4. Implement durable effects, transactions, factories, and authorization.
5. Add MCP/UI projection and local emulator/debugger.
6. Make this port compile, run, and pass the same E2E suite as Python.

Anything earlier is a demo. The third supported language starts when a real app
can move between SDKs without changing its guarantees.
