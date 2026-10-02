# Draft Rust rewrite

This is the Family Tasks backend rewritten for a **hypothetical first-class
Rust Reboot SDK**. It is intentionally not buildable today: the `reboot` Rust
runtime, its derives, and generated actor bindings do not exist.

The application code is nevertheless intended to be the actual target DX:

- `serde` structs plus stable `#[reboot(tag = N)]` tags replace Pydantic/Zod
  definitions.
- `#[reboot::service]` defines durable actor methods, their effect kind, MCP
  exposure, factory status, and UI metadata.
- `Reader`, `Writer`, and `Transaction` capability values make illegal writes
  from reads and non-atomic cross-actor work unrepresentable in the public API.
- Actor implementation code keeps the current application’s household
  membership and authorization rules intact.

`src/lib.rs` carries the complete API, service declarations, and implementation
sketch. `src/main.rs` shows intended application assembly.

Before this becomes executable, Reboot needs descriptor/code generation,
state-protocol and retry support, ordered-map bindings, OAuth context
propagation, and a cross-language conformance suite that runs the existing BDD
scenarios against both Python and Rust implementations.
