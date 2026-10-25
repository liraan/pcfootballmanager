## Exception: `pcfm-debug`

ADR-003 states that only `pcfm-match` and `pcfm-gen` may declare a dependency on `pcfm-hidden`. The `pcfm-debug` crate is a **controlled exception**, and this section documents why.

### What `pcfm-debug` does

`pcfm-debug` is a **developer tool**, not a shipping component:

- It constructs a `GroundTruth` to feed the match engine for demo and health-check purposes.
- It reads `pcfm-hidden` **only** to build the input to `pcfm-match`, never to inspect latent values directly.
- It is not distributed to players; it is not part of the game binary; it is not reachable from the UI.

### Why the exception is safe

The ADR-003 rule exists to prevent **truth leakage** from Plane A to Plane C or to the UI. `pcfm-debug` does not leak: it is a test harness that feeds the engine, and it is excluded from any release artifact.

The dependency is confined to:

- `crates/pcfm-debug/src/demo.rs` — constructs demo `GroundTruth` values.
- `crates/pcfm-debug/src/check.rs` — runs the health check, which calls the public API of `pcfm-match`.

No other module in the workspace reads `pcfm-hidden` outside of `pcfm-match` and `pcfm-gen`.

### Enforcement

If a future crate needs `pcfm-hidden` for a reason other than match engine input, it must be added to this exception list **with an explicit justification** and a note in `Cargo.toml`. Silent additions are a violation.

### References

- [`crates/pcfm-debug/Cargo.toml`](../../crates/pcfm-debug/Cargo.toml) — the dependency declaration
- [`modules/00-foundations.md`](../../modules/00-foundations.md) §3 — the Three Planes doctrine
- [`crates/pcfm-debug/src/check.rs`](../../crates/pcfm-debug/src/check.rs) — the health check that uses the exception
