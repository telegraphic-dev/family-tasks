# Rust Family Tasks boundaries

The executable Rust slice intentionally covers one durable Task actor lifecycle:
`CreateTask`, `UpdateTask`, `CompleteTask`, `ReopenTask`, and
`GetTaskDetails`, backed by generated unary Tonic adapters and Reboot's
Database gRPC sidecar. It preserves idempotent create replay and rejects
invalid lifecycle writes (duplicate creation and mutations of missing tasks).

It is not a replacement for the existing Python Reboot application. The
following must exist before the complete Family Tasks service can move to Rust:

1. **Authenticated caller context and authorization.** The Python application
   validates OAuth identities and enforces household membership/ownership.
   Current Rust adapters carry caller metadata but do not validate it.
2. **Cross-actor semantics.** Creating a household, inviting a member, and
   indexing tasks require factory/internal calls and transaction semantics that
   this SDK slice deliberately does not claim.
3. **Ordered collection support.** The Python household board uses Reboot's
   ordered-map service. There is no equivalent generated Rust integration.
4. **MCP/UI projection and app lifecycle.** Generated Rust adapters are Tonic
   services; they do not project MCP tools/UIs or provide `rbt dev run --rust`.
5. **Cross-language conformance.** The proto tags here preserve the existing
   task contract, but a full migration needs scenario-level proof against the
   Python implementation and a real Database sidecar deployment.

These are product/runtime capabilities, not gaps hidden by local scaffolding.
