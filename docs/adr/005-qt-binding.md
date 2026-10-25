# ADR-005 — Qt Binding

- **Status:** proposed
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `L3-ui` (UI implementation), `fmr-bridge` (FFI crate), `L0.04` (if UI affects match display)

## Context

The UI requires Qt (QML for skin-friendly screens, QWidgets/model-view for data grids). Two mature Rust-Qt bindings exist: `cxx-qt` (KDAB) and `qmetaobject` (woboq). The choice determines the shape of `fmr-bridge` and `L3-ui`, and affects the entire FFI strategy (ADR-004).

## Options considered

### Option A — `cxx-qt` (KDAB)

- **Pro:**
  - Maintained by KDAB (Qt specialists), active releases (`0.9.1`, `0.10.0` in 2026)[reference:0].
  - Supports **QWidgets** (limited) and **QML** — both needed by PCFM[reference:1].
  - Bidirectional Rust ⇄ C++ bindings, safe Rust, integrates with CMake and Cargo[reference:2].
  - Tested on Linux, Windows, macOS, WebAssembly[reference:3].
- **Con:**
  - "Early development, API changes frequently" (README)[reference:4].
  - QWidgets support is "limited" — may require raw C++ for complex data grids[reference:5].
  - Build complexity: `cxx-qt-build` generates C++ code, adds build time.

### Option B — `qmetaobject` (woboq)

- **Pro:**
  - Mature, stable API (`0.2.10` since 2026)[reference:6].
  - Simpler build: no C++ generation, pure Rust macros[reference:7].
  - Good QML support.
- **Con:**
  - **No QWidgets support** at all[reference:8]. PCFM needs QWidgets for data grids (model/view).
  - Requires Qt ≥ 5.8, but QWidgets absent means it cannot cover the full UI surface.
  - Less active maintenance than cxx-qt (single maintainer, 615 stars vs 941)[reference:9][reference:10].

### Option C — Hybrid: `cxx-qt` for QWidgets, `qmetaobject` for QML

- **Pro:** best of both.
- **Con:** two FFI layers, two dependency trees, double the integration cost. Rejected for v1.0.

## Decision

**Option A — `cxx-qt`.**

`cxx-qt` is the only binding that covers both QML (skins) and QWidgets (data grids), which are both hard requirements. The "early development" risk is mitigated by pinning the version and isolating the dependency in `fmr-bridge` (ADR-004 already requires a thin FFI boundary).

### What this means for the architecture

- `fmr-bridge` is the **only** crate that depends on `cxx-qt`.
- `fmr-ui` depends on `fmr-bridge`, not on `cxx-qt` directly.
- `L3-ui` uses QML for skin-friendly screens, QWidgets for data-heavy tables.
- The FFI boundary follows ADR-004: `catch_unwind`, no panic propagation, contract tests.

### Version policy

- Pin the `cxx-qt` version in `Cargo.toml`. Do not use `*`.
- Upgrade deliberately, not incidentally. An upgrade may change the generated C++ ABI.
- Every upgrade requires: full test suite, golden match test, UI smoke test.

## Consequences

### Easier
- One binding covers both QML and QWidgets.
- KDAB maintenance means the project tracks Qt releases.
- The FFI boundary is already mandated thin by ADR-004, so `cxx-qt` fits naturally.

### Harder
- `cxx-qt` API is unstable; upgrades may require code changes.
- QWidgets support is "limited"; some complex model/view features may need raw C++ shims.
- Build times increase (C++ code generation).

### Locked in
- `cxx-qt` is the Qt binding.
- `fmr-bridge` is the only crate that imports it.
- `qmetaobject` is not used (unless a future ADR supersedes this).

## Implementation notes

1. Ratify this ADR.
2. Add `cxx-qt` as a dependency of `fmr-bridge` only.
3. Write a minimal `fmr-bridge` example: a QObject with one signal, one slot.
4. Verify the FFI boundary with ADR-004 rules (`catch_unwind`, no panic across FFI).
5. Add `cxx-qt` to `deny.toml` allow-list with a pinned version.
6. Write a `L3-ui` smoke test: one QML screen, one QWidgets table.

## References

- `cxx-qt` repository: https://github.com/KDAB/cxx-qt
- CXX-Qt Book: https://kdab.github.io/cxx-qt/book/
- Comparison table (KDAB README)[reference:11]
- ADR-004 — panic strategy at FFI boundary
- ADR-003 — Plane A enforcement (no latent types in `fmr-bridge`)
