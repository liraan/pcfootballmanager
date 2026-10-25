# ADR-004 — Panic Strategy at the FFI Boundary

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `fmr-bridge` (FFI crate), `L3-ui` (Qt UI)

## Context

The previous draft of `01-architecture.md` stated two incompatible things at once:

1. Release profile uses `panic = "abort"`.
2. Every FFI boundary must `catch_unwind` to prevent a panic from propagating into C++.

With `panic = "abort"` there is no unwinding, so `catch_unwind` cannot catch anything: the process aborts. A panic in any Rust code that is reached from a Qt callback would terminate the application. This ADR resolves the contradiction.

## Options considered

### Option A — `panic = "abort"` everywhere
- **Pro:** smaller binary; fail-fast.
- **Con:** `catch_unwind` is impossible; a panic in match simulation, market calculation, or any Qt-called Rust code kills the user session with no recovery.

### Option B — `panic = "unwind"` everywhere, `catch_unwind` mandatory at FFI
- **Pro:** panic is catchable; a failure in one calculation does not kill the process; the user can save and continue.
- **Con:** slight performance and binary-size cost; requires discipline at every FFI function.

### Option C — `panic = "unwind"` in dev, `panic = "abort"` in release
- **Pro:** simple.
- **Con:** the release profile — the one users run — has no recovery. Rejected.

### Option D — `panic = "unwind"` in `dev` and `release`, `panic = "abort"` in an optional `release-hardened` profile
- **Pro:** safe default; fail-fast available to those who prefer it; clear separation.
- **Con:** none meaningful.

## Decision

**Option D.**

### Build profiles

| Profile | `panic` | Purpose |
|---|---|---|
| `dev` | `unwind` | Development, backtraces, `catch_unwind` works |
| `release` | `unwind` | Default distribution; `catch_unwind` works at FFI |
| `release-hardened` | `abort` | Optional build for users who prefer fail-fast |

### FFI rules

Every Rust function exposed to C++ through `fmr-bridge` must:

1. Be wrapped in `std::panic::catch_unwind`.
2. Convert a caught panic into a C-compatible error code.
3. Log the panic (with `tracing` or equivalent) before returning.
4. Never let a panic cross the C++ boundary.

### Canonical pattern

```rust
// fmr-bridge — every function exposed to C++
#[no_mangle]
pub extern "C" fn fmr_query_player(id: u64, out: *mut PlayerDto) -> i32 {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // ... logic ...
        Ok(())
    }));

    match result {
        Ok(Ok(())) => 0,
        Ok(Err(e)) => { log_error(e); -1 }
        Err(_) => { log_panic(); -2 }   // Panic caught, not propagated
    }
}
